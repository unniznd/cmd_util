mod cmd {
    pub mod touch;
    pub mod ls;
    pub mod echo;
}

use std::io::{self, Write};
use std::path::PathBuf;
use colored::Colorize;
use std::env;
use clearscreen;

use crate::cmd::touch::cmd_touch;
use crate::cmd::ls::cmd_ls;
use crate::cmd::echo::cmd_echo;

fn main() {
    let mut working_dir = String::new();
    println!("{}", "Enter path of working directory:".green());


    io::stdin()
        .read_line(&mut working_dir)
        .expect("Failed to read input. Try again!");
    let working_dir: &str = working_dir.trim(); 

    let curr_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    
    let mut path: PathBuf = PathBuf::from(working_dir);
    if working_dir.starts_with('.') {
        path = curr_dir.join(working_dir).canonicalize().expect("Failed to resolve path");
    }

    loop {
        let mut command: String = String::new();

        print!("{} > ", path.to_str().unwrap_or("Unknown Path"));
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read input. Try again!");
        let command_vec: Vec<&str> = command.split_whitespace().collect();

        if command_vec.is_empty() {
            continue;
        }

        match command_vec[0] {
            "ls" => cmd_ls(&path),
            "touch" => {
                if command_vec.len() < 2 {
                    println!("{}", "touch: missing file operand".red());
                } else {
                    cmd_touch(command_vec, &path);
                }
            }
            "exit" => {
                println!("{}", "Thank you for using cmd utils in Rust.".green());
                break;
            }
            "echo" => cmd_echo(&command_vec),
            "clear" => {
                if command_vec.len() > 1 {
                    println!("No usage of {} found in clear command.", command_vec[1].red());
                } else {
                    clearscreen::clear().unwrap();
                }
            }
            _ => println!("{}", "Command not found!".red()),
        }
    }
}
