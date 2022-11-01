use std::collections::HashMap;

use crate::api;

pub mod board;
pub mod bug;
pub mod engine;
pub mod fix;
pub mod login;

pub struct TraceState {
    pub is_fix: bool,
    pub resolve_email: String,
    pub resolve_time: u64,
}

impl ToString for TraceState {
    fn to_string(&self) -> String {
        fn join_kv(k: &str, v: &str, isend: bool) -> String {
            format!("{k}={v}", k = k, v = v) + if isend { "" } else { "&" }
        }
        return join_kv("is_fix", &self.is_fix.to_string(), false)
            + &join_kv("resolve_email", &self.resolve_email, false)
            + &join_kv("resolve_time", &self.resolve_time.to_string(), true).as_str();
    }
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

pub fn get_trace_state(trace: String) -> Option<String> {
    let trace_l = api::get_bugstat_list();
    if let Ok(tl) = trace_l {
        if let Some(data) = tl.data {
            let stat = data.get(&trace);
            if stat.is_some() {
                return Some(String::from(stat.unwrap()));
            }
        }
    }
    None
}
#[test]
fn test_reg() {
    use regex::Regex;
    assert_eq!(
        Regex::new(r"(cocos2d-jsb.js:)")
            .unwrap()
            .is_match("src/cocos2d-jsb.js:47398"),
        true
    );
    // assert_eq!(, true);
}
