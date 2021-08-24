use crate::error::Error;
use crate::node::Node;
use crate::Opts;
use std::io::Read;

pub type Result<T> = std::result::Result<T, Error>;

pub trait FetchProvider {
    fn fetch(&self, url: &str, opts: &Opts) -> Result<Node>;
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
    fn fetch(&self, url: &str, options: &Opts) -> Result<Node> {
        let mut res = self.client.get(url).send().map_err(|e| (url, e))?;

        if !res.status().is_success() {
            error!("{}: {}", res.status(), url);
        } else {
            info!("{}: {}", res.status(), url);
        }

        let mut body = String::new();
        res.read_to_string(&mut body).map_err(|e| (url, e))?;

        Ok(Node::new(url, &body, options.clone()))
    }
}
