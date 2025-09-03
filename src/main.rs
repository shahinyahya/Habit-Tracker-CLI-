mod cli;
mod model;
mod store;
mod util;
mod commands;

use anyhow::Result;
use cli::{Cli, Command};
use clap::Parser;

fn main() -> Result<()>{
  let args = Cli::parse();
  commands::dispatch(args)
}