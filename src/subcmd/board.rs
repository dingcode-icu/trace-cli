use std::fmt::Display;

use clap::ArgMatches;
use prettytable::{row, Table};

use super::engine::ccv::{ccv_board_info, ccv_type_info, CCSTraceType};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BugListType {
    All,
    Fixed, //default show unfixed
    UnFixed,
}

impl From<&str> for BugListType {
    fn from(estr: &str) -> Self {
        match estr {
            estr if estr == "all" => BugListType::All,
            estr if estr == "unfixed" => BugListType::UnFixed,
            estr if estr == "fixed" => BugListType::Fixed,
            _ => BugListType::UnFixed,
        }
    }
}

impl Display for BugListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BugListType::All => write!(f, "all"),
            BugListType::Fixed => write!(f, "fixed"),
            BugListType::UnFixed => write!(f, "unfixed"),
        }
    }
}

fn table_for_boardtype(trace_type: CCSTraceType, show_type: BugListType) {
    let mut tb = Table::new();
    tb.add_row(row!["TraceType", "BugList"]);
    let ret = ccv_type_info(show_type);
    let list = ret.get(&trace_type).unwrap();
    for k in list {
        tb.add_row(row![trace_type.to_string(), k]);
    }
    tb.printstd();
    println!(
        "[board]Trace in type = <{}>  list in show_type=<{}>  count =<{}>:3",
        trace_type.to_string(),
        show_type.to_string(),
        list.len()
    );
}

fn table_for_boardmain() {
    let (type_l, num_l, per_l, fix_l) = ccv_board_info();
    let mut tb = Table::new();
    let title_row = row!["TraceType", "Count", "Per", "Fixed"];
    tb.add_row(title_row);

    let type_num = CCSTraceType::len();
    for i in 0..type_num {
        tb.add_row(row![
            type_l[i],
            num_l[i].to_string(),
            format!("{:.4}%", (per_l[i] * 100.).to_string()),
            fix_l[i]
        ]);
    }
    tb.printstd();
}

pub fn run(args: &ArgMatches) {
    let trace_type = args.get_one::<String>("tracer_type");
    let mut buglist_type = BugListType::UnFixed;
    let r_list_type = args.try_get_one::<String>("list_type");
    if let Ok(Some(list_type)) = r_list_type {
        buglist_type = BugListType::from(list_type.as_str());
    }
    if trace_type.is_some() {
        let type_t = CCSTraceType::from(trace_type.unwrap().to_string().to_lowercase());
        table_for_boardtype(type_t, buglist_type);
        return;
    }
    table_for_boardmain();
}
