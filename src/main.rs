#![doc = include_str!("../README.md")]

pub mod cli;
pub mod errors;
pub mod macros;
pub mod parser;
mod status;
pub mod utils;

use crate::status::Status;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let matches = cli::cmd().get_matches();
    let files: Vec<&PathBuf> = matches.get_many("files").unwrap().collect();
    let output = |orig: &PathBuf| -> PathBuf {
        matches
            .get_one("output")
            .cloned()
            .or_else(|| {
                matches.get_one::<PathBuf>("out-dir").map(|p| {
                    p.join(orig.file_name().expect("valid file name"))
                        .with_extension("mcfunction")
                })
            })
            .unwrap_or_else(|| orig.with_extension("mcfunction"))
    };

    let mut total = 0;
    let mut amount = 0;
    let mut failed = 0;
    for path in &files {
        let now = Instant::now();
        println!("{} {}", Status::Transpiling, path.display());
        match parser::parse_file(&path) {
            Err(e) => {
                eprintln!("{} in {}: {}", Status::Error, path.display(), e);
                failed += 1;
            }
            Ok(result) => {
                let took = now.elapsed().as_micros();
                let dest = output(path);
                println!("{} {} in {}μs", Status::Transpiled, path.display(), took);
                if let Err(e) = fs::write(&dest, result) {
                    eprintln!(
                        "{} while writing to {}: {}",
                        Status::Error,
                        dest.display(),
                        e
                    );
                    failed += 1;
                };
                total += took;
                amount += 1;
            }
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
