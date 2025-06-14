use clap::{CommandFactory, Parser};

mod deserializer;
mod container;
mod cli;
mod runner;
mod utils;

use crate::cli::{Cli, Command};

fn main() {
  let cli = Cli::parse();

  match cli.command {
    // TODO: Add error handling + exit code
    Some(Command::Up { .. }) => runner::run_services(cli.file),
    Some(Command::Down) => runner::stop_and_remove_services(cli.file),
    None => Cli::command().print_help().unwrap(),
  }
}
