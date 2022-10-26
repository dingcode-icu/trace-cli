use std::{collections::HashMap, error::Error, fmt::Display, hash::Hash};

use log::error;
use regex::Regex;

use crate::api;

pub mod board;
pub mod bug;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum CCSTraceType {
    Unknown,
    Engine,

    InnerJS,
    BundleJS,
}

impl Display for CCSTraceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CCSTraceType::Unknown => write!(f, "Unknown"),
            CCSTraceType::Engine => write!(f, "Engine"),
            CCSTraceType::InnerJS => write!(f, "InnerJS"),
            CCSTraceType::BundleJS => write!(f, "BundleJS"),
        }
    }
}

impl From<String> for CCSTraceType {
    fn from(f: String) -> Self {
        match f {
            f if f == "unknown" => CCSTraceType::Unknown,
            f if f == "engine" => CCSTraceType::Engine,
            f if f == "innerjs" => CCSTraceType::InnerJS,
            f if f == "bundlejs" => CCSTraceType::BundleJS,
            _ => CCSTraceType::Unknown,
        }
    }
}

//todo : support multiple engine
enum UnityTraceType {
    Unknown,
    Engine,

    InnerCsharp,
    BundleCsharp,

    InnerLua,
    BundleLua,
}

fn ccs_bug_filter(trace: &str) -> Result<CCSTraceType, Box<dyn Error>> {
    let rex_engine = Regex::new(r"(cocos2d-jsb.js:)")?;
    let rex_inner_js = Regex::new(r"(@assets)")?;
    let rex_bundle_js = Regex::new(r"(/downLoaderTest/bundle_)")?;

    if rex_engine.is_match(trace) {
        return Ok(CCSTraceType::Engine);
    }
    if rex_inner_js.is_match(trace) {
        return Ok(CCSTraceType::InnerJS);
    }
    if rex_bundle_js.is_match(trace) {
        return Ok(CCSTraceType::BundleJS);
    }

    Ok(CCSTraceType::Unknown)
}

///獲取單品類信息
pub fn ccv_type_info() -> HashMap<CCSTraceType, Vec<String>> {
    let buglist = api::api::get_buglist("*".to_string());
    let mut ret: HashMap<CCSTraceType, Vec<String>> = HashMap::new();
    ret.insert(CCSTraceType::Engine, Vec::new());
    ret.insert(CCSTraceType::InnerJS, Vec::new());
    ret.insert(CCSTraceType::BundleJS, Vec::new());
    ret.insert(CCSTraceType::Unknown, Vec::new());

    if let Ok(resp) = buglist {
        if resp.code != 0 {
            panic!("Net error:{:?}", resp.msg);
        }
        if let Some(bl) = resp.data {
            for b in &bl {
                let ret_ = ccs_bug_filter(b.as_str());
                if let Ok(r) = ret_ {
                    match r {
                        CCSTraceType::Engine => {
                            let l = ret.get_mut(&CCSTraceType::Engine).unwrap();
                            l.insert(0, b.to_string());
                        }
                        CCSTraceType::InnerJS => {
                            let l = ret.get_mut(&CCSTraceType::InnerJS).unwrap();
                            l.insert(0, b.to_string());
                        }
                        CCSTraceType::BundleJS => {
                            let l = ret.get_mut(&CCSTraceType::BundleJS).unwrap();
                            l.insert(0, b.to_string());
                        }
                        CCSTraceType::Unknown => {
                            let l = ret.get_mut(&CCSTraceType::Unknown).unwrap();
                            l.insert(0, b.to_string());
                        }
                    }
                }
            }
        }
    }
    return ret;
}

///获取错误大盘信息
pub fn ccv_board_info() -> (Vec<CCSTraceType>, Vec<u32>, Vec<f32>) {
    let mut type_l = vec![
        CCSTraceType::Engine,
        CCSTraceType::InnerJS,
        CCSTraceType::BundleJS,
        CCSTraceType::Unknown,
    ];
    let mut typenum_l = vec![0, 0, 0, 0];
    let mut typeper_l = vec![0., 0., 0., 0.];
    let buglist = api::api::get_buglist("*".to_string());
    if let Ok(resp) = buglist {
        if resp.code != 0 {
            panic!("Net error:{:?}", resp.msg);
        }
        if let Some(bl) = resp.data {
            for b in &bl {
                let ret = ccs_bug_filter(b.as_str());
                if let Ok(r) = ret {
                    match r {
                        CCSTraceType::Engine => typenum_l[0] += 1,
                        CCSTraceType::InnerJS => typenum_l[1] += 1,
                        CCSTraceType::BundleJS => typenum_l[2] += 1,
                        CCSTraceType::Unknown => typenum_l[3] += 1,
                    }
                }
            }
            typeper_l[0] = (typenum_l[0] as f32) / (bl.len() as f32);
            typeper_l[1] = (typenum_l[1] as f32) / (bl.len() as f32);
            typeper_l[2] = (typenum_l[2] as f32) / (bl.len() as f32);
            typeper_l[3] = (typenum_l[3] as f32) / (bl.len() as f32);
        }
        return (type_l, typenum_l, typeper_l);
    }
    panic!("Get buglist from remote failed!")
}

#[test]
fn test_reg() {
    assert_eq!(
        Regex::new(r"(cocos2d-jsb.js:)")
            .unwrap()
            .is_match("src/cocos2d-jsb.js:47398"),
        true
    );
    // assert_eq!(, true);
}
