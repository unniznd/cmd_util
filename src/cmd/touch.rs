use std::fs::OpenOptions;
use std::path::PathBuf;
use colored::Colorize;

pub fn cmd_touch(command_vec: Vec<&str>, base_path: &PathBuf) {
    if command_vec.len() == 1 {
        println!("{}", "touch: missing file operand".red());
        return;
    }

    for file in &command_vec[1..] {
       
        let full_path: PathBuf = base_path.join(file);


        match OpenOptions::new().write(true).create_new(true).open(&full_path) {
            Ok(_) => (),
            Err(e) => eprintln!("{}: {}", "touch".red(), e.to_string().red()),  
        }
    }
}
