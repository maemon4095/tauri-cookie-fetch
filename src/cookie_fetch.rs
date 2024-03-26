mod headermap;
mod method;
mod redirect;

use crate::{
    cookie_client::{CookieClient, CookieClientPool, RedirectPolicy},
    cookie_fetch_ipc::{IpcSession, IpcState},
};
use futures::{SinkExt, Stream, StreamExt, TryStream};
use headermap::HeaderMap;
use method::Method;
use redirect::Redirect;
use reqwest::RequestBuilder;
use std::collections::HashMap;
use tauri::{AppHandle, Manager, Runtime, State};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct FetchOptions {
    #[serde(default = "default_method")]
    method: Method,
    #[serde(default = "HeaderMap::new")]
    headers: HeaderMap,
    #[serde(default = "HashMap::new")]
    cookies: HashMap<String, String>,
    #[serde(default = "default_redirect_policy")]
    redirect: Redirect,
}

fn default_redirect_policy() -> Redirect {
    Redirect::Follow
}

fn default_method() -> Method {
    Method::GET
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    url: String,
    status: u16,
    headers: HeaderMap,
    cookies: HashMap<String, String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Error {
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

#[tauri::command]
async fn fetch<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, CookieFetchState>,
    ipc_state: State<'_, IpcState>,
    url: String,
    id: usize,
    options: Option<FetchOptions>,
) -> Result<Response, Error> {
    let session = match ipc_state.session(id) {
        Ok(e) => e,
        Err(e) => return Err(Error::from_std_error(&e)),
    };

    let client = state.client_pool.get().await;
    let url = match reqwest::Url::parse(&url) {
        Ok(v) => v,
        Err(e) => return Err(Error::from_std_error(&e)),
    };

    let Some(options) = options else {
        return fetch_core(
            app_handle,
            id,
            &client,
            session,
            client.request(reqwest::Method::GET, url),
        )
        .await;
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

    let mut builder = client.request(options.method.into(), url);

    builder = builder.headers(options.headers.into());

    return fetch_core(app_handle, id, &client, session, builder).await;
}

async fn fetch_core<R: tauri::Runtime>(
    app: AppHandle<R>,
    id: usize,
    client: &CookieClient,
    session: IpcSession,
    mut request: RequestBuilder,
) -> Result<Response, Error> {
    let body = reqwest::Body::wrap_stream(no_error_stream(session.request_receiver));
    request = request.body(body);
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
    let mut stream = res.bytes_stream();
    tauri::async_runtime::spawn({
        let mut sender = session.response_sender;
        async move {
            loop {
                let Some(result) = stream.next().await else {
                    break;
                };
                let Ok(chunk) = result else {
                    break;
                };

                let Ok(_) = sender.send(chunk).await else {
                    break;
                };

                app.emit_all("cookie-fetch-ipc:ready-to-pop", id).unwrap();
            }
            sender.close().await.unwrap();
        }
    });

    let res = Response {
        url,
        status,
        headers,
        cookies,
    };

    Ok(res)
}

fn no_error_stream<S: Stream>(
    stream: S,
) -> impl TryStream<Ok = S::Item, Error = Box<dyn std::error::Error + 'static + Send + Sync>> {
    stream.map(|s| Ok(s))
}

struct CookieFetchState {
    client_pool: CookieClientPool,
}

pub fn init<R: Runtime>(builder: tauri::plugin::Builder<R>) -> tauri::plugin::Builder<R> {
    builder
        .setup(|app| {
            app.manage(CookieFetchState {
                client_pool: CookieClientPool::new(),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch])
}

#[derive(Debug, thiserror::Error)]
enum SendingError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Connection(#[from] futures::channel::mpsc::SendError),
}
