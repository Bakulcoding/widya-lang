//! Security & Compliance module for Widya Enterprise Edition
//! Provides FIPS 140-3 cryptography, authentication, authorization, and compliance features

pub mod crypto;
pub mod auth;
pub mod encryption;
pub mod audit;
pub mod tls;
pub mod mtls;

// Re-export commonly used types
pub use crypto::{CryptoProvider, HashAlgorithm, SymmetricAlgorithm, AsymmetricAlgorithm};
pub use auth::{Authenticator, Authorization, JwtValidator, OAuthClient, RbacEngine};
pub use encryption::{FieldEncryption, DataMasker, KeyManager};
pub use audit::{AuditLogger, ComplianceChecker, GdprProcessor};
pub use tls::{TlsServer, TlsClient, TlsServerConfig, TlsClientConfig, TlsCipherSuite, MutualTlsAuthenticator, TlsSessionManager};
pub use mtls::{MutualTlsConfig, MutualTlsAuthenticator as MtlsAuthenticator, MutualTlsAuthResult};

/// Security errors
#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Cryptography error: {0}")]
    CryptoError(String),
    
    #[error("Authentication error: {0}")]
    AuthError(String),
    
    #[error("Authorization error: {0}")]
    AuthorizationError(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    #[error("Key management error: {0}")]
    KeyManagementError(String),
    
    #[error("Audit error: {0}")]
    AuditError(String),
    
    #[error("Compliance error: {0}")]
    ComplianceError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Token error: {0}")]
    TokenError(String),
}

/// Result type for security operations
pub type Result<T> = std::result::Result<T, SecurityError>;

/// Security configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    /// Cryptography configuration
    pub crypto: CryptoConfig,
    
    /// Authentication configuration
    pub auth: AuthConfig,
    
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    
    /// Audit configuration
    pub audit: AuditConfig,
    
    /// Compliance configuration
    pub compliance: ComplianceConfig,
}

/// Cryptography configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CryptoConfig {
    /// Enable FIPS 140-3 mode
    pub fips_mode: bool,
    
    /// Default hash algorithm
    pub default_hash: HashAlgorithm,
    
    /// Default symmetric algorithm
    pub default_symmetric: SymmetricAlgorithm,
    
    /// Default asymmetric algorithm
    pub default_asymmetric: AsymmetricAlgorithm,
    
    /// Key rotation period in days
    pub key_rotation_days: u32,
    
    /// Hardware Security Module (HSM) URL (optional)
    pub hsm_url: Option<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthConfig {
    /// Enable JWT authentication
    pub jwt_enabled: bool,
    
    /// JWT secret key
    pub jwt_secret: String,
    
    /// JWT issuer
    pub jwt_issuer: String,
    
    /// JWT audience
    pub jwt_audience: String,
    
    /// JWT expiration in seconds
    pub jwt_expiration_secs: u64,
    
    /// Enable OAuth 2.0
    pub oauth_enabled: bool,
    
    /// OAuth client ID
    pub oauth_client_id: Option<String>,
    
    /// OAuth client secret
    pub oauth_client_secret: Option<String>,
    
    /// OAuth authorization URL
    pub oauth_auth_url: Option<String>,
    
    /// OAuth token URL
    pub oauth_token_url: Option<String>,
    
    /// OAuth userinfo URL
    pub oauth_userinfo_url: Option<String>,
    
    /// Enable OpenID Connect
    pub oidc_enabled: bool,
    
    /// OIDC discovery URL
    pub oidc_discovery_url: Option<String>,
}

/// Encryption configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfig {
    /// Enable field-level encryption
    pub field_encryption_enabled: bool,
    
    /// Master encryption key ID
    pub master_key_id: String,
    
    /// Key encryption key ID
    pub kek_id: String,
    
    /// Data encryption key rotation days
    pub dek_rotation_days: u32,
    
    /// Enable data masking for PII
    pub data_masking_enabled: bool,
    
    /// PII fields to mask
    pub pii_fields: Vec<String>,
    
    /// Masking character
    pub masking_char: char,
    
    /// Masking pattern (e.g., "####-####-####-####" for credit cards)
    pub masking_pattern: Option<String>,
}

/// Audit configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    
    /// Audit log destination (file, syslog, both)
    pub destination: String,
    
    /// Audit log file path
    pub file_path: Option<String>,
    
    /// Retention period in days
    pub retention_days: u32,
    
    /// SOC 2 compliance mode
    pub soc2_compliance: bool,
    
    /// PCI-DSS compliance mode
    pub pci_dss_compliance: bool,
    
    /// HIPAA compliance mode
    pub hipaa_compliance: bool,
    
    /// GDPR compliance mode
    pub gdpr_compliance: bool,
}

/// Compliance configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceConfig {
    /// Enable GDPR compliance features
    pub gdpr_enabled: bool,
    
    /// Data retention period in days
    pub data_retention_days: u32,
    
    /// Right to be forgotten enabled
    pub right_to_be_forgotten: bool,
    
    /// Data portability enabled
    pub data_portability: bool,
    
    /// Consent management enabled
    pub consent_management: bool,
    
    /// Data classification enabled
    pub data_classification: bool,
    
    /// Data minimization enabled
    pub data_minimization: bool,
}

/// Hash algorithms (FIPS 140-3 compliant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HashAlgorithm {
    /// SHA-256 (FIPS 180-4)
    Sha256,
    
    /// SHA-384 (FIPS 180-4)
    Sha384,
    
    /// SHA-512 (FIPS 180-4)
    Sha512,
    
    /// SHA-3-256 (FIPS 202)
    Sha3_256,
    
    /// SHA-3-384 (FIPS 202)
    Sha3_384,
    
    /// SHA-3-512 (FIPS 202)
    Sha3_512,
}

/// Symmetric algorithms (FIPS 140-3 compliant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SymmetricAlgorithm {
    /// AES-128-GCM (NIST SP 800-38D)
    Aes128Gcm,
    
    /// AES-192-GCM (NIST SP 800-38D)
    Aes192Gcm,
    
    /// AES-256-GCM (NIST SP 800-38D)
    Aes256Gcm,
    
    /// AES-128-CBC (NIST SP 800-38A)
    Aes128Cbc,
    
    /// AES-192-CBC (NIST SP 800-38A)
    Aes192Cbc,
    
    /// AES-256-CBC (NIST SP 800-38A)
    Aes256Cbc,
    
    /// ChaCha20-Poly1305 (RFC 8439)
    ChaCha20Poly1305,
}

/// Asymmetric algorithms (FIPS 140-3 compliant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AsymmetricAlgorithm {
    /// RSA-2048 (FIPS 186-4)
    Rsa2048,
    
    /// RSA-3072 (FIPS 186-4)
    Rsa3072,
    
    /// RSA-4096 (FIPS 186-4)
    Rsa4096,
    
    /// ECDSA P-256 (FIPS 186-4)
    EcdsaP256,
    
    /// ECDSA P-384 (FIPS 186-4)
    EcdsaP384,
    
    /// ECDSA P-521 (FIPS 186-4)
    EcdsaP521,
    
    /// Ed25519 (RFC 8032)
    Ed25519,
    
    /// Ed448 (RFC 8032)
    Ed448,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            crypto: CryptoConfig::default(),
            auth: AuthConfig::default(),
            encryption: EncryptionConfig::default(),
            audit: AuditConfig::default(),
            compliance: ComplianceConfig::default(),
        }
    }
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            fips_mode: true,
            default_hash: HashAlgorithm::Sha256,
            default_symmetric: SymmetricAlgorithm::Aes256Gcm,
            default_asymmetric: AsymmetricAlgorithm::Rsa3072,
            key_rotation_days: 90,
            hsm_url: None,
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_enabled: true,
            jwt_secret: "change-this-secret-key".to_string(),
            jwt_issuer: "widya".to_string(),
            jwt_audience: "widya-clients".to_string(),
            jwt_expiration_secs: 3600, // 1 hour
            oauth_enabled: false,
            oauth_client_id: None,
            oauth_client_secret: None,
            oauth_auth_url: None,
            oauth_token_url: None,
            oauth_userinfo_url: None,
            oidc_enabled: false,
            oidc_discovery_url: None,
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            field_encryption_enabled: true,
            master_key_id: "master-key-1".to_string(),
            kek_id: "key-encryption-key-1".to_string(),
            dek_rotation_days: 30,
            data_masking_enabled: true,
            pii_fields: vec![
                "email".to_string(),
                "phone".to_string(),
                "ssn".to_string(),
                "credit_card".to_string(),
                "password".to_string(),
            ],
            masking_char: '*',
            masking_pattern: None,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            destination: "file".to_string(),
            file_path: Some("/var/log/widya/audit.log".to_string()),
            retention_days: 365,
            soc2_compliance: true,
            pci_dss_compliance: false,
            hipaa_compliance: false,
            gdpr_compliance: true,
        }
    }
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            gdpr_enabled: true,
            data_retention_days: 730, // 2 years
            right_to_be_forgotten: true,
            data_portability: true,
            consent_management: true,
            data_classification: true,
            data_minimization: true,
        }
    }
}

/// Initialize security system
pub fn init(config: SecurityConfig) -> Result<SecuritySystem> {
    let system = SecuritySystem::new(config)?;
    Ok(system)
}

/// Security system
pub struct SecuritySystem {
    config: SecurityConfig,
    crypto_provider: crypto::CryptoProvider,
    authenticator: auth::Authenticator,
    encryption_manager: encryption::EncryptionManager,
    audit_logger: audit::AuditLogger,
    compliance_checker: audit::ComplianceChecker,
}

impl SecuritySystem {
    /// Create a new security system
    pub fn new(config: SecurityConfig) -> Result<Self> {
        let crypto_provider = crypto::CryptoProvider::new(&config.crypto)?;
        let authenticator = auth::Authenticator::new(&config.auth)?;
        let encryption_manager = encryption::EncryptionManager::new(&config.encryption)?;
        let audit_logger = audit::AuditLogger::new(&config.audit)?;
        let compliance_checker = audit::ComplianceChecker::new(&config.compliance)?;
        
        Ok(Self {
            config,
            crypto_provider,
            authenticator,
            encryption_manager,
            audit_logger,
            compliance_checker,
        })
    }
    
    /// Get cryptography provider
    pub fn crypto_provider(&self) -> &crypto::CryptoProvider {
        &self.crypto_provider
    }
    
    /// Get authenticator
    pub fn authenticator(&self) -> &auth::Authenticator {
        &self.authenticator
    }
    
    /// Get encryption manager
    pub fn encryption_manager(&self) -> &encryption::EncryptionManager {
        &self.encryption_manager
    }
    
    /// Get audit logger
    pub fn audit_logger(&self) -> &audit::AuditLogger {
        &self.audit_logger
    }
    
    /// Get compliance checker
    pub fn compliance_checker(&self) -> &audit::ComplianceChecker {
        &self.compliance_checker
    }
    
    /// Get configuration
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }
    
    /// Shutdown security system
    pub fn shutdown(&self) -> Result<()> {
        // Cleanup sensitive data
        self.crypto_provider.cleanup()?;
        self.encryption_manager.cleanup()?;
        
        // Flush audit logs
        self.audit_logger.flush()?;
        
        Ok(())
    }
}

/// User principal for authentication
#[derive(Debug, Clone)]
pub struct UserPrincipal {
    /// User ID
    pub user_id: String,
    
    /// Username
    pub username: String,
    
    /// Email
    pub email: Option<String>,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<uuid::Uuid>,
    
    /// Roles
    pub roles: Vec<String>,
    
    /// Permissions
    pub permissions: Vec<String>,
    
    /// Authentication method
    pub auth_method: AuthMethod,
    
    /// Authentication time
    pub auth_time: chrono::DateTime<chrono::Utc>,
    
    /// Session ID
    pub session_id: Option<String>,
}

/// Authentication method
#[derive(Debug, Clone)]
pub enum AuthMethod {
    /// JWT token
    Jwt,
    
    /// OAuth 2.0
    OAuth2,
    
    /// OpenID Connect
    OpenIdConnect,
    
    /// API key
    ApiKey,
    
    /// Basic authentication
    Basic,
    
    /// Certificate
    Certificate,
}

/// Authorization context
#[derive(Debug, Clone)]
pub struct AuthorizationContext {
    /// User principal
    pub principal: UserPrincipal,
    
    /// Resource being accessed
    pub resource: String,
    
    /// Action being performed
    pub action: String,
    
    /// Environment context
    pub environment: HashMap<String, String>,
    
    /// Request time
    pub request_time: chrono::DateTime<chrono::Utc>,
}

impl AuthorizationContext {
    /// Create new authorization context
    pub fn new(principal: UserPrincipal, resource: String, action: String) -> Self {
        Self {
            principal,
            resource,
            action,
            environment: HashMap::new(),
            request_time: chrono::Utc::now(),
        }
    }
    
    /// Add environment variable
    pub fn with_environment(mut self, key: String, value: String) -> Self {
        self.environment.insert(key, value);
        self
    }
    
    /// Check if user has role
    pub fn has_role(&self, role: &str) -> bool {
        self.principal.roles.iter().any(|r| r == role)
    }
    
    /// Check if user has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.principal.permissions.iter().any(|p| p == permission)
    }
}

/// Data classification levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataClassification {
    /// Public data (no restrictions)
    Public,
    
    /// Internal data (company internal)
    Internal,
    
    /// Confidential data (restricted access)
    Confidential,
    
    /// Secret data (highly restricted)
    Secret,
    
    /// Top secret data (maximum protection)
    TopSecret,
}

/// PII (Personally Identifiable Information) types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PiiType {
    /// Name
    Name,
    
    /// Email address
    Email,
    
    /// Phone number
    Phone,
    
    /// Social security number
    Ssn,
    
    /// Credit card number
    CreditCard,
    
    /// Date of birth
    DateOfBirth,
    
    /// Address
    Address,
    
    /// Health information
    HealthInfo,
    
    /// Financial information
    FinancialInfo,
}

/// Compliance requirement
#[derive(Debug, Clone)]
pub struct ComplianceRequirement {
    /// Regulation name (GDPR, HIPAA, PCI-DSS, etc.)
    pub regulation: String,
    
    /// Requirement ID
    pub requirement_id: String,
    
    /// Requirement description
    pub description: String,
    
    /// Is mandatory
    pub mandatory: bool,
    
    /// Implementation status
    pub implemented: bool,
    
    /// Last audit date
    pub last_audit: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Next audit due date
    pub next_audit_due: Option<chrono::DateTime<chrono::Utc>>,
}

use std::collections::HashMap;
use serde::{Serialize, Deserialize};