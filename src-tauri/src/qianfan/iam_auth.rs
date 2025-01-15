use std::collections::HashMap;
use tauri_plugin_http::reqwest::Url;
use crate::utils::{get_utc_time, sha_256_mac};


// 生成认证字符串 https://cloud.baidu.com/doc/Reference/s/njwvz1yfu
//const URL: &str = "https://aip.baidubce.com/rpc/2.0/ai_custom/v1/wenxinworkshop/chat/ernie-lite-8k";

const BASE_URL: &str = "https://aip.baidubce.com";

//const URL_TEMPLATE: &str = "{}/rpc/2.0/ai_custom/v1/wenxinworkshop{}";

//const SESSION_KEY_TEMPLATE: &str = "bce-auth-v1/{}/{}/{}";

const METHOD: &str = "POST";

const AUTHORIZATION: &str ="Authorization";

const XBD: &str = "X-Bce-Date";

const IAM_SIGN_EXPIRATION_SEC: &str = "1800";


#[allow(unused)]
#[derive(Clone,Debug)]
pub struct  IAMAuth{

    access_key: String,

    secret_key: String,
}

// 
#[warn(dead_code)]
impl IAMAuth {
    
    pub fn new(access_key: String,secret_key: String) ->IAMAuth{
        IAMAuth{
            access_key,
            secret_key
        }
    }

    pub fn sign_request(self) -> HttpRequest{

        let request = HttpRequest::new();

        let authorization = self.sigen(request.method.clone(), request.url.clone());

        return request
            .add_header(AUTHORIZATION.to_owned(), authorization)
            .add_header(XBD.to_owned(), get_utc_time());
    }


    fn sigen(self, method:String, url:String) -> String{

        let raw_session_key = format!("bce-auth-v1/{}/{}/{}", self.access_key.clone(), get_utc_time(), IAM_SIGN_EXPIRATION_SEC);
        
        let session_key = sha_256_mac(raw_session_key.clone(),self.secret_key.clone());


        let url = Url::parse(&url).unwrap();

        let  canonical_query_string = "";

        let mut headers:HashMap<String, String> = HashMap::new();

        headers.insert("host".to_string(), url.host_str().unwrap().to_string());
        headers.insert("x-bce-date".to_string(), get_utc_time());

        println!("headers = {:?}",headers);

        let mut canonical_headers:Vec<String> = Vec::new();
        
        for (key,value) in &headers{
            let mut key = key.to_lowercase();
            let mut value = value.trim().to_string();

            key = urlencoding::encode(key.as_str())
            .replace("+", "%20")
            .replace("%21", "!")
            .replace("%27", "'")
            .replace("%28", "(")
            .replace("%29", ")")
            .replace("%2A", "*")
            .replace("%2F", "/");

            value = urlencoding::encode(&value)
            .replace("+", "%20")
            .replace("%21", "!")
            .replace("%27", "'")
            .replace("%28", "(")
            .replace("%29", ")")
            .replace("%2A", "*")
            .replace("%2F", "/");

            canonical_headers.push(format!("{}:{}",key,value));
        }

        canonical_headers.sort();

        let canonical_headers = canonical_headers.join("\n");

        let signed_headers = "host;x-bce-date";

        let raw_signature = format!("{}\n{}\n{}\n{}",method,url.path(),canonical_query_string,canonical_headers);

        let signature = sha_256_mac(raw_signature.clone(),session_key);

        format!("{}/{}/{}",raw_session_key,signed_headers,signature)
    }


}


#[allow(unused)]
#[derive(Clone,Debug)]
pub struct HttpRequest{

    pub url: String,

    pub method:String,

    pub headers: HashMap<String,String>,
}


impl HttpRequest {

    pub fn new() -> HttpRequest{
        let headers:HashMap<String,String> = HashMap::new();
        let final_endpoint = "/chat/ernie-4.0-turbo-8k";
        //let finalEndpoint = endpointRetriever.getEndpoint(baseRequest.getType(), baseRequest.getModel(), baseRequest.getEndpoint());
        let url = format!("{}/rpc/2.0/ai_custom/v1/wenxinworkshop{}", BASE_URL.to_owned(), final_endpoint);

        HttpRequest{
            url: url,
            method: METHOD.to_owned(),
            headers:headers
        }
    }

    pub fn add_header(mut self,key: String,value: String) -> Self{
        self.headers.insert(key, value);

        self
    }
    
}




