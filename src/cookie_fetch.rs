mod headermap;
mod method;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime, State,
};

use crate::cookie_client::CookieClientPool;

#[derive(Debug, serde::Deserialize)]
struct FetchOptions {
    method: method::Method,
    headers: Option<headermap::HeaderMap>,
    body: Option<tauri::api::http::Body>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Response {}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum Error {}

#[tauri::command(rename_all = "snake_case")]
async fn fetch(
    state: State<'_, CookieFetchState>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, Error> {
    let client = state.pool.get().await;

    let Some(options) = options else {
        let result = client.get(url).send().await;
        todo!();
    };

    let builder = client.request(options.method.into(), url);

    let builder = if let Some(headers) = options.headers {
        builder.headers(headers.into())
    } else {
        builder
    };

    let builder = if let Some(body) = options.body {
        builder.body(match body {
            tauri::api::http::Body::Form(_) => todo!(),
            tauri::api::http::Body::Json(_) => todo!(),
            _ => todo!(),
        })
    } else {
        builder
    };

    todo!()
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("cookie_fetch")
        .setup(|app| {
            app.manage(CookieFetchState {
                pool: CookieClientPool::new(),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch])
        .build()
}

struct CookieFetchState {
    pool: CookieClientPool,
}
