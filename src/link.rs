use crate::util::*;
use crate::error::Error;

use colored::*;
use std::env;
use std::fs;

#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink_file;
#[cfg(not(windows))]
use std::os::unix::fs::symlink as symlink_file;

#[cfg(windows)]
fn ext_helper(exe: &std::ffi::OsStr, alias: &str) -> String {
    let e = exe.to_str().unwrap_or("");
    match e.rfind(".") {
        None => alias.to_owned(),
        Some(i) => {
            let suffix = &e[i..e.len()];
            if alias.ends_with(suffix) {
                alias.to_owned()
            }
            else {
                format!("{}{}", alias, suffix)
            }
        }
    }
}

#[cfg(not(windows))]
fn ext_helper(exe: &std::ffi::OsStr, alias: &str) -> String {
    alias.to_owned()
}

pub fn file(path: &str, alias: Option<&str>, verbose: bool) -> Result<(), Error> {
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

    match alias {
        Some(a) => dst.push(ext_helper(exe, a)),
        None => dst.push(exe)
    };
    
    verbose_msg(verbose, format!("Linking {}", src.to_str().unwrap_or("")));

    symlink_file(&src, dst)?;

    println!("{} {:?}{}", "Added symlink to".green(), exe/*.to_os_string()*/, match alias {
        Some(a) => format!(" as \"{}\"", a),
        None => String::new()
    });

    Ok(())
}

#[cfg(test)]
#[cfg(windows)]
mod windows_tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn ext_helper_alias_has_same_ext() {
        assert_eq!("alias.exe", ext_helper(OsStr::new("src.exe"), "alias.exe"))
    }

    #[test]
    fn ext_helper_alias_no_ext() {
        assert_eq!("alias.exe", ext_helper(OsStr::new("src.exe"), "alias"))
    }

    #[test]
    fn ext_helper_src_has_no_ext() {
        assert_eq!("alias", ext_helper(OsStr::new("src"), "alias"))
    }

    #[test]
    fn ext_helper_alias_has_different_ext() {
        assert_eq!("alias.bat.exe", ext_helper(OsStr::new("src.exe"), "alias.bat"))
    }
}
