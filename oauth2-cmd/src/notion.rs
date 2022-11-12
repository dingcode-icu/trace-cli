use crate::iapi_processer::APIProcesser;

#[derive(Default, Clone, Copy)]
pub struct NotionProcesser {}

impl NotionProcesser {}

impl APIProcesser for NotionProcesser {
    fn webbrowser_login(&self, red_uri: String) {
        todo!()
    }

    fn api_login(&self, red_uri: String) -> Result<String, ureq::Error> {
        todo!()
    }

    fn api_gettoken(&self, code: String, red_uri: String) -> Result<String, ureq::Error> {
        todo!()
    }

    fn api_userinfo(&self, token: String) -> Result<ureq::serde_json::Value, ureq::Error> {
        todo!()
    }
}
