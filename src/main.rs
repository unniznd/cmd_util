use std::io;
use std::fs;
use std::path::PathBuf;
use colored::Colorize;
use std::fs::OpenOptions;

fn main() {
    let mut working_dir: String = String::new();
    println!("{}","Enter path of working directory".green());
    io::stdin().read_line(&mut working_dir).expect("Failed to read input. Try again!");
    let _ = working_dir.pop();

    loop{
        let mut command: String = String::new();
        println!("{}", "Enter command".green());
        io::stdin().read_line(&mut command).expect("Failed to read input. Try again!");
        let command_split: Vec<&str> = command.split_whitespace().collect();


        
    
        if command_split[0] == "ls"{
            
            let paths: Vec<PathBuf> = fs::read_dir(&mut working_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .collect::<Vec<_>>();
            println!("");
            for path in paths {
                let filename: String = path.file_name()
                                          .unwrap()
                                          .to_str()
                                          .expect("Failed")
                                          .to_string();
                if path.is_dir(){
                    
                    if filename.starts_with("."){
                        continue;
                    }
                    print!("{}  ", filename.blue());
                }else{
                    if filename.starts_with("."){
                        continue;
                    }
                    print!("{}  ", filename);
                }
            }
            println!("");
        } else if command_split[0] == "touch"{
            if command_split.len() == 1{
                println!("{}", "touch: missing file operand".red());
                continue;
            }
            let mut file_count:usize = 1;
            while file_count<command_split.len(){
                
                let mut filename: String = working_dir.clone();
                if !filename.ends_with("/"){
                    filename.push('/');
                }
                filename.push_str(command_split[file_count]);
                let _ =  OpenOptions::new().write(true)
                             .create_new(true)
                             .open(filename);
                file_count += 1;
            }


        }else if command_split[0] == "exit"{
            println!("{}","Thank you for using cmd utils using rust");
            break;
        }else if command_split[0] == "clear"{
            if command_split.len() > 1{
                println!("No usage of {} found in clear command.", command_split[1].red());
                continue;
            }
            clearscreen::clear().unwrap();
        }else {
            println!("{}","Command not found!".red());
            continue;
        }

    }
}
