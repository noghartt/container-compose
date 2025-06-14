use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Command>
}

#[derive(Subcommand)]
pub enum Command {
  #[command(about = "Create and run containers")]
  // Usage:  docker compose up [OPTIONS] [SERVICE...]
  // Follow this structure for the Up command
  Up {
    #[arg(short, long)]
    detach: bool,

    #[arg()]
    service: Vec<String>,
  }
}
