#[allow(unused_imports)]
use std::io::{self, Write};
use itertools::Itertools;

// Parse user input command and return some value when command is not exit, or return error when user decide to exit
fn run_command(input: &str) -> Result<(), i32>{
    let mut splits = input.trim().split(' ');
    match splits.nth(0) {
        Some("exit") => {
            return Err(0);
        },
        Some("echo") => {
            println!("{}",splits.join(" "));
        },
        Some(cmd)  => {
            println!("{}: command not found", cmd);
        },
        None => {
            // do nothing
            println!("");
        }
    }
    Ok(())
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
        match run_command(input.as_ref()) {
            Ok(_) => {
                // do nothing
            },
            Err(_) => {
                break
            }

        };
        print!("$ ");
        io::stdout().flush().unwrap();
    }

    // strip line breaker from input
    // Improvement
    // if let Some(input) = input.strip_suffix("\n") {
    //     println!("{}: command not found", input)
    // };
}
