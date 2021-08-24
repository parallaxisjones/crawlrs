use crate::client::FetchProvider;
use crate::session_stats::SessionStats;
use crate::Opts;
use std::collections::HashSet;

// #[derive(Copy, Debug)]
pub struct Crawler {
    pub options: Opts,
    client: Box<dyn FetchProvider>,
    visited: Box<HashSet<String>>,
    stats: SessionStats,
}

impl Crawler {
    pub fn new(client: Box<dyn FetchProvider>, opts: Opts) -> Self {
        Self {
            options: opts,
            client,
            visited: Box::new(HashSet::<String>::new()),
            stats: SessionStats::new(),
        }
    }
    pub fn stats(&self) -> &SessionStats {
        // let mut sorted = self.visited.iter().collect::<Vec<&String>>();
        // sorted.sort();
        // self.stats.print_stats();
        // println!("{}", itertools::join(sorted, "\n"));
        &self.stats
    }
    pub fn crawl(&mut self) -> Box<HashSet<String>> {
        self.stats.start_session();
        info!("started crawl {:#?}", self.options.urls);

        let mut found_urls = self
            .options
            .urls
            .iter()
            .map(|url| {
                let root_node = self.client.fetch(&url, &self.options).unwrap();
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
                .map(|url| match self.client.fetch(url, &self.options) {
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
        self.visited.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Crawler;
    use crate::client::FetchProvider;
    use crate::error::FetchError;
    use crate::node::Node;
    use crate::Opts;
    #[test]
    fn test_crawl_same_domain_should_pass() {
        struct FakeClient {}
        impl FetchProvider for FakeClient {
            fn fetch(&self, url: &str, opts: &Opts) -> Result<Node, FetchError> {
                let content =  match url.contains("start") {
                    false => "<html><head></head><body></body></html>",
                    true => "<html><head></head><body><a href=\"https://test1.com/end\">bootles</a></body></html>",
                };
                println!("{}, {}", url, content);
                Ok(Node::new(url, content, opts.clone()))
            }
        }
        let mut session = Crawler::new(
            Box::new(FakeClient {}),
            Opts {
                urls: vec!["https://test1.com/start".to_string()],
                json: false,
                same_domain: true,
                stats: false,
            },
        );
        session.crawl();

        assert_eq!(session.visited.len(), 2);
    }

    #[test]
    fn test_crawl_same_domain_should_fail() {
        struct FakeClient {}
        impl FetchProvider for FakeClient {
            fn fetch(&self, url: &str, opts: &Opts) -> Result<Node, FetchError> {
                let content =  match url.contains("start") {
                    false => "<html><head></head><body></body></html>",
                    true => "<html><head></head><body><a href=\"https://test1.com/end\">bootles</a></body></html>",
                };
                println!("{}, {}", url, content);
                Ok(Node::new(url, content, opts.clone()))
            }
        }
        let mut session = Crawler::new(
            Box::new(FakeClient {}),
            Opts {
                urls: vec!["https://google.com".to_string()],
                json: false,
                same_domain: true,
                stats: false,
            },
        );
        session.crawl();

        assert_eq!(session.visited.len(), 1);
    }
}
