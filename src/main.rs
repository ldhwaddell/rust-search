use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use rust_search::arguments::{Arguments, Commands};
use rust_search::model::Model;

fn run() -> Result<(), Box<dyn Error>> {
    let args = Arguments::parse();

    // Build the model from user or default path
    let mut model = Model::from(&args.path)?;

    match &args.command {
        Commands::Add { path } => model.add(path)?,
        Commands::Search { query, interactive } => {
            if *interactive {
                run_interactive_search(&model)?;
            } else if let Some(q) = query {
                let results = model.query(q);
                format_result(&results);
            } else {
                eprintln!("No query provided.");
            }
        }
    }

    Ok(())
}

fn run_interactive_search(model: &Model) -> Result<(), Box<dyn Error>> {
    use std::io::{self, Write};

    loop {
        print!("Enter your search query (or type ':q' to exit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        if !input.is_empty() {
            let results = model.query(&input);
            format_result(&results);
        }
    }

    Ok(())
}

fn format_result(results: &Vec<(f32, &PathBuf)>) {
    if results.len() == 0 {
        println!("No documents matched your query.")
    } else {
        for (index, (score, path)) in results.iter().enumerate() {
            println!("{}. {:.2} - {}", index + 1, score, path.display());
        }
    }
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
