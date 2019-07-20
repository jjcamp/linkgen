use crate::error::Error;

use colored::*;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;

pub fn verbose_msg(verbose: bool, msg: impl Display) {
    if verbose {
        println!("{} {}", "Info:".yellow(), msg);
    }
}

pub fn bail_error<R>(err: Error) -> R {
    eprintln!("{} {}", "Error:".red(), err);
    std::process::exit(err.exit);
}

pub fn this_dir() -> PathBuf {
    // Just bail if anything goes wrong, if errors happen here
    // there are bigger problems
    match (match env::current_exe() {
        Ok(pb) => pb,
        Err(e) => bail_error(Error::from(e))
    }).parent() {
        Some(p) => PathBuf::from(p),
        None => PathBuf::from("/") // N/A for Windows
    }
}
