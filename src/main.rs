use crate::subcmd::board;
use clap::{self, Arg, ArgAction, Command};
use subcmd::{bug, fix, login};
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
                .args([
                    Arg::new("tracer_type")
                        .short('t')
                        .help("the tracer type to show single type details"),
                    Arg::new("showall")
                        .short('a')
                        .long("showall")
                        .help("if show all bug trace include stat is fixed")
                        .action(ArgAction::SetTrue),
                ]),
        )
        .subcommand(
            Command::new("bug")
                .about("show the fliterd detail of bug by input")
                .args([
                    Arg::new("tracer")
                        .required(true)
                        .help("the tracer key to show single bug details"),
                    Arg::new("count")
                        .required(false)
                        .help("the max count to get bug cellinfo list.default is only 1"),
                ]),
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
