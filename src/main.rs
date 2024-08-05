use clap::Parser;

use rust_search::cli::{Arguments, Commands};

fn main() {
    let args = Arguments::parse();

    match &args.command {
        Commands::Search { query, interactive } => search_mode(query, interactive),
        _ => (),
    }

    fn search_mode(query: &Option<String>, interactive: &bool) {
        if !interactive {
            // Query model here
            return;
        }

        // Start interactive session
    }
}
