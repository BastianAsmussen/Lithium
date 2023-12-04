use clap::Parser;

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

    let contents = match lexer::read_file(&args.file) {
        Ok(contents) => contents,
        Err(why) => {
            eprintln!("Failed to read file: {why}");

            std::process::exit(1);
        }
    };

    let mut lexer = lexer::Lexer::new(&contents);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(why) => {
            eprintln!("Failed to tokenize input: {why}");

            std::process::exit(1);
        }
    };
    println!("Tokens: {tokens:#?}");

    let mut parser = parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(why) => {
            eprintln!("Failed to parse tokens: {why}");

            std::process::exit(1);
        }
    };

    println!("AST: {ast:#?}");
}
