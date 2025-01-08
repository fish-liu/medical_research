use serde::{Deserialize, Serialize};
use tauri::Manager;
use crate::{qianfan::{ChatBuilder, ChatMessage}, QianFanHandler};


#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[derive(Deserialize,Serialize)]
pub struct ChatResult{
    code:i32,

    message:Option<String>,

    data:Option<String>

}


#[tauri::command]
pub async fn chat<R: tauri::Runtime>(app: tauri::AppHandle<R>, messages:Vec<ChatMessage>) -> ChatResult{
    println!("--messages len --- : {}",messages.len());

    if messages.len() > 0 {
        
        let qianfan_handler = &app.state::<QianFanHandler>();
        let chat_builder: ChatBuilder = qianfan_handler.qianfan_client.clone().chat_completion();

        //let response = chat_builder.add_message("user".to_string(), "北京在那里".to_string()).execute().await;
        let response = chat_builder.add_message_list(messages).execute().await;
        println!("-------{:?}",response);

        match response {
            Ok(res) => {
                return ChatResult{
                    code:200,
                    message:None,
                    data:Some(res.result)
                };
            }
            Err(e) =>{
                return ChatResult{
                    code:0,
                    message:Some(e.to_string()),
                    data:None
                };
            }
        }
    }else {
        return ChatResult{
            code:100,
            message:Some("参数为空".to_string()),
            data:None,
        };
    }


}




