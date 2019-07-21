use crate::util::*;
use crate::error::Error;

use colored::*;
use std::env;
use std::fs;

#[cfg(target_os = "windows")]
use std::os::windows::fs::symlink_file as symlink_file;
#[cfg(target_os = "linux")]
use std::os::unix::fs::symlink as symlink_file;
#[cfg(target_os = "macos")]
use std::os::unix::fs::symlink as symlink_file;

pub fn file(path: &str, verbose: bool) -> Result<(), Error> {
    let src = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(e) => {
            return Error::result(e)
            // TODO: Try to hint if already on path
        }
    };

    let exe = match src.file_name() {
        Some(s) => s,
        None => return Error::new_result("Not a usable path")
    };

    let mut dst = env::current_exe().and_then(|mut f| { f.pop(); return Ok(f); })?;

    dst.push(exe);
    
    verbose_msg(verbose, format!("Linking {}", src.to_str().unwrap_or("")));

    symlink_file(&src, dst)?;

    println!("{} {:?}", "Added symlink to".green(), exe.to_os_string());

    Ok(())
}
