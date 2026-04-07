#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod mft_parser;
mod handle;
mod tree;

use std::sync::OnceLock;
use clap::Parser;
use crate::tree::build_tree;

static VERBOSE: OnceLock<bool> = OnceLock::new();

#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {
        if crate::is_verbose() {
            println!($($arg)*);
        }
    };
}

#[derive(Parser, Debug)]
struct Args {
    path: String,

    #[arg(short, long)]
    verbose: bool,

    #[arg(long)]
    export: bool,
}

fn main() {
    let args = Args::parse();
    if VERBOSE.set(args.verbose).is_err() {
        eprintln!("[*]: Verbose was already set");
    }

    let results = mft_parser::parse(&args.path);

    let safe_results = match results {
        Ok(results) => results,
        Err(e) => {
            vprintln!("Unable to parse MFT file:\n{}", e);
            return;
        }
    };

    build_tree(&safe_results);
}

pub fn is_verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}
