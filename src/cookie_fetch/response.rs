use super::{CookieProps, HeaderMap};
use bytes::Bytes;
use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub url: String,
    pub status: u16,
    pub headers: HeaderMap,
    pub cookies: HashMap<String, HashMap<String, CookieProps>>,
    pub body: Bytes,
}
