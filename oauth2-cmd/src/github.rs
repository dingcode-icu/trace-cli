use crate::APIProcesser;

#[derive(Default)]
pub struct GithubProcesser {}

impl APIProcesser for GithubProcesser {
    fn chk_loc_token(&self) -> bool {
        return false;
    }

    fn clear(&self) {
        println!("clear tokne in local!")
    }
}
