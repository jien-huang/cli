extern crate getopts;
use getopts::Options;
use std::io::{stdout, Write};
use std::env;
use std::process::Command;
use curl::easy::Easy;
use std::process::exit;

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: \n \
        \t{} [SUBCOMMAND] [OPTIONS]", program);
    print!("{}", opts.usage(&brief));
}

// Print a web page onto stdout
fn main() {
//    let mut easy = Easy::new();
//    get_url(&mut easy, "https://www.rust-lang.org/");
//
//    println!("{}", easy.response_code().unwrap());
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    if args.len() <=1 {
        help();
        exit(1);
    }
    let subcommand = args[1].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    do_work(&input, output);
}

fn help() {
    println!("USAGE:
    cli SUBCOMMAND [OPTIONS]

    SUBCOMMANDS:
        help            Print help information (No option need)
        deploy          Deploy a docker instance
                            -p [port number: integer] (optional, default 8090)
        set             Set instance's configuration
                            -e [environment variable: key=value] (mandatory)
                            -i [instance id] (mandatory)
        upload          Upload script file
                            -f [file name] (mandatory)
                            -i [instance id] (mandatory)
        download        Download result
        git                 Pull scripts from git
        start           Start test
                            -i [instance id] (mandatory)
        stop            Stop test
                            -i [instance id] (mandatory)
        halt            Stop docker instance
                            -i [instance id] (mandatory)
        clear           Clear scripts, results or both
                            -i [instance id] (mandatory)
                            -s clear all scripts
                            -r clear all results
                            no option means clear all scripts and results
        status          Show status of instance
                            -i [instance id] (mandatory) will return all configurations
        list            List instance, scripts or results
                            -i [instance id] (optional, if not specified, then will list all instances)
                            -s list all scripts in the specified instance
                            -r list all results in the specified instance
        version         Show version of this program

    OPTIONS:
        -v, --verbose       Use verbose output
        -i  --instance      Instance id
        -s  --script        Script name or id
        -r  --result        Result name or id
        -p  --port          Port number
        -e  --environment   Environment varialbe, key=value
        -f  --filename      File Name

     Enjoy!
        ");
}

fn get_url(easy: &mut Easy, url: &str) {
    easy.url(url).unwrap();
    easy.write_function(|data| {
//        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_url() {
        let mut easy = Easy::new();
        get_url(&mut easy, "https://www.google.com/");
        assert!(easy.response_code().unwrap() > 0);
    }

}