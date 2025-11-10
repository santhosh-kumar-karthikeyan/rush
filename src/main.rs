use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::process;
use std::env;

enum Command {
    ExitCommand { status: u8 },
    EchoCommand { display_string: String},
    TypeCommand { command: String, command_type: CommandType},
    ExecCommand,
    NotFound { command: String}
}

enum CommandType {
    Builtin,
    Executable { location: String},
    NotFound
}

impl Command {
    fn from_input(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command_type = Self::get_command_type(parts[0]);
        match command_type {
            CommandType::NotFound => Self::NotFound { command: parts[0].to_string() },
            CommandType::Builtin => Self::get_builtin_command(parts),
            CommandType::Executable {location} => {
                let _ = location;
                let mut command = process::Command::new(parts[0]);
                if parts.len() > 1 {
                    command.args(parts[1..].iter());
                }
                command.status().unwrap();
                Self::ExecCommand
            }
        }
    }
    fn get_builtin_command(parts: Vec<&str>) -> Self {
        match parts[0] {
            "exit" => {
                if parts.len() > 1 {
                    let status: u8 = parts[1].parse().unwrap();
                    Self::ExitCommand { status }
                } else {
                    Self::ExitCommand { status: 0 }
                }
            },
            "echo" => {
                if parts.len() > 1 {
                    let command_str: String = parts[1..].join(" ");
                    Self::EchoCommand { display_string: command_str }
                } else {
                    Self::EchoCommand { display_string: String::new() }
                }
            },
            "type" => {
                if parts.len() == 1 {
                    return Self::NotFound { command: String::new() };
                }
                let command_type = Self::get_command_type(parts[1]);
                return Self::TypeCommand { command: parts[1].to_string(), command_type };
            }
            _ => {
                Self::NotFound { command: String::from(parts[0]) }
            }
        }
    }
    fn get_command_type(command: &str) -> CommandType {
        let builtins = ["echo", "type", "exit"];
        if builtins.contains(&command) {
            return CommandType::Builtin;
        }
        let env_path = env::var("PATH").unwrap();
        for dir in env::split_paths(&env_path) {
            if !dir.is_dir() {
                continue;
            }
            let Ok(entries) = fs::read_dir(&dir) else { continue };
            for entry in entries {
                let Ok(entry) = entry else { continue };
                let exec = String::from(entry.file_name().to_string_lossy().to_owned());
                if exec == command {
                    let metadata = fs::metadata(entry.path()).unwrap();
                    let permissions = metadata.permissions();
                    if permissions.mode() & 0o111 == 0 {
                        continue;
                    }
                    return CommandType::Executable {location: entry.path().display().to_string()};
                }
            }
        }
        CommandType::NotFound
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let command: Command = Command::from_input(&command);
        match command {
            Command::ExitCommand { status } => process::exit(i32::from(status)),
            Command::EchoCommand { display_string } => println!("{display_string}"),
            Command::TypeCommand { command , command_type} => {
                match command_type {
                    CommandType::Builtin => println!("{command} is a shell builtin"),
                    CommandType::Executable {location} => println!("{command} is {location}"),
                    CommandType::NotFound => println!("{command}: not found")
                }
            },
            Command::ExecCommand => {},
            Command::NotFound {command} => println!("{command}: command not found")
        }
    }
}