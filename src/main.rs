mod lang;
mod util;

fn main() {
    // Read the file parsed by the user.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let mut lithium = lang::Lithium::new();

    lithium.run_file(&args[1]);
}
