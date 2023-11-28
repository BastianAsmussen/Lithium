use crate::lang::lexer::{self, Lexer};
use crate::utils::timer;
use crate::utils::timer::Timer;
use clap::Parser;

mod lang;
mod utils;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to a .lt file.
    #[arg(short, long)]
    path: String,
}

fn main() {
    let mut timer = Timer::new();

    let args = Args::parse();
    let contents = match lexer::read_file(&args.path) {
        Ok(contents) => contents,
        Err(cause) => {
            eprintln!("{cause}");

            std::process::exit(1);
        }
    };

    let mut lexer = Lexer::new(&contents);
    let (tokens, elapsed) = timer.time(|| lexer.scan_tokens());
    println!("Tokens: {tokens:#?}");
    println!("Lexing took {time}.", time = timer::format_time(elapsed));
}
