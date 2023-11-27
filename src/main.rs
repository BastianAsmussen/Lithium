use crate::lang::lexer::Scanner;
use crate::utils::timer;
use crate::utils::timer::Timer;
use anyhow::Result;

mod lang;
mod utils;

fn main() -> Result<()> {
    let mut timer = Timer::new();

    let contents = std::fs::read_to_string("/home/bastian/Projects/Lithium/examples/test.lt")?;
    let mut lexer = Scanner::new(&*contents);
    let (tokens, elapsed) = timer.time(|| lexer.scan_tokens());
    println!("Lexing took {time}.", time = timer::format_time(elapsed));
    println!("Tokens: {:#?}", tokens);

    Ok(())
}
