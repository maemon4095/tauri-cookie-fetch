use super::{cookie_props::CookieProps, headermap::HeaderMap, method::Method, redirect::Redirect};
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchOptions {
    #[serde(default = "default_method")]
    pub method: Method,
    #[serde(default = "HeaderMap::new")]
    pub headers: HeaderMap,
    #[serde(default = "HashMap::new")]
    pub cookies: HashMap<String, HashMap<String, CookieProps>>,
    #[serde(default = "default_redirect_policy")]
    pub redirect: Redirect,
    #[serde(default = "Vec::new")]
    pub body: Vec<u8>,
}

fn default_redirect_policy() -> Redirect {
    Redirect::Follow
}

fn default_method() -> Method {
    Method::GET
}
