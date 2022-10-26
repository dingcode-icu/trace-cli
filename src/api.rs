pub mod api {

    use std::{sync::Mutex, time::Duration};

    use lazy_static::lazy_static;
    use log::error;
    use ureq::{serde::Deserialize, Agent};

    use url::Url;

    #[derive(Debug, Deserialize)]
    pub struct Resp<T> {
        pub code: u16,
        pub msg: String,
        pub data: Option<T>,
    }

    struct ReqConfig {
        pub host: String,
    }

    lazy_static! {
        static ref http_agent: Agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(5))
            .timeout_read(Duration::from_secs(5))
            .build();
        static ref BASEINFO: Mutex<ReqConfig> = Mutex::new(ReqConfig {
            host: String::from("https://tracer.bbclient.icu "),
        });
    }

    fn url_parse(path: &str) -> Result<Url, url::ParseError> {
        let h = &BASEINFO.lock().unwrap().host;
        Url::parse(h)?.join(path)
    }

    ///api:[get]buglist
    pub fn get_buglist(fmtstr: String) -> Result<Resp<Vec<String>>, ureq::Error> {
        let url = url_parse("/api/trace/buglist");

        if let Err(r) = url {
            error!("[http get_butlist]parse url raise error!");
            let err = ureq::Error::from(r);
            return Err(err);
        }
        let ret: Resp<Vec<String>> = ureq::get(url.unwrap().as_str())
            .query("trace_key", &fmtstr)
            .set("Content-Type", "application/json;")
            .call()?
            .into_json()?;

        Ok(ret)
    }

    

    ///api:[get]buginfo
    pub fn get_buginfo(
        bugkey: String,
        pageidx: u32,
        pagenum: u32,
    ) -> Result<Resp<Vec<String>>, ureq::Error> {
        let url = url_parse("/v1/ccv2");
        if let Err(r) = url {
            error!("[http get_butlist]parse url raise error!");
            let err = ureq::Error::from(r);
            return Err(err);
        }
        let ret: Resp<Vec<String>> = ureq::get(url.unwrap().as_str())
            .query("pageidx", pageidx.to_string().as_str())
            .query("pagenum", pagenum.to_string().as_str())
            .set("Content-Type", "application/json;")
            .send_string(bugkey.as_str())?
            .into_json()?;

        Ok(ret)
    }
}

#[test]
fn test_api() {
    use self::api::get_buglist;
    use log::info;
    let ret = get_buglist("*".to_string());
    info!("ret is {:?}", ret);
}
