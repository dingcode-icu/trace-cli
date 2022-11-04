use chrono::{DateTime, Utc};
use clap::ArgMatches;
use core::panic;
use prettytable::{row, Table};
use serde_json::Value;
use std::time::{Duration, UNIX_EPOCH};

use crate::api::get_buginfo;

fn table_for_bug(trace: &str, topnum: u32) -> Vec<String> {
    println!("max couunt is {}", topnum);
    let bug_cells: Vec<String>;
    if let Ok(resp) = get_buginfo(trace.to_string(), 0, topnum) {
        if resp.code != 0 {
            panic!("Net error:{:?}", resp.msg);
        }
        bug_cells = resp.data.unwrap_or_default();
    } else {
        panic!("[bug] api get_buginfo failed!");
    }
    if bug_cells.len() == 0 {
        panic!("[bug] bug detail info count is 0?");
    }
    let mut tb = Table::new();
    tb.add_row(row!["Count", "FirstTime"]);
    let count = if bug_cells.len() > 99 {
        ">=100".to_string()
    } else {
        let len = bug_cells.len().to_string();
        len
    };
    let json_obj: Value = serde_json::from_str(bug_cells[0].as_str()).unwrap();
    let first_time = json_obj["common_param"]["timestamp"].as_str().unwrap();
    let d = UNIX_EPOCH + Duration::from_secs(first_time.to_string().parse::<u64>().unwrap() / 1000);
    let datetime = DateTime::<Utc>::from(d);
    // Formats the combined date and time with the specified format string.
    let timestamp_str = datetime.format("%Y%m%d %H:%M:%S").to_string();
    let stack_list = json_obj["game_ctx"]["stack"].as_array().unwrap();
    // let hotver = json_obj[""]
    tb.add_row(row![count, timestamp_str]);
    println!("[bug]stack->");
    for s in stack_list {
        println!("{}", s);
    }
    tb.printstd();
    println!(
        "[bug] trace top 1 detail info of json:{}",
        json_obj["common_param"]
    );
    return bug_cells;
}

pub fn run(args: &ArgMatches) {
    //require
    let trace = args.get_one::<String>("tracer").unwrap();
    let count = args.try_get_one::<String>("count").unwrap();
    let mut max_c = 0;
    if count.is_some() {
        max_c = count.unwrap().parse::<u32>().unwrap();
    }

    table_for_bug(trace, if max_c > 99 { 99 } else { max_c });
}
