use clap::ArgMatches;

pub fn run(_: &ArgMatches) {
    let ret = oauth2_cmd::login(None);
    if ret.is_ok() {
        println!("login suc!");
    }
}
