pub mod api {

    use std::sync::Mutex;

    use log::error;
    use reqwest::{Url, blocking::Response};
    use lazy_static::lazy_static;
    use serde::{Serialize, Deserialize};


    #[derive(Serialize, Deserialize )] 
    pub struct Resp<T>{
        code: u16, 
        msg: String, 
        data: Option<T>
    }

    struct ReqConfig{
        pub host: String,
        con_timeout: u16,
        req_timeout: u16,
    }

    lazy_static! {
        static ref BASEINFO: Mutex<ReqConfig> = Mutex::new(ReqConfig {
            host: String::from("http://trace.bbclient.icu"),
            con_timeout: 30,
            req_timeout: 30
        });
    }

    // pub fn get_buglist(fmtstr:String) -> Option<Resp<Vec<String>>>{
    pub fn get_buglist(fmtstr:String) -> Option<Response>{
        let host = &BASEINFO.lock().unwrap().host;
        let url = Url::parse(host).and_then(|p|p.join("api/buglist"));
        if let Err(r) = url {
            error!("[http get_butlist]parse url raise error!");
            return None;
        }

        let ret =reqwest::blocking::get("api/buglist");

        if let Ok(resp) = ret {
            let rere:Resp<Vec<String>> = resp.json().unwrap();
            return Some(resp.json());
            // Some()    
        }       
        None
    }
}