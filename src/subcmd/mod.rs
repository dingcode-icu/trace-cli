pub mod board;
pub mod bug;
pub mod engine;
pub mod fix;
pub mod login;

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
