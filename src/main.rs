use reqwest::Url;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Predicate;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::io::Read;
use std::time::Instant;

#[derive(Debug)]
struct FetchError {
    details: String,
}

struct Node {
    url: String,
    body: String,
}
struct Session {
    root: String,
    started_at: Instant,
    visited: HashSet<String>,
    client: reqwest::blocking::Client,
}

impl Session {
    fn new(site: &str) -> Session {
        Session {
            started_at: Instant::now(),
            client: reqwest::blocking::Client::new(),
            visited: HashSet::<String>::new(),
            root: site.to_string(),
        }
    }
    fn fetch(&self, site: &str) -> Result<Node, FetchError> {
        let mut res = match self.client.get(site).send() {
            Ok(res) => res,
            Err(_e) => return Err(FetchError::new("error getting content")),
        };
        println!("Status for {}: {}", site, res.status());
        let mut body = String::new();
        match res.read_to_string(&mut body) {
            Ok(_) => Ok(Node::new(site, &body)),
            Err(_e) => return Err(FetchError::new("malformed content")),
        }
    }
    fn crawl(mut self) {
        println!("{}", self.started_at.elapsed().as_secs());
        let root_node = self.fetch(&self.root).unwrap();
        let found_urls = root_node.get_links_from_html();
        self.visited.insert(root_node.url.to_string());
        let mut new_urls = found_urls
            .difference(&self.visited)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        while !new_urls.is_empty() {
            let found_urls: HashSet<String> = new_urls
                .iter()
                .map(|url| match self.fetch(url) {
                    Ok(node) => {
                        let links = node.get_links_from_html();
                        println!("Visited: {} found {} links", url, links.len());
                        links
                    }
                    Err(_e) => {
                        println!("malformed response {}", url);
                        HashSet::<String>::new()
                    }
                })
                .fold(HashSet::<String>::new(), |mut acc, links| {
                    acc.extend(links);
                    acc
                });
            // this moves the found_urls, so we need to make a new set
            self.visited.extend(new_urls);
            new_urls = found_urls
                .difference(&self.visited)
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
            println!("New urls: {}", new_urls.len())
        }
        println!("URLs: {:#?}", found_urls);
    }
}
impl Node {
    fn new(url: &str, body: &str) -> Node {
        Node {
            url: url.to_string(),
            body: body.to_string(),
        }
    }
    fn get_links_from_html(&self) -> HashSet<String> {
        Document::from(self.body.as_str())
            .find(Name("a").or(Name("link")))
            .filter_map(|n| n.attr("href"))
            // filter out the links that are images
            .filter_map(|href| {
                match href.contains(".png")
                    || href.contains(".jpg")
                    || href.contains(".jpeg")
                    || href.contains(".gif")
                    || href.contains(".svg")
                    || href.contains(".ico")
                    || href.contains(".pdf")
                    || href.contains(".css")
                    || href.contains(".js")
                {
                    false => Some(href),
                    true => None,
                }
            })
            .filter_map(Node::normalize_url)
            .collect::<HashSet<String>>()
    }
    fn normalize_url(url: &str) -> Option<String> {
        match Url::parse(url) {
            Ok(new_url) => {
                if new_url.has_host() {
                    Some(url.to_string())
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl FetchError {
    fn new(msg: &str) -> FetchError {
        FetchError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FetchError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn main() {
    let session = Session::new("https://parkerjones.dev/");
    session.crawl();
}
