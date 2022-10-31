use oauth2_cmd::{login, API};

fn main() {
    login(Some(API::Github));
}
