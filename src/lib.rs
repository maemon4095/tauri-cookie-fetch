pub mod cookie_client;
mod cookie_fetch;

use cookie_fetch::{CookieFetchState, FetchError, FetchOptions, Response};
use tauri::{AppHandle, Manager};
use tauri_plugin_bin_ipc::{bin_command, generate_bin_handler, PluginBuilderBinIpcExtension};

#[bin_command]
async fn fetch<R: tauri::Runtime>(
    app: AppHandle<R>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, FetchError> {
    let state = app.state::<CookieFetchState>();
    let res = cookie_fetch::fetch(state, url, options).await?;
    Ok(res)
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("cookie-fetch")
        .bin_ipc_handler("cookie-fetch", generate_bin_handler![fetch])
        .setup(cookie_fetch::setup)
        .build()
}
