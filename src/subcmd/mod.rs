use crate::api;

pub mod board;
pub mod bug;
pub mod engine;
pub mod fix;
pub mod login;

struct TraceState<'a> {
    is_fix: bool,
    resolve_email: &'a str,
}

pub fn get_state<'c>(trace: String) -> Option<&'c String> {
    let trace_l = api::get_bugstat_list();
    if let Ok(tl) = trace_l {
        if tl.data.is_some() {
            let stat = tl.data?.get(&trace);
            return stat;
        }
    }
    None
    // let state =
}
#[test]
fn test_reg() {
    use regex::Regex;
    assert_eq!(
        Regex::new(r"(cocos2d-jsb.js:)")
            .unwrap()
            .is_match("src/cocos2d-jsb.js:47398"),
        true
    );
    // assert_eq!(, true);
}
