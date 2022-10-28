use subcmd::bug;

use crate::subcmd::board;

use clap::{self, arg, Arg, Command};

mod api;
mod subcmd;

fn main() {
    let mut cmd = Command::new("trace-cli");
    let mts = cmd
        .clone()
        .version("0.1.0")
        .author("dwb <dwb@dwb.ren>")
        .about("command for trace")
        .subcommand(
            Command::new("board")
                .about("show trace overview info")
                .args([Arg::new("trace_type")
                    .short('t')
                    .help("the trace type to show single type details")]),
        )
        .subcommand(
            Command::new("bug")
                .about("show the fliterd detail of bug by input")
                .args([Arg::new("trace")
                    .required(true)
                    .help("the trace key to show single bug details")]),
        )
        .subcommand(
            Command::new("fix")
                .about("show the trace stat record")
                .args([Arg::new("trace")
                    .required(true)
                    .help("the trace key to show single bug details")]),
        )
        .subcommand(
            Command::new("login")
                .about("login by oauth2 api, default is github")
                .args([Arg::new("email")
                    // .required(true)
                    .help("email")
                    .help("the email of sdf")]),
        )
        .get_matches();

    match mts.subcommand() {
        Some(("board", sub_matchs)) => board::run(sub_matchs),
        Some(("bug", sub_matchs)) => bug::run(sub_matchs),
        _ => {
            cmd.print_help().unwrap();
        }
    }
}
