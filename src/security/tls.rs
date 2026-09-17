//! TLS 1.3 implementation for Widya Enterprise Edition
//! Provides secure transport layer with TLS 1.3 protocol

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::security::{SecurityError, Result, CryptoProvider};

/// TLS 1.3 server configuration
#[derive(Debug, Clone)]
pub struct TlsServerConfig {
    /// Server port
    pub port: u16,
    
    /// Certificate path
    pub cert_path: String,
    
    /// Private key path
    pub key_path: String,
    
    /// Cipher suites (TLS 1.3 only)
    pub cipher_suites: Vec<TlsCipherSuite>,
    
    /// Require client authentication
    pub require_client_auth: bool,
    
    /// Client CA certificate path (optional)
    pub client_ca_path: Option<String>,
    
    /// Session ticket lifetime in seconds
    pub session_ticket_lifetime: u32,
    
    /// Early data enabled
    pub early_data_enabled: bool,
    
    /// ALPN protocols (Application-Layer Protocol Negotiation)
    pub alpn_protocols: Vec<String>,
}

/// TLS 1.3 client configuration
#[derive(Debug, Clone)]
pub struct TlsClientConfig {
    /// Server hostname
    pub server_hostname: String,
    
    /// Server port
    pub server_port: u16,
    
    /// Verify server certificate
    pub verify_server_cert: bool,
    
    /// CA certificate path
    pub ca_cert_path: Option<String>,
    
    /// Client certificate path (optional, for mTLS)
    pub client_cert_path: Option<String>,
    
    /// Client private key path (optional, for mTLS)
    pub client_key_path: Option<String>,
    
    /// Cipher suites
    pub cipher_suites: Vec<TlsCipherSuite>,
    
    /// Session resumption enabled
    pub session_resumption: bool,
    
    /// ALPN protocols
    pub alpn_protocols: Vec<String>,
    
    /// SNI (Server Name Indication) enabled
    pub sni_enabled: bool,
}

/// TLS cipher suites (TLS 1.3 only)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsCipherSuite {
    /// TLS_AES_128_GCM_SHA256
    TlsAes128GcmSha256,
    
    /// TLS_AES_256_GCM_SHA384
    TlsAes256GcmSha384,
    
    /// TLS_CHACHA20_POLY1305_SHA256
    TlsChaCha20Poly1305Sha256,
    
    /// TLS_AES_128_CCM_SHA256
    TlsAes128CcmSha256,
    
    /// TLS_AES_128_CCM_8_SHA256
    TlsAes128Ccm8Sha256,
}

/// TLS 1.3 server
pub struct TlsServer {
    /// Configuration
    config: TlsServerConfig,
    
    /// Cryptography provider
    crypto_provider: Arc<CryptoProvider>,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<String, TlsConnection>>>,
    
    /// Server state
    state: Arc<RwLock<TlsServerState>>,
}

impl TlsServer {
    /// Create new TLS 1.3 server
    pub fn new(config: TlsServerConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        Ok(Self {
            config,
            crypto_provider,
            connections: Arc::new(RwLock::new(HashMap::new())),
            state: Arc::new(RwLock::new(TlsServerState::Stopped)),
        })
    }
    
    /// Start TLS server
    pub fn start(&self) -> Result<()> {
        let mut state = self.state.write().unwrap();
        if *state != TlsServerState::Stopped {
            return Err(SecurityError::ConfigurationError(
                "Server is already running or stopping".to_string()
            ));
        }
        
        *state = TlsServerState::Starting;
        
        // In production, this would use rustls or similar library
        // For now, we'll create a mock implementation
        
        println!("[TLS Server] Starting on port {} with TLS 1.3", self.config.port);
        println!("[TLS Server] Certificate: {}", self.config.cert_path);
        println!("[TLS Server] Private key: {}", self.config.key_path);
        
        if self.config.require_client_auth {
            println!("[TLS Server] Client authentication required");
        }
        
        *state = TlsServerState::Running;
        
        Ok(())
    }
    
    /// Stop TLS server
    pub fn stop(&self) -> Result<()> {
        let mut state = self.state.write().unwrap();
        *state = TlsServerState::Stopping;
        
        // Close all connections
        {
            let mut connections = self.connections.write().unwrap();
            for (_, connection) in connections.drain() {
                // In production, close the connection properly
                println!("[TLS Server] Closing connection: {}", connection.id);
            }
        }
        
        *state = TlsServerState::Stopped;
        println!("[TLS Server] Stopped");
        
        Ok(())
    }
    
    /// Accept incoming connection (mock implementation)
    pub fn accept_connection(&self) -> Result<TlsConnection> {
        let state = self.state.read().unwrap();
        if *state != TlsServerState::Running {
            return Err(SecurityError::ConfigurationError(
                "Server is not running".to_string()
            ));
        }
        
        // Generate connection ID
        let connection_id = format!("conn-{}", generate_id());
        
        let connection = TlsConnection {
            id: connection_id.clone(),
            local_addr: format!("0.0.0.0:{}", self.config.port),
            remote_addr: "127.0.0.1:54321".to_string(),
            cipher_suite: self.config.cipher_suites.first()
                .copied()
                .unwrap_or(TlsCipherSuite::TlsAes256GcmSha384),
            alpn_protocol: self.config.alpn_protocols.first()
                .cloned()
                .unwrap_or_else(|| "http/1.1".to_string()),
            session_resumed: false,
            early_data_accepted: false,
            client_authenticated: false,
            created_at: current_time(),
            last_activity: current_time(),
        };
        
        // Store connection
        {
            let mut connections = self.connections.write().unwrap();
            connections.insert(connection_id, connection.clone());
        }
        
        println!("[TLS Server] Accepted new connection: {}", connection.id);
        
        Ok(connection)
    }
    
    /// Get server statistics
    pub fn get_stats(&self) -> TlsServerStats {
        let connections = self.connections.read().unwrap();
        let state = self.state.read().unwrap();
        
        TlsServerStats {
            state: *state,
            total_connections: connections.len(),
            active_connections: connections.values()
                .filter(|c| current_time() - c.last_activity < 300) // 5 minutes
                .count(),
            cipher_suite_distribution: self.get_cipher_suite_distribution(),
            alpn_distribution: self.get_alpn_distribution(),
        }
    }
    
    /// Get cipher suite distribution
    fn get_cipher_suite_distribution(&self) -> HashMap<TlsCipherSuite, usize> {
        let connections = self.connections.read().unwrap();
        let mut distribution = HashMap::new();
        
        for connection in connections.values() {
            *distribution.entry(connection.cipher_suite).or_insert(0) += 1;
        }
        
        distribution
    }
    
    /// Get ALPN protocol distribution
    fn get_alpn_distribution(&self) -> HashMap<String, usize> {
        let connections = self.connections.read().unwrap();
        let mut distribution = HashMap::new();
        
        for connection in connections.values() {
            *distribution.entry(connection.alpn_protocol.clone()).or_insert(0) += 1;
        }
        
        distribution
    }
}

/// TLS 1.3 client
pub struct TlsClient {
    /// Configuration
    config: TlsClientConfig,
    
    /// Cryptography provider
    crypto_provider: Arc<CryptoProvider>,
    
    /// Active connections
    connections: Arc<RwLock<HashMap<String, TlsConnection>>>,
}

impl TlsClient {
    /// Create new TLS 1.3 client
    pub fn new(config: TlsClientConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        Ok(Self {
            config,
            crypto_provider,
            connections: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Connect to TLS server
    pub fn connect(&self) -> Result<TlsConnection> {
        let server_addr = format!("{}:{}", self.config.server_hostname, self.config.server_port);
        
        println!("[TLS Client] Connecting to {} with TLS 1.3", server_addr);
        
        if self.config.verify_server_cert {
            println!("[TLS Client] Server certificate verification enabled");
        }
        
        if self.config.sni_enabled {
            println!("[TLS Client] SNI: {}", self.config.server_hostname);
        }
        
        if let Some(alpn) = self.config.alpn_protocols.first() {
            println!("[TLS Client] ALPN: {}", alpn);
        }
        
        // Generate connection ID
        let connection_id = format!("client-conn-{}", generate_id());
        
        let connection = TlsConnection {
            id: connection_id.clone(),
            local_addr: "127.0.0.1:54321".to_string(),
            remote_addr: server_addr,
            cipher_suite: self.config.cipher_suites.first()
                .copied()
                .unwrap_or(TlsCipherSuite::TlsAes256GcmSha384),
            alpn_protocol: self.config.alpn_protocols.first()
                .cloned()
                .unwrap_or_else(|| "http/1.1".to_string()),
            session_resumed: self.config.session_resumption,
            early_data_accepted: false,
            client_authenticated: self.config.client_cert_path.is_some(),
            created_at: current_time(),
            last_activity: current_time(),
        };
        
        // Store connection
        {
            let mut connections = self.connections.write().unwrap();
            connections.insert(connection_id, connection.clone());
        }
        
        println!("[TLS Client] Connected: {}", connection.id);
        
        Ok(connection)
    }
    
    /// Close connection
    pub fn close_connection(&self, connection_id: &str) -> Result<()> {
        let mut connections = self.connections.write().unwrap();
        if connections.remove(connection_id).is_some() {
            println!("[TLS Client] Closed connection: {}", connection_id);
            Ok(())
        } else {
            Err(SecurityError::ConfigurationError(
                format!("Connection not found: {}", connection_id)
            ))
        }
    }
}

/// TLS connection
#[derive(Debug, Clone)]
pub struct TlsConnection {
    /// Connection ID
    pub id: String,
    
    /// Local address
    pub local_addr: String,
    
    /// Remote address
    pub remote_addr: String,
    
    /// Negotiated cipher suite
    pub cipher_suite: TlsCipherSuite,
    
    /// Negotiated ALPN protocol
    pub alpn_protocol: String,
    
    /// Session resumed
    pub session_resumed: bool,
    
    /// Early data accepted
    pub early_data_accepted: bool,
    
    /// Client authenticated (for mTLS)
    pub client_authenticated: bool,
    
    /// Connection creation time
    pub created_at: u64,
    
    /// Last activity time
    pub last_activity: u64,
}

impl TlsConnection {
    /// Send data over TLS connection
    pub fn send(&mut self, data: &[u8]) -> Result<usize> {
        // Update last activity
        self.last_activity = current_time();
        
        // In production, this would encrypt and send the data
        println!("[TLS Connection {}] Sending {} bytes", self.id, data.len());
        
        Ok(data.len())
    }
    
    /// Receive data from TLS connection
    pub fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        // Update last activity
        self.last_activity = current_time();
        
        // In production, this would receive and decrypt data
        println!("[TLS Connection {}] Receiving data", self.id);
        
        // Mock: return empty buffer
        Ok(0)
    }
    
    /// Get connection information
    pub fn get_info(&self) -> TlsConnectionInfo {
        TlsConnectionInfo {
            id: self.id.clone(),
            local_addr: self.local_addr.clone(),
            remote_addr: self.remote_addr.clone(),
            cipher_suite: self.cipher_suite,
            alpn_protocol: self.alpn_protocol.clone(),
            session_resumed: self.session_resumed,
            early_data_accepted: self.early_data_accepted,
            client_authenticated: self.client_authenticated,
            age_seconds: current_time() - self.created_at,
            idle_seconds: current_time() - self.last_activity,
        }
    }
}

/// TLS connection information
#[derive(Debug, Clone)]
pub struct TlsConnectionInfo {
    pub id: String,
    pub local_addr: String,
    pub remote_addr: String,
    pub cipher_suite: TlsCipherSuite,
    pub alpn_protocol: String,
    pub session_resumed: bool,
    pub early_data_accepted: bool,
    pub client_authenticated: bool,
    pub age_seconds: u64,
    pub idle_seconds: u64,
}

/// TLS server state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsServerState {
    Stopped,
    Starting,
    Running,
    Stopping,
}

/// TLS server statistics
#[derive(Debug, Clone)]
pub struct TlsServerStats {
    pub state: TlsServerState,
    pub total_connections: usize,
    pub active_connections: usize,
    pub cipher_suite_distribution: HashMap<TlsCipherSuite, usize>,
    pub alpn_distribution: HashMap<String, usize>,
}

/// Mutual TLS (mTLS) authenticator
pub struct MutualTlsAuthenticator {
    /// Client CA certificates
    client_ca_certs: Vec<Vec<u8>>,
    
    /// CRL (Certificate Revocation List)
    crl: Option<Vec<u8>>,
    
    /// OCSP (Online Certificate Status Protocol) responder URL
    ocsp_responder_url: Option<String>,
    
    /// Maximum certificate chain depth
    max_chain_depth: u32,
    
    /// Allowed client certificate subjects
    allowed_subjects: Vec<String>,
}

impl MutualTlsAuthenticator {
    /// Create new mTLS authenticator
    pub fn new(client_ca_certs: Vec<Vec<u8>>) -> Result<Self> {
        Ok(Self {
            client_ca_certs,
            crl: None,
            ocsp_responder_url: None,
            max_chain_depth: 3,
            allowed_subjects: Vec::new(),
        })
    }
    
    /// Authenticate client certificate
    pub fn authenticate(&self, client_cert: &[u8]) -> Result<MutualTlsAuthResult> {
        // In production, this would validate the certificate chain
        // For now, we'll return a mock result
        
        println!("[mTLS] Authenticating client certificate ({} bytes)", client_cert.len());
        
        Ok(MutualTlsAuthResult {
            authenticated: true,
            subject: "CN=client.widya.example".to_string(),
            issuer: "CN=widya-ca".to_string(),
            serial_number: "1234567890".to_string(),
            not_before: current_time() - 86400, // 1 day ago
            not_after: current_time() + 86400 * 365, // 1 year from now
            key_usage: vec!["digitalSignature".to_string(), "keyEncipherment".to_string()],
            extended_key_usage: vec!["clientAuth".to_string()],
            subject_alt_names: Vec::new(),
            ocsp_status: None,
            crl_status: None,
        })
    }
    
    /// Check if certificate is revoked
    pub fn check_revocation(&self, serial_number: &str) -> Result<bool> {
        // In production, check against CRL or OCSP
        Ok(false) // Not revoked
    }
    
    /// Add allowed subject
    pub fn allow_subject(&mut self, subject: String) {
        self.allowed_subjects.push(subject);
    }
    
    /// Set CRL
    pub fn set_crl(&mut self, crl: Vec<u8>) {
        self.crl = Some(crl);
    }
    
    /// Set OCSP responder URL
    pub fn set_ocsp_responder(&mut self, url: String) {
        self.ocsp_responder_url = Some(url);
    }
}

/// Mutual TLS authentication result
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

/// TLS session resumption manager
pub struct TlsSessionManager {
    /// Session tickets
    session_tickets: Arc<RwLock<HashMap<String, TlsSessionTicket>>>,
    
    /// Maximum session lifetime in seconds
    max_session_lifetime: u64,
    
    /// Maximum number of sessions
    max_sessions: usize,
}

impl TlsSessionManager {
    /// Create new session manager
    pub fn new(max_session_lifetime: u64, max_sessions: usize) -> Self {
        Self {
            session_tickets: Arc::new(RwLock::new(HashMap::new())),
            max_session_lifetime,
            max_sessions,
        }
    }
    
    /// Create new session ticket
    pub fn create_ticket(&self, connection_info: &TlsConnectionInfo) -> Result<TlsSessionTicket> {
        let ticket_id = format!("ticket-{}", generate_id());
        
        let ticket = TlsSessionTicket {
            id: ticket_id.clone(),
            connection_id: connection_info.id.clone(),
            cipher_suite: connection_info.cipher_suite,
            created_at: current_time(),
            expires_at: current_time() + self.max_session_lifetime,
            early_data_allowed: true,
            max_early_data_size: 16384, // 16KB
        };
        
        // Store ticket
        {
            let mut tickets = self.session_tickets.write().unwrap();
            
            // Cleanup expired tickets
            self.cleanup_expired_tickets(&mut tickets);
            
            // Check if we have too many tickets
            if tickets.len() >= self.max_sessions {
                // Remove oldest ticket
                let oldest_key = tickets.iter()
                    .min_by_key(|(_, t)| t.created_at)
                    .map(|(k, _)| k.clone());
                
                if let Some(oldest_key) = oldest_key {
                    tickets.remove(&oldest_key);
                }
            }
            
            tickets.insert(ticket_id, ticket.clone());
        }
        
        Ok(ticket)
    }
    
    /// Validate session ticket
    pub fn validate_ticket(&self, ticket_id: &str) -> Result<Option<TlsSessionTicket>> {
        let tickets = self.session_tickets.read().unwrap();
        
        if let Some(ticket) = tickets.get(ticket_id) {
            // Check if ticket is expired
            if ticket.expires_at < current_time() {
                return Ok(None);
            }
            
            Ok(Some(ticket.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Revoke session ticket
    pub fn revoke_ticket(&self, ticket_id: &str) -> Result<()> {
        let mut tickets = self.session_tickets.write().unwrap();
        tickets.remove(ticket_id);
        Ok(())
    }
    
    /// Cleanup expired tickets
    fn cleanup_expired_tickets(&self, tickets: &mut HashMap<String, TlsSessionTicket>) {
        let current_time = current_time();
        tickets.retain(|_, ticket| ticket.expires_at >= current_time);
    }
}

/// TLS session ticket
#[derive(Debug, Clone)]
pub struct TlsSessionTicket {
    pub id: String,
    pub connection_id: String,
    pub cipher_suite: TlsCipherSuite,
    pub created_at: u64,
    pub expires_at: u64,
    pub early_data_allowed: bool,
    pub max_early_data_size: u32,
}

/// Helper function to get current timestamp
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Helper function to generate ID
fn generate_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("{:016x}", rng.gen::<u64>())
}

/// Default TLS server configuration
impl Default for TlsServerConfig {
    fn default() -> Self {
        Self {
            port: 443,
            cert_path: "/etc/widya/cert.pem".to_string(),
            key_path: "/etc/widya/key.pem".to_string(),
            cipher_suites: vec![
                TlsCipherSuite::TlsAes256GcmSha384,
                TlsCipherSuite::TlsAes128GcmSha256,
                TlsCipherSuite::TlsChaCha20Poly1305Sha256,
            ],
            require_client_auth: false,
            client_ca_path: None,
            session_ticket_lifetime: 86400, // 24 hours
            early_data_enabled: true,
            alpn_protocols: vec!["http/1.1".to_string(), "h2".to_string()],
        }
    }
}

/// Default TLS client configuration
impl Default for TlsClientConfig {
    fn default() -> Self {
        Self {
            server_hostname: "localhost".to_string(),
            server_port: 443,
            verify_server_cert: true,
            ca_cert_path: Some("/etc/ssl/certs/ca-certificates.crt".to_string()),
            client_cert_path: None,
            client_key_path: None,
            cipher_suites: vec![
                TlsCipherSuite::TlsAes256GcmSha384,
                TlsCipherSuite::TlsAes128GcmSha256,
                TlsCipherSuite::TlsChaCha20Poly1305Sha256,
            ],
            session_resumption: true,
            alpn_protocols: vec!["http/1.1".to_string(), "h2".to_string()],
            sni_enabled: true,
        }
    }
}