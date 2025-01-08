// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use qianfan::QianFanClient;
use tauri::Manager;

mod qianfan;
mod cmd;
mod utils;

//pub const AC_KE: &str = "ALTAKioNowgHFo1gEWYT4QqRZW_1";

//pub const SE_KE: &str = "2676be91677d4532be42eeea7328fe66_1";


#[warn(dead_code)]
struct QianFanHandler{
    qianfan_client: QianFanClient
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cmd::greet,cmd::chat])
        .setup(|app|{

            app.manage(QianFanHandler{
                qianfan_client: QianFanClient::new("ALTAKioNowgHFo1gEWYT4QqRZW".to_owned(),"2676be91677d4532be42eeea7328fe66".to_owned())
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
