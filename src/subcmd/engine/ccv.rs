use std::{collections::HashMap, error::Error, fmt::Display};

use regex::Regex;

use crate::{
    api::{get_buglist, get_bugstat_list},
    subcmd::board::BugListType,
};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum CCSTraceType {
    Engine = 0,
    InnerJS,
    BundleJS,
    Unknown,
}

impl CCSTraceType {
    pub fn len() -> usize {
        4
    }
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

///获取服务器记录
pub fn get_svr_record() -> (Vec<String>, HashMap<String, String>) {
    println!("[board]api to list(1/3)...");
    let mut buglist: Vec<String> = Vec::new();
    if let Ok(resp) = get_buglist("*".to_string()) {
        if resp.code != 0 {
            panic!("Net error:{:?}", resp.msg);
        }
        buglist = resp.data.unwrap_or_default();
    };
    println!("[board]api to stat(2/3)...");
    let mut statlist: HashMap<String, String> = HashMap::new();
    if let Ok(resp) = get_bugstat_list() {
        if resp.code != 0 {
            panic!("Net error:{:?}", resp.msg);
        }
        statlist = resp.data.unwrap_or_default();
    };
    println!("[board]done!");
    return (buglist, statlist);
}

///获取单个类型的错误信息
pub fn ccv_type_info(show_type: BugListType) -> HashMap<CCSTraceType, Vec<String>> {
    let (buglist, statlist) = get_svr_record();
    let mut ret: HashMap<CCSTraceType, Vec<String>> = HashMap::new();
    ret.insert(CCSTraceType::Engine, Vec::new());
    ret.insert(CCSTraceType::InnerJS, Vec::new());
    ret.insert(CCSTraceType::BundleJS, Vec::new());
    ret.insert(CCSTraceType::Unknown, Vec::new());

    for b in &buglist {
        if show_type != BugListType::All {
            //todo : check stat detail
            // let stat = statlist.get(b).unwrap_or(&"".to_string());
            let is_fixedbug = statlist.contains_key(b);
            if show_type == BugListType::Fixed && !is_fixedbug {
                continue;
            }
            if show_type == BugListType::UnFixed && is_fixedbug {
                continue;
            }
        }
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
    return ret;
}

///获取错误大盘信息
pub fn ccv_board_info() -> (Vec<CCSTraceType>, Vec<u32>, Vec<f32>, Vec<u32>) {
    let (buglist, statlist) = get_svr_record();
    let type_l = vec![
        CCSTraceType::Engine,
        CCSTraceType::InnerJS,
        CCSTraceType::BundleJS,
        CCSTraceType::Unknown,
    ];
    //总数列表
    let mut typenum_l = vec![0, 0, 0, 0];
    //百分比列表
    let mut typeper_l = vec![0., 0., 0., 0.];
    //修复数量
    let mut typenumfixed_l = vec![0, 0, 0, 0];

    let mut handle_all_num = |ct: CCSTraceType| {
        let idx = ct as usize;
        typenum_l[idx] += 1;
    };

    let mut handle_allper_num = |ct: CCSTraceType, is_fix: bool| {
        let idx = ct as usize;
        if is_fix {
            typenumfixed_l[idx] += 1;
        }
    };
    for b in &buglist {
        let ret = ccs_bug_filter(b.as_str());
        if let Ok(r) = ret {
            handle_all_num(r);
            handle_allper_num(r, statlist.contains_key(b))
        }
    }
    let bug_len = buglist.len();
    for i in 0..CCSTraceType::len() {
        typeper_l[i] = (typenum_l[i] as f32) / (bug_len as f32);
    }

    return (type_l, typenum_l, typeper_l, typenumfixed_l);
}
