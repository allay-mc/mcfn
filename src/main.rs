pub mod cli;
pub mod errors;
pub mod macros;
pub mod parser;
mod status;
pub mod utils;

use crate::status::Status;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let matches = cli::cmd().get_matches();
    let files: Vec<&PathBuf> = matches.get_many("files").unwrap().collect();

    let mut total = 0;
    let mut amount = 0;
    let mut failed = 0;
    for path in &files {
        let now = Instant::now();
        println!("{} {}", Status::Transpiling, path.display());
        if let Err(e) = parser::parse_file(&path) {
            eprintln!("{} in {}: {}", Status::Error, path.display(), e);
            failed += 1;
        } else {
            let took = now.elapsed().as_micros();
            println!("{} {} in {}μs", Status::Transpiled, path.display(), took);
            total += took;
            amount += 1;
        }
    }
    if files.len() > 1 && amount > 0 {
        println!(
            "{} transpiling {}/{} files in {}μs",
            Status::Finished,
            amount,
            amount + failed,
            total
        );
    }

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
