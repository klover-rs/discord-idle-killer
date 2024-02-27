use std::fs::File;
use asar::{AsarWriter, HashAlgorithm};
use walkdir::WalkDir;
use std::fs;

pub async fn pack_asar(path: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = AsarWriter::new_with_algorithm(HashAlgorithm::Sha256);

    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_content = fs::read(entry.path())?;

            let relative_path = entry.path().strip_prefix(path)?;
            let relative_path_str = relative_path.to_str().unwrap_or("");
            writer.write_file(relative_path_str, &file_content, false)?;
        }
    }

    let mut output_file = File::create(dest)?;
    let bytes_written = writer.finalize(&mut output_file)?;

    println!("wrote {} bytes to {}", bytes_written * 1000, dest);

    Ok(())
}