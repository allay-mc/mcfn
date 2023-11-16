pub mod cli;
pub mod errors;
pub mod macros;
pub mod parser;
pub mod utils;

use std::io;
use std::os::fd::AsFd;
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    let matches = cli::cmd().get_matches();
    let files: Vec<&PathBuf> = matches.get_many("files").unwrap().collect();

    let mut total = 0;
    let mut amount = 0;
    let mut failed = 0;
    for path in &files {
        let now = Instant::now();
        if let Err(e) = parser::parse_file(&path) {
            let style = anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)));
            let _ = style.write_to(&mut stderr);
            eprintln!("Error while parsing {}: {}", path.display(), e);
            let _ = style.write_reset_to(&mut stderr);
            failed += 1;
        } else {
            let took = now.elapsed().as_micros();
            println!("Transpiled {} in {}μs", path.display(), took);
            total += took;
            amount += 1;
        }
    }
    if files.len() > 1 && amount > 0 {
        let style = anstyle::Style::new()
            .bold()
            .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green)));
        let _ = style.write_to(&mut stdout);
        println!(
            "Transpiled {}/{} files in {}μs",
            amount,
            amount + failed,
            total
        );
        let _ = style.write_reset_to(&mut stdout);
    }

    if failed > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
