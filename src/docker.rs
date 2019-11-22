use std::env;
use std::process::Command;
use std::process::exit;

pub fn get_all_dockers() {
    println!("get all dockers here")
}

// check if docker and curl existed, if not, then exit
pub fn init_check() {

    run_command("curl --help");
    println!("docker or curl not existed, please make sure you have installed them.");
    exit(1);
}

pub fn run_command(cmd : &str) {
    if cfg!(windows) {
        return Command::new("cmd").args(&["/C", cmd]).status().expect("failed:" );
    } else {
        return Command::new("sh").arg("-c").arg(cmd).status().expect("failed:" );
    }

}