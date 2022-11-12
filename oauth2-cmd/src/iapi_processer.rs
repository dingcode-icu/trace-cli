use std::{path::Path, fs};
use std::str::{FromStr};
use ureq::serde_json;

use crate::get_global_cachedir;

///获取usr缓存
fn chk_loc_usrinfo() -> serde_json::Value {
    let cache_dir = get_global_cachedir();
    let f_usr = Path::new(&cache_dir).join(".usr");
    let f_usr_str = fs::read_to_string(f_usr).unwrap_or("{}".to_string());
    serde_json::Value::from_str(&f_usr_str).unwrap()
}

pub trait APIProcesser {
    fn get_usr_info(&self) -> Option<serde_json::Value> {
        let usr = chk_loc_usrinfo();
        Some(usr)
    }

    fn webbrowser_login(&self, red_uri: String);

    fn api_login(&self, red_uri: String) -> Result<String, ureq::Error>;

    fn api_gettoken(&self, code: String, red_uri: String) -> Result<String, ureq::Error>;

    fn api_userinfo(&self, token: String) -> Result<serde_json::Value, ureq::Error>;
}