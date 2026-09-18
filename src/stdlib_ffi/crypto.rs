use sha2::{Digest, Sha256};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Utilitas Kriptografi Standar untuk Widya-Lang
pub struct Crypto;

impl Crypto {
    /// Menghasilkan hash SHA-256 dalam format heksadesimal
    pub fn sha256_hex(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        hex::encode(result)
    }

    /// Melakukan encoding byte ke format string Base64
    pub fn base64_encode(data: &[u8]) -> String {
        BASE64.encode(data)
    }

    /// Melakukan decoding string Base64 ke byte asli
    pub fn base64_decode(encoded: &str) -> Result<Vec<u8>, String> {
        BASE64.decode(encoded).map_err(|e| format!("Base64 Decode Error: {}", e))
    }

    /// Menghitung HMAC SHA-256
    pub fn hmac_sha256(key: &[u8], message: &[u8]) -> String {
        // Implementasi HMAC sederhana RFC 2104 berbasis Sha256
        let block_size = 64;
        let mut key_pad = vec![0u8; block_size];
        if key.len() > block_size {
            let mut hasher = Sha256::new();
            hasher.update(key);
            let k = hasher.finalize();
            key_pad[..32].copy_from_slice(&k);
        } else {
            key_pad[..key.len()].copy_from_slice(key);
        }

        let mut o_key_pad = vec![0u8; block_size];
        let mut i_key_pad = vec![0u8; block_size];

        for i in 0..block_size {
            o_key_pad[i] = key_pad[i] ^ 0x5c;
            i_key_pad[i] = key_pad[i] ^ 0x36;
        }

        // Inner hash
        let mut inner_hasher = Sha256::new();
        inner_hasher.update(&i_key_pad);
        inner_hasher.update(message);
        let inner_hash = inner_hasher.finalize();

        // Outer hash
        let mut outer_hasher = Sha256::new();
        outer_hasher.update(&o_key_pad);
        outer_hasher.update(&inner_hash);
        let final_hash = outer_hasher.finalize();

        hex::encode(final_hash)
    }

    /// Menghasilkan token acak berbasis waktu & deterministik
    pub fn generate_token(prefix: &str) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let raw = format!("{}:{}:widya_salt", prefix, now);
        Self::sha256_hex(raw.as_bytes())
    }
}
