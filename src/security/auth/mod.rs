//! Authentication & Authorization module for Widya Enterprise Edition
//! Provides JWT, OAuth 2.0, OpenID Connect, and RBAC functionality

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::security::{SecurityError, Result, AuthConfig, UserPrincipal, AuthorizationContext, CryptoProvider};

mod rbac;
pub use rbac::{RbacEngine, DefaultRbacConfig, Authorization};

/// JWT (JSON Web Token) validator
pub struct JwtValidator {
    /// Configuration
    config: AuthConfig,
    
    /// Cryptography provider
    crypto_provider: Arc<CryptoProvider>,
    
    /// Token blacklist (for revocation)
    token_blacklist: Arc<RwLock<HashSet<String>>>,
    
    /// Key store for different issuers
    issuer_keys: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl JwtValidator {
    /// Create new JWT validator
    pub fn new(config: &AuthConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            crypto_provider,
            token_blacklist: Arc::new(RwLock::new(HashSet::new())),
            issuer_keys: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Validate JWT token
    pub fn validate_token(&self, token: &str) -> Result<JwtClaims> {
        // Parse token parts
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(SecurityError::TokenError(
                "Invalid JWT format: expected 3 parts".to_string()
            ));
        }
        
        let header_b64 = parts[0];
        let payload_b64 = parts[1];
        let signature_b64 = parts[2];
        
        // Decode header
        let header_json = base64_decode_url(header_b64)?;
        let header: JwtHeader = serde_json::from_slice(&header_json)
            .map_err(|e| SecurityError::TokenError(format!("Invalid JWT header: {}", e)))?;
        
        // Check algorithm
        if header.alg != "HS256" && header.alg != "HS384" && header.alg != "HS512" {
            return Err(SecurityError::TokenError(
                format!("Unsupported JWT algorithm: {}", header.alg)
            ));
        }
        
        // Verify signature
        self.verify_signature(header_b64, payload_b64, signature_b64, &header.alg)?;
        
        // Decode payload
        let payload_json = base64_decode_url(payload_b64)?;
        let claims: JwtClaims = serde_json::from_slice(&payload_json)
            .map_err(|e| SecurityError::TokenError(format!("Invalid JWT claims: {}", e)))?;
        
        // Check if token is blacklisted
        {
            let blacklist = self.token_blacklist.read().unwrap();
            if blacklist.contains(&claims.jti.clone().unwrap_or_default()) {
                return Err(SecurityError::TokenError("Token has been revoked".to_string()));
            }
        }
        
        // Validate claims
        self.validate_claims(&claims)?;
        
        Ok(claims)
    }
    
    /// Verify JWT signature
    fn verify_signature(&self, header_b64: &str, payload_b64: &str, signature_b64: &str, algorithm: &str) -> Result<()> {
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let signature = base64_decode_url(signature_b64)?;
        
        // Get key based on algorithm
        let key = match algorithm {
            "HS256" | "HS384" | "HS512" => self.config.jwt_secret.as_bytes(),
            _ => return Err(SecurityError::TokenError(
                format!("Unsupported algorithm for verification: {}", algorithm)
            )),
        };
        
        // Hash algorithm to use
        let hash_algorithm = match algorithm {
            "HS256" => crate::security::HashAlgorithm::Sha256,
            "HS384" => crate::security::HashAlgorithm::Sha384,
            "HS512" => crate::security::HashAlgorithm::Sha512,
            _ => return Err(SecurityError::TokenError(
                format!("Unsupported algorithm: {}", algorithm)
            )),
        };
        
        // Compute HMAC
        let computed_hmac = self.crypto_provider.hmac(signing_input.as_bytes(), key, hash_algorithm)?;
        
        // Compare signatures (constant-time comparison)
        if !constant_time_compare(&computed_hmac, &signature) {
            return Err(SecurityError::TokenError("Invalid JWT signature".to_string()));
        }
        
        Ok(())
    }
    
    /// Validate JWT claims
    fn validate_claims(&self, claims: &JwtClaims) -> Result<()> {
        let current_time = current_time();
        
        // Check expiration
        if let Some(exp) = claims.exp {
            if exp < current_time {
                return Err(SecurityError::TokenError("Token has expired".to_string()));
            }
        }
        
        // Check not before
        if let Some(nbf) = claims.nbf {
            if nbf > current_time {
                return Err(SecurityError::TokenError("Token not yet valid".to_string()));
            }
        }
        
        // Check issuer
        if let Some(iss) = &claims.iss {
            if iss != &self.config.jwt_issuer {
                return Err(SecurityError::TokenError(
                    format!("Invalid issuer: expected {}, got {}", self.config.jwt_issuer, iss)
                ));
            }
        }
        
        // Check audience
        if let Some(aud) = &claims.aud {
            if aud != &self.config.jwt_audience {
                return Err(SecurityError::TokenError(
                    format!("Invalid audience: expected {}, got {}", self.config.jwt_audience, aud)
                ));
            }
        }
        
        Ok(())
    }
    
    /// Generate JWT token
    pub fn generate_token(&self, claims: JwtClaims) -> Result<String> {
        // Set default values if not provided
        let mut claims = claims;
        
        if claims.iss.is_none() {
            claims.iss = Some(self.config.jwt_issuer.clone());
        }
        
        if claims.aud.is_none() {
            claims.aud = Some(self.config.jwt_audience.clone());
        }
        
        if claims.iat.is_none() {
            claims.iat = Some(current_time());
        }
        
        if claims.exp.is_none() {
            claims.exp = Some(current_time() + self.config.jwt_expiration_secs);
        }
        
        if claims.jti.is_none() {
            claims.jti = Some(generate_jti());
        }
        
        // Create header
        let header = JwtHeader {
            typ: "JWT".to_string(),
            alg: "HS256".to_string(), // Using HS256 by default
            kid: None,
        };
        
        // Encode header and payload
        let header_json = serde_json::to_vec(&header)
            .map_err(|e| SecurityError::TokenError(format!("Failed to serialize header: {}", e)))?;
        let header_b64 = base64_encode_url(&header_json);
        
        let payload_json = serde_json::to_vec(&claims)
            .map_err(|e| SecurityError::TokenError(format!("Failed to serialize claims: {}", e)))?;
        let payload_b64 = base64_encode_url(&payload_json);
        
        // Create signature
        let signing_input = format!("{}.{}", header_b64, payload_b64);
        let signature = self.crypto_provider.hmac(
            signing_input.as_bytes(),
            self.config.jwt_secret.as_bytes(),
            crate::security::HashAlgorithm::Sha256,
        )?;
        let signature_b64 = base64_encode_url(&signature);
        
        // Create full token
        let token = format!("{}.{}.{}", header_b64, payload_b64, signature_b64);
        
        Ok(token)
    }
    
    /// Revoke token (add to blacklist)
    pub fn revoke_token(&self, jti: &str) -> Result<()> {
        let mut blacklist = self.token_blacklist.write().unwrap();
        blacklist.insert(jti.to_string());
        Ok(())
    }
    
    /// Clear expired tokens from blacklist
    pub fn cleanup_blacklist(&self) -> Result<usize> {
        // In production implementation, would remove expired tokens
        // based on their expiration time
        Ok(0)
    }
}

/// JWT header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtHeader {
    /// Token type (always "JWT")
    #[serde(rename = "typ")]
    pub typ: String,
    
    /// Signing algorithm
    #[serde(rename = "alg")]
    pub alg: String,
    
    /// Key ID (optional)
    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

/// JWT claims (RFC 7519)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Issuer
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub iss: Option<String>,
    
    /// Subject (user ID)
    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub sub: Option<String>,
    
    /// Audience
    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    
    /// Expiration time
    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    
    /// Not before
    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub nbf: Option<u64>,
    
    /// Issued at
    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub iat: Option<u64>,
    
    /// JWT ID
    #[serde(rename = "jti", skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    
    /// Custom claims
    #[serde(flatten)]
    pub custom: HashMap<String, serde_json::Value>,
}

impl JwtClaims {
    /// Create new JWT claims
    pub fn new(subject: String) -> Self {
        Self {
            iss: None,
            sub: Some(subject),
            aud: None,
            exp: None,
            nbf: None,
            iat: None,
            jti: None,
            custom: HashMap::new(),
        }
    }
    
    /// Add custom claim
    pub fn with_claim(mut self, key: &str, value: serde_json::Value) -> Self {
        self.custom.insert(key.to_string(), value);
        self
    }
    
    /// Add role claim
    pub fn with_role(mut self, role: &str) -> Self {
        self.custom.insert("role".to_string(), serde_json::Value::String(role.to_string()));
        self
    }
    
    /// Add roles claim (array of roles)
    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        let roles_json: Vec<serde_json::Value> = roles.into_iter()
            .map(|r| serde_json::Value::String(r))
            .collect();
        self.custom.insert("roles".to_string(), serde_json::Value::Array(roles_json));
        self
    }
    
    /// Add permissions claim
    pub fn with_permissions(mut self, permissions: Vec<String>) -> Self {
        let perms_json: Vec<serde_json::Value> = permissions.into_iter()
            .map(|p| serde_json::Value::String(p))
            .collect();
        self.custom.insert("permissions".to_string(), serde_json::Value::Array(perms_json));
        self
    }
    
    /// Add tenant ID claim
    pub fn with_tenant_id(mut self, tenant_id: uuid::Uuid) -> Self {
        self.custom.insert("tenant_id".to_string(), serde_json::Value::String(tenant_id.to_string()));
        self
    }
}

/// Authenticator for multiple authentication methods
pub struct Authenticator {
    /// JWT validator
    jwt_validator: JwtValidator,
    
    /// OAuth 2.0 client
    oauth_client: Option<OAuthClient>,
    
    /// User store (in production, would connect to database)
    user_store: Arc<RwLock<HashMap<String, UserPrincipal>>>,
    
    /// Session store
    session_store: Arc<RwLock<HashMap<String, UserSession>>>,
}

impl Authenticator {
    /// Create new authenticator
    pub fn new(config: &AuthConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        let jwt_validator = JwtValidator::new(config, crypto_provider.clone())?;
        
        let oauth_client = if config.oauth_enabled {
            Some(OAuthClient::new(config, crypto_provider)?)
        } else {
            None
        };
        
        Ok(Self {
            jwt_validator,
            oauth_client,
            user_store: Arc::new(RwLock::new(HashMap::new())),
            session_store: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Authenticate with JWT token
    pub fn authenticate_jwt(&self, token: &str) -> Result<UserPrincipal> {
        let claims = self.jwt_validator.validate_token(token)?;
        
        // Extract user information from claims
        let user_id = claims.sub.ok_or_else(|| 
            SecurityError::AuthError("JWT missing subject claim".to_string())
        )?;
        
        // Get or create user principal
        let principal = {
            let store = self.user_store.read().unwrap();
            store.get(&user_id).cloned()
        };
        
        let principal = principal.unwrap_or_else(|| {
            // Create new principal from claims
            let mut principal = UserPrincipal {
                user_id: user_id.clone(),
                username: user_id.clone(),
                email: None,
                tenant_id: None,
                roles: Vec::new(),
                permissions: Vec::new(),
                auth_method: crate::security::AuthMethod::Jwt,
                auth_time: chrono::Utc::now(),
                session_id: claims.jti.clone(),
            };
            
            // Extract roles from claims
            if let Some(roles_value) = claims.custom.get("roles") {
                if let Some(roles_array) = roles_value.as_array() {
                    principal.roles = roles_array.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                }
            }
            
            // Extract permissions from claims
            if let Some(perms_value) = claims.custom.get("permissions") {
                if let Some(perms_array) = perms_value.as_array() {
                    principal.permissions = perms_array.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect();
                }
            }
            
            // Extract tenant ID from claims
            if let Some(tenant_value) = claims.custom.get("tenant_id") {
                if let Some(tenant_str) = tenant_value.as_str() {
                    if let Ok(tenant_id) = uuid::Uuid::parse_str(tenant_str) {
                        principal.tenant_id = Some(tenant_id);
                    }
                }
            }
            
            // Store principal
            {
                let mut store = self.user_store.write().unwrap();
                store.insert(user_id, principal.clone());
            }
            
            principal
        });
        
        // Create session
        if let Some(session_id) = &principal.session_id {
            let session = UserSession {
                id: session_id.clone(),
                user_id: principal.user_id.clone(),
                created_at: current_time(),
                expires_at: claims.exp.unwrap_or(current_time() + 3600),
                ip_address: None,
                user_agent: None,
                active: true,
            };
            
            let mut sessions = self.session_store.write().unwrap();
            sessions.insert(session_id.clone(), session);
        }
        
        Ok(principal)
    }
    
    /// Authenticate with username/password
    pub fn authenticate_password(&self, username: &str, password: &str) -> Result<UserPrincipal> {
        // In production, this would verify against a database
        // For now, we'll create a mock authentication
        
        let password_hash = self.jwt_validator.crypto_provider.hash_password(password)?;
        println!("[Auth] Password hash for {}: {}", username, password_hash);
        
        // Create user principal
        let principal = UserPrincipal {
            user_id: username.to_string(),
            username: username.to_string(),
            email: Some(format!("{}@example.com", username)),
            tenant_id: Some(uuid::Uuid::new_v4()),
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string(), "write".to_string()],
            auth_method: crate::security::AuthMethod::Basic,
            auth_time: chrono::Utc::now(),
            session_id: Some(generate_jti()),
        };
        
        // Store principal
        {
            let mut store = self.user_store.write().unwrap();
            store.insert(username.to_string(), principal.clone());
        }
        
        Ok(principal)
    }
    
    /// Logout user (invalidate session)
    pub fn logout(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.session_store.write().unwrap();
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.active = false;
            
            // Also blacklist JWT if it exists
            if let Some(jti) = session_id.strip_prefix("jwt-") {
                self.jwt_validator.revoke_token(jti)?;
            }
            
            Ok(())
        } else {
            Err(SecurityError::AuthError("Session not found".to_string()))
        }
    }
    
    /// Get active sessions for user
    pub fn get_user_sessions(&self, user_id: &str) -> Vec<UserSession> {
        let sessions = self.session_store.read().unwrap();
        sessions.values()
            .filter(|s| s.user_id == user_id && s.active && s.expires_at > current_time())
            .cloned()
            .collect()
    }
    
    /// Cleanup expired sessions
    pub fn cleanup_sessions(&self) -> Result<usize> {
        let current_time = current_time();
        let mut sessions = self.session_store.write().unwrap();
        
        let initial_count = sessions.len();
        sessions.retain(|_, session| session.expires_at > current_time && session.active);
        
        Ok(initial_count - sessions.len())
    }
}

/// User session
#[derive(Debug, Clone)]
pub struct UserSession {
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub active: bool,
}

/// OAuth 2.0 client
pub struct OAuthClient {
    /// Configuration
    config: AuthConfig,
    
    /// Cryptography provider
    crypto_provider: Arc<CryptoProvider>,
    
    /// Token cache
    token_cache: Arc<RwLock<HashMap<String, OAuthToken>>>,
}

impl OAuthClient {
    /// Create new OAuth 2.0 client
    pub fn new(config: &AuthConfig, crypto_provider: Arc<CryptoProvider>) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            crypto_provider,
            token_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    /// Get authorization URL
    pub fn get_authorization_url(&self, state: &str, redirect_uri: &str, scope: &str) -> Result<String> {
        let auth_url = self.config.oauth_auth_url.as_ref()
            .ok_or_else(|| SecurityError::ConfigurationError("OAuth auth URL not configured".to_string()))?;
        
        let client_id = self.config.oauth_client_id.as_ref()
            .ok_or_else(|| SecurityError::ConfigurationError("OAuth client ID not configured".to_string()))?;
        
        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            auth_url, client_id, redirect_uri, scope, state
        );
        
        Ok(url)
    }
    
    /// Exchange authorization code for access token
    pub fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<OAuthToken> {
        // In production, this would make HTTP request to token endpoint
        // For now, we'll return a mock token
        
        println!("[OAuth] Exchanging code for token: {}", code);
        
        let token = OAuthToken {
            access_token: generate_jti(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some(generate_jti()),
            scope: Some("read write".to_string()),
            id_token: None,
        };
        
        // Cache token
        {
            let mut cache = self.token_cache.write().unwrap();
            cache.insert(token.access_token.clone(), token.clone());
        }
        
        Ok(token)
    }
    
    /// Refresh access token
    pub fn refresh_token(&self, refresh_token: &str) -> Result<OAuthToken> {
        println!("[OAuth] Refreshing token: {}", refresh_token);
        
        let token = OAuthToken {
            access_token: generate_jti(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: Some(generate_jti()), // New refresh token
            scope: Some("read write".to_string()),
            id_token: None,
        };
        
        // Update cache
        {
            let mut cache = self.token_cache.write().unwrap();
            cache.insert(token.access_token.clone(), token.clone());
        }
        
        Ok(token)
    }
    
    /// Validate access token
    pub fn validate_token(&self, access_token: &str) -> Result<OAuthToken> {
        let cache = self.token_cache.read().unwrap();
        
        if let Some(token) = cache.get(access_token) {
            // Check if token is expired
            // In production, would check with introspection endpoint
            Ok(token.clone())
        } else {
            Err(SecurityError::TokenError("Invalid or expired OAuth token".to_string()))
        }
    }
    
    /// Get user info from OAuth provider
    pub fn get_user_info(&self, access_token: &str) -> Result<UserPrincipal> {
        // In production, this would call userinfo endpoint
        // For now, return mock user
        
        let token = self.validate_token(access_token)?;
        
        let principal = UserPrincipal {
            user_id: "oauth-user-123".to_string(),
            username: "oauth_user".to_string(),
            email: Some("user@example.com".to_string()),
            tenant_id: None,
            roles: vec!["user".to_string()],
            permissions: vec!["read".to_string()],
            auth_method: crate::security::AuthMethod::OAuth2,
            auth_time: chrono::Utc::now(),
            session_id: Some(token.access_token),
        };
        
        Ok(principal)
    }
}

/// OAuth 2.0 token
#[derive(Debug, Clone)]
pub struct OAuthToken {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

// Helper functions

/// Base64 URL decode
fn base64_decode_url(input: &str) -> Result<Vec<u8>> {
    use base64::Engine;
    base64::prelude::BASE64_URL_SAFE_NO_PAD.decode(input)
        .map_err(|e| SecurityError::TokenError(format!("Base64 decode error: {}", e)))
}

/// Base64 URL encode
fn base64_encode_url(input: &[u8]) -> String {
    use base64::Engine;
    base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(input)
}

/// Constant-time comparison
fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    
    result == 0
}

/// Generate JWT ID
fn generate_jti() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    format!("jwt-{:016x}", rng.gen::<u64>())
}

/// Get current timestamp
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}