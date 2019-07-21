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

        let (ftype, extra) = match fs::symlink_metadata(de.path()) {
            Err(e) => {
                verbose_msg(verbose, e);
                ("error".red(), None)
            },
            Ok(slmd) => {
                let ft = slmd.file_type();
                if ft.is_symlink() { ("symlink".blue(), std::fs::read_link(de.path()).ok()) }
                else if ft.is_dir() { ("directory".cyan(), None) }
                else if ft.is_file() { ("file".clear(), None) }
                else {
                    verbose_msg(verbose, "Could not determine file type");
                    ("unkown".red(), None)
                }
            }
        };

        let extra_str = match extra {
            Some(pb) => pb.into_os_string().into_string().unwrap_or(String::from("")),
            None => String::from("")
        };

        println!("{:20.20} {:>10.10} {}", name, ftype, extra_str);
    });

    Ok(())
}
