use log::info;

use crate::{api::api::get_buglist, subcmd::board};

use clap::{self, command, Command};

mod api;
mod subcmd;



fn main() {
    let mut cmd = Command::new("trace-cli");
    let mts = cmd.clone()
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
        _ => {cmd.print_help().unwrap();},
    }
}   