mod api;
mod client;
mod crawler;
mod error;
mod node;
mod session_stats;

use crate::api::crawl::crawl;
use crate::api::{Command, Opts};
use crate::crawler::Crawler;
use failure;
use structopt::StructOpt;
#[macro_use]
extern crate log;

extern crate convey;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

fn main() -> Result<(), failure::Error> {
    env_logger::init();
    let args = Opts::from_args();
    match args.cmd {
        Command::Crawl(crawl_args) => crawl(crawl_args),
    }
    Ok(())
}
