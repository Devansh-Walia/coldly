use std::fs::File;
use std::io::{self, Read};
use std::fs;
use std::path::PathBuf;

pub fn read_file_to_string(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
} 

pub fn get_attachment(templates_path: &str) -> io::Result<Vec<PathBuf>> {
    let pdf_files: Vec<PathBuf> = fs::read_dir(templates_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "pdf" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    Ok(pdf_files)
}