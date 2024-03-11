use clap::{App, Arg, ArgMatches};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: ArgMatches = App::new("find_tool")
        .about("Searches for files and directories in the specified path")
        .arg(
            Arg::with_name("path")
                .help("Sets the search path")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("files")
                .short('f')
                .long("files")
                .help("Find files"),
        )
        .arg(
            Arg::with_name("directories")
                .short('d')
                .long("directories")
                .help("Find directories"),
        )
        .get_matches();

    let search_path: &str = matches.value_of("path").unwrap();
    let mut find_files: bool = matches.is_present("files");
    let mut find_dirs: bool = matches.is_present("directories");

    // If no flags are provided, set both find_files and find_dirs to true
    if !find_files && !find_dirs {
        find_files = true;
        find_dirs = true;
    }

    let search_dir: PathBuf = PathBuf::from(search_path);
    for entry in fs::read_dir(search_dir)? {
        let entry: fs::DirEntry = entry?;
        let entry_path: PathBuf = entry.path();
        if entry_path.is_file() && find_files {
            println!("{}", entry_path.display().to_string().blink());
        } else if entry_path.is_dir() && find_dirs {
            println!("{}", entry_path.display().to_string().bright_cyan());
        }
    }
    Ok(())
}
