use anyhow::{anyhow, Result};
use iam_auth::{HttpRequest, IAMAuth};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri_plugin_http::reqwest::{Client, RequestBuilder};

mod iam_auth;

// pub const MEMORY_ARR: [std::string::String; 2] = [String::from("ALTAKioNowgHFo1gEWYT4QqRZW"), String::from("ALTAKioNowgHFo1gEWYT4QqRZW")];

#[allow(unused)]
#[derive(Clone)]
pub struct QianFanClient {
    auth: IAMAuth,
}

#[allow(unused)]
#[derive(Clone)]
pub struct ChatBuilder {
    request: HttpRequest,

    messages: Vec<ChatMessage>,

    chat_type: Option<String>,
}

#[allow(unused)]
#[derive(Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    // 角色，可选 "user", "assistant", "function"
    role: String,

    // 对话内容
    content: String,
}

#[allow(unused)]
#[derive(Deserialize, Serialize)]
pub struct ChatRequest {
    // 聊天上下文信息
    messages: Vec<ChatMessage>,

    // 较高的数值会使输出更加随机，而较低的数值会使其更加集中和确定，范围 (0, 1.0]，不能为0
    temperature: Option<f32>,

    // 影响输出文本的多样性，取值越大，生成文本的多样性越强,默认0.7，取值范围 [0, 1.0]
    top_p: Option<f32>,

    // 通过对已生成的token增加惩罚，减少重复生成的现象,值越大表示惩罚越大,默认1.0，取值范围：[1.0, 2.0]
    penalty_score: Option<f32>,
    //模型人设，主要用于人设设定，例如：你是xxx公司制作的AI助手，说明：（1）长度限制，message中的content总长度和system字段总内容不能超过24000个字符
    system: Option<String>,

    // 是否强制关闭实时搜索功能，可选值：· true：关闭  · false：表示不关闭，默认false
    disable_search:bool,

    // 是否开启系统记忆，说明： true：表示开启，开启后，system_memory_id字段必填
    enable_system_memory: bool,

    // 统记忆ID，说明：
    system_memory_id: Option<String>, 

    // 表示最终用户的唯一标识符
    user_id: Option<String>,
}

// chat的返回值
#[allow(unused)]
#[derive(Deserialize, Serialize, Debug)]
pub struct ChatResponse {
    // 本轮对话的id
    pub id: String,

    // 时间戳
    pub created: i32,

    // 当前生成的结果是否被截断
    pub is_truncated: bool,

    // 对话返回结果
    pub result: String,

    // 表示用户输入是否存在安全风险，是否关闭当前会话，清理历史会话信息
    // true：是，表示用户输入存在安全风险，建议关闭当前会话，清理历史会话信息。false：否，表示用户输入无安全风险
    pub need_clear_history: bool,

    // 当need_clear_history为true时，此字段会告知第几轮对话有敏感信息，如果是当前问题，ban_round=-1
    #[serde(default)]
    pub ban_round: i32,

    // token统计信息
    pub usage: Usage,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Usage {
    // 问题tokens数
    prompt_tokens: i32,

    // 回答tokens数
    completion_tokens: i32,

    // tokens总数
    total_tokens: i32,
}

impl QianFanClient {
    // 关联函数，创建一个 QianFanClient
    pub fn new(key: String) -> QianFanClient {
        let tem: Vec<&str> = key.split("_").collect();
        QianFanClient {
            auth: IAMAuth::new(tem[1].into(), tem[2].into()),
        }
    }

    pub fn chat_completion(self) -> ChatBuilder {
        let request = self.auth.sign_request();
        let messages = Vec::new();

        return ChatBuilder {
            request,
            messages,
            chat_type: None,
        };
    }
}

impl ChatBuilder {
    #[allow(unused)]
    pub fn add_message(mut self, role: String, content: String) -> Self {
        self.messages.push(ChatMessage {
            role: role,
            content: content,
        });
        self
    }

    pub fn add_message_list(mut self, messages: Vec<ChatMessage>) -> Self {
        for message in messages.iter() {
            self.messages.push(message.clone());
        }
        self
    }

    pub fn chat_type(mut self, chat_type: String) -> Self {
        self.chat_type = Some(chat_type);
        self
    }

    // 执行 api 的请求
    pub async fn execute(self) -> Result<ChatResponse> {
        let temp: HttpRequest = self.request.clone();
        //println!("----request----:{:?}",self.request);

        let client = Client::new();
        let data = json!(self.build());

        //println!("---data-----:{:?}",data);

        let mut request_builder: RequestBuilder = client.post(temp.url);

        request_builder = request_builder.header("Content-Type", "application/json");

        for (key, value) in temp.headers {
            request_builder = request_builder.header(key, value);
        }

        let response = request_builder.json(&data).send().await?;
        let status = response.status();
        println!("--------response--status-----{}", status);

        let response_str = response.text().await.unwrap();
        println!("--------response-------{:?}", response_str);

        if status == tauri_plugin_http::reqwest::StatusCode::OK {
            if response_str.contains("error_code") {
                return Err(anyhow!("参数异常"));
            } else {
                let chat_response: ChatResponse = serde_json::from_str(response_str.as_str())?;
                return Ok(chat_response);
            }
        } else {
            println!("-----err1-----");
            return Err(anyhow!("请求异常"));
        }
    }

    fn build(self) -> ChatRequest {
        #[warn(unused_assignments)]
        let mut tem_system: Option<String> = None;

        let tem_type = "1";  // 高共情   
        let memory_id :Option<String> = None;
        if self.chat_type.unwrap().contains(tem_type) {
            if self.messages.len() == 1 {   // 1 代表客户的第一次询问，调用api的时候，会带着之前的数据，这个你随便改，改完保存，程序就会加载
                tem_system = Some("你是一位专业的医生，你的任务是理解患者的感受和需求并给予患者共情性的医疗回复，以患者为中心进行治疗和沟通。你的能力和特征包括：- 理解患者处境、观点和感受：能够从患者的角度理解他们的患病经历和情绪。- 交流并保证理解的准确性：与患者沟通时确保信息的准确传递和理解。- 以有利于患者的方式治疗：在治疗过程中考虑患者的需求和感受。- 认知共情：理解患者的感受和想法，并采取相应的行动。- 情感共情：感知并体验患者的情绪，与患者建立情感联系。- 使用积极的言语：在沟通中使用积极、鼓励性的语言。- 详细的询问：全面了解患者的情况，包括病情和相关的个人生活习惯。- 耐心解释：清晰、耐心地向患者解释病情和治疗方案。- 征求患者意见：在制定治疗计划时考虑患者的意愿和偏好。输出内容风格要求是：- 使用第一人称单数“我”自称。- 用第一人称复数“我们”拉近与患者的距离。- 用第二人称“您”称呼患者及其所属。- 使用现在时叙述。- 使用祈使句和“请”。- 使用主动句。- 邀请患者分享他们的情绪和感受。- 避免使用感叹号。- 经常对患者的回复表达感谢。- 展示对患者情绪状态的理解。- 表达关心声明。代表性言语说辞为：- “很抱歉听到这个消息，您最近一定很难受。”- “有这样的症状，您最近有些焦虑是很正常的。”- “您有任何情绪都可以说给我听，不论是现在还是症状出现时。”- “您别担心，我会和您一起面对这些症状的，让我们一起想一想...”- “您别担心，我会帮助您一起寻找症结所在，并给您相应的建议”- “希望您能在调整相应行为/服用药物后，身体早日恢复健康。”- “您别担心，我可以和您一起面对这个病症并解决它。”- “我非常能理解您现在的顾虑和担忧。”- “您放心，只要您多加休息，调整心态，规律作息，按时服药，症状一定会好转，身体也会越来越好”- “我很关心您的病症和相关经历。”- “谢谢您告诉我这些，您提供的信息很有用，谢谢。”- “我想请您想一想...”- “祝您生活愉快，身体健康，万事如意。”- “再见~如果您有任何症状，都可以再来咨询，我会一直在这里陪伴您。” 病人刚刚描述了症状，请询问病人的性别，年龄和既往病史，并针对性地询问一个重要问题。字数范围控制在200-220字。".to_string());
            } else if self.messages.len() < 7 {
                tem_system = Some("基于病人的回答，请继续询问一个相关的重要问题。请记住你是专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。字数范围控制在200-220字。".to_string());
            } else if self.messages.len() == 7 {
                tem_system = Some("现在请基于所有收集到的信息，给出完整的诊断和建议。包括：1.诊断结果 2.可能的原因 3.建议的检查项目 4.治疗建议 5.生活建议  请记住你是专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。".to_string());
            } else {
                tem_system = Some("请直接回答病人的问题，不要再询问新的问题。请记住你是专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。回答字数范围控制在200-220字。".to_string());
            }
            //memory_id = MEMORY_ARR[1];
        } else {
            if self.messages.len() == 1 {
                tem_system = Some("你是一位专业的医生，你的任务是执行医疗程序，以任务为中心进行治疗和沟通。你的能力和特征包括：- 基本的医疗知识。- 执行医疗问诊任务。- 基本的和患者进行沟通的技巧。- 缺乏对患者处境的深入理解：不能充分理解患者的经历和情绪。- 以自己的方式进行治疗：忽视患者的部分感受和需求。- 缺乏认知共情：不能理解、不能识别患者的感受和想法。- 缺乏情感共情：不能感知或体验患者的情绪。- 使用中性而非积极的言语：在沟通中尽量多使用中性而非积极的语言。- 缺乏耐心解释：不能十分耐心地回复患者、向患者解释病情和治疗方案。- 忽视患者意见：在制定治疗计划时不能完全考虑患者的意愿和偏好，不询问患者对治疗计划的意见或问题。输出内容风格要求是：- 使用“您”称呼患者。- 不使用“我”。- 不使用祈使句和“请”。- 不使用第一人称复数“我们”代指自己和患者。- 不使用第三人称称呼患者的病症等无生命物体。- 适时使用过去时进行叙述。- 多使用被动句。- 多使用感叹号。- 更多使用中性语言与患者沟通。- 很少对患者表达感谢。- 缺乏对患者情绪状态的理解。- 缺乏对患者的关心声明。代表性言语说辞为：- “您需要按时服药、健康作息。”- “谢谢您的配合。”- “建议您...”- “欢迎您再次咨询。”- “您的病症是可以被解决的。”- “您之前有无相关的病症？”。- “您按照以上建议，病情就可以获得缓解。” 病人刚刚描述了症状，请询问病人的性别，年龄和既往病史，并针对性地询问一个重要问题。字数范围控制在200-220字".to_string());
            } else if self.messages.len() < 7 {
                tem_system = Some("基于病人的回答，请继续询问一个相关的重要问题。请记住你是共情能力较低的专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。字数范围控制在200-220字".to_string());
            } else if self.messages.len() == 7 {
                tem_system = Some("现在请基于所有收集到的信息，给出完整的诊断和建议。包括：1.诊断结果 2.可能的原因 3.建议的检查项目 4.治疗建议 5.生活建议  请记住你是共情能力较低的专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。".to_string());
            } else {
                tem_system = Some("请直接回答病人的问题，不要再询问新的问题。请记住你是共情能力较低的专业医生，请体现你的特定任务、能力和特征、输出内容风格以及代表性说辞。回答字数范围控制在200-220字".to_string());
            }
            // memory_id = MEMORY_ARR[0];
        }

        ChatRequest {
            messages: self.messages,
            temperature: None,
            top_p: None,
            penalty_score: None,
            system: tem_system,
            disable_search:true,
            enable_system_memory: false,
            system_memory_id: memory_id,
            user_id: None,
        }
    }
}
