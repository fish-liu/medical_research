use anyhow::{anyhow, Result};
use iam_auth::{HttpRequest, IAMAuth};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use serde_json::json;
mod iam_auth;


#[allow(unused)]
#[derive(Clone)]
pub struct QianFanClient{
    auth: IAMAuth,
}

#[allow(unused)]
#[derive(Clone)]
pub struct ChatBuilder{

    request:HttpRequest,

    messages: Vec<ChatMessage>
}

#[allow(unused)]
#[derive(Clone,Deserialize,Serialize)]
pub struct ChatMessage{
    // 角色，可选 "user", "assistant", "function"
    role:String,

    // 对话内容
    content:String,
}

#[allow(unused)]
#[derive(Deserialize,Serialize)]
pub struct ChatRequest{

    // 聊天上下文信息
    messages:Vec<ChatMessage>,

    // 较高的数值会使输出更加随机，而较低的数值会使其更加集中和确定，范围 (0, 1.0]，不能为0
    temperature:Option<f32>,

    // 影响输出文本的多样性，取值越大，生成文本的多样性越强,默认0.7，取值范围 [0, 1.0]
    top_p:Option<f32>,

    // 通过对已生成的token增加惩罚，减少重复生成的现象,值越大表示惩罚越大,默认1.0，取值范围：[1.0, 2.0]
    penalty_score:Option<f32>,
    //模型人设，主要用于人设设定，例如：你是xxx公司制作的AI助手，说明：（1）长度限制，message中的content总长度和system字段总内容不能超过24000个字符
    system:Option<String>,

    // 表示最终用户的唯一标识符
    user_id:Option<String>,
}

// chat的返回值
#[allow(unused)]
#[derive(Deserialize,Serialize,Debug)]
pub struct ChatResponse{
    // 本轮对话的id
    pub id:String,

    // 时间戳
    pub created:i32, 

    // 当前生成的结果是否被截断
    pub is_truncated:bool,

    // 对话返回结果
    pub result:String,

    // 表示用户输入是否存在安全风险，是否关闭当前会话，清理历史会话信息
    // true：是，表示用户输入存在安全风险，建议关闭当前会话，清理历史会话信息。false：否，表示用户输入无安全风险
    pub need_clear_history:bool,

    // 当need_clear_history为true时，此字段会告知第几轮对话有敏感信息，如果是当前问题，ban_round=-1
    #[serde(default)]
    pub ban_round:i32,

    // token统计信息
    pub usage: Usage,

}

#[derive(Deserialize,Serialize,Debug)]
pub struct Usage{

    // 问题tokens数
    prompt_tokens:i32,

    // 回答tokens数
    completion_tokens:i32,

    // tokens总数
    total_tokens:i32,
}    

impl QianFanClient {

    // 关联函数，创建一个 QianFanClient 
    pub fn new(key: String) -> QianFanClient{

        let tem :Vec<&str> = key.split("_").collect();

        QianFanClient{
            auth: IAMAuth::new(tem[1].into(), tem[2].into())
        }
    }

    pub fn chat_completion(self) -> ChatBuilder{

        let request = self.auth.sign_request();
        let messages = Vec::new();

        return ChatBuilder{
            request,
            messages,
        };
    }

}

impl ChatBuilder {

    pub fn add_message(mut self,role:String,content:String) -> Self{
        self.messages.push(ChatMessage { role: role, content: content });
        self
    }

    pub fn add_message_list(mut self,messages:Vec<ChatMessage>) -> Self{

        for message in messages.iter() {
            self.messages.push(message.clone());
        }

        self
    }

    // 执行 api 的请求
    pub async fn execute(self) -> Result<ChatResponse> {

        let temp :HttpRequest = self.request.clone();
        //println!("----request----:{:?}",self.request);

        let client = Client::new();
        let data = json!(self.build());

        //println!("---data-----:{:?}",data);

        let mut request_builder:RequestBuilder =  client.post(temp.url);

        request_builder = request_builder.header("Content-Type", "application/json");

        for (key,value) in temp.headers {
            request_builder = request_builder.header(key, value);
        }
    
        let response = request_builder.json(&data).send().await?;

        let status = response.status();
        println!("--------response--status-----{}", status);

        let response_str = response.text().await.unwrap();
        println!("--------response-------{:?}", response_str);

        if status == reqwest::StatusCode::OK {
            if response_str.contains("error_code") {
                return Err(anyhow!("参数异常"));
            }else {
                let chat_response:ChatResponse = serde_json::from_str(response_str.as_str())?;
                return Ok(chat_response);
            }
        }else {
            println!("-----err1-----");
            return Err(anyhow!("请求异常"));
        }
        //println!("--------response-------{:?}", chat_response);
        //Ok(chat_response)
    }
    

    fn build(self) -> ChatRequest{
        ChatRequest{
            messages:self.messages,
            temperature:None,
            top_p:None,
            penalty_score:None,
            system:None,
            user_id:None
        }
    }

}

