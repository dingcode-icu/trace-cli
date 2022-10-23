use std::{collections::HashMap, error::Error};

use clap::ArgMatches;
use log::error;
use regex::Regex;

use crate::api;

#[derive(Debug)]
enum CCSTraceType {
    Unknown,
    Engine, 
    
    InnerJS, 
    BundleJS
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
    let rex_engine = Regex::new(r"[cocos2d-jsb.js:]")?;
    let rex_inner_js = Regex::new(r"[@assets])")?;
    let rex_bundle_js = Regex::new(r"[/files/downLoaderTest/]")?;

    if rex_engine.is_match(trace) {return Ok(CCSTraceType::Engine);}
    if rex_inner_js.is_match(trace) {return Ok(CCSTraceType::InnerJS);}
    if rex_bundle_js.is_match(trace) {return Ok(CCSTraceType::BundleJS);}

    Ok(CCSTraceType::Unknown)
}

// fn get_bug_tb<T>(tracel: Vec<String>) -> HashMap<T, u16>{
//     for t in tracel {
        
//     }
// }


pub fn run(args: &ArgMatches){
    println!("run now");
    let buglist = api::api::get_buglist("*".to_string());
    
    if let Ok(resp) = buglist{
        println!("-->>resp{:?}", resp);
        if resp.code != 0 {
            error!("Net error:{:?}", resp.msg);
            return ;
        }
        let bl = resp.data.unwrap_or(vec![]);
        for b in bl{
            let ret = ccs_bug_filter(b.as_str());
            if let Ok(r) = ret{
                println!("trace ->{} type is ->{:?}", b, r);
            }
        }
        
    }
        

    error!("Get buglist from remote failed!")
}

#[test]
fn test_reg(){
    assert_eq!( Regex::new(r"[cocos2d-jsb.js:]").unwrap().is_match("src/cocos2d-jsb.js:47398"), true);
    // assert_eq!(, true);
    run(&ArgMatches::default());
}