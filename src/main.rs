use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::thread; // Import Colorize trait

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the search path and flags from command line arguments
    let args: Vec<String> = env::args().skip(1).collect(); // Convert iterator to Vec<String>
    let search_path = args.first().cloned().unwrap_or_else(|| {
        println!("Usage: find_tool <search_path> [-f] [-d]");
        std::process::exit(1);
    });
    let find_files = args.iter().any(|arg| arg == "-f");
    let find_dirs = args.iter().any(|arg| arg == "-d");

    // Traverse the directory and find files and directories
    let search_dir: PathBuf = PathBuf::from(&search_path);
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];
    for entry in fs::read_dir(search_dir)? {
        let entry: fs::DirEntry = entry?;
        let entry_path: PathBuf = entry.path();
        let find_files_clone: bool = find_files;
        let find_dirs_clone: bool = find_dirs;
        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            if entry_path.is_file() && find_files_clone {
                println!("{}", entry_path.display().to_string().green()); // Apply green color
            } else if entry_path.is_dir() && find_dirs_clone {
                println!("{}", entry_path.display().to_string().blue()); // Apply blue color
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Error joining thread");
    }
    Ok(())
}
