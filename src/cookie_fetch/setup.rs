use super::{CookieClientPool, CookieFetchState};
use tauri::{AppHandle, Manager};

pub fn setup<R: tauri::Runtime>(
    app: &AppHandle<R>,
) -> Result<(), Box<dyn std::error::Error + 'static>> {
    app.manage(CookieFetchState {
        client_pool: CookieClientPool::new(),
    });

    Ok(())
}
