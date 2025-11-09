#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

enum Command {
    ExitCommand { status: u8 },
    EchoCommand { display_string: String},
    NotFound { command: String}
}

impl Command {
    fn from_input(input: &str) -> Self {
        if input.is_empty() {
            return Self::NotFound { command: String::new()};
        }
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts[0] == "exit" {
            if parts.len() > 1 {
                let status: u8 = parts[1].parse().unwrap();
                Self::ExitCommand { status }
            } else {
                Self::ExitCommand { status: 0 }
            }
        } else if parts[0] == "echo" {
            if parts.len() > 1 {
                let command_str: String = parts[1..].join(" ");
                Self::EchoCommand { display_string: command_str }
            } else {
                Self::EchoCommand { display_string: String::new() }
            }
        } else {
            Self::NotFound { command: String::from(parts[0])}
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: Command = Command::from_input(&command);
        match command {
            Command::ExitCommand { status } => process::exit(i32::from(status)),
            Command::EchoCommand { display_string } => println!("{display_string}"),
            Command::NotFound {command} => println!("{command}: command not found")
        }
    }
}