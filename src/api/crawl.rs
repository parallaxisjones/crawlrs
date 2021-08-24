use crate::api::OutputFormat;
use crate::client::CrawlrsClient;
use crate::session_stats::SessionStats;
use crate::Crawler;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub struct CrawlOpts {
    #[structopt(short = "u", long = "urls", help = "The url(s) to start crawling from")]
    pub urls: Vec<String>,
    #[structopt(
        short = "s",
        long = "same-domain",
        help = "Only crawl pages from the same domain"
    )]
    pub same_domain: bool,
    #[structopt(long = "stats", help = "Include session stats in output")]
    pub stats: bool,
    #[structopt(short = "o", long = "json", help = "output format")]
    pub output: Option<OutputFormat>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrawlOutput {
    pub seeds: Vec<String>,
    pub links: Vec<String>,
    pub stats: Option<SessionStats>,
}

pub fn crawl(craw_opts: CrawlOpts) {
    let client = Box::new(CrawlrsClient::new());
    let mut session = Crawler::new(client, craw_opts);
    let links = session.crawl();
    let mut sorted = links.iter().cloned().collect::<Vec<String>>();
    sorted.sort();
    serde_json::to_string_pretty(&CrawlOutput {
        links: sorted,
        seeds: session.options.urls.clone(),
        stats: Some(*session.stats()),
    })
    .unwrap();
}
