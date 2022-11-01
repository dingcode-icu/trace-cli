use std::{
    collections::HashMap,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
    thread,
};

use github::GithubProcesser;
use tiny_http::Server;
use ureq::serde_json::{self, Value};

/// Useful login library for command tool by oauth2 api
/// oath2 API include but not limit to
/// * github
/// * ...
///
mod github;

pub enum API {
    Github,
    Dingcode,
}

pub enum CacheType {
    Token,
    Usr,
}

impl Display for CacheType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CacheType::Token => write!(f, "token"),
            CacheType::Usr => write!(f, "usr"),
        }
    }
}

///查看可用的port
fn get_avaliable_port() -> u16 {
    std::net::TcpListener::bind("0.0.0.0:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

///获取缓存token
fn get_global_cachedir() -> PathBuf {
    let ret = std::env::home_dir().unwrap().join(".oauth2-cmd");
    if !Path::new(&ret).is_dir() {
        let _ = std::fs::create_dir_all(&ret);
    }
    ret
}

///获取token缓存
fn chk_loc_token() -> String {
    let cache_dir = get_global_cachedir();
    let f_token = Path::new(&cache_dir).join(".token");
    fs::read_to_string(f_token).unwrap_or_default()
}

///获取usr缓存
fn chk_loc_usrinfo() -> serde_json::Value {
    let cache_dir = get_global_cachedir();
    let f_usr = Path::new(&cache_dir).join(".usr");
    let f_usr_str = fs::read_to_string(f_usr).unwrap_or("{}".to_string());
    serde_json::Value::from_str(&f_usr_str).unwrap()
}

///缓存到cache文件夹
fn record_loc_cache(ctype: CacheType, cont: String) -> Result<(), io::Error> {
    let cache_dir = get_global_cachedir();
    let f = match ctype {
        CacheType::Token => Path::new(&cache_dir).join(".token"),
        CacheType::Usr => Path::new(&cache_dir).join(".usr"),
    };

    fs::write(f, &cont)?;
    Ok(())
}

///获取query kv对
fn query_to_tuple(query_str: &str) -> HashMap<String, String> {
    let query_l: Vec<&str> = query_str.split("&").collect();
    let mut ret: HashMap<String, String> = HashMap::new();
    for ql in query_l {
        let t: Vec<&str> = ql.split("=").collect();
        ret.insert(t[0].to_string(), t[1].to_string());
    }
    return ret;
}

fn svr_for_redirect<F>(port: u16, cb: F) -> Option<Value>
where
    F: Fn(&str) -> Value,
{
    let addr = format!("127.0.0.1:{}", port);
    let svr = Server::http(&addr).unwrap();
    let mut ret: Option<Value> = None;
    for req in svr.incoming_requests() {
        let query_hash = query_to_tuple(&req.url()[2..]);
        if query_hash.contains_key("code") {
            let r = cb(query_hash.get("code").unwrap());
            svr.unblock();
            ret = Some(r)
        } else {
            panic!("[getcode]raise error!")
        }
    }
    ret
}

pub(crate) trait APIProcesser {
    fn get_usr_info() -> Option<serde_json::Value> {
        let usr = chk_loc_usrinfo();
        Some(usr)
    }

    fn webbrowser_login(&self, red_uri: String);

    fn api_login(&self, red_uri: String) -> Result<String, ureq::Error>;

    fn api_gettoken(&self, code: String, red_uri: String) -> Result<String, ureq::Error>;

    fn api_userinfo(&self, token: String) -> Result<serde_json::Value, ureq::Error>;
}

pub fn get_usr_json() -> serde_json::Value {
    chk_loc_usrinfo()
}

pub fn login(api: Option<API>) -> Result<serde_json::Value, serde_json::Error> {
    if is_login() {
        logout()
    };
    let api_type = api.unwrap_or(API::Github);
    let procer = match api_type {
        API::Github => GithubProcesser::default(),
        _ => GithubProcesser::default(),
    };
    let port = get_avaliable_port();
    //redirect url listen
    let red_uri = format!("http://127.0.0.1:{}", port.to_string());
    let svr = thread::spawn(move || {
        println!("start local svr to listen the github callback...");
        let ret = svr_for_redirect(port, |code| {
            println!("output code is{}", code);
            let token = procer
                .api_gettoken(code.to_string(), red_uri.clone())
                .unwrap();
            println!("output token is {}", token);
            let query_kv = query_to_tuple(&token);
            println!("output query_kv is{:?}", query_kv);
            let usr = procer
                .api_userinfo(format!(
                    "{} {}",
                    query_kv.get("token_type").unwrap(),
                    query_kv.get("access_token").unwrap()
                ))
                .unwrap();
            println!("usrinfo is{:?}", usr);
            //record token
            let rr = record_loc_cache(CacheType::Token, token.to_string());
            //record usr
            let rr = record_loc_cache(CacheType::Usr, usr.to_string());
            usr
        });
        ret
    });

    //get token by loc/remote
    let token = chk_loc_token();
    if token.is_empty() {
        procer.webbrowser_login(format!("http://127.0.0.1:{}", port.to_string()));
        let ret = svr.join().unwrap();
        if ret.is_none() {
            panic!("[error]svr get req github api failed!");
        }
    }
    let usr = chk_loc_usrinfo();
    Ok(usr)
}

pub fn logout() {
    let cache_dir = get_global_cachedir();
    if Path::new(&cache_dir).is_dir() {
        let _ = fs::remove_dir(cache_dir);
    }
    println!("Done!");
}

pub fn is_login() -> bool {
    chk_loc_token().len() > 0
}

#[test]
fn test_getenv() {
    use std::env::home_dir;
    println!("home dir is {}", home_dir().unwrap().display());

    println!("available port is {}", get_avaliable_port());

    let html_str = login(Some(API::Github));
    println!("ret is {:?}", html_str);
}
