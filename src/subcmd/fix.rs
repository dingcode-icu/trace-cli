use clap::ArgMatches;
use log::warn;
use serde_json::Value;

pub fn run(args: &ArgMatches){
    //require
    let trace = args.get_one::<String>("trace").unwrap();

}