use std::io;
use std::path::{PathBuf, Path};
use std::fs;
use colored::Colorize;

pub fn cmd_cat(command_vec: &[&str], base_path: &PathBuf) {
    if command_vec.len() <= 1 {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let trimmed_input = input.trim();
            if trimmed_input == "exit" {
                return;
            }
            println!("{}", trimmed_input);
        }
    } else {
        for &file in &command_vec[1..] {
            let temp_base_path = base_path.join(file);
            if Path::new(&temp_base_path).exists() {
                match fs::read_to_string(&temp_base_path) {
                    Ok(contents) => println!("{}", contents),
                    Err(err) => eprintln!("Error reading file '{}': {}", file, err),
                }
            } else {
                eprintln!("File not found: '{}'", file.red());
            }
        }
    }
}
