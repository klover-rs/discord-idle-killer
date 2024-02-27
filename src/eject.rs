use std::fs;
use std::path::PathBuf;


const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_RESET: &str = "\x1b[0m";

pub fn eject(which_discord: &str) -> Result<(), Box<dyn std::error::Error>>{
    let start_dir: PathBuf;
    match std::env::consts::OS {
        "windows" => {
            start_dir = dirs::home_dir().unwrap().join(format!("AppData\\Local\\{}", which_discord));
        }
        "linux" => {
            start_dir = dirs::home_dir().unwrap().join(format!(".config/{}", which_discord));
        }
        "macos" => {
            start_dir = dirs::home_dir().unwrap().join(format!("Library/Application Support/{}", which_discord));
        }
        _ => {
            panic!("{}", format!("{}OS not supported{}", ANSI_RED, ANSI_RESET));
        }
    }

    println!("start dir: {:?}", start_dir);
    let file_name = "core.asar.backup";

    match search_file(&start_dir, file_name) {
        Some(path) => {
            
            println!("path: {:?}", path);
            fs::remove_file(path.join(path.join("core.asar"))).unwrap();
            fs::rename(path.join("core.asar.backup"), path.join("core.asar")).unwrap();
            println!("{}ejected successfully ✓{}feel free to start your discord client now.", ANSI_GREEN, ANSI_RESET);

            Ok(())
        }
        None => {
            println!("{}File not found{}", ANSI_RED, ANSI_RESET);
            Ok(())
        }
    }
}

fn search_file(start_dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
    if let Ok(entries) = fs::read_dir(start_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                let metadata = fs::metadata(&path);

                if let Ok(metadata) = metadata {
                    if metadata.is_dir() {
                        if let Some(found_path) = search_file(&path, file_name) {
                            return Some(found_path);
                        }
                    } else if path.file_name().map(|f| f == file_name).unwrap_or(false) {
                        return Some(start_dir.to_path_buf());
                    }
                }
            }
        }
    }

    None
}