use std::{collections::HashMap, error::Error, fmt::Display};

use clap::ArgMatches;
use log::error;
use prettytable::{Table, row};
use regex::Regex;

use crate::api;

#[derive(Debug)]
enum CCSTraceType {
    Unknown,
    Engine, 
    
    InnerJS, 
    BundleJS
}

impl Display for CCSTraceType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CCSTraceType::Unknown => write!(f, "Unknown"),
            CCSTraceType::Engine => write!(f, "Engine"),
            CCSTraceType::InnerJS => write!(f, "InnerJS"),
            CCSTraceType::BundleJS => write!(f, "BudnleJS"),
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
    BundleLua
}

fn ccs_bug_filter(trace:&str) -> Result<CCSTraceType, Box<dyn Error>>{
    let rex_engine = Regex::new(r"(cocos2d-jsb.js:)")?;
    let rex_inner_js = Regex::new(r"(@assets)")?;
    let rex_bundle_js = Regex::new(r"(/files/downLoaderTest/)")?;

    if rex_engine.is_match(trace) {return Ok(CCSTraceType::Engine);}
    if rex_inner_js.is_match(trace) {return Ok(CCSTraceType::InnerJS);}
    if rex_bundle_js.is_match(trace) {return Ok(CCSTraceType::BundleJS);}

    Ok(CCSTraceType::Unknown)
}

fn table_for_board(num_l: Vec<u32>, per_l: Vec<f32>) {
    let mut tb = Table::new();
    tb.add_row(row!["TraceType", "Count", "Per"]);
    tb.add_row(row![CCSTraceType::Engine, num_l[0].to_string(), format!("{:.4}%", (per_l[0] * 100.).to_string())]);
    tb.add_row(row![CCSTraceType::InnerJS, num_l[1].to_string(), format!("{:.4}%", (per_l[1] * 100.).to_string())]);
    tb.add_row(row![CCSTraceType::BundleJS, num_l[2].to_string(), format!("{:.4}%", (per_l[2] * 100.).to_string())]);
    tb.add_row(row![CCSTraceType::Unknown, num_l[3].to_string(), format!("{:.4}%", (per_l[3] * 100.).to_string())]);
    tb.printstd();
}

fn table_for_type(){
    
}

pub fn run(args: &ArgMatches){
    let mut type_l = vec![CCSTraceType::Engine, CCSTraceType::InnerJS, CCSTraceType::BundleJS, CCSTraceType::Unknown];
    let mut typenum_l = vec![0, 0, 0, 0];
    let mut typeper_l = vec![0., 0., 0., 0.];
    let buglist = api::api::get_buglist("*".to_string());
    if let Ok(resp) = buglist{
        if resp.code != 0 {
            error!("Net error:{:?}", resp.msg);
            return ;
        }
        if let Some(bl) = resp.data {
            for b in &bl{
                let ret = ccs_bug_filter(b.as_str());
                if let Ok(r) = ret{
                    match r {
                        CCSTraceType::Unknown => typenum_l[3] += 1,
                        CCSTraceType::Engine => typenum_l[0] += 1,
                        CCSTraceType::InnerJS => typenum_l[1] += 1,
                        CCSTraceType::BundleJS => typenum_l[2] += 1,
                    }
                }
            }
            typeper_l[0] = (typenum_l[0] as f32) / (bl.len() as f32);
            typeper_l[1] = (typenum_l[1] as f32) / (bl.len() as f32);
            typeper_l[2] = (typenum_l[2] as f32) / (bl.len() as f32);
            typeper_l[3] = (typenum_l[3] as f32) / (bl.len() as f32);
            table_for_board(typenum_l, typeper_l)
        }
    }
    error!("Get buglist from remote failed!")
}

#[test]
fn test_reg(){
    assert_eq!( Regex::new(r"(cocos2d-jsb.js:)").unwrap().is_match("src/cocos2d-jsb.js:47398"), true);
    // assert_eq!(, true);
    run(&ArgMatches::default());
}
