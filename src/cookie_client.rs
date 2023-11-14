use std::sync::{Arc, MutexGuard};
struct ClientPoolManager;
pub struct CookieClient {
    client: reqwest::Client,
    cookie_store: Arc<reqwest_cookie_store::CookieStoreMutex>,
}

impl CookieClient {
    pub fn get<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.get(url)
    }

    pub fn post<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.post(url)
    }

    pub fn put<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.put(url)
    }

    pub fn patch<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.patch(url)
    }

    pub fn delete<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.delete(url)
    }

    pub fn head<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.client.head(url)
    }

    pub fn request<U: reqwest::IntoUrl>(
        &self,
        method: reqwest::Method,
        url: U,
    ) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }
    pub fn execute(
        &self,
        request: reqwest::Request,
    ) -> impl std::future::Future<Output = Result<reqwest::Response, reqwest::Error>> {
        self.client.execute(request)
    }

    pub fn cookie_store<'a>(&'a self) -> MutexGuard<'a, reqwest_cookie_store::CookieStore> {
        self.cookie_store.lock().unwrap()
    }
}

#[async_trait::async_trait]
impl deadpool::managed::Manager for ClientPoolManager {
    type Type = CookieClient;
    type Error = ();

    async fn create(&self) -> Result<Self::Type, Self::Error> {
        let cookie_store = reqwest_cookie_store::CookieStore::new(None);
        let cookie_store = reqwest_cookie_store::CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);
        let client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .unwrap();

        Ok(CookieClient {
            client,
            cookie_store,
        })
    }

    async fn recycle(
        &self,
        value: &mut Self::Type,
        _: &deadpool::managed::Metrics,
    ) -> deadpool::managed::RecycleResult<Self::Error> {
        let mut cookie_store = value.cookie_store.lock().unwrap();
        cookie_store.clear();
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
