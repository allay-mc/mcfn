pub mod cli;
pub mod errors;
pub mod macros;
pub mod parser;
pub mod utils;

use std::path::PathBuf;
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    let matches = cli::cmd().get_matches();
    let files: Vec<&PathBuf> = matches.get_many("files").unwrap().collect();

    let mut total = 0;
    for path in &files {
        let now = Instant::now();
        parser::parse_file(&path)?;
        let took = now.elapsed().as_micros();
        println!("Transpiled {} in {}μs", path.display(), took);
        total += took;
    }
    if files.len() > 1 {
        println!("Transpiled files in {}μs", total);
    }

    Ok(())
}
