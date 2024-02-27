use std::{fs, path::PathBuf};

use crate::asar::extract::extract_asar;
use crate::asar::pack::pack_asar;

pub async fn inject(which_discord: &str) -> Result<bool, Box<dyn std::error::Error>> {
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
            panic!("OS not supported");
        }
    }

    let file_name = "core.asar";

    match search_file(&start_dir, file_name) {
        Some(path) => {
            println!("File found at: {:?}", path);
			let dest_path = path.join("unpacked").to_string_lossy().to_string();

			if let Ok(metadata) = fs::metadata(path.join("core.asar.backup")) {
				if metadata.is_file() {
					return Ok(true)
				}
					
				
			}

			fs::copy(path.join("core.asar"), path.join("core.asar.backup")).unwrap();

			extract_asar(path.join("core.asar").to_str().unwrap(), &dest_path).await.unwrap();

			replace_power_monitor(&path.join("unpacked").join("app").join("discord_native").join("browser").join("powerMonitor.js").to_string_lossy().to_string()).unwrap();
			
			pack_asar(path.join("unpacked").to_str().unwrap(), path.join("core.asar").to_str().unwrap()).await.unwrap();

			fs::remove_dir_all(&path.join("unpacked")).unwrap();

			Ok(false)
        }
        None => {
            println!("File not found");
			Err("File not found".into())
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

pub fn replace_power_monitor(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match fs::copy("powerMonitor.js", file_path) {
        Ok(_) => {
            println!("File copied successfully.");
            return Ok(());
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("Source file not found.");
            return Ok(());
        }
        Err(e) => return Err(e.into()),
    }  
}