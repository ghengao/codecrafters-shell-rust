#[allow(unused_imports)]
use std::io::{self, Write};

fn run_command(input: &str) {
    println!("{}: command not found", input.trim())
}

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    loop {
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        run_command(input.as_ref());
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    // strip line breaker from input
    // Improvement
    // if let Some(input) = input.strip_suffix("\n") {
    //     println!("{}: command not found", input)
    // };
}
