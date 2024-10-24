pub fn cmd_echo(command_vec: &Vec<&str>){
    let cmd_len = command_vec.len();
    let mut idx = 1;
    while idx < cmd_len{
        print!("{}", command_vec[idx]);
        if idx != cmd_len - 1 {
            print!(" ");
        }
        idx += 1;
    }
    println!();
}