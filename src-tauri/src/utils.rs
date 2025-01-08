
use chrono::{DateTime, SecondsFormat, Utc};
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;
use rustc_serialize::hex::ToHex;

// 返回 utc 的时间 ： 2025-01-04T14:44:19Z
#[allow(unused)]
pub(crate) fn get_utc_time() -> String {
    let utc: DateTime<Utc> = Utc::now();
    utc.to_rfc3339_opts(SecondsFormat::Secs, true)
}


// https://juejin.cn/post/7270792087435558912 
// https://blog.csdn.net/ap114/article/details/119947997
#[allow(unused)]
pub(crate) fn sha_256_mac(data:String,key:String) -> String{

    //let key = "123456789";  
    //let message = "dashen.tech";
    let mut hmac = Hmac::new(Sha256::new(), key.as_bytes());
    hmac.input(data.as_bytes());
    //println!("HMAC digest: {}", hmac.result().code().to_hex());
    
    hmac.result().code().to_hex()
}


#[cfg(test)]
mod tests {
    use super::get_utc_time;
    use super::sha_256_mac;

    #[test]
    fn it_works() {

        let time = get_utc_time();
        println!("---{}",time);

        let result = sha_256_mac("dashen.tech".to_string(),"123456789".to_string());

        println!("HMAC digest: {}", result);
    }

}


