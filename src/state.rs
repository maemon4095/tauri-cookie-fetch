use crate::CookieClientPool;

pub struct CookieFetchState {
    pub client_pool: CookieClientPool,
    pub config: crate::config::Config,
}
