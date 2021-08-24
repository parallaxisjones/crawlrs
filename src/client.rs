use crate::error::FetchError;
use crate::node::Node;
use crate::Opts;
use std::io::Read;

pub trait FetchProvider {
    fn fetch(&self, url: &str, opts: &Opts) -> Result<Node, FetchError>;
}

pub struct CrawlrsClient {
    client: reqwest::blocking::Client,
}

impl CrawlrsClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl FetchProvider for CrawlrsClient {
    fn fetch(&self, site: &str, options: &Opts) -> Result<Node, FetchError> {
        let mut res = match self.client.get(site).send() {
            Ok(res) => res,
            Err(_e) => return Err(FetchError::new("error getting content")),
        };
        if !res.status().is_success() {
            error!("{}: {}", res.status(), site);
            return Err(FetchError::new("404"));
        } else {
            info!("{}: {}", res.status(), site);
        }
        let mut body = String::new();
        match res.read_to_string(&mut body) {
            Ok(_) => Ok(Node::new(site, &body, options.clone())),
            Err(_e) => return Err(FetchError::new("malformed content")),
        }
    }
}
