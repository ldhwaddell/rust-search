use std::process::ExitCode;

use clap::Parser;
use std::error::Error;

use rust_search::arguments::{Arguments, Commands};
use rust_search::model::Model;

fn run() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();

    // Build the model from user or default path
    let mut model = Model::from(&args.path)?;

    match &args.command {
        Commands::Add { path } => model.add(path)?,
        _ => (),
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}
