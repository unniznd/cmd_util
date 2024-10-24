use std::path::PathBuf;
use std::fs;
use colored::Colorize;

pub fn cmd_find(command_vec: &[&str], base_path: &PathBuf) {
    let mut stack: Vec<PathBuf> = vec![base_path.to_owned()];

    while let Some(current_dir) = stack.pop() {
        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries.filter_map(|e| e.ok()) {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                        match path.strip_prefix(base_path) {
                            Ok(relative_path) => {
                                if command_vec.len() == 1 {
                                    println!("{}", relative_path.display());
                                } else if command_vec.len() == 2 {
                                    let mut filter = command_vec[1].to_string();
                                    
                                    // Remove "*" from the start of the filter if present
                                    if filter.starts_with('*') {
                                        filter = filter.replacen("*", "", 1);
                                    }else{
                                        print!("");
                                        return;
                                    }
                                    
                                    // Check if the filename contains the filter
                                    if filename.contains(&filter) {
                                        println!("{}", relative_path.display());
                                    }
                                }
                            }
                            Err(_) => {
                                eprintln!("{}: Failed to create relative path", "Error".red());
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: {}", "Error reading directory".red(), e.to_string().red());
            }
        }
    }
}
