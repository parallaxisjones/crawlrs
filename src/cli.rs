use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "crawlrs")]
pub struct Opts {
    #[structopt(short = "u", long = "urls", help = "The url to start crawling from")]
    pub urls: Vec<String>,
    #[structopt(long = "same-domain", help = "Only crawl pages from the same domain")]
    pub same_domain: bool,
}
