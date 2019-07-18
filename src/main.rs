use colored::*;
use std::env;
use std::fs;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.iter().any(|s| { s == "--help" || s == "-?"}) {
        print_help(&args[0]);
        return Ok(());
    }
    else if args.iter().any(|s| { s == "--version" || s == "-v" }) {
        print_version();
        return Ok(());
    }

    let src = match fs::canonicalize(&args[1]) {
        Ok(p) => p,
        Err(_e) => {
            print_invalid_arg(&args[0], &args[1], "Invalid path");
            std::process::exit(1);
            // TODO: Try to hint if already on path
        }
    };

    let exe = match src.file_name() {
        Some(s) => s,
        None => {
            print_invalid_arg(&args[0], &args[1], "Not a usable path");
            std::process::exit(1);
        }
    };

    let mut dst = match env::current_exe().and_then(|mut f| { f.pop(); return Ok(f); }) {
        Ok(p) => p,
        Err(e) => {
            use std::error::Error;
            print_other_error(&args[0], format!("{}", e.description()));
            std::process::exit(1);
        }
    };

    dst.push(exe);

    match std::os::windows::fs::symlink_file(&src, dst) {
        Err(e) => {
            use std::error::Error;
            print_other_error(&args[0], format!("{}", e.description()));
            std::process::exit(1);
        }
        _ => ()
    };

    println!("{} {:?}", "Added symlink to".green(), exe.to_os_string());

    return Ok(());
}

fn print_version() {
    println!("{} — version {}", "Link Generator".green().bold(), VERSION);
    println!("{}", AUTHORS);
}

fn print_about() {
    print_version();
    println!("\n{}", "Creates soft links to executables.".cyan());
    println!("{} The soft links are installed {},", "NOTE:".bold(), "in the directory of this executable".italic());
    println!("(presumably in your path) {} {}", "not".bold(), "in the current working directory.");
}

fn print_usage(this: &String) {
    println!("\nUsage:");
    println!("{} PATH_TO_EXECUTABLE", this);
    println!("{:5}where 'PATH_TO_EXECUTABLE' is the executable to link", "");
}

fn print_flags() {
    println!("\nAdditional commands:");
    println!("{:10}", "--help".magenta());
    println!("{:10}{}\n", "-?".magenta(), "Prints this help message and quits");
    println!("{:10}", "--version".magenta());
    println!("{:10}{}\n", "-v".magenta(), "Prints the version and quits.");
}

fn print_help(this: &String) {
    print_about();
    print_usage(this);
    print_flags();
}

fn print_short_help(this: &String) {
    println!("\n Use '{} --help' for help.", this);
}

fn print_invalid_arg(this: &String, arg: &String, why: &'static str) {
    println!("{} {}", "Invalid argument:".red(), arg);
    println!("{:5}Reason: {}", "", why);
    print_short_help(this);
}

fn print_other_error(this: &String, what: String) {
    println!("{} {}", "Error: ".red(), what);
    print_short_help(this);
}
