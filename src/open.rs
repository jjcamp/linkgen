use crate::util::*;
use crate::error::Error;

use std::process::Command;

#[cfg(target_os = "windows")]
pub fn dir(verbose: bool) -> Result<(), Error> {
    dir_with("explorer", verbose)
}

#[cfg(target_os = "windows")]
pub fn about_msg() -> &'static str {
    "Opens the host directory in Explorer"
}

#[cfg(target_os = "macos")]
pub fn about_msg() -> &'static str {
    "Opens the host directory in Finder"
}

#[cfg(target_os = "macos")]
pub fn dir(verbose: bool) -> Result<(), Error> {
    dir_with("open", verbose)
}

#[cfg(target_os = "linux")]
pub fn dir(verbose: bool) -> Result<(), Error> {
    // TODO: probably probe for different possible commands
    println!("{} command not implemented for your operating system".red());
    Err(())
}

#[cfg(target_os = "linux")]
pub fn about_msg() -> &'static str {
    "Not implemented for your system"
}

fn dir_with(cmd: &str, verbose: bool) -> Result<(), Error> {
    let this_dir = this_dir();
    let path = this_dir.to_str().unwrap_or(".");

    verbose_msg(verbose, format!("running command '{} {}'", cmd, path));

    match Command::new(cmd).arg(path).spawn() {
        Ok(_c) => Ok(()),
        Err(e) => Error::result(e)
    }
}
