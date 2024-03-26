use std::sync::{Arc, Mutex, MutexGuard};

use reqwest::redirect::{self, Attempt};
struct ClientPoolManager;
pub struct CookieClient {
    client: reqwest::Client,
    cookie_store: Arc<reqwest_cookie_store::CookieStoreMutex>,
    redirect_policy: Arc<Mutex<RedirectPolicy>>,
}

pub enum RedirectPolicy {
    Follow,
    Limited(usize),
}

impl RedirectPolicy {
    pub fn follow() -> Self {
        Self::Follow
    }

    pub fn limited(max: usize) -> Self {
        Self::Limited(max)
    }

    fn check(&mut self, attempt: Attempt<'_>) -> redirect::Action {
        match self {
            RedirectPolicy::Follow => attempt.follow(),
            RedirectPolicy::Limited(n) => {
                if *n == 0 {
                    attempt.stop()
                } else {
                    *n -= 1;
                    attempt.follow()
                }
            }
        }
    }
}

fn default_redirect_policy() -> RedirectPolicy {
    RedirectPolicy::limited(10)
}

impl CookieClient {
    pub fn request<U: reqwest::IntoUrl>(
        &self,
        method: reqwest::Method,
        url: U,
    ) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }

    pub fn cookie_store<'a>(&'a self) -> MutexGuard<'a, reqwest_cookie_store::CookieStore> {
        self.cookie_store.lock().unwrap()
    }

    pub fn redirect_policy<'a>(&'a self) -> MutexGuard<'a, RedirectPolicy> {
        self.redirect_policy.lock().unwrap()
    }
}

#[async_trait::async_trait]
impl deadpool::managed::Manager for ClientPoolManager {
    type Type = CookieClient;
    type Error = ();

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let redirect_policy = default_redirect_policy();
        let redirect_policy = Mutex::new(redirect_policy);
        let redirect_policy = Arc::new(redirect_policy);

        let cookie_store = reqwest_cookie_store::CookieStore::new(None);
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = Arc::new(cookie_store);
        let client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .redirect(redirect::Policy::custom({
                let policy = redirect_policy.clone();
                move |a| policy.lock().unwrap().check(a)
            }))
            .build()
            .unwrap();

        Ok(CookieClient {
            client,
            cookie_store,
            redirect_policy,
        })
    }

    async fn recycle(
        &self,
        value: &mut Self::Type,
        _: &deadpool::managed::Metrics,
    ) -> deadpool::managed::RecycleResult<Self::Error> {
        let mut cookie_store = value.cookie_store();
        cookie_store.clear();
        let mut redirect_policy = value.redirect_policy();
        *redirect_policy = default_redirect_policy();
        Ok(())
    }
}

pub struct CookieClientPool {
    client_pool: deadpool::managed::Pool<ClientPoolManager>,
}

impl CookieClientPool {
    pub fn new() -> CookieClientPool {
        Self {
            client_pool: deadpool::managed::Pool::builder(ClientPoolManager)
                .build()
                .unwrap(),
        }
    }

    pub async fn get(
        &self,
    ) -> impl std::ops::Deref<Target = CookieClient> + std::ops::DerefMut + Drop {
        self.client_pool.get().await.unwrap()
    }
}
