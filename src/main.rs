use std::{
    io::{self, Write},
    path::PathBuf,
};

use anyhow::Result;
use dirs::home_dir;

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

#[derive(Clone)]
struct Pwd;
impl Command for Pwd {
    fn execute(&self, _: Option<&str>) -> CommandResult {
        if let Ok(s) = std::env::current_dir() {
            println!("{}", s.display());
        }
        CommandResult::Success
    }
}

#[derive(Clone)]
struct Cd;
impl Command for Cd {
    fn execute(&self, args: Option<&str>) -> CommandResult {
        if let Some(args) = args {
            let _ = resolve_path(args).map(|p| {
                std::env::set_current_dir(p)
                    .map(|_| CommandResult::Success)
                    .map_err(|_| {
                        println!("cd: {}: No such file or directory", args);
                        CommandResult::Fail
                    })
            });
        }
        CommandResult::Success
    }
}

// Given path must be valid directory
fn resolve_path(path: &str) -> Result<PathBuf> {
    let mut tmp_path = PathBuf::from(path);
    if path.starts_with("~") {
        let home_path = home_dir().ok_or(anyhow::anyhow!("no home directory found"))?;
        let home: &str = home_path
            .to_str()
            .ok_or(anyhow::anyhow!("cannot cast path string"))?;
        let p = path.replace("~", home);
        tmp_path = PathBuf::from(p);
    }
    if tmp_path.is_relative() {
        tmp_path = std::env::current_dir()?.join(path);
    }

    let mut res_path = PathBuf::from("/");

    for parts in tmp_path.iter().map(|s| s.to_str()) {
        match parts {
            Some("..") => {
                res_path.pop();
            }
            Some(".") | None => {}
            Some(s) => {
                res_path.push(s);
            }
        }
    }
    Ok(res_path.into())
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

#[derive(Clone)]
struct Executable(String);

impl Command for Executable {
    fn execute(&self, args: Option<&str>) -> CommandResult {
        let mut r = std::process::Command::new(self.0.as_str());
        if args != None {
            r.arg(args.unwrap());
        }
        let _ = r
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status();
        CommandResult::Success
    }

    fn get_type(&self) -> &str {
        self.0.as_str()
    }
}

// Given a cmd like `cat` try to find the executable in the env PATH
fn search_path(cmd: &str) -> Option<String> {
    std::env::split_paths(&std::env::var_os("PATH")?)
        .map(|p| p.join(cmd))
        .find(|p| p.is_file())
        .and_then(|p| p.to_str().map(|x| x.to_string()))
}

fn get_command(s: &str) -> Option<Box<dyn Command>> {
    match s {
        "exit" => Some(Box::new(Exit {})),
        "echo" => Some(Box::new(Echo {})),
        "type" => Some(Box::new(Type {})),
        "pwd" => Some(Box::new(Pwd {})),
        "cd" => Some(Box::new(Cd {})),
        _ => Some(Box::new(Executable(search_path(s)?))),
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
