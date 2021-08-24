use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "crawlrs")]
pub struct Opts {
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
    #[structopt(long = "json", help = "output json")]
    pub json: bool,
}
