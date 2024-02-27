use std::path::PathBuf;
use std::fs;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ClientInfo {
    pub path: String,
    pub basename: String,
    pub injected: bool,
}

pub fn check_installed_clients() -> Vec<ClientInfo> {
    let target_dir: PathBuf;
    

    match std::env::consts::OS {
        "windows" => {
            target_dir = dirs::home_dir().unwrap().join("AppData\\Local");
        }
        "linux" => {
            target_dir = dirs::home_dir().unwrap().join(".config");
        }
        "macos" => {
            target_dir = dirs::home_dir().unwrap().join("Library/Application Support");
        }
        _ => {
            panic!("OS not supported");
        }
    }

    let mut client_info_list = Vec::new();

    for folder in if cfg!(target_os = "windows") { vec!["Discord", "DiscordCanary", "DiscordPTB"] } else { vec!["discord", "discordcanary", "discordptb"] } {
        let folder_path = target_dir.join(folder);
        if folder_path.is_dir() {
            let core_folder = match search_file(&folder_path, "core.asar") {
                Some(path) => path,
                None => {
                    println!("File not found");
                    continue;
                }
            };

            

            let core_backup_path = core_folder.join("core.asar.backup");
            let file_exists = if let Ok(metadata) = fs::metadata(&core_backup_path) {
                metadata.is_file()
            } else {
                false
            };

            let client_info = ClientInfo {
                path: core_folder.to_string_lossy().into_owned(),
                basename: folder.to_string(),
                injected: file_exists,
            };

            client_info_list.push(client_info);

        }
    };

    client_info_list

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