use super::{CookieProps, FetchError, FetchOptions, Redirect, Response};
use crate::{CookieClient, CookieFetchState, RedirectPolicy};
use reqwest::RequestBuilder;
use std::collections::HashMap;
use tauri::{Manager, State};

// TODO: エラーをわかりやすくしたい。bin-ipcに手をいれる必要があるかも。
pub async fn fetch<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, FetchError> {
    let url = match reqwest::Url::parse(&url) {
        Ok(v) => v,
        Err(_) => return Err(FetchError::InvalidUrl),
    };

    let state: State<'_, CookieFetchState> = app.state();

    if !state.config.scope.is_allowed(&url) {
        return Err(FetchError::NotAllowed);
    }

    let client = state.client_pool.get().await;

    let Some(options) = options else {
        return fetch_core(&client, client.request(reqwest::Method::GET, url)).await;
    };

    {
        let mut cookies_store = client.cookie_store();

        let mut url_buf = reqwest::Url::parse("http://placeholder.example.com").unwrap();
        for (domain, pairs) in options.cookies {
            for (name, mut props) in pairs {
                url_buf
                    .set_host(Some(&domain))
                    .map_err(|_| FetchError::InvalidCookieDomain(domain.clone()))?;
                url_buf.set_path(&props.path);

                let mut cookie = reqwest_cookie_store::RawCookie::new(name.clone(), props.value);

                if let Some(v) = props.httponly.take() {
                    cookie.set_http_only(v);
                }

                if let Some(v) = props.secure.take() {
                    cookie.set_secure(v);
                }

                cookie.set_max_age(props.max_age.take());
                cookie.set_expires(props.expires.take());
                cookie.set_same_site(props.same_site.take());

                cookies_store
                    .insert_raw(&cookie, &url)
                    .map_err(|_| FetchError::InvalidCookie {
                        domain: domain.clone(),
                        name,
                    })?;
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

async fn fetch_core(
    client: &CookieClient,
    request: RequestBuilder,
) -> Result<Response, FetchError> {
    let res = match request.send().await {
        Ok(v) => v,
        Err(e) => return Err(FetchError::Reqwest(e)),
    };

    let cookies: HashMap<String, HashMap<String, CookieProps>> = {
        let store = client.cookie_store();
        let mut cookies: HashMap<String, _> = HashMap::new();

        for c in store.iter_any() {
            let Some(domain) = &c.domain.as_cow() else {
                continue;
            };

            let pairs: &mut HashMap<_, _> = { cookies.entry(domain.to_string()).or_default() };

            pairs.insert(
                c.name().to_string(),
                CookieProps {
                    value: c.value().to_string(),
                    path: c.path.as_ref().to_string(),
                    httponly: c.http_only(),
                    secure: c.secure(),
                    max_age: c.max_age(),
                    expires: c.expires().and_then(|e| match e {
                        cookie::Expiration::DateTime(v) => Some(v),
                        cookie::Expiration::Session => None,
                    }),
                    same_site: c.same_site(),
                },
            );
        }

        cookies
    };

    let url = res.url().to_string();
    let status = res.status().as_u16();
    let headers = res.headers().clone().into();
    let body = match res.bytes().await {
        Ok(v) => v,
        Err(e) => return Err(FetchError::Reqwest(e)),
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
