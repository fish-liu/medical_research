// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use qianfan::QianFanClient;
use std::sync::Arc;
use tauri::{tray::TrayIconBuilder, Manager};
use tauri_plugin_store::{Store, StoreExt};

mod qianfan;
mod cmd;
mod utils;

pub const AS_VALUE: &str = "search_ALTAKioNowgHFo1gEWYT4QqRZW_2676be91677d4532be42eeea7328fe66";

#[warn(dead_code)]
struct QianFanHandler{
    qianfan_client: QianFanClient
}

#[allow(unused)]
struct Storage<R: tauri::Runtime>{
    store: Arc<Store<R>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            cmd::greet,
            cmd::get_chat_type,
            cmd::set_chat_type,
            cmd::chat])
        .setup(|app|{

            app.manage(QianFanHandler{
                qianfan_client: QianFanClient::new(AS_VALUE.to_owned())
            });

            let store = app.store("app_data.json").unwrap();
            app.manage(Storage{store});

            let _ = TrayIconBuilder::with_id("main")
            .icon(app.default_window_icon().unwrap().clone())
            .title("MedConsult")
            .tooltip("MedConsult")
            .build(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
