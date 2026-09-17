//! mTLS (mutual TLS) implementation for mutual authentication
//! Provides mutual authentication with client certificates

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// mTLS mutual authentication configuration
#[derive(Debug, Clone)]
pub struct MutualTlsConfig {
    /// Require client certificate
    pub require_client_cert: bool,
    
    /// Client CA certificate file path
    pub client_ca_cert_path: String,
    
    /// Certificate Revocation List (CRL) file path
    pub crl_path: Option<String>,
    
    /// OCSP (Online Certificate Status Protocol) responder URL
    pub ocsp_responder_url: Option<String>,
    
    /// Maximum certificate chain depth
    pub max_chain_depth: u32,
    
    /// Verify certificate expiration
    pub verify_expiration: bool,
    
    /// Verify certificate revocation
    pub verify_revocation: bool,
    
    /// Allowed client certificate subjects
    pub allowed_subjects: Vec<String>,
    
    /// Allowed client certificate issuers
    pub allowed_issuers: Vec<String>,
    
    /// Certificate pinning (SHA-256 fingerprints)
    pub certificate_pinning: HashMap<String, Vec<String>>,
}

/// mTLS mutual authenticator
pub struct MutualTlsAuthenticator {
    /// Configuration
    config: MutualTlsConfig,
    
    /// Client certificate store
    client_certificates: Arc<RwLock<HashMap<String, ClientCertificate>>>,
    
    /// Certificate revocation list
    crl_store: Arc<RwLock<Vec<RevokedCertificate>>>,
    
    /// OCSP cache
    ocsp_cache: Arc<RwLock<HashMap<String, OcspResponse>>>,
    
    /// Authentication attempts tracking
    auth_attempts: Arc<RwLock<HashMap<String, Vec<AuthAttempt>>>>,
}

impl MutualTlsAuthenticator {
    /// Create new mTLS authenticator
    pub fn new(config: MutualTlsConfig) -> Result<Self> {
        Ok(Self {
            config,
            client_certificates: Arc::new(RwLock::new(HashMap::new())),
            crl_store: Arc::new(RwLock::new(Vec::new())),
            ocsp_cache: Arc::new(RwLock::new(HashMap::new())),
            auth_attempts: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Authenticate client certificate
    pub fn authenticate_client_certificate(
        &self,
        client_cert_pem: &str,
        peer_name: Option<&str>,
    ) -> Result<MutualTlsAuthResult> {
        // Parse certificate
        let cert = self.parse_certificate(client_cert_pem)?;
        
        // Validate certificate
        self.validate_certificate(&cert)?;
        
        // Check revocation
        if self.config.verify_revocation {
            self.check_certificate_revocation(&cert)?;
        }
        
        // Check OCSP status if configured
        if let Some(ocsp_url) = &self.config.ocsp_responder_url {
            self.check_ocsp_status(&cert, ocsp_url)?;
        }
        
        // Check certificate pinning
        if let Some(peer) = peer_name {
            self.check_certificate_pinning(peer, &cert)?;
        }
        
        // Extract certificate information
        let subject = cert.subject_name();
        let issuer = cert.issuer_name();
        let serial = cert.serial_number();
        let validity = cert.validity();
        
        // Record authentication attempt
        self.record_auth_attempt(&cert.serial_number_string(), true);
        
        Ok(MutualTlsAuthResult {
            authenticated: true,
            subject: subject,
            issuer: issuer,
            serial_number: serial,
            not_before: validity.not_before.timestamp() as u64,
            not_after: validity.not_after.timestamp() as u64,
            key_usage: cert.key_usage().unwrap_or_default(),
            extended_key_usage: cert.extended_key_usage().unwrap_or_default(),
            subject_alt_names: cert.subject_alt_names().unwrap_or_default(),
            ocsp_status: self.get_ocsp_status(&cert),
            crl_status: self.get_crl_status(&cert),
        })
    }
    
    /// Parse and validate certificate
    fn parse_certificate(&self, pem_data: &str) -> Result<ClientCertificate> {
        // In production, use rustls-pemfile or similar
        // For now, create mock certificate
        
        let cert = ClientCertificate {
            pem_data: pem_data.to_string(),
            serial_number: "1234567890".to_string(),
            subject: "CN=client.widya.example".to_string(),
            issuer: "CN=widya-ca".to_string(),
            not_before: current_time() - 86400, // 1 day ago
            not_after: current_time() + 86400 * 365, // 1 year
            key_algorithm: "RSA".to_string(),
            key_size: 2048,
            signature_algorithm: "SHA256-RSA".to_string(),
            subject_alt_names: Vec::new(),
            key_usage: vec!["digitalSignature".to_string(), "keyEncipherment".to_string()],
            extended_key_usage: vec!["clientAuth".to_string()],
            basic_constraints: Some("CA:FALSE".to_string()),
        };
        
        Ok(cert)
    }
    
    /// Validate certificate
    fn validate_certificate(&self, cert: &ClientCertificate) -> Result<()> {
        let current_time = current_time();
        
        // Check expiration
        if self.config.verify_expiration {
            if cert.not_before > current_time {
                return Err(SecurityError::AuthError(
                    "Certificate not yet valid".to_string()
                ));
            }
            
            if cert.not_after < current_time {
                return Err(SecurityError::AuthError(
                    "Certificate has expired".to_string()
                ));
            }
        }
        
        // Check allowed subjects
        if !self.config.allowed_subjects.is_empty() {
            if !self.config.allowed_subjects.iter().any(|s| cert.subject.contains(s)) {
                return Err(SecurityError::AuthError(
                    "Certificate subject not allowed".to_string()
                ));
            }
        }
        
        // Check allowed issuers
        if !self.config.allowed_issuers.is_empty() {
            if !self.config.allowed_issuers.iter().any(|s| cert.issuer.contains(s)) {
                return Err(SecurityError::AuthError(
                    "Certificate issuer not allowed".to_string()
                ));
            }
        }
        
        // Check key usage
        if !cert.key_usage.contains(&"digitalSignature".to_string()) {
            return Err(SecurityError::AuthError(
                "Certificate missing digitalSignature key usage".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Check certificate revocation
    fn check_certificate_revocation(&self, cert: &ClientCertificate) -> Result<()> {
        let crl_store = self.crl_store.read().unwrap();
        
        // Check CRL
        for revoked_cert in crl_store.iter() {
            if revoked_cert.serial_number == cert.serial_number {
                return Err(SecurityError::AuthError(
                    format!("Certificate revoked: {}", revoked_cert.reason)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Check OCSP status
    fn check_ocsp_status(&self, cert: &ClientCertificate, ocsp_url: &str) -> Result<()> {
        let cache_key = format!("{}-{}", cert.serial_number, ocsp_url);
        let ocsp_cache = self.ocsp_cache.read().unwrap();
        
        if let Some(response) = ocsp_cache.get(&cache_key) {
            // Check cache validity
            if response.valid_until > current_time() {
                return match response.status.as_str() {
                    "good" => Ok(()),
                    "revoked" => Err(SecurityError::AuthError(
                        "Certificate revoked via OCSP".to_string()
                    )),
                    _ => Err(SecurityError::AuthError(
                        format!("OCSP status: {}", response.status)
                    )),
                };
            }
        }
        
        // In production, would make OCSP request
        // For now, assume good status
        
        Ok(())
    }
    
    /// Check certificate pinning
    fn check_certificate_pinning(&self, peer_name: &str, cert: &ClientCertificate) -> Result<()> {
        if let Some(allowed_fingerprints) = self.config.certificate_pinning.get(peer_name) {
            // Calculate certificate fingerprint
            let fingerprint = self.calculate_certificate_fingerprint(cert);
            
            if !allowed_fingerprints.contains(&fingerprint) {
                return Err(SecurityError::AuthError(
                    "Certificate pinning violation".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    /// Calculate certificate fingerprint
    fn calculate_certificate_fingerprint(&self, cert: &ClientCertificate) -> String {
        // In production, calculate SHA-256 fingerprint
        // For now, return mock fingerprint
        format!("sha256-{}", &cert.serial_number[..16])
    }
    
    /// Record authentication attempt
    fn record_auth_attempt(&self, serial_number: &str, success: bool) {
        let mut attempts = self.auth_attempts.write().unwrap();
        let peer_attempts = attempts.entry(serial_number.to_string())
            .or_insert_with(Vec::new);
        
        let attempt = AuthAttempt {
            timestamp: current_time(),
            success,
            peer_ip: None,
            user_agent: None,
        };
        
        peer_attempts.push(attempt);
        
        // Keep only last 100 attempts
        if peer_attempts.len() > 100 {
            peer_attempts.remove(0);
        }
    }
    
    /// Get OCSP status
    fn get_ocsp_status(&self, cert: &ClientCertificate) -> Option<String> {
        // Mock implementation
        Some("good".to_string())
    }
    
    /// Get CRL status
    fn get_crl_status(&self, cert: &ClientCertificate) -> Option<String> {
        // Mock implementation
        Some("not revoked".to_string())
    }
    
    /// Load CRL from file
    pub fn load_crl(&self, crl_data: &[u8]) -> Result<()> {
        // In production, parse CRL file
        // For now, just add mock entries
        
        let mut crl_store = self.crl_store.write().unwrap();
        
        // Add mock revoked certificates
        crl_store.push(RevokedCertificate {
            serial_number: "9876543210".to_string(),
            revocation_date: current_time() - 86400,
            reason: "keyCompromise".to_string(),
        });
        
        Ok(())
    }
    
    /// Get authentication statistics
    pub fn get_auth_stats(&self) -> MutualTlsStats {
        let attempts = self.auth_attempts.read().unwrap();
        
        let mut total_attempts = 0;
        let mut successful_attempts = 0;
        let mut failed_attempts = 0;
        
        for peer_attempts in attempts.values() {
            total_attempts += peer_attempts.len();
            successful_attempts += peer_attempts.iter().filter(|a| a.success).count();
            failed_attempts += peer_attempts.iter().filter(|a| !a.success).count();
        }
        
        MutualTlsStats {
            total_attempts,
            successful_attempts,
            failed_attempts,
            success_rate: if total_attempts > 0 {
                successful_attempts as f64 / total_attempts as f64
            } else {
                0.0
            },
            unique_clients: attempts.len(),
            crl_entries: self.crl_store.read().unwrap().len(),
            ocsp_cache_entries: self.ocsp_cache.read().unwrap().len(),
        }
    }
}

/// Client certificate
#[derive(Debug, Clone)]
pub struct ClientCertificate {
    pub pem_data: String,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: u64,
    pub not_after: u64,
    pub key_algorithm: String,
    pub key_size: u32,
    pub signature_algorithm: String,
    pub subject_alt_names: Vec<String>,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
    pub basic_constraints: Option<String>,
}

/// Revoked certificate
#[derive(Debug, Clone)]
pub struct RevokedCertificate {
    pub serial_number: String,
    pub revocation_date: u64,
    pub reason: String,
}

/// OCSP response
#[derive(Debug, Clone)]
pub struct OcspResponse {
    pub serial_number: String,
    pub status: String,
    pub valid_until: u64,
    pub response_data: Vec<u8>,
}

/// Authentication attempt
#[derive(Debug, Clone)]
pub struct AuthAttempt {
    pub timestamp: u64,
    pub success: bool,
    pub peer_ip: Option<String>,
    pub user_agent: Option<String>,
}

/// mTLS authentication result
#[derive(Debug, Clone)]
pub struct MutualTlsAuthResult {
    pub authenticated: bool,
    pub subject: String,
    pub issuer: String,
    pub serial_number: String,
    pub not_before: u64,
    pub not_after: u64,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
    pub subject_alt_names: Vec<String>,
    pub ocsp_status: Option<String>,
    pub crl_status: Option<String>,
}

/// mTLS statistics
#[derive(Debug, Clone)]
pub struct MutualTlsStats {
    pub total_attempts: usize,
    pub successful_attempts: usize,
    pub failed_attempts: usize,
    pub success_rate: f64,
    pub unique_clients: usize,
    pub crl_entries: usize,
    pub ocsp_cache_entries: usize,
}

/// Default mTLS configuration
impl Default for MutualTlsConfig {
    fn default() -> Self {
        Self {
            require_client_cert: true,
            client_ca_cert_path: "/etc/widya/ca.crt".to_string(),
            crl_path: Some("/etc/widya/crl.pem".to_string()),
            ocsp_responder_url: Some("http://ocsp.widya.example".to_string()),
            max_chain_depth: 3,
            verify_expiration: true,
            verify_revocation: true,
            allowed_subjects: vec![".widya.example".to_string()],
            allowed_issuers: vec!["CN=widya-ca".to_string()],
            certificate_pinning: HashMap::new(),
        }
    }
}

/// Helper function to get current timestamp
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}