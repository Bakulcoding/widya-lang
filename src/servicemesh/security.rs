//! Security module for Widya service mesh
//! Provides mTLS authentication, certificate management, and service identity

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::servicemesh::{SecurityConfig, ServiceMeshError, Result};

/// mTLS engine for mutual authentication
pub struct mTLSEngine {
    /// Configuration
    config: SecurityConfig,
    
    /// Valid certificates
    certificates: Arc<RwLock<HashMap<String, Certificate>>>,
    
    /// Service identities
    identities: Arc<RwLock<HashMap<String, ServiceIdentity>>>,
    
    /// Certificate revocation list
    crl: Arc<RwLock<HashSet<String>>>,
    
    /// Certificate cache
    cert_cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl mTLSEngine {
    /// Create new mTLS engine
    pub fn new(config: &SecurityConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            certificates: Arc::new(RwLock::new(HashMap::new())),
            identities: Arc::new(RwLock::new(HashMap::new())),
            crl: Arc::new(RwLock::new(HashSet::new())),
            cert_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Authenticate service certificate
    pub fn authenticate(&self, cert_der: &[u8]) -> Result<ServiceIdentity> {
        let cert = self.parse_certificate(cert_der)?;
        
        // Check if certificate is revoked
        {
            let crl = self.crl.read().unwrap();
            if crl.contains(&cert.serial_number) {
                return Err(ServiceMeshError::SecurityError(
                    "Certificate has been revoked".to_string()
                ));
            }
        }
        
        // Verify certificate chain
        self.verify_certificate_chain(&cert)?;
        
        // Check expiration
        self.verify_certificate_expiration(&cert)?;
        
        // Validate service identity
        let identity = self.validate_identity(&cert)?;
        
        // Cache the certificate
        self.cache_certificate(&cert)?;
        
        Ok(identity)
    }
    
    /// Parse certificate from DER format
    fn parse_certificate(&self, der: &[u8]) -> Result<Certificate> {
        // In production, use rustls or similar
        // For now, create mock certificate
        Ok(Certificate {
            der_data: der.to_vec(),
            serial_number: "1234567890".to_string(),
            subject: "spiffe://widya.example/ns/default/sa/default".to_string(),
            issuer: "spiffe://widya.example/ns/widya-system/sa/widya-ca".to_string(),
            not_before: chrono::Utc::now() - Duration::days(1),
            not_after: chrono::Utc::now() + Duration::days(365),
            san_dns: vec![],
            san_uri: vec!["spiffe://widya.example/ns/default/sa/default".to_string()],
            key_usage: vec!["digitalSignature".to_string()],
            extended_key_usage: vec!["serverAuth".to_string(), "clientAuth".to_string()],
        })
    }
    
    /// Verify certificate chain
    fn verify_certificate_chain(&self, cert: &Certificate) -> Result<()> {
        // In production, verify against CA
        // For now, assume valid
        Ok(())
    }
    
    /// Verify certificate expiration
    fn verify_certificate_expiration(&self, cert: &Certificate) -> Result<()> {
        let now = chrono::Utc::now();
        
        if now < cert.not_before {
            return Err(ServiceMeshError::SecurityError(
                "Certificate not yet valid".to_string()
            ));
        }
        
        if now > cert.not_after {
            return Err(ServiceMeshError::SecurityError(
                "Certificate has expired".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Validate service identity
    fn validate_identity(&self, cert: &Certificate) -> Result<ServiceIdentity> {
        // Extract SPIFFE ID from certificate SAN
        let spiiffe_id = cert.san_uri.first()
            .ok_or_else(|| ServiceMeshError::SecurityError("No SPIFFE ID in certificate".to_string()))?;
        
        let identity = ServiceIdentity::from_spiffe_id(spiiffe_id)?;
        
        // Store identity
        {
            let mut identities = self.identities.write().unwrap();
            identities.insert(spiiffe_id.clone(), identity.clone());
        }
        
        Ok(identity)
    }
    
    /// Cache certificate
    fn cache_certificate(&self, cert: &Certificate) -> Result<()> {
        let mut cache = self.cert_cache.write().unwrap();
        
        let entry = CacheEntry {
            certificate: cert.clone(),
            cached_at: Instant::now(),
            expires_at: cert.not_after - Duration::days(7), // Cache until 7 days before expiry
        };
        
        cache.insert(cert.serial_number.clone(), entry);
        
        Ok(())
    }
    
    /// Revoke certificate
    pub fn revoke_certificate(&self, serial_number: &str) -> Result<()> {
        let mut crl = self.crl.write().unwrap();
        crl.insert(serial_number.to_string());
        
        // Invalidate cache
        let mut cache = self.cert_cache.write().unwrap();
        cache.remove(serial_number);
        
        Ok(())
    }
    
    /// Get certificate from cache
    pub fn get_cached_certificate(&self, serial_number: &str) -> Option<Certificate> {
        let cache = self.cert_cache.read().unwrap();
        cache.get(serial_number).map(|e| e.certificate.clone())
    }
}

/// Service identity
#[derive(Debug, Clone)]
pub struct ServiceIdentity {
    /// SPIFFE ID
    pub spiffe_id: String,
    
    /// Trust domain
    pub trust_domain: String,
    
    /// Namespace
    pub namespace: String,
    
    /// Service account
    pub service_account: String,
    
    /// Service name
    pub service_name: String,
}

impl ServiceIdentity {
    /// Create from SPIFFE ID
    pub fn from_spiffe_id(spiffe_id: &str) -> Result<Self> {
        // Parse SPIFFE ID: spiffe://<trust-domain>/ns/<namespace>/sa/<service-account>
        // or spiffe://<trust-domain>/ns/<namespace>/svc/<service-name>
        
        let parts: Vec<&str> = spiffe_id.split('/').collect();
        
        if parts.len() < 6 {
            return Err(ServiceMeshError::SecurityError(
                format!("Invalid SPIFFE ID: {}", spiffe_id)
            ));
        }
        
        let trust_domain = parts[2].to_string();
        let namespace = parts[4].to_string();
        let service_name = if parts[5] == "sa" {
            parts[6].to_string()
        } else if parts[5] == "svc" {
            parts[6].to_string()
        } else {
            return Err(ServiceMeshError::SecurityError(
                format!("Invalid SPIFFE ID path: {}", spiffe_id)
            ));
        };
        
        let service_account = if parts[5] == "sa" {
            service_name.clone()
        } else {
            "default".to_string()
        };
        
        Ok(Self {
            spiffe_id: spiffe_id.to_string(),
            trust_domain,
            namespace,
            service_account,
            service_name,
        })
    }
    
    /// Check if identity matches pattern
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        // Simple pattern matching
        pattern == "*" || 
        pattern == self.spiffe_id ||
        pattern == self.service_name ||
        pattern == format!("{}/{}", self.namespace, self.service_name)
    }
}

/// Certificate information
#[derive(Debug, Clone)]
pub struct Certificate {
    pub der_data: Vec<u8>,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: chrono::DateTime<chrono::Utc>,
    pub not_after: chrono::DateTime<chrono::Utc>,
    pub san_dns: Vec<String>,
    pub san_uri: Vec<String>,
    pub key_usage: Vec<String>,
    pub extended_key_usage: Vec<String>,
}

/// Certificate cache entry
struct CacheEntry {
    certificate: Certificate,
    cached_at: Instant,
    expires_at: chrono::DateTime<chrono::Utc>,
}

/// Certificate manager for service mesh
pub struct CertificateManager {
    /// Root CA certificate
    root_ca: Option<Certificate>,
    
    /// Intermediate CA certificates
    intermediate_cas: Vec<Certificate>,
    
    /// Certificate rotation schedule
    rotation_schedule: HashMap<String, CertificateRotationSchedule>,
}

impl CertificateManager {
    /// Create new certificate manager
    pub fn new() -> Self {
        Self {
            root_ca: None,
            intermediate_cas: Vec::new(),
            rotation_schedule: HashMap::new(),
        }
    }
    
    /// Set root CA certificate
    pub fn set_root_ca(&mut self, cert: Certificate) {
        self.root_ca = Some(cert);
    }
    
    /// Add intermediate CA
    pub fn add_intermediate_ca(&mut self, cert: Certificate) {
        self.intermediate_cas.push(cert);
    }
    
    /// Generate service certificate
    pub fn generate_service_cert(&self, service_identity: &ServiceIdentity) -> Result<Certificate> {
        // In production, use CSR and sign with CA
        // For now, return mock certificate
        Ok(Certificate {
            der_data: vec![0u8; 1024], // Mock DER data
            serial_number: format!("{}-{}", service_identity.service_name, chrono::Utc::now().timestamp()),
            subject: service_identity.spiffe_id.clone(),
            issuer: "spiffe://widya.example/ns/widya-system/sa/widya-ca".to_string(),
            not_before: chrono::Utc::now(),
            not_after: chrono::Utc::now() + Duration::days(30),
            san_dns: vec![],
            san_uri: vec![service_identity.spiffe_id.clone()],
            key_usage: vec!["digitalSignature".to_string()],
            extended_key_usage: vec!["serverAuth".to_string(), "clientAuth".to_string()],
        })
    }
}

/// Certificate rotation schedule
struct CertificateRotationSchedule {
    cert_id: String,
    last_rotation: chrono::DateTime<chrono::Utc>,
    next_rotation: chrono::DateTime<chrono::Utc>,
    rotation_interval_days: u32,
}

/// mTLS authentication result
#[derive(Debug, Clone)]
pub struct MtlsAuthResult {
    /// Whether authentication succeeded
    pub authenticated: bool,
    
    /// Service identity
    pub identity: Option<ServiceIdentity>,
    
    /// Certificate chain
    pub cert_chain: Vec<Certificate>,
    
    /// Error message (if authentication failed)
    pub error: Option<String>,
}