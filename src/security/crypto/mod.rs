//! Cryptography module for Widya Enterprise Edition
//! Provides FIPS 140-3 compliant cryptographic operations

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::security::{SecurityError, Result, CryptoConfig, HashAlgorithm, SymmetricAlgorithm, AsymmetricAlgorithm};

/// Cryptography provider for FIPS 140-3 compliant operations
pub struct CryptoProvider {
    /// Configuration
    config: CryptoConfig,
    
    /// Key store for symmetric keys
    symmetric_keys: Arc<RwLock<HashMap<String, SymmetricKey>>>,
    
    /// Key store for asymmetric key pairs
    asymmetric_keys: Arc<RwLock<HashMap<String, AsymmetricKeyPair>>>,
    
    /// Hardware Security Module client (optional)
    hsm_client: Option<HsmClient>,
    
    /// Random number generator
    rng: Arc<RwLock<rand::rngs::OsRng>>,
}

impl CryptoProvider {
    /// Create new cryptography provider
    pub fn new(config: &CryptoConfig) -> Result<Self> {
        let hsm_client = if let Some(hsm_url) = &config.hsm_url {
            Some(HsmClient::new(hsm_url)?)
        } else {
            None
        };
        
        Ok(Self {
            config: config.clone(),
            symmetric_keys: Arc::new(RwLock::new(HashMap::new())),
            asymmetric_keys: Arc::new(RwLock::new(HashMap::new())),
            hsm_client,
            rng: Arc::new(RwLock::new(rand::rngs::OsRng)),
        })
    }
    
    /// Generate hash of data
    pub fn hash(&self, data: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>> {
        match algorithm {
            HashAlgorithm::Sha256 => {
                use sha2::Sha256;
                use sha2::Digest;
                let mut hasher = Sha256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha384 => {
                use sha2::Sha384;
                use sha2::Digest;
                let mut hasher = Sha384::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha512 => {
                use sha2::Sha512;
                use sha2::Digest;
                let mut hasher = Sha512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha3_256 => {
                use sha3::Sha3_256;
                use sha3::Digest;
                let mut hasher = Sha3_256::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha3_384 => {
                use sha3::Sha3_384;
                use sha3::Digest;
                let mut hasher = Sha3_384::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
            HashAlgorithm::Sha3_512 => {
                use sha3::Sha3_512;
                use sha3::Digest;
                let mut hasher = Sha3_512::new();
                hasher.update(data);
                Ok(hasher.finalize().to_vec())
            }
        }
    }
    
    /// Generate HMAC (Hash-based Message Authentication Code)
    pub fn hmac(&self, data: &[u8], key: &[u8], algorithm: HashAlgorithm) -> Result<Vec<u8>> {
        match algorithm {
            HashAlgorithm::Sha256 => {
                use hmac::{Hmac, Mac};
                type HmacSha256 = Hmac<sha2::Sha256>;
                let mut mac = HmacSha256::new_from_slice(key)
                    .map_err(|e| SecurityError::CryptoError(format!("HMAC key error: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HashAlgorithm::Sha384 => {
                use hmac::{Hmac, Mac};
                type HmacSha384 = Hmac<sha2::Sha384>;
                let mut mac = HmacSha384::new_from_slice(key)
                    .map_err(|e| SecurityError::CryptoError(format!("HMAC key error: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            HashAlgorithm::Sha512 => {
                use hmac::{Hmac, Mac};
                type HmacSha512 = Hmac<sha2::Sha512>;
                let mut mac = HmacSha512::new_from_slice(key)
                    .map_err(|e| SecurityError::CryptoError(format!("HMAC key error: {}", e)))?;
                mac.update(data);
                Ok(mac.finalize().into_bytes().to_vec())
            }
            _ => Err(SecurityError::CryptoError(
                format!("HMAC not supported for algorithm {:?}", algorithm)
            )),
        }
    }
    
    /// Generate symmetric key
    pub fn generate_symmetric_key(&self, algorithm: SymmetricAlgorithm) -> Result<SymmetricKey> {
        let key_size = match algorithm {
            SymmetricAlgorithm::Aes128Gcm | SymmetricAlgorithm::Aes128Cbc => 16,
            SymmetricAlgorithm::Aes192Gcm | SymmetricAlgorithm::Aes192Cbc => 24,
            SymmetricAlgorithm::Aes256Gcm | SymmetricAlgorithm::Aes256Cbc => 32,
            SymmetricAlgorithm::ChaCha20Poly1305 => 32,
        };
        
        let mut key = vec![0u8; key_size];
        let mut rng = self.rng.write().unwrap();
        rng.fill_bytes(&mut key);
        
        let key_id = format!("sym-{}-{}", 
            algorithm_name(algorithm), 
            generate_key_id());
        
        let symmetric_key = SymmetricKey {
            id: key_id.clone(),
            algorithm,
            key,
            created_at: current_time(),
            expires_at: current_time() + self.config.key_rotation_days as u64 * 86400,
        };
        
        // Store key
        {
            let mut keys = self.symmetric_keys.write().unwrap();
            keys.insert(key_id, symmetric_key.clone());
        }
        
        Ok(symmetric_key)
    }
    
    /// Encrypt data with symmetric key
    pub fn encrypt_symmetric(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>> {
        let key = {
            let keys = self.symmetric_keys.read().unwrap();
            keys.get(key_id).cloned()
                .ok_or_else(|| SecurityError::CryptoError(format!("Key not found: {}", key_id)))?
        };
        
        // Check if key has expired
        if key.expires_at < current_time() {
            return Err(SecurityError::CryptoError(
                format!("Key {} has expired", key_id)
            ));
        }
        
        match key.algorithm {
            SymmetricAlgorithm::Aes256Gcm => {
                use aes_gcm::{
                    aead::{Aead, KeyInit, OsRng},
                    Aes256Gcm, Nonce
                };
                
                let cipher = Aes256Gcm::new_from_slice(&key.key)
                    .map_err(|e| SecurityError::CryptoError(format!("AES-GCM init error: {}", e)))?;
                
                // Generate random nonce
                let mut nonce_bytes = [0u8; 12];
                let mut rng = self.rng.write().unwrap();
                rng.fill_bytes(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                // Encrypt
                let ciphertext = cipher.encrypt(nonce, data)
                    .map_err(|e| SecurityError::CryptoError(format!("AES-GCM encryption error: {}", e)))?;
                
                // Combine nonce + ciphertext
                let mut result = nonce_bytes.to_vec();
                result.extend_from_slice(&ciphertext);
                Ok(result)
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                use chacha20poly1305::{
                    aead::{Aead, KeyInit, OsRng},
                    ChaCha20Poly1305, Nonce
                };
                
                let cipher = ChaCha20Poly1305::new_from_slice(&key.key)
                    .map_err(|e| SecurityError::CryptoError(format!("ChaCha20Poly1305 init error: {}", e)))?;
                
                // Generate random nonce
                let mut nonce_bytes = [0u8; 12];
                let mut rng = self.rng.write().unwrap();
                rng.fill_bytes(&mut nonce_bytes);
                let nonce = Nonce::from_slice(&nonce_bytes);
                
                // Encrypt
                let ciphertext = cipher.encrypt(nonce, data)
                    .map_err(|e| SecurityError::CryptoError(format!("ChaCha20Poly1305 encryption error: {}", e)))?;
                
                // Combine nonce + ciphertext
                let mut result = nonce_bytes.to_vec();
                result.extend_from_slice(&ciphertext);
                Ok(result)
            }
            _ => Err(SecurityError::CryptoError(
                format!("Algorithm {:?} not yet implemented", key.algorithm)
            )),
        }
    }
    
    /// Decrypt data with symmetric key
    pub fn decrypt_symmetric(&self, ciphertext: &[u8], key_id: &str) -> Result<Vec<u8>> {
        let key = {
            let keys = self.symmetric_keys.read().unwrap();
            keys.get(key_id).cloned()
                .ok_or_else(|| SecurityError::CryptoError(format!("Key not found: {}", key_id)))?
        };
        
        // Check if key has expired
        if key.expires_at < current_time() {
            return Err(SecurityError::CryptoError(
                format!("Key {} has expired", key_id)
            ));
        }
        
        match key.algorithm {
            SymmetricAlgorithm::Aes256Gcm => {
                use aes_gcm::{
                    aead::{Aead, KeyInit},
                    Aes256Gcm, Nonce
                };
                
                if ciphertext.len() < 12 {
                    return Err(SecurityError::CryptoError("Ciphertext too short".to_string()));
                }
                
                let cipher = Aes256Gcm::new_from_slice(&key.key)
                    .map_err(|e| SecurityError::CryptoError(format!("AES-GCM init error: {}", e)))?;
                
                // Split nonce and ciphertext
                let nonce = Nonce::from_slice(&ciphertext[..12]);
                let ciphertext_data = &ciphertext[12..];
                
                // Decrypt
                cipher.decrypt(nonce, ciphertext_data)
                    .map_err(|e| SecurityError::CryptoError(format!("AES-GCM decryption error: {}", e)))
            }
            SymmetricAlgorithm::ChaCha20Poly1305 => {
                use chacha20poly1305::{
                    aead::{Aead, KeyInit},
                    ChaCha20Poly1305, Nonce
                };
                
                if ciphertext.len() < 12 {
                    return Err(SecurityError::CryptoError("Ciphertext too short".to_string()));
                }
                
                let cipher = ChaCha20Poly1305::new_from_slice(&key.key)
                    .map_err(|e| SecurityError::CryptoError(format!("ChaCha20Poly1305 init error: {}", e)))?;
                
                // Split nonce and ciphertext
                let nonce = Nonce::from_slice(&ciphertext[..12]);
                let ciphertext_data = &ciphertext[12..];
                
                // Decrypt
                cipher.decrypt(nonce, ciphertext_data)
                    .map_err(|e| SecurityError::CryptoError(format!("ChaCha20Poly1305 decryption error: {}", e)))
            }
            _ => Err(SecurityError::CryptoError(
                format!("Algorithm {:?} not yet implemented", key.algorithm)
            )),
        }
    }
    
    /// Generate asymmetric key pair
    pub fn generate_asymmetric_keypair(&self, algorithm: AsymmetricAlgorithm) -> Result<AsymmetricKeyPair> {
        match algorithm {
            AsymmetricAlgorithm::Rsa3072 => {
                use rsa::{RsaPrivateKey, RsaPublicKey};
                use rand::rngs::OsRng;
                
                let mut rng = OsRng;
                let bits = match algorithm {
                    AsymmetricAlgorithm::Rsa2048 => 2048,
                    AsymmetricAlgorithm::Rsa3072 => 3072,
                    AsymmetricAlgorithm::Rsa4096 => 4096,
                    _ => 3072,
                };
                
                let private_key = RsaPrivateKey::new(&mut rng, bits)
                    .map_err(|e| SecurityError::CryptoError(format!("RSA key generation error: {}", e)))?;
                let public_key = RsaPublicKey::from(&private_key);
                
                let key_id = format!("asym-{}-{}", 
                    algorithm_name(algorithm), 
                    generate_key_id());
                
                let keypair = AsymmetricKeyPair {
                    id: key_id.clone(),
                    algorithm,
                    private_key: private_key_to_bytes(&private_key)?,
                    public_key: public_key_to_bytes(&public_key)?,
                    created_at: current_time(),
                    expires_at: current_time() + self.config.key_rotation_days as u64 * 86400,
                };
                
                // Store key pair
                {
                    let mut keys = self.asymmetric_keys.write().unwrap();
                    keys.insert(key_id, keypair.clone());
                }
                
                Ok(keypair)
            }
            AsymmetricAlgorithm::Ed25519 => {
                use ed25519_dalek::{SigningKey, VerifyingKey};
                use rand::rngs::OsRng;
                
                let mut rng = OsRng;
                let signing_key = SigningKey::generate(&mut rng);
                let verifying_key = VerifyingKey::from(&signing_key);
                
                let key_id = format!("asym-{}-{}", 
                    algorithm_name(algorithm), 
                    generate_key_id());
                
                let keypair = AsymmetricKeyPair {
                    id: key_id.clone(),
                    algorithm,
                    private_key: signing_key.to_bytes().to_vec(),
                    public_key: verifying_key.to_bytes().to_vec(),
                    created_at: current_time(),
                    expires_at: current_time() + self.config.key_rotation_days as u64 * 86400,
                };
                
                // Store key pair
                {
                    let mut keys = self.asymmetric_keys.write().unwrap();
                    keys.insert(key_id, keypair.clone());
                }
                
                Ok(keypair)
            }
            _ => Err(SecurityError::CryptoError(
                format!("Algorithm {:?} not yet implemented", algorithm)
            )),
        }
    }
    
    /// Sign data with private key
    pub fn sign(&self, data: &[u8], key_id: &str) -> Result<Vec<u8>> {
        let keypair = {
            let keys = self.asymmetric_keys.read().unwrap();
            keys.get(key_id).cloned()
                .ok_or_else(|| SecurityError::CryptoError(format!("Key not found: {}", key_id)))?
        };
        
        // Check if key has expired
        if keypair.expires_at < current_time() {
            return Err(SecurityError::CryptoError(
                format!("Key {} has expired", key_id)
            ));
        }
        
        match keypair.algorithm {
            AsymmetricAlgorithm::Rsa3072 => {
                use rsa::{RsaPrivateKey, pkcs1v15::SigningKey, sha2::Sha256};
                use signature::{RandomizedSigner, SignatureEncoding};
                
                // Hash the data first
                let hash = self.hash(data, HashAlgorithm::Sha256)?;
                
                // Create signing key
                let private_key: RsaPrivateKey = rsa::pkcs1::DecodeRsaPrivateKey::from_pkcs1_der(&keypair.private_key)
                    .map_err(|e| SecurityError::CryptoError(format!("RSA private key decode error: {}", e)))?;
                
                let signing_key = SigningKey::<Sha256>::new(private_key);
                
                // Sign
                let mut rng = self.rng.write().unwrap();
                let signature = signing_key.sign_with_rng(&mut *rng, &hash);
                
                Ok(signature.to_vec())
            }
            AsymmetricAlgorithm::Ed25519 => {
                use ed25519_dalek::{SigningKey, Signature, Signer};
                
                let signing_key = SigningKey::from_bytes(&keypair.private_key[..32].try_into()
                    .map_err(|_| SecurityError::CryptoError("Invalid private key length".to_string()))?);
                
                let signature: Signature = signing_key.sign(data);
                Ok(signature.to_bytes().to_vec())
            }
            _ => Err(SecurityError::CryptoError(
                format!("Algorithm {:?} not yet implemented", keypair.algorithm)
            )),
        }
    }
    
    /// Verify signature with public key
    pub fn verify(&self, data: &[u8], signature: &[u8], key_id: &str) -> Result<bool> {
        let keypair = {
            let keys = self.asymmetric_keys.read().unwrap();
            keys.get(key_id).cloned()
                .ok_or_else(|| SecurityError::CryptoError(format!("Key not found: {}", key_id)))?
        };
        
        match keypair.algorithm {
            AsymmetricAlgorithm::Rsa3072 => {
                use rsa::{RsaPublicKey, pkcs1v15::VerifyingKey, sha2::Sha256};
                use signature::{Signature, Verifier};
                
                // Hash the data first
                let hash = self.hash(data, HashAlgorithm::Sha256)?;
                
                // Create verifying key
                let public_key: RsaPublicKey = rsa::pkcs1::DecodeRsaPublicKey::from_pkcs1_der(&keypair.public_key)
                    .map_err(|e| SecurityError::CryptoError(format!("RSA public key decode error: {}", e)))?;
                
                let verifying_key = VerifyingKey::<Sha256>::new(public_key);
                
                // Create signature object
                let rsa_signature = rsa::pkcs1v15::Signature::try_from(signature)
                    .map_err(|e| SecurityError::CryptoError(format!("Signature decode error: {}", e)))?;
                
                // Verify
                verifying_key.verify(&hash, &rsa_signature)
                    .map(|_| true)
                    .map_err(|e| SecurityError::CryptoError(format!("Signature verification error: {}", e)))
            }
            AsymmetricAlgorithm::Ed25519 => {
                use ed25519_dalek::{VerifyingKey, Signature, Verifier};
                
                let verifying_key = VerifyingKey::from_bytes(&keypair.public_key[..32].try_into()
                    .map_err(|_| SecurityError::CryptoError("Invalid public key length".to_string()))?);
                
                let signature = Signature::from_bytes(&signature[..64].try_into()
                    .map_err(|_| SecurityError::CryptoError("Invalid signature length".to_string()))?);
                
                verifying_key.verify(data, &signature)
                    .map(|_| true)
                    .map_err(|e| SecurityError::CryptoError(format!("Signature verification error: {}", e)))
            }
            _ => Err(SecurityError::CryptoError(
                format!("Algorithm {:?} not yet implemented", keypair.algorithm)
            )),
        }
    }
    
    /// Generate cryptographically secure random bytes
    pub fn random_bytes(&self, size: usize) -> Result<Vec<u8>> {
        let mut bytes = vec![0u8; size];
        let mut rng = self.rng.write().unwrap();
        rng.fill_bytes(&mut bytes);
        Ok(bytes)
    }
    
    /// Generate password hash with salt
    pub fn hash_password(&self, password: &str) -> Result<String> {
        // Use Argon2id (FIPS-approved password hashing)
        use argon2::{
            Argon2, Algorithm, Version, Params,
            password_hash::{PasswordHasher, SaltString}
        };
        
        let salt = SaltString::generate(&mut *self.rng.write().unwrap());
        
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(65536, 2, 1, Some(32))
                .map_err(|e| SecurityError::CryptoError(format!("Argon2 params error: {}", e)))?,
        );
        
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| SecurityError::CryptoError(format!("Password hash error: {}", e)))?
            .to_string();
        
        Ok(password_hash)
    }
    
    /// Verify password against hash
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        use argon2::{
            Argon2, Algorithm, Version, Params,
            password_hash::{PasswordHash, PasswordVerifier}
        };
        
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| SecurityError::CryptoError(format!("Hash parse error: {}", e)))?;
        
        let argon2 = Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(65536, 2, 1, Some(32))
                .map_err(|e| SecurityError::CryptoError(format!("Argon2 params error: {}", e)))?,
        );
        
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    /// Cleanup sensitive data
    pub fn cleanup(&self) -> Result<()> {
        // Zeroize keys in memory
        {
            let mut keys = self.symmetric_keys.write().unwrap();
            for key in keys.values_mut() {
                key.key.zeroize();
            }
            keys.clear();
        }
        
        {
            let mut keys = self.asymmetric_keys.write().unwrap();
            for keypair in keys.values_mut() {
                keypair.private_key.zeroize();
            }
            keys.clear();
        }
        
        Ok(())
    }
}

/// Symmetric key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymmetricKey {
    /// Key ID
    pub id: String,
    
    /// Algorithm
    pub algorithm: SymmetricAlgorithm,
    
    /// Key material (will be zeroized on drop)
    #[serde(with = "serde_bytes")]
    pub key: Vec<u8>,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Expiration timestamp
    pub expires_at: u64,
}

impl Drop for SymmetricKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

/// Asymmetric key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsymmetricKeyPair {
    /// Key pair ID
    pub id: String,
    
    /// Algorithm
    pub algorithm: AsymmetricAlgorithm,
    
    /// Private key (will be zeroized on drop)
    #[serde(with = "serde_bytes")]
    pub private_key: Vec<u8>,
    
    /// Public key
    #[serde(with = "serde_bytes")]
    pub public_key: Vec<u8>,
    
    /// Creation timestamp
    pub created_at: u64,
    
    /// Expiration timestamp
    pub expires_at: u64,
}

impl Drop for AsymmetricKeyPair {
    fn drop(&mut self) {
        self.private_key.zeroize();
    }
}

/// Hardware Security Module client (mock implementation)
struct HsmClient {
    url: String,
}

impl HsmClient {
    fn new(url: &str) -> Result<Self> {
        Ok(Self {
            url: url.to_string(),
        })
    }
}

/// Helper function to get current timestamp
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Helper function to generate key ID
fn generate_key_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}", rng.gen::<u64>())
}

/// Helper function to get algorithm name
fn algorithm_name(algorithm: SymmetricAlgorithm) -> &'static str {
    match algorithm {
        SymmetricAlgorithm::Aes128Gcm => "aes-128-gcm",
        SymmetricAlgorithm::Aes192Gcm => "aes-192-gcm",
        SymmetricAlgorithm::Aes256Gcm => "aes-256-gcm",
        SymmetricAlgorithm::Aes128Cbc => "aes-128-cbc",
        SymmetricAlgorithm::Aes192Cbc => "aes-192-cbc",
        SymmetricAlgorithm::Aes256Cbc => "aes-256-cbc",
        SymmetricAlgorithm::ChaCha20Poly1305 => "chacha20-poly1305",
    }
}

/// Helper function to get algorithm name for asymmetric
fn algorithm_name_asym(algorithm: AsymmetricAlgorithm) -> &'static str {
    match algorithm {
        AsymmetricAlgorithm::Rsa2048 => "rsa-2048",
        AsymmetricAlgorithm::Rsa3072 => "rsa-3072",
        AsymmetricAlgorithm::Rsa4096 => "rsa-4096",
        AsymmetricAlgorithm::EcdsaP256 => "ecdsa-p256",
        AsymmetricAlgorithm::EcdsaP384 => "ecdsa-p384",
        AsymmetricAlgorithm::EcdsaP521 => "ecdsa-p521",
        AsymmetricAlgorithm::Ed25519 => "ed25519",
        AsymmetricAlgorithm::Ed448 => "ed448",
    }
}

/// Convert RSA private key to bytes
fn private_key_to_bytes(key: &rsa::RsaPrivateKey) -> Result<Vec<u8>> {
    use rsa::pkcs1::EncodeRsaPrivateKey;
    key.to_pkcs1_der()
        .map_err(|e| SecurityError::CryptoError(format!("RSA private key encode error: {}", e)))
        .map(|der| der.as_bytes().to_vec())
}

/// Convert RSA public key to bytes
fn public_key_to_bytes(key: &rsa::RsaPublicKey) -> Result<Vec<u8>> {
    use rsa::pkcs1::EncodeRsaPublicKey;
    key.to_pkcs1_der()
        .map_err(|e| SecurityError::CryptoError(format!("RSA public key encode error: {}", e)))
        .map(|der| der.as_bytes().to_vec())
}

/// Zeroize trait for secure memory cleanup
trait Zeroize {
    fn zeroize(&mut self);
}

impl Zeroize for Vec<u8> {
    fn zeroize(&mut self) {
        for byte in self.iter_mut() {
            *byte = 0;
        }
    }
}

impl Zeroize for [u8] {
    fn zeroize(&mut self) {
        for byte in self.iter_mut() {
            *byte = 0;
        }
    }
}

// Required trait imports
use rand::RngCore;
use zeroize::Zeroize as ZeroizeTrait;
use std::convert::TryInto;