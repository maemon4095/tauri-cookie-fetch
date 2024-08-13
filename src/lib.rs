pub mod cookie_client;
mod cookie_fetch;

use cookie_fetch::{CookieFetchState, Error, FetchOptions, Response};
use tauri::{generate_handler, State};

#[tauri::command]
async fn fetch(
    state: State<'_, CookieFetchState>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, Error> {
    cookie_fetch::fetch(state, url, options).await
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("cookie_fetch")
        .setup(cookie_fetch::setup)
        .invoke_handler(generate_handler![fetch])
        .build()
}
