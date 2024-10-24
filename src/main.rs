mod cmd {
    pub mod touch;
    pub mod ls;
    pub mod echo;
    pub mod cat;
    pub mod find;
}

use std::io::{self, Write};
use std::path::PathBuf;
use std::env;
use std::fs;

use colored::Colorize;
use clearscreen;


use crate::cmd::touch::cmd_touch;
use crate::cmd::ls::cmd_ls;
use crate::cmd::echo::cmd_echo;
use crate::cmd::cat::cmd_cat;
use crate::cmd::find::cmd_find;

pub fn prompt_for_directory() -> PathBuf {
    loop {
        let mut working_dir = String::new();
        println!("{}", "Enter path of working directory:".green());

        io::stdin()
            .read_line(&mut working_dir)
            .expect("Failed to read input. Try again!");

        // Trim the input to remove trailing newline or whitespace
        let working_dir = working_dir.trim();

        // Get current directory
        let curr_dir: PathBuf = env::current_dir().expect("Failed to get current directory");

        // Resolve relative paths
        let mut path: PathBuf = PathBuf::from(working_dir);
        if working_dir.starts_with('.') {
            path = curr_dir.join(working_dir).canonicalize().expect("Failed to resolve path");
        }

        // Try to read the directory, if successful return the path
        match fs::read_dir(&path) {
            Ok(_) => {
                // If the directory is valid, break the loop and return the valid path
                return path;
            }
            Err(e) => {
                // Print error and prompt again
                eprintln!("{}: {}", "Error reading directory".red(), e.to_string().red());
                println!("{}", "Please enter a valid directory path.".yellow());
            }
        }
    }
}

fn main() {

    let path: PathBuf = prompt_for_directory();

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
            "ls" => cmd_ls(&path, false),
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
            "cat" => cmd_cat(&command_vec, &path),
            "find" => cmd_find(&command_vec, &path),
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
