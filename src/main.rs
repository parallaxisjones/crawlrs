use crate::cli::Opts;
use crate::session::Session;
use structopt::StructOpt;
#[macro_use]
extern crate log;

mod cli;
mod node;
mod session;

fn main() {
    env_logger::init();
    let args = Opts::from_args();
    let mut session = Session::new(args);
    session.crawl();
    session.stats();
}
