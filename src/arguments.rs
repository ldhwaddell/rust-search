use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    author = "Your Name",
    version = "1.0",
    about = "A simple CLI search engine",
    long_about = "This is a simple CLI search engine written in Rust. It supports searching and adding new entries."
)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(default_value = "./.search_model.json", help = "The path to the model")]
    pub path: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Enter search mode
    Search {
        /// The search query
        query: Option<String>,

        /// Interactive mode flag
        #[arg(short, long, help = "Interactive mode: stays open until you quit")]
        interactive: bool,
    },
    /// Add a new entry
    Add {
        #[arg(help = "A path to a file to add to the model")]
        path: PathBuf,
    },

}