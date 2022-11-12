use oauth2_cmd::{is_login, login, logout, API};

fn main() {
    //test clean
    let is = is_login();
    if is {
        logout();
    }
    assert_eq!(is_login(), false);
    //clean first

    let ret = login(Some(API::Github));
    println!(
        "login of api <github> {}",
        ret.expect("<github> test failed!")
    );
}
