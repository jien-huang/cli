use std::{io};
use std::process::{Command, Output};
use std::process::exit;

pub fn get_all_dockers() {
    println!("get all dockers here")
}

pub fn set_environment_variables() {
    println!("set env vars");
}

// check if docker and curl existed, if not, then exit
pub fn init_check() {
    let output = run_command_return_output("curl --help");
    if !output.unwrap().status.success() {
        println!("curl is not existed, please make sure you have installed them.");
        exit(1);
    }
    let output = run_command_return_output("docker --help");
    if !output.unwrap().status.success() {
        println!("docker is not existed, please make sure you have installed them.");
        exit(1);
    }
}

pub fn run_command_no_return(cmd: &str) {
    let output = run_command_return_output(cmd).unwrap();
    if !output.status.success() {
        // if fail, print the command output
        let err = String::from_utf8(output.stderr);
        println!("Internal call command failed:\n {}", err.unwrap())
    }
}

pub fn run_command_with_return(cmd: &str) {
    let output = run_command_return_output(cmd).unwrap();
    if !output.status.success() {
        // if fail, print the command output
        let err = String::from_utf8(output.stderr);
        println!("Internal call command failed:\n {}", err.unwrap())
    } else {
        let out = String::from_utf8(output.stdout);
        println!("{}", out.unwrap());
    }
}

pub fn run_command_return_output(cmd: &str) -> io::Result<Output> {
//    println!("{}", cmd);
    if cfg!(windows) {
        return Command::new("cmd").args(&["/C", cmd]).output();
    } else {
        return Command::new("sh").arg("-c").arg(cmd).output();
    }
}