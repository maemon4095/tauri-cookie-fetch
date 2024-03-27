use tauri::{generate_handler, AppHandle, State};

pub mod cookie_client;
mod cookie_fetch;
mod cookie_fetch_ipc;

#[tauri::command]
async fn connect(state: State<'_, cookie_fetch_ipc::IpcState>) -> Result<usize, String> {
    cookie_fetch_ipc::connect(state).await
}

#[tauri::command]
async fn fetch<R: tauri::Runtime>(
    app_handle: AppHandle<R>,
    state: State<'_, cookie_fetch::CookieFetchState>,
    ipc_state: State<'_, cookie_fetch_ipc::IpcState>,
    url: String,
    id: usize,
    options: Option<cookie_fetch::FetchOptions>,
) -> Result<cookie_fetch::Response, cookie_fetch::Error> {
    cookie_fetch::fetch(app_handle, state, ipc_state, url, id, options).await
}

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::<R>::new("cookie_fetch")
        .setup(|app| {
            cookie_fetch_ipc::setup(app);
            cookie_fetch::setup(app);
            Ok(())
        })
        .register_uri_scheme_protocol("cookie-fetch-ipc", cookie_fetch_ipc::uri_scheme_protocol)
        .invoke_handler(generate_handler![connect, fetch])
        .build()
}
