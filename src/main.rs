use crate::cli::Opts;
use crate::crawler::Crawler;
use crate::output::CrawlOutput;
use failure;
use structopt::StructOpt;
#[macro_use]
extern crate log;

extern crate convey;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use convey::{human, json};

mod cli;
mod client;
mod crawler;
mod error;
mod node;
mod output;
mod session_stats;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let args = Opts::from_args();
    let out = if args.json {
        convey::new().add_target(json::stdout()?)?
    } else {
        convey::new().add_target(human::stdout()?)?
    };
    let client = Box::new(client::CrawlrsClient::new());

    let mut session = Crawler::new(client, args);
    let links = session.crawl();
    let mut sorted = links.iter().cloned().collect::<Vec<String>>();
    sorted.sort();
    let output = CrawlOutput {
        links: sorted,
        seeds: session.options.urls.clone(),
        stats: Some(*session.stats()),
    };
    out.print(serde_json::to_string_pretty(&output).unwrap())?;
    Ok(())
}
