use github::GithubProcesser;

/// Useful  login library for command tool by oauth2 api
/// oath2 API include but not limit to
/// * github
/// * ...
///
mod github;

pub enum API {
    Github,
    Dingcode,
}
pub(crate) trait APIProcesser {
    fn chk_loc_token(&self) -> bool {
        return false;
    }

    fn clear(&self) {
        println!("clear tokne in local!")
    }
}

pub fn login(api: Option<API>) {
    let api_type = api.unwrap_or(API::Github);

    let api_procer = match api_type {
        API::Github => GithubProcesser::default(),
        _ => GithubProcesser::default(),
    };
    api_procer.chk_loc_token();
    // api_procer.
}

#[test]
fn test_getenv() {
    use std::env::home_dir;
    println!("home dir is {}", home_dir().unwrap().display());
}
