use std::fs;
use std::path::PathBuf;
use colored::Colorize;

pub fn cmd_ls(working_dir: &PathBuf) {
    match fs::read_dir(working_dir) {
        Ok(entries) => {
            for entry in entries.filter_map(|e| e.ok()) {
                let path: PathBuf = entry.path();
                if let Some(filename) = path.file_name().and_then(|name| name.to_str()) {
                    if filename.starts_with('.') {
                        continue;
                    }
                    if path.is_dir() {
                        print!("{}  ", filename.blue());
                    } else {
                        print!("{}  ", filename);
                    }
                }
            }
            println!("");
        }
        Err(e) => eprintln!("{}: {}", "Error reading directory".red(), e.to_string().red()),
    }
}
