use crate::node::Node;
use crate::Opts;
use chrono::Duration;
use chrono::{DateTime, Utc};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::io::Read;

// #[derive(Copy, Debug)]
pub struct Session {
    options: Opts,
    visited: Box<HashSet<String>>,
    client: reqwest::blocking::Client,
    stats: SessionStats,
}

#[derive(Debug)]
struct SessionStats {
    total_visited: u64,
    started_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
}

impl SessionStats {
    fn new() -> SessionStats {
        SessionStats {
            total_visited: 0,
            started_at: None,
            finished_at: None,
        }
    }

    fn add_visit(&mut self, count: Option<u64>) {
        match count {
            Some(visited) => self.total_visited += visited as u64,
            None => self.total_visited += 1,
        };
    }
    fn start_session(&mut self) {
        self.started_at = Some(Utc::now());
    }
    fn finish_session(&mut self) {
        self.finished_at = Some(Utc::now());
    }
    fn elapsed_time(&self) -> Duration {
        match (self.started_at, self.finished_at) {
            (Some(started), Some(finished)) => finished - started,
            (Some(started), None) => Utc::now() - started,
            _ => panic!("Session has not been started or finished"),
        }
    }
    fn print_stats(&self) {
        println!(
            "Start: {}\nEnd: {}\nTotal visited: {}\nElapsed Time: {} sec",
            self.started_at.unwrap(),
            self.finished_at.unwrap(),
            self.total_visited,
            self.elapsed_time().num_seconds()
        );
    }
}

impl Session {
    pub fn new(opts: Opts) -> Session {
        Session {
            options: opts,
            client: reqwest::blocking::Client::new(),
            visited: Box::new(HashSet::<String>::new()),
            stats: SessionStats::new(),
        }
    }
    fn fetch(&self, site: &str) -> Result<Node, FetchError> {
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
            Ok(_) => Ok(Node::new(site, &body, &self.options)),
            Err(_e) => return Err(FetchError::new("malformed content")),
        }
    }
    pub fn stats(&self) {
        let mut sorted = self.visited.iter().collect::<Vec<&String>>();
        sorted.sort();
        self.stats.print_stats();
        println!("{}", itertools::join(sorted, "\n"));
    }
    pub fn crawl(&mut self) {
        self.stats.start_session();
        info!("started crawl {:#?}", self.options.urls);

        let mut found_urls = self
            .options
            .urls
            .iter()
            .map(|url| {
                let root_node = self.fetch(&url).unwrap();
                let links = root_node.get_links_from_html();
                links
            })
            .fold(HashSet::<String>::new(), |mut acc, links| {
                acc.extend(links);
                acc
            });
        self.visited.extend(self.options.urls.iter().cloned());
        self.stats.add_visit(Some(self.options.urls.len() as u64));
        let mut new_urls = found_urls
            .difference(&self.visited)
            .map(|x| x.to_string())
            .collect::<HashSet<String>>();
        while !new_urls.is_empty() {
            found_urls = new_urls
                .iter()
                .map(|url| match self.fetch(url) {
                    Ok(node) => {
                        let links = node.get_links_from_html();
                        info!("Visited: {} found {} links", node.url, links.len());
                        links
                    }
                    Err(_e) => HashSet::<String>::new(),
                })
                .fold(HashSet::<String>::new(), |mut acc, links| {
                    acc.extend(links);
                    acc
                });
            self.stats.add_visit(Some(new_urls.len() as u64));
            // this moves the found_urls, so we need to make a new set
            self.visited.extend(new_urls);
            new_urls = found_urls
                .difference(&self.visited)
                .map(|x| x.to_string())
                .collect::<HashSet<String>>();
        }
        self.stats.finish_session();
    }
}

#[derive(Debug)]
struct FetchError {
    details: String,
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
