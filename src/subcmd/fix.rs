use super::{get_trace_state, query_to_tuple};
use crate::{api::post_modify_bugstat, subcmd::TraceState};
use clap::ArgMatches;
use oauth2_cmd::{get_usr_json, is_login};
use std::time::SystemTime;

pub fn run(args: &ArgMatches) {
    if !is_login() {
        println!("[fix] not login yet! run 'tracer login' first!");
        return;
    };

    //require
    let trace = args.get_one::<String>("tracer").unwrap();
    let cur_state = get_trace_state(trace.to_string());

    if let None = cur_state {
        let cur_t = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("[fix]get system time faile!");

        let usr = get_usr_json();

        let chk_email = usr.get("email").expect("[fix]At least, u must set an email address in your github's profile！");

        let stat = TraceState {
            is_fix: true,
            resolve_email: chk_email.as_str().unwrap().to_string(),
            resolve_time: cur_t.as_secs(),
        };
        let stat_str = stat.to_string();
        let ret = post_modify_bugstat(trace.to_string(), stat_str);
        println!("[fix]set stat result is {:?}", ret);
        if let Ok(resp) = ret {
            if resp.code != 0 {
                panic!("[fix]fix api raise error");
            }
            println!("[fix]suc!")
        }
    } else {
        let ret = query_to_tuple(cur_state.unwrap().as_str());
        println!(
            "[fix]the bug had already been fixed by <{email}> in <{time}>",
            email = ret.get("resolve_email").unwrap_or(&"unknown".to_string()),
            time = ret.get("resolve_time").unwrap_or(&"unknown".to_string())
        )
    }
}

#[test]
fn test_fix() {
    run(&ArgMatches::default())
}
