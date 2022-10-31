use clap::ArgMatches;
use prettytable::{row, Table};

use super::engine::ccv::{ccv_board_info, ccv_type_info, CCSTraceType};

fn tabel_for_boardtype(trace_type: CCSTraceType) {
    let mut tb = Table::new();
    tb.add_row(row!["TraceType", "BugList"]);
    let ret = ccv_type_info();
    let list = ret.get(&trace_type).unwrap();
    for k in list {
        tb.add_row(row![trace_type.to_string(), k]);
    }
    tb.printstd();
}

fn table_for_boardmain() {
    let (type_l, num_l, per_l) = ccv_board_info();
    let mut tb = Table::new();
    tb.add_row(row!["TraceType", "Count", "Per"]);
    tb.add_row(row![
        type_l[0],
        num_l[0].to_string(),
        format!("{:.4}%", (per_l[0] * 100.).to_string())
    ]);
    tb.add_row(row![
        type_l[1],
        num_l[1].to_string(),
        format!("{:.4}%", (per_l[1] * 100.).to_string())
    ]);
    tb.add_row(row![
        type_l[2],
        num_l[2].to_string(),
        format!("{:.4}%", (per_l[2] * 100.).to_string())
    ]);
    tb.add_row(row![
        type_l[3],
        num_l[3].to_string(),
        format!("{:.4}%", (per_l[3] * 100.).to_string())
    ]);
    tb.printstd();
}

pub fn run(args: &ArgMatches) {
    let trace_type = args.get_one::<String>("trace_type");
    if trace_type.is_some() {
        let type_t = CCSTraceType::from(trace_type.unwrap().to_string());
        tabel_for_boardtype(type_t);
        return;
    }
    table_for_boardmain();
}
