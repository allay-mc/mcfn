use std::path::PathBuf;

use clap::{Arg, Command};

pub fn cmd() -> Command {
    Command::new("mcfn")
        .about("mcfunction preprocessor")
        .author("Jonas da Silva")
        .arg(
            Arg::new("files")
                .help("The paths to the file that should be converted")
                .num_args(..)
                .value_parser(clap::value_parser!(PathBuf)),
        )
}
