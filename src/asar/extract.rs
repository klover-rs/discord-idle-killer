use std::path::PathBuf;
use asar::{AsarReader, Result as AsarResult};
use std::fs;


pub async fn extract_asar(asar_file: &str, output: &str) -> AsarResult<String> {
    let asar_file = fs::read(asar_file)?;
    let asar = AsarReader::new(&asar_file, None)?;

    let total_files = asar.files().len() as f64;
    let mut counter = 0;

    for (path, file_info) in asar.files() {
        let output_path = PathBuf::from(output).join(path);

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = file_info.data();
        fs::write(&output_path, content)?;
        counter += 1;

        let progress_percentage = (counter as f64 / total_files) * 100.0;

        println!("extracted: {} | {:.2}%", output_path.display(), progress_percentage);
    }

    println!("total files: {}", total_files);
    Ok(total_files.to_string())
}