use clap::ArgMatches;
use log::warn;
use serde_json::Value;

use crate::api::get_buginfo;

pub fn run(args: &ArgMatches) {
    //require
    let trace = args.get_one::<String>("trace").unwrap();
    let single = get_buginfo(trace.to_string(), 0, 1);

    let json_detail = single.unwrap().data.unwrap();
    if json_detail.len() > 0 {
        let json_obj: Value = serde_json::from_str(json_detail[0].as_str()).unwrap();
        let stack_list = json_obj["game_ctx"]["stack"].as_array().unwrap();
        println!("stack->");
        for s in stack_list {
            println!("{}", s);
        }
    } else {
        warn!("find trace list len is 0!");
    }
}
