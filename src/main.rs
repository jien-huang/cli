extern crate getopts;

use std::env;
use std::process::exit;

use getopts::Options;

mod command_line;

fn do_works(inp: &str, out: Vec<String>) {
    println!("{}", inp);
    println!("{:?}", out);
}

fn do_work(inp: &str, out: Option<String>) {
    println!("{}", inp);
    match out {
        Some(x) => println!("{}", x),
        None => println!("No Output"),
    }
}

fn help() {
    println!("USAGE:
    cli SUBCOMMAND [OPTIONS]

    SUB COMMANDS:
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
                            -s Script name or id
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
        -e  --environment   Environment variable, key=value
        -f  --filename      File Name
        -h  --host          Host name, if not provided, default is localhost
        ");
}

fn main() {
    command_line::init_check();
    let args: Vec<String> = env::args().collect();
    let _program = args[0].clone();
    if args.len() <= 1 {
        help();
        println!("Please call with correct parameters");
        exit(1);
    }
    let subcommand = args[1].clone();

    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Use verbose output");
    opts.optopt("i", "instance", "Instance id", "hex format string");
    opts.optopt("h", "host", "host machine name", "Host name, if not provided, default is localhost");
    opts.optopt("s", "script", "Script name", "test script name");
    opts.optopt("r", "result", "Result id", "result id");
    opts.optopt("p", "port", "Port number", "An integer port number, default is 8090");
    opts.optmulti("e", "environment", "Environment variable for/of instance", "key=value");
    opts.optmulti("f", "filename", "File Name", "File Name for upload");
    let matches = match opts.parse(&args[2..]) {
        Ok(m) => { m }
        Err(_f) => {
            println!("Please check your options. if need help, please type: 'cli help' .");
            exit(1);
        }
    };
    if subcommand.eq(&"help".to_string()) {
        help();
        println!("Enjoy!");
        return;
    }
    if matches.opt_present("i") {
        do_work(&subcommand, matches.opt_str("i"));
    };
    if matches.opt_present("h") {
        do_work(&subcommand, matches.opt_str("h"));
    };
    if matches.opt_present("s") {
        do_work(&subcommand, matches.opt_str("s"));
    };
    if matches.opt_present("r") {
        do_work(&subcommand, matches.opt_str("r"));
    };
    if matches.opt_present("p") {
        do_work(&subcommand, matches.opt_str("p"));
    };
    if matches.opt_count("e") > 0 {
        do_works(&subcommand, matches.opt_strs("e"));
    }
    if matches.opt_count("f") > 0 {
        do_works(&subcommand, matches.opt_strs("f"));
    }
    if subcommand.eq(&"list".to_string()) {
        command_line::get_all_dockers();
    }

}
