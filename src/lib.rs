mod config;
mod cookie_fetch;
mod scope;
mod state;

pub mod cookie_client;

use cookie_client::{CookieClient, CookieClientPool, RedirectPolicy};
use cookie_fetch::{FetchError, FetchOptions, Response};
use state::CookieFetchState;
use tauri::{AppHandle, Manager};
use tauri_plugin_bin_ipc::{bin_command, generate_bin_handler, PluginBuilderBinIpcExtension};

#[bin_command]
async fn fetch<R: tauri::Runtime>(
    app: AppHandle<R>,
    url: String,
    options: Option<FetchOptions>,
) -> Result<Response, FetchError> {
    let res = cookie_fetch::fetch(app, url, options).await?;
    Ok(res)
}

const PLUGIN_NAME: &str = "cookie-fetch";

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R, config::Config> {
    tauri::plugin::Builder::new(PLUGIN_NAME)
        .bin_ipc_handler(PLUGIN_NAME, generate_bin_handler![fetch])
        .setup_with_config(|app, config| {
            app.manage(CookieFetchState {
                client_pool: CookieClientPool::new(),
                config,
            });

            Ok(())
        })
        .build()
}
