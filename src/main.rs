#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command: Vec<&str> = command.trim().split(" ").collect();
        if command[0] == "exit" {
            if command.len() > 1 {
                let status: &i32 = &command[1].parse().unwrap();
                process::exit(*status);
            }
            process::exit(0);
        } else {
            println!("{}: command not found", command[0]);
        }
    }
}
