use crate::Opts;
use reqwest::Url;
use select::document::Document;
use select::predicate::Name;
use select::predicate::Predicate;
use std::collections::HashSet;
use std::path::Path;

pub struct Node {
    options: Opts,
    pub url: String,
    pub body: String,
}

impl Node {
    pub fn new(url: &str, body: &str, options: Opts) -> Node {
        Node {
            url: url.to_string(),
            body: body.to_string(),
            options,
        }
    }
    pub fn get_links_from_html(&self) -> HashSet<String> {
        Document::from(self.body.as_str())
            .find(Name("a").or(Name("link")))
            .filter_map(|n| n.attr("href"))
            // filter out the links that are images
            .filter_map(|href| {
                if self.options.same_domain {
                    let url = Url::parse(&self.url).unwrap();
                    match href.starts_with("/") || href.contains(url.domain().unwrap()) {
                        false => None,
                        true => Some(href),
                    }
                } else {
                    Some(href)
                }
            })
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
            .filter_map(|href| self.normalize_url(&href))
            .collect::<HashSet<String>>()
    }
    #[allow(dead_code)]
    // NB: this perhaps shoud be used in place of the above filter map
    fn has_extension(url: &&str) -> bool {
        Path::new(url).extension().is_none()
    }
    fn normalize_url(&self, url: &str) -> Option<String> {
        if url.starts_with("/") {
            match Url::parse(&(self.url.to_string())) {
                Ok(parsed) => {
                    let mut formatted =
                        format!("{}://{}{}", parsed.scheme(), parsed.host().unwrap(), url);
                    if formatted.ends_with("/") {
                        formatted.pop();
                    }
                    Some(formatted)
                }
                Err(_) => None,
            }
        } else {
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
}
