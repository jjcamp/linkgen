mod util;
use util::*;
mod error;
mod link;
mod ls;
mod open;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;

// Major Features/Refactorings to do:
// - rename on link option
// - allow overwrites except on --force

const PKGNAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<(), ()> {

    let args = parse_args();

    if let Some(args) = args.subcommand_matches("ls") {
        match ls::list_files(args.is_present("verbose")) {
            Ok(()) => return Ok(()),
            Err(e) => bail_error(e)
        }
    }

    if let Some(args) = args.subcommand_matches("open") {
        match open::dir(args.is_present("verbose")) {
            Ok(()) => return Ok(()),
            Err(e) => bail_error(e)
        }
    }

    let verbose = args.is_present("verbose");
    let file_str = args.value_of("PATH_TO_EXECUTABLE").unwrap();
    
    match link::file(file_str, verbose) {
        Ok(()) => return Ok(()),
        Err(e) => bail_error(e)
    }
}

fn parse_args() -> ArgMatches<'static> {
    return App::new(PKGNAME.green().bold().to_string())
        .version(VERSION)
        .author(AUTHORS)
        .about("Creates soft links to executables.")
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::SubcommandsNegateReqs)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Verbose output"))
        .arg(Arg::with_name("PATH_TO_EXECUTABLE")
            .help("Path to the executable to link")
            .required(true)
            .index(1))
        .subcommand(SubCommand::with_name("ls")
            .alias("dir")
            .about("Enumerate the host directory")
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose output")))
        .subcommand(SubCommand::with_name("open")
            .aliases(&["start", "explorer", "nautilus"])
            .about(open::about_msg())
            .arg(Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Verbose output")))
        .get_matches();
}
