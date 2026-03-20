#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod mft_parser;

use std::sync::OnceLock;
use clap::Parser;

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
}

fn main() {
    let args = Args::parse();
    if VERBOSE.set(args.verbose).is_err() {
        eprintln!("[*]: Verbose was already set");
    }

    mft_parser::parse(&args.path);
}

pub fn is_verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}
