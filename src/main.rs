use clap::Parser;
use lang::semantics::SemanticAnalyzer;

/// The Lithium compiler CLI.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The input file.
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let contents = match lang::lexer::read_file(&args.file) {
        Ok(contents) => contents,
        Err(why) => {
            eprintln!("Failed to read file: {why}");

            std::process::exit(1);
        }
    };

    let mut lexer = lang::lexer::Lexer::new(&contents);
    let (result, elapsed) = {
        let start = std::time::Instant::now();

        (lexer.tokenize(), start.elapsed())
    };
    let tokens = match result {
        Ok(tokens) => tokens,
        Err(why) => {
            eprintln!("Failed to tokenize input: {why}");

            std::process::exit(1);
        }
    };

    println!("Tokens: {tokens:#?}");
    println!("Lexical analysis took {elapsed:#?}.");

    let mut parser = lang::parser::Parser::new(&tokens);
    let (result, elapsed) = {
        let start = std::time::Instant::now();

        (parser.parse(), start.elapsed())
    };
    let ast = match result {
        Ok(statements) => statements,
        Err(why) => {
            eprintln!("Failed to parse input: {why}");

            std::process::exit(1);
        }
    };

    println!("AST: {ast:#?}");
    println!("Parsing took {elapsed:#?}.");

    let mut semantics = SemanticAnalyzer::new(&ast);
    let (result, elapsed) = {
        let start = std::time::Instant::now();

        (semantics.analyze(), start.elapsed())
    };
    match result {
        Ok(()) => {}
        Err(why) => {
            eprintln!("Failed to analyze input: {why}");

            std::process::exit(1);
        }
    }

    println!("Semantics: {semantics:#?}");
    println!("Semantic analysis took {elapsed:#?}.");
}
