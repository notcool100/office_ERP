use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::{Result, anyhow};
use base64::{Engine, engine::general_purpose::STANDARD};

const NONCE_LEN: usize = 12;

fn cipher() -> Result<Aes256Gcm> {
    let key_b64 = std::env::var("GITHUB_TOKEN_ENC_KEY")
        .map_err(|_| anyhow!("GITHUB_TOKEN_ENC_KEY is not set"))?;
    let key_bytes = STANDARD
        .decode(key_b64.trim())
        .map_err(|_| anyhow!("GITHUB_TOKEN_ENC_KEY must be valid base64"))?;
    if key_bytes.len() != 32 {
        return Err(anyhow!(
            "GITHUB_TOKEN_ENC_KEY must decode to exactly 32 bytes"
        ));
    }
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    Ok(Aes256Gcm::new(key))
}

/// Encrypts a GitHub PAT for storage. Output is base64(nonce || ciphertext).
pub fn encrypt_token(plaintext: &str) -> Result<String> {
    let cipher = cipher()?;
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| anyhow!("Failed to encrypt GitHub token"))?;

    let mut combined = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);
    Ok(STANDARD.encode(combined))
}

pub fn decrypt_token(encoded: &str) -> Result<String> {
    let cipher = cipher()?;
    let combined = STANDARD
        .decode(encoded)
        .map_err(|_| anyhow!("Stored GitHub token is not valid base64"))?;
    if combined.len() < NONCE_LEN {
        return Err(anyhow!("Stored GitHub token is malformed"));
    }
    let (nonce_bytes, ciphertext) = combined.split_at(NONCE_LEN);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow!("Failed to decrypt GitHub token"))?;
    String::from_utf8(plaintext).map_err(|_| anyhow!("Decrypted GitHub token is not valid UTF-8"))
}
