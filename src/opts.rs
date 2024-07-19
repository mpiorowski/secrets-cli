use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Secrets CLI", long_about = "A CLI for managing secrets")]
pub struct Opts {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Action {
    #[command(about = "Copy the secrets")]
    Copy(Project),
    #[command(about = "Show the secrets")]
    Show(Project),
    #[command(about = "Transform to fish shell")]
    Fish(Project),
    #[command(about = "Set a secrets folder")]
    Set(Set),
    #[command(about = "Print the current configuration")]
    Config,
}

#[derive(Args, Debug, PartialEq)]
pub struct Project {
    pub project: Option<String>,
}

#[derive(Args, Debug, PartialEq)]
pub struct Set {
    pub path: PathBuf,
    pub clipboard: String,
}
