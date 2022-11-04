use std::time::Duration;

use crate::APIProcesser;
use ureq::{serde_json, Agent};
use url::Url;

const GIT_LOGINAPI: &str = "https://github.com/login/oauth/authorize";
const GIT_ACCTOKEN: &str = "https://github.com/login/oauth/access_token";
const GIT_USER: &str = "https://api.github.com/user";

const GIT_API_SEC: &str = "6d597c438885fbb72cd45f24c21db16709d1eb20";
const GIT_API_ID: &str = "5c355773a257861477cd";
const GIT_SCOPE: &str = "user:email";

/// github user-api response exampel
/// ```
/// {
/// "login": "dwbmio",
/// "id": 8945783,
/// "node_id": "MDQ6VXNlcjg5NDU3ODM=",
/// "avatar_url": "https://avatars.githubusercontent.com/u/8945783?v=4",
/// "gravatar_id": "",
/// "url": "https://api.github.com/users/dwbmio",
/// "html_url": "https://github.com/dwbmio",
/// "followers_url": "https://api.github.com/users/dwbmio/followers",
/// "following_url": "https://api.github.com/users/dwbmio/following{/other_user}",
/// "gists_url": "https://api.github.com/users/dwbmio/gists{/gist_id}",
/// "starred_url": "https://api.github.com/users/dwbmio/starred{/owner}{/repo}",
/// "subscriptions_url": "https://api.github.com/users/dwbmio/subscriptions",
/// "organizations_url": "https://api.github.com/users/dwbmio/orgs",
/// "repos_url": "https://api.github.com/users/dwbmio/repos",
/// "events_url": "https://api.github.com/users/dwbmio/events{/privacy}",
/// "received_events_url": "https://api.github.com/users/dwbmio/received_events",
/// "type": "User",
/// "site_admin": false,
/// "name": "dwbmio",
/// "company": "Home",
/// "blog": "https://dwb.ren",
/// "location": "BJ ICU ",
/// "email": "dwb@dwb.ren",
/// "hireable": null,
/// "bio": "have fun :3",
/// "twitter_username": null,
/// "public_repos": 29,
/// "public_gists": 0,
/// "followers": 0,
/// "following": 1,
/// "created_at": "2014-09-28T08:24:11Z",
/// "updated_at": "2022-10-31T09:16:50Z"
/// }
/// ```

#[derive(Default, Clone, Copy)]
pub struct GithubProcesser {}

impl GithubProcesser {
    pub fn get_login_url(red_uri: String) -> Url {
        let mut url = url::Url::parse((String::from(GIT_LOGINAPI)).as_str()).unwrap();
        url.set_query(Some(
            format!(
                "client_secret={}&client_id={}&scopes={scope}&note={scope}&redirect_uri={url}",
                GIT_API_SEC,
                GIT_API_ID,
                scope = GIT_SCOPE,
                url = red_uri
            )
            .as_str(),
        ));
        url
    }

    pub fn get_acctoken_url(code: String, red_uri: String) -> Url {
        let mut url = url::Url::parse((String::from(GIT_ACCTOKEN)).as_str()).unwrap();
        url.set_query(Some(
            format!(
                "client_secret={}&client_id={}&redirect_uri={url}&code={code}",
                GIT_API_SEC,
                GIT_API_ID,
                code = code,
                url = red_uri
            )
            .as_str(),
        ));
        url
    }

    pub fn get_userinfo_url() -> Url {
        url::Url::parse(GIT_USER).unwrap()
    }

    pub fn get_httpagent() -> Agent {
        let ag = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(15))
        .build();
        ag
    }
}

impl APIProcesser for GithubProcesser {
    fn webbrowser_login(&self, red_uri: String) {
        let url = GithubProcesser::get_login_url(red_uri);
        opener::open(url.as_str()).unwrap()
    }

    fn api_login(&self, red_uri: String) -> Result<String, ureq::Error> {
        let url = GithubProcesser::get_login_url(red_uri);
        println!("[github]url is {}", &url.as_str());
        let resp = GithubProcesser::get_httpagent()
            .get(url.as_str())
            .call()?
            .into_string()?;
        println!("[github]resp is {}", &resp);

        Ok(resp)
    }

    fn api_gettoken(&self, code: String, red_uri: String) -> Result<String, ureq::Error> {
        let url = GithubProcesser::get_acctoken_url(code, red_uri);
        let resp = GithubProcesser::get_httpagent()
            .post(url.as_str())
            .call()?
            .into_string()?;

        Ok(resp)
    }

    fn api_userinfo(&self, token: String) -> Result<serde_json::Value, ureq::Error> {
        let url = GithubProcesser::get_userinfo_url();
        let resp: serde_json::Value = ureq::get(url.as_str())
            .set("Authorization", token.as_str())
            .call()?
            .into_json()?;
        Ok(resp)
    }
}
