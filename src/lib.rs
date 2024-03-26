pub mod cookie_client;
mod cookie_fetch;
mod cookie_fetch_ipc;

use tauri::plugin::{Builder, TauriPlugin};

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    let mut builder = Builder::<R>::new("cookie_fetch");
    builder = cookie_fetch_ipc::init(builder);
    builder = cookie_fetch::init(builder);
    builder.build()
}
