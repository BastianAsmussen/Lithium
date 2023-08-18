use crate::util::timer::{format_time, Timer};
use std::io::Write;
use std::path::Path;

mod lexer;
mod parser;

/// The Lithium program.
pub struct Lithium {
    pub had_error: bool,
}

impl Lithium {
    /// Create a new Lithium program.
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, file_path: &str) {
        if let Err(error) = self.is_valid_file(file_path) {
            println!("Error: {}", error);
            return;
        }

        let content = std::fs::read_to_string(file_path).expect("Unable to read file!");

        self.run(content);
    }

    pub fn run_prompt(&mut self) {
        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            self.run(input);
        }
    }

    /// Check if a file is a valid Lithium file.
    ///
    /// A valid Lithium file is a file with the '.lt' extension.
    ///
    /// # Arguments
    /// * `file_path` - The path to the file to check.
    ///
    /// # Returns
    /// * `Ok(())` if the file is valid.
    /// * `Err(&str)` if the file is not valid.
    fn is_valid_file(&self, file_path: &str) -> Result<(), &str> {
        let path = Path::new(file_path);

        // Check if the path points to an existing file
        if !path.exists() || !path.is_file() {
            return Err("The file does not exist.");
        }

        // Check if the file has the '.lt' extension
        if let Some(extension) = path.extension() {
            if extension != "lt" {
                return Err("The file is not a valid Lithium file.");
            }
        } else {
            return Err("The file is not a valid Lithium file.");
        }

        Ok(())
    }

    /// Run the Lithium program.
    ///
    /// # Arguments
    /// * `source` - The source code to run.
    pub fn run(&mut self, source: String) {
        let mut timer = Timer::new();

        // Tokenize the source code.
        println!("Tokenizing...");
        let (time, tokens) = timer.time(|| lexer::Scanner::new(&source).scan_tokens());

        println!("Tokens:\n{:#?}", tokens);
        println!("Tokenization took {}.", format_time(time));

        // // Parse the tokens.
        // println!("Parsing...");
        // let (time, syntax_tree) = timer.time(|| parser::Parser::new(&tokens).parse());

        // println!("Syntax tree:\n{:#?}", syntax_tree);
        // println!("Parsing took {}.", format_time(time));

        // if syntax_tree.is_err() {
        //     self.had_error = true;
        //     return;
        // }

        // Generate the assembly code.
        //println!("Generating code...");
        //let (time, assembly) =
        //    timer.time(|| generator::Generator::new(syntax_tree.unwrap()).generate());

        //println!("Assembly:\n{}", assembly);
        //println!("Code generation took {}.", format_time(time));

        println!("Total time: {}.", format_time(timer.total_time()));
    }
}
