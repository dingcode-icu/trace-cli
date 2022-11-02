use clap::ArgMatches;
use prettytable::{row, Table};

use super::engine::ccv::{ccv_board_info, ccv_type_info, CCSTraceType};

fn table_for_boardtype(trace_type: CCSTraceType, is_showall: bool) {
    let mut tb = Table::new();
    tb.add_row(row!["TraceType", "BugList"]);
    let ret = ccv_type_info(is_showall);
    let list = ret.get(&trace_type).unwrap();
    for k in list {
        tb.add_row(row![trace_type.to_string(), k]);
    }
    tb.printstd();
    println!(
        "[board]Trace in type = <{}> still {} to fix!:3",
        trace_type.to_string(),
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
    let is_showall = args.get_flag("showall");
    if trace_type.is_some() {
        let type_t = CCSTraceType::from(trace_type.unwrap().to_string());
        table_for_boardtype(type_t, is_showall);
        return;
    }
    table_for_boardmain();
}
