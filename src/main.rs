use std::io::{self, Write};

enum CommandResult {
    Success,
    Fail,
    Exit,
}

trait Command {
    // fn name(&self) -> String {
    //     std::any::type_name::<Self>().to_lowercase()
    // }

    fn execute(&self, args: Option<&str>) -> CommandResult;

    fn get_type(&self) -> &str {
        "a shell builtin"
    }
}

#[derive(Clone)]
struct Exit;

impl Command for Exit {
    fn execute(&self, _: Option<&str>) -> CommandResult {
        CommandResult::Exit
    }
}

#[derive(Clone)]
struct Echo;

impl Command for Echo {
    fn execute(&self, args: Option<&str>) -> CommandResult {
        if let Some(args) = args {
            println!("{}", args);
        }
        CommandResult::Success
    }
}

#[derive(Clone)]
struct Cat;

impl Command for Cat {
    fn execute(&self, _: Option<&str>) -> CommandResult {
        CommandResult::Success
    }

    fn get_type(&self) -> &str {
        "/bin/cat"
    }
}

#[derive(Clone)]
struct Type;

impl Command for Type {
    fn execute(&self, args: Option<&str>) -> CommandResult {
        if let Some(args) = args {
            let args = args.trim();
            if let (Some(cmd_str), _) = parse_command(args) {
                match get_command(cmd_str) {
                    Some(cmd) => {
                        println!("{} is {}", cmd_str, cmd.get_type())
                    }
                    None => {
                        println!("{}: not found", cmd_str)
                    }
                }
            }
        }
        CommandResult::Success
    }
}

// parse the input from user and output separately for the command and arguments
// handles the empty input
fn parse_command(s: &str) -> (Option<&str>, Option<&str>) {
    if s.len() == 0 {
        return (None, None);
    }

    let splits = s.split_once(" ");
    match splits {
        Some((cmd, args)) => return (Some(cmd.trim()), Some(args.trim())),
        None => return (Some(s), None),
    }
}

fn get_command(s: &str) -> Option<Box<dyn Command>> {
    match s {
        "exit" => Some(Box::new(Exit {})),
        "echo" => Some(Box::new(Echo {})),
        "type" => Some(Box::new(Type {})),
        "cat" => Some(Box::new(Cat {})),
        _ => None,
    }
}

// Parse user input command and return some value when command is not exit, or return error when user decide to exit
// It handles when input string is empty or input with all white spaces
fn run_command(input: &str) -> CommandResult {
    let input = input.trim();
    // if input is empty
    if input.len() == 0 {
        return CommandResult::Fail;
    }

    let (cmd_str, args_str) = parse_command(input);
    if let Some(cmd_str) = cmd_str {
        // if command is registered
        if let Some(cmd) = get_command(cmd_str) {
            return cmd.execute(args_str);
        }
        // if command is not empty and does not exists
        println!("{}: command not found", cmd_str)
    }
    CommandResult::Fail
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
            CommandResult::Exit => break,
            _ => {}
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
