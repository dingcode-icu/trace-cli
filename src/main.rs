use subcmd::{bug, fix, login};

use crate::subcmd::board;

use clap::{self, arg, Arg, Command};

mod api;
mod subcmd;

fn main() {
    let mut cmd = Command::new("tracer-cli");
    let mts = cmd
        .clone()
        .version("0.1.0")
        .author("dwb <dwb@dwb.ren>")
        .about("command for tracer")
        .subcommand(
            Command::new("board")
                .about("show tracer overview info")
                .args([Arg::new("tracer_type")
                    .short('t')
                    .help("the tracer type to show single type details")]),
        )
        .subcommand(
            Command::new("bug")
                .about("show the fliterd detail of bug by input")
                .args([Arg::new("tracer")
                    .required(true)
                    .help("the tracer key to show single bug details")]),
        )
        .subcommand(
            Command::new("fix")
                .about("show the tracer stat record")
                .args([Arg::new("tracer")
                    .required(true)
                    .help("the tracer key to show single bug details")]),
        )
        .subcommand(Command::new("login").about("login by oauth2 api, default *API* IS GITHUB"))
        .get_matches();

    match mts.subcommand() {
        Some(("board", sub_matchs)) => board::run(sub_matchs),
        Some(("bug", sub_matchs)) => bug::run(sub_matchs),
        Some(("login", sub_matchs)) => login::run(sub_matchs),
        Some(("fix", sub_matchs)) => fix::run(sub_matchs),
        _ => {
            cmd.print_help().unwrap();
        }
    }
}
