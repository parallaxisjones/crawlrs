use crate::api::crawl::CrawlOpts;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "crawlrs")]
pub struct Opts {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug, Clone)]
pub enum OutputFormat {
    Json,
    Text,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "text" => Ok(OutputFormat::Text),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

#[derive(StructOpt, Debug, Clone)]
pub enum Command {
    Crawl(CrawlOpts),
}

pub mod crawl;
