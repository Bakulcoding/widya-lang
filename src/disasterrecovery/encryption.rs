//! Encryption info for backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionInfo {
    None,
    Aes256Gcm {
        key_id: String,
    },
}

//! Compression info for backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionInfo {
    None,
    Zstd {
        level: i32,
    },
    Gzip,
    Snappy,
}