mod headermap;
mod method;
mod redirect;

use crate::cookie_client::{CookieClient, CookieClientPool, RedirectPolicy};
use bytes::Bytes;
use headermap::HeaderMap;
use method::Method;
use redirect::Redirect;
use reqwest::RequestBuilder;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchOptions {
    #[serde(default = "default_method")]
    method: Method,
    #[serde(default = "HeaderMap::new")]
    headers: HeaderMap,
    #[serde(default = "HashMap::new")]
    cookies: HashMap<String, String>,
    #[serde(default = "default_redirect_policy")]
    redirect: Redirect,
    #[serde(default = "Vec::new")]
    body: Vec<u8>,
}

fn default_redirect_policy() -> Redirect {
    Redirect::Follow
}

fn default_method() -> Method {
    Method::GET
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    url: String,
    status: u16,
    headers: HeaderMap,
    cookies: HashMap<String, String>,
    body: Bytes,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Error {
    url: Option<String>,
    message: String,
}

impl Error {
    fn from(e: reqwest::Error) -> Self {
        Error {
            url: e.url().map(|e| e.to_string()),
            message: format!("{}", e),
        }
    }

    fn from_std_error(e: &dyn std::error::Error) -> Self {
        Error {
            url: None,
            message: format!("{}", e),
        }
    }
}

pub async fn fetch(
    state: State<'_, CookieFetchState>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, Error> {
    let client = state.client_pool.get().await;
    let url = match reqwest::Url::parse(&url) {
        Ok(v) => v,
        Err(e) => return Err(Error::from_std_error(&e)),
    };

    let Some(options) = options else {
        return fetch_core(&client, client.request(reqwest::Method::GET, url)).await;
    };

    {
        let mut cookies_store = client.cookie_store();
        for (name, value) in options.cookies {
            let cookie = reqwest_cookie_store::RawCookie::new(name, value);
            if let Some(e) = cookies_store.insert_raw(&cookie, &url).err() {
                return Err(Error::from_std_error(&e));
            }
        }
    }

    {
        let mut redirect_policy = client.redirect_policy();
        match options.redirect {
            Redirect::Follow => *redirect_policy = RedirectPolicy::follow(),
            Redirect::Manual => *redirect_policy = RedirectPolicy::limited(0),
            Redirect::Limit { limit } => *redirect_policy = RedirectPolicy::limited(limit),
        }
    }

    let builder = client
        .request(options.method.into(), url)
        .headers(options.headers.into())
        .body(options.body);

    return fetch_core(&client, builder).await;
}

async fn fetch_core(client: &CookieClient, request: RequestBuilder) -> Result<Response, Error> {
    let res = match request.send().await {
        Ok(v) => v,
        Err(e) => return Err(Error::from(e)),
    };

    let cookies: HashMap<String, String> = client
        .cookie_store()
        .iter_any()
        .map(|e| (e.name().to_string(), e.value().to_string()))
        .collect();

    let url = res.url().to_string();
    let status = res.status().as_u16();
    let headers = res.headers().clone().into();
    let body = match res.bytes().await {
        Ok(v) => v,
        Err(e) => return Err(Error::from(e)),
    };

    let res = Response {
        url,
        status,
        headers,
        cookies,
        body,
    };

    Ok(res)
}

pub struct CookieFetchState {
    client_pool: CookieClientPool,
}

pub fn setup<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    app.manage(CookieFetchState {
        client_pool: CookieClientPool::new(),
    });

    Ok(())
}
