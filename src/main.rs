mod lang;
mod util;

fn main() {
    println!("Lithium v{}!", env!("CARGO_PKG_VERSION"));
    
    // Read the file parsed by the user.
    let args: Vec<String> = std::env::args().collect();

    // If the user didn't pass a file, enter the REPL and print the usage.
    let mut lithium = lang::Lithium::new();
    
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        println!("No file is passed, entering REPL...");

        lithium.run_prompt();

        return;
    }

    // Run the file.
    lithium.run_file(&args[1]);
}
