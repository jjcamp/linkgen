use crate::util::*;
use crate::error::Error;

use colored::*;
use std::fs;

pub fn list_files(verbose: bool) -> Result<(), Error> {
    let itr = this_dir().read_dir()?;

    itr.for_each(|der| {
        let de = match der {
            Ok(de) => de,
            Err(e) => {
                verbose_msg(verbose, e);
                return;
            }
        };

        let name = match de.file_name().into_string() {
            Ok(n) => n,
            Err(fail_string) => {
                verbose_msg(verbose, format!("{:?}", fail_string));
                String::from("")
            }
        };

        let ftype = match fs::symlink_metadata(de.path()) {
            Err(e) => {
                verbose_msg(verbose, e);
                "error".red()
            },
            Ok(slmd) => {
                let ft = slmd.file_type();
                // TODO: Use std::os::windows::fs to get file or dir symlink on windows
                if ft.is_symlink() { "symlink".blue() }
                else if ft.is_dir() { "directory".cyan() }
                else if ft.is_file() { "file".clear() }
                else {
                    verbose_msg(verbose, "Could not determine file type");
                    "unkown".red()
                }
            }
        };

        println!("{:20.20} {:>10}", name, ftype);
    });

    Ok(())
}
