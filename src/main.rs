use clap::{CommandFactory, Parser};

mod deserializer;
mod container;
mod cli;
mod runner;

use crate::cli::{Cli, Command};

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Some(Command::Up { service, .. }) => runner::create_and_run_containers(service),
    None => Cli::command().print_help().unwrap(),
  }
}
