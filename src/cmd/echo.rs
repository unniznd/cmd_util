pub fn cmd_echo(command_vec: &[&str]) {
    if command_vec.len() <= 1 {
        println!();
        return;
    }
    println!("{}", command_vec[1..].join(" "));
}
