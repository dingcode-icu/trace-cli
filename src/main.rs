use log::info;

use crate::{api::api::get_buglist, subcmd::board};

use clap::{self, command, Command};

mod api;
mod subcmd;



fn main() {
    let mts = Command::new("trace-cli")
        .version("0.1.0")
        .author("dwb <dwb@dwb.ren>")
        .about("command for trace")
        .subcommand(
            Command::new("board")
            .about("show trace overview info")

        )
        .get_matches();
    
    match mts.subcommand() {
        Some(("board", sub_matchs)) => board::run(sub_matchs),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }


    println!("Hello, world!");
    let ret = get_buglist("*".to_string());
    println!("ret is 1{}", ret.is_ok());
    println!("ret is2 {:?}", ret.unwrap());
}