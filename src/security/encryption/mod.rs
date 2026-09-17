//! Encryption module for Widya Enterprise Edition
//! Provides Field-Level Encryption (FLE), Data Masking, and Key Management

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::security::{SecurityError, Result, EncryptionConfig, CryptoProvider, PiiType, DataClassification};

/// Field-Level Encryption (FLE) manager
pub struct FieldEncryption {
    /// Configuration
    config: EncryptionConfig,
    
    /// Cryptography provider
    crypto_provider: Arc<CryptoProvider>,
    
    /// Data Encryption Keys (DEKs) per tenant
    data_keys: Arc<RwLock<HashMap<String, HashMap<String, DataEncryptionKey>>>>,
    
    /// Field encryption policies
    field_policies: Arc<RwLock<HashMap<String, FieldEncryptionPolicy>>>,
    
    /// Key rotation schedule
    rotation_schedule: Arc<RwLock<HashMap<String, KeyRotationSchedule>>>,
}

impl FieldEncryption {
    /// Create new FLE manager
    pub fn new(config: &EncryptionConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            crypto_provider,
            data_keys: Arc::new(RwLock::new(HashMap::new())),
            field_policies: Arc::new(RwLock::new(HashMap::new())),
            rotation_schedule: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Encrypt field value
    pub fn encrypt_field(
        &self,
        field_name: &str,
        plaintext: &str,
        tenant_id: &str,
        classification: DataClassification,
    ) -> Result<EncryptedField> {
        // Get or create DEK for tenant
        let dek = self.get_or_create_dek(tenant_id, field_name, classification)?;
        
        // Encrypt the data
        let ciphertext = self.crypto_provider.encrypt_symmetric(
            plaintext.as_bytes(),
            &dek.key_id,
        )?;
        
        // Create encrypted field
        let encrypted_field = EncryptedField {
            field_name: field_name.to_string(),
            ciphertext: ciphertext,
            key_id: dek.key_id.clone(),
            key_version: dek.version,
            algorithm: dek.algorithm,
            iv: vec![], // IV is included in ciphertext
            auth_tag: vec![], // Auth tag is included in ciphertext
            tenant_id: tenant_id.to_string(),
            classification,
            created_at: current_time(),
        };
        
        Ok(encrypted_field)
    }
    
    /// Decrypt field value
    pub fn decrypt_field(&self, encrypted_field: &EncryptedField) -> Result<String> {
        // Verify classification allows decryption
        self.verify_decryption_permission(&encrypted_field.classification)?;
        
        // Decrypt the data
        let plaintext_bytes = self.crypto_provider.decrypt_symmetric(
            &encrypted_field.ciphertext,
            &encrypted_field.key_id,
        )?;
        
        // Convert to string
        let plaintext = String::from_utf8(plaintext_bytes)
            .map_err(|e| SecurityError::EncryptionError(format!("Invalid UTF-8: {}", e)))?;
        
        Ok(plaintext)
    }
    
    /// Get or create Data Encryption Key (DEK)
    fn get_or_create_dek(
        &self,
        tenant_id: &str,
        field_name: &str,
        classification: DataClassification,
    ) -> Result<DataEncryptionKey> {
        let mut data_keys = self.data_keys.write().unwrap();
        
        // Get tenant keys
        let tenant_keys = data_keys.entry(tenant_id.to_string())
            .or_insert_with(HashMap::new);
        
        // Check if we have an active key for this field
        let key_id = format!("dek-{}-{}", tenant_id, field_name);
        
        if let Some(existing_key) = tenant_keys.get(&key_id) {
            // Check if key needs rotation
            if self.needs_key_rotation(existing_key) {
                return self.rotate_key(tenant_id, field_name, classification);
            }
            return Ok(existing_key.clone());
        }
        
        // Create new key
        let dek = self.create_dek(tenant_id, field_name, classification)?;
        tenant_keys.insert(key_id, dek.clone());
        
        Ok(dek)
    }
    
    /// Create new Data Encryption Key
    fn create_dek(
        &self,
        tenant_id: &str,
        field_name: &str,
        classification: DataClassification,
    ) -> Result<DataEncryptionKey> {
        // Generate symmetric key
        let symmetric_key = self.crypto_provider.generate_symmetric_key(
            crate::security::SymmetricAlgorithm::Aes256Gcm,
        )?;
        
        let key_id = format!("dek-{}-{}-v1", tenant_id, field_name);
        
        let dek = DataEncryptionKey {
            id: key_id,
            version: 1,
            algorithm: crate::security::SymmetricAlgorithm::Aes256Gcm,
            key_material: symmetric_key.key,
            tenant_id: tenant_id.to_string(),
            field_name: field_name.to_string(),
            classification,
            created_at: current_time(),
            expires_at: current_time() + (self.config.dek_rotation_days as u64 * 86400),
            active: true,
            previous_versions: Vec::new(),
        };
        
        // Schedule key rotation
        self.schedule_key_rotation(&dek)?;
        
        Ok(dek)
    }
    
    /// Rotate encryption key
    fn rotate_key(
        &self,
        tenant_id: &str,
        field_name: &str,
        classification: DataClassification,
    ) -> Result<DataEncryptionKey> {
        let mut data_keys = self.data_keys.write().unwrap();
        let tenant_keys = data_keys.entry(tenant_id.to_string())
            .or_insert_with(HashMap::new);
        
        let old_key_id = format!("dek-{}-{}", tenant_id, field_name);
        
        // Get old key
        let old_key = tenant_keys.get(&old_key_id)
            .cloned()
            .ok_or_else(|| SecurityError::KeyManagementError(
                format!("Key not found: {}", old_key_id)
            ))?;
        
        // Mark old key as inactive
        let mut old_key = old_key;
        old_key.active = false;
        
        // Create new key
        let new_version = old_key.version + 1;
        let new_key_id = format!("dek-{}-{}-v{}", tenant_id, field_name, new_version);
        
        let new_symmetric_key = self.crypto_provider.generate_symmetric_key(
            crate::security::SymmetricAlgorithm::Aes256Gcm,
        )?;
        
        let new_dek = DataEncryptionKey {
            id: new_key_id.clone(),
            version: new_version,
            algorithm: crate::security::SymmetricAlgorithm::Aes256Gcm,
            key_material: new_symmetric_key.key,
            tenant_id: tenant_id.to_string(),
            field_name: field_name.to_string(),
            classification,
            created_at: current_time(),
            expires_at: current_time() + (self.config.dek_rotation_days as u64 * 86400),
            active: true,
            previous_versions: vec![old_key.id.clone()],
        };
        
        // Update old key's previous versions
        let mut updated_old_key = old_key.clone();
        updated_old_key.previous_versions.push(new_key_id.clone());
        
        // Store both keys
        tenant_keys.insert(old_key_id, updated_old_key);
        tenant_keys.insert(new_key_id, new_dek.clone());
        
        // Schedule rotation for new key
        self.schedule_key_rotation(&new_dek)?;
        
        Ok(new_dek)
    }
    
    /// Check if key needs rotation
    fn needs_key_rotation(&self, key: &DataEncryptionKey) -> bool {
        let current_time = current_time();
        
        // Check expiration
        if key.expires_at <= current_time {
            return true;
        }
        
        // Check rotation schedule
        let schedule = self.rotation_schedule.read().unwrap();
        if let Some(sched) = schedule.get(&key.id) {
            if sched.next_rotation <= current_time {
                return true;
            }
        }
        
        false
    }
    
    /// Schedule key rotation
    fn schedule_key_rotation(&self, key: &DataEncryptionKey) -> Result<()> {
        let rotation_time = key.created_at + (self.config.dek_rotation_days as u64 * 86400) - 86400; // 1 day before expiration
        
        let schedule = KeyRotationSchedule {
            key_id: key.id.clone(),
            last_rotation: key.created_at,
            next_rotation: rotation_time,
            rotation_interval_days: self.config.dek_rotation_days,
        };
        
        let mut rotation_schedule = self.rotation_schedule.write().unwrap();
        rotation_schedule.insert(key.id.clone(), schedule);
        
        Ok(())
    }
    
    /// Verify decryption permission based on classification
    fn verify_decryption_permission(&self, classification: &DataClassification) -> Result<()> {
        // In production, this would check user permissions
        // For now, just allow all classifications
        
        match classification {
            DataClassification::Public | DataClassification::Internal => Ok(()),
            DataClassification::Confidential | DataClassification::Secret | DataClassification::TopSecret => {
                // Log access to sensitive data
                println!("[FLE] Accessing {} data", classification_name(*classification));
                Ok(())
            }
        }
    }
    
    /// Add field encryption policy
    pub fn add_field_policy(&self, policy: FieldEncryptionPolicy) -> Result<()> {
        let mut policies = self.field_policies.write().unwrap();
        policies.insert(policy.field_name.clone(), policy);
        Ok(())
    }
    
    /// Get encryption policy for field
    pub fn get_field_policy(&self, field_name: &str) -> Option<FieldEncryptionPolicy> {
        let policies = self.field_policies.read().unwrap();
        policies.get(field_name).cloned()
    }
    
    /// Cleanup expired keys
    pub fn cleanup_expired_keys(&self) -> Result<usize> {
        let mut data_keys = self.data_keys.write().unwrap();
        let mut cleanup_count = 0;
        
        for tenant_keys in data_keys.values_mut() {
            let expired_keys: Vec<String> = tenant_keys.iter()
                .filter(|(_, key)| !key.active && current_time() - key.created_at > 90 * 86400) // 90 days after deactivation
                .map(|(id, _)| id.clone())
                .collect();
            
            for key_id in expired_keys {
                tenant_keys.remove(&key_id);
                cleanup_count += 1;
            }
        }
        
        Ok(cleanup_count)
    }
}

/// Data Encryption Key (DEK)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEncryptionKey {
    pub id: String,
    pub version: u32,
    pub algorithm: crate::security::SymmetricAlgorithm,
    #[serde(with = "serde_bytes")]
    pub key_material: Vec<u8>,
    pub tenant_id: String,
    pub field_name: String,
    pub classification: DataClassification,
    pub created_at: u64,
    pub expires_at: u64,
    pub active: bool,
    pub previous_versions: Vec<String>,
}

impl Drop for DataEncryptionKey {
    fn drop(&mut self) {
        self.key_material.zeroize();
    }
}

/// Encrypted field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedField {
    pub field_name: String,
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
    pub key_id: String,
    pub key_version: u32,
    pub algorithm: crate::security::SymmetricAlgorithm,
    #[serde(with = "serde_bytes")]
    pub iv: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub auth_tag: Vec<u8>,
    pub tenant_id: String,
    pub classification: DataClassification,
    pub created_at: u64,
}

/// Field encryption policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldEncryptionPolicy {
    pub field_name: String,
    pub classification: DataClassification,
    pub pii_type: Option<PiiType>,
    pub encryption_required: bool,
    pub searchable: bool,
    pub indexable: bool,
    pub allowed_operations: Vec<String>, // "eq", "range", "search", etc.
    pub retention_days: Option<u32>,
}

/// Key rotation schedule
#[derive(Debug, Clone)]
struct KeyRotationSchedule {
    key_id: String,
    last_rotation: u64,
    next_rotation: u64,
    rotation_interval_days: u32,
}

/// Data masker for PII (Personally Identifiable Information)
pub struct DataMasker {
    /// Configuration
    config: EncryptionConfig,
    
    /// Masking patterns for different PII types
    masking_patterns: Arc<RwLock<HashMap<PiiType, MaskingPattern>>>,
    
    /// Field masking rules
    field_rules: Arc<RwLock<HashMap<String, FieldMaskingRule>>>,
}

impl DataMasker {
    /// Create new data masker
    pub fn new(config: &EncryptionConfig) -> Self {
        let mut masker = Self {
            config: config.clone(),
            masking_patterns: Arc::new(RwLock::new(HashMap::new())),
            field_rules: Arc::new(RwLock::new(HashMap::new())),
        };
        
        // Initialize default masking patterns
        masker.initialize_default_patterns();
        
        masker
    }
    
    /// Initialize default masking patterns
    fn initialize_default_patterns(&mut self) {
        let mut patterns = self.masking_patterns.write().unwrap();
        
        // Email masking
        patterns.insert(PiiType::Email, MaskingPattern {
            pii_type: PiiType::Email,
            regex: r"^([^@]+)@([^@]+)$".to_string(),
            replacement: r"${1:0:3}***@${2}".to_string(),
            example: "john.doe@example.com → joh***@example.com",
        });
        
        // Phone number masking
        patterns.insert(PiiType::Phone, MaskingPattern {
            pii_type: PiiType::Phone,
            regex: r"^(\d{3})-(\d{3})-(\d{4})$".to_string(),
            replacement: r"***-***-${3}".to_string(),
            example: "123-456-7890 → ***-***-7890",
        });
        
        // SSN masking
        patterns.insert(PiiType::Ssn, MaskingPattern {
            pii_type: PiiType::Ssn,
            regex: r"^(\d{3})-(\d{2})-(\d{4})$".to_string(),
            replacement: r"***-**-${3}".to_string(),
            example: "123-45-6789 → ***-**-6789",
        });
        
        // Credit card masking
        patterns.insert(PiiType::CreditCard, MaskingPattern {
            pii_type: PiiType::CreditCard,
            regex: r"^(\d{4})-(\d{4})-(\d{4})-(\d{4})$".to_string(),
            replacement: r"****-****-****-${4}".to_string(),
            example: "1234-5678-9012-3456 → ****-****-****-3456",
        });
        
        // Name masking
        patterns.insert(PiiType::Name, MaskingPattern {
            pii_type: PiiType::Name,
            regex: r"^(\w+)\s+(\w+)$".to_string(),
            replacement: r"${1:0:1}. ${2:0:1}.".to_string(),
            example: "John Doe → J. D.",
        });
    }
    
    /// Mask PII data
    pub fn mask_pii(&self, pii_type: PiiType, data: &str) -> Result<String> {
        let patterns = self.masking_patterns.read().unwrap();
        
        if let Some(pattern) = patterns.get(&pii_type) {
            self.apply_masking_pattern(pattern, data)
        } else {
            // Apply default masking
            self.apply_default_masking(data)
        }
    }
    
    /// Apply masking pattern
    fn apply_masking_pattern(&self, pattern: &MaskingPattern, data: &str) -> Result<String> {
        use regex::Regex;
        
        let re = Regex::new(&pattern.regex)
            .map_err(|e| SecurityError::EncryptionError(format!("Invalid regex pattern: {}", e)))?;
        
        if re.is_match(data) {
            let result = re.replace(data, &pattern.replacement).to_string();
            Ok(result)
        } else {
            // If pattern doesn't match, apply default masking
            self.apply_default_masking(data)
        }
    }
    
    /// Apply default masking
    fn apply_default_masking(&self, data: &str) -> Result<String> {
        if data.len() <= 3 {
            // For very short strings, mask everything
            Ok(self.config.masking_char.to_string().repeat(data.len()))
        } else {
            // Show first 3 chars, mask the rest
            let visible_part = &data[..3.min(data.len())];
            let mask_len = data.len() - visible_part.len();
            let masked_part = self.config.masking_char.to_string().repeat(mask_len);
            Ok(format!("{}{}", visible_part, masked_part))
        }
    }
    
    /// Add field masking rule
    pub fn add_field_rule(&self, rule: FieldMaskingRule) -> Result<()> {
        let mut rules = self.field_rules.write().unwrap();
        rules.insert(rule.field_name.clone(), rule);
        Ok(())
    }
    
    /// Mask field based on rules
    pub fn mask_field(&self, field_name: &str, value: &str) -> Result<String> {
        let rules = self.field_rules.read().unwrap();
        
        if let Some(rule) = rules.get(field_name) {
            if rule.masking_enabled {
                if let Some(pii_type) = rule.pii_type {
                    self.mask_pii(pii_type, value)
                } else {
                    self.apply_default_masking(value)
                }
            } else {
                Ok(value.to_string())
            }
        } else {
            // Check if field is in PII fields list
            if self.config.pii_fields.contains(&field_name.to_string()) {
                self.apply_default_masking(value)
            } else {
                Ok(value.to_string())
            }
        }
    }
    
    /// Check if field should be masked
    pub fn should_mask_field(&self, field_name: &str) -> bool {
        let rules = self.field_rules.read().unwrap();
        
        if let Some(rule) = rules.get(field_name) {
            rule.masking_enabled
        } else {
            self.config.pii_fields.contains(&field_name.to_string())
        }
    }
    
    /// Get all PII fields
    pub fn get_pii_fields(&self) -> Vec<String> {
        self.config.pii_fields.clone()
    }
}

/// Masking pattern for PII
#[derive(Debug, Clone)]
struct MaskingPattern {
    pii_type: PiiType,
    regex: String,
    replacement: String,
    example: String,
}

/// Field masking rule
#[derive(Debug, Clone)]
pub struct FieldMaskingRule {
    pub field_name: String,
    pub pii_type: Option<PiiType>,
    pub masking_enabled: bool,
    pub masking_pattern: Option<String>,
    pub visible_chars_start: Option<usize>,
    pub visible_chars_end: Option<usize>,
    pub unmasked_roles: Vec<String>,
}

/// Key manager for encryption keys
pub struct KeyManager {
    /// Master keys
    master_keys: Arc<RwLock<HashMap<String, MasterKey>>>,
    
    /// Key encryption keys (KEKs)
    key_encryption_keys: Arc<RwLock<HashMap<String, KeyEncryptionKey>>>,
    
    /// Key metadata
    key_metadata: Arc<RwLock<HashMap<String, KeyMetadata>>>,
}

impl KeyManager {
    /// Create new key manager
    pub fn new() -> Self {
        Self {
            master_keys: Arc::new(RwLock::new(HashMap::new())),
            key_encryption_keys: Arc::new(RwLock::new(HashMap::new())),
            key_metadata: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Generate master key
    pub fn generate_master_key(&self, key_id: &str) -> Result<MasterKey> {
        let master_key = MasterKey {
            id: key_id.to_string(),
            version: 1,
            created_at: current_time(),
            expires_at: current_time() + 365 * 86400, // 1 year
            active: true,
            metadata: KeyMetadata {
                key_id: key_id.to_string(),
                key_type: "master".to_string(),
                algorithm: "AES-256".to_string(),
                key_size: 256,
                creation_date: current_time(),
                activation_date: current_time(),
                deactivation_date: None,
                state: "active".to_string(),
                origin: "software".to_string(),
                operations: vec!["encrypt".to_string(), "decrypt".to_string()],
            },
        };
        
        let mut keys = self.master_keys.write().unwrap();
        keys.insert(key_id.to_string(), master_key.clone());
        
        let mut metadata = self.key_metadata.write().unwrap();
        metadata.insert(key_id.to_string(), master_key.metadata.clone());
        
        Ok(master_key)
    }
    
    /// Generate key encryption key (KEK)
    pub fn generate_kek(&self, key_id: &str, master_key_id: &str) -> Result<KeyEncryptionKey> {
        let kek = KeyEncryptionKey {
            id: key_id.to_string(),
            master_key_id: master_key_id.to_string(),
            version: 1,
            created_at: current_time(),
            expires_at: current_time() + 180 * 86400, // 6 months
            active: true,
            metadata: KeyMetadata {
                key_id: key_id.to_string(),
                key_type: "kek".to_string(),
                algorithm: "AES-256".to_string(),
                key_size: 256,
                creation_date: current_time(),
                activation_date: current_time(),
                deactivation_date: None,
                state: "active".to_string(),
                origin: "software".to_string(),
                operations: vec!["wrap".to_string(), "unwrap".to_string()],
            },
        };
        
        let mut keks = self.key_encryption_keys.write().unwrap();
        keks.insert(key_id.to_string(), kek.clone());
        
        let mut metadata = self.key_metadata.write().unwrap();
        metadata.insert(key_id.to_string(), kek.metadata.clone());
        
        Ok(kek)
    }
    
    /// Get key metadata
    pub fn get_key_metadata(&self, key_id: &str) -> Option<KeyMetadata> {
        let metadata = self.key_metadata.read().unwrap();
        metadata.get(key_id).cloned()
    }
    
    /// List all keys
    pub fn list_keys(&self) -> Vec<KeyMetadata> {
        let metadata = self.key_metadata.read().unwrap();
        metadata.values().cloned().collect()
    }
    
    /// Rotate key
    pub fn rotate_key(&self, key_id: &str) -> Result<KeyMetadata> {
        let mut metadata = self.key_metadata.write().unwrap();
        
        if let Some(mut key_meta) = metadata.get_mut(key_id) {
            // Deactivate old key
            key_meta.state = "deactivated".to_string();
            key_meta.deactivation_date = Some(current_time());
            
            // Create new version
            let new_key_id = format!("{}-v{}", key_id, key_meta.key_size + 1);
            let new_metadata = KeyMetadata {
                key_id: new_key_id.clone(),
                key_type: key_meta.key_type.clone(),
                algorithm: key_meta.algorithm.clone(),
                key_size: key_meta.key_size,
                creation_date: current_time(),
                activation_date: current_time(),
                deactivation_date: None,
                state: "active".to_string(),
                origin: key_meta.origin.clone(),
                operations: key_meta.operations.clone(),
            };
            
            metadata.insert(new_key_id, new_metadata.clone());
            
            Ok(new_metadata)
        } else {
            Err(SecurityError::KeyManagementError(
                format!("Key not found: {}", key_id)
            ))
        }
    }
    
    /// Cleanup expired keys
    pub fn cleanup_expired_keys(&self) -> Result<usize> {
        let current_time = current_time();
        let mut metadata = self.key_metadata.write().unwrap();
        
        let expired_keys: Vec<String> = metadata.iter()
            .filter(|(_, meta)| {
                meta.state == "deactivated" && 
                meta.deactivation_date.map_or(false, |d| current_time - d > 90 * 86400)
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        let cleanup_count = expired_keys.len();
        
        for key_id in expired_keys {
            metadata.remove(&key_id);
        }
        
        Ok(cleanup_count)
    }
}

/// Master key
#[derive(Debug, Clone)]
pub struct MasterKey {
    pub id: String,
    pub version: u32,
    pub created_at: u64,
    pub expires_at: u64,
    pub active: bool,
    pub metadata: KeyMetadata,
}

/// Key encryption key (KEK)
#[derive(Debug, Clone)]
pub struct KeyEncryptionKey {
    pub id: String,
    pub master_key_id: String,
    pub version: u32,
    pub created_at: u64,
    pub expires_at: u64,
    pub active: bool,
    pub metadata: KeyMetadata,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    pub key_id: String,
    pub key_type: String,
    pub algorithm: String,
    pub key_size: u32,
    pub creation_date: u64,
    pub activation_date: u64,
    pub deactivation_date: Option<u64>,
    pub state: String,
    pub origin: String,
    pub operations: Vec<String>,
}

/// Helper function to get current timestamp
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Helper function to get classification name
fn classification_name(classification: DataClassification) -> &'static str {
    match classification {
        DataClassification::Public => "Public",
        DataClassification::Internal => "Internal",
        DataClassification::Confidential => "Confidential",
        DataClassification::Secret => "Secret",
        DataClassification::TopSecret => "TopSecret",
    }
}

/// Zeroize trait implementation
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