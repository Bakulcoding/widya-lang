//! Service Mesh module for Widya Enterprise Edition
//! Provides Kubernetes-native service mesh support with xDS protocol, traffic management, and mTLS

pub mod traffic;
pub mod security;
pub mod discovery;
pub mod kubernetes;

// Re-export commonly used types
pub use traffic::{TrafficManager, CircuitBreaker, RateLimiter};
pub use security::{mTLSEngine, CertificateManager, ServiceIdentity};
pub use discovery::{ServiceDiscovery, Endpoint, HealthChecker};
pub use kubernetes::{K8sIntegration, CRDManager, SidecarInjector};

/// Service mesh errors
#[derive(Debug, thiserror::Error)]
pub enum ServiceMeshError {
    #[error("Discovery error: {0}")]
    DiscoveryError(String),
    
    #[error("Traffic management error: {0}")]
    TrafficError(String),
    
    #[error("Security error: {0}")]
    SecurityError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Kubernetes error: {0}")]
    K8sError(String),
    
    #[error("Circuit breaker error: {0}")]
    CircuitBreakerError(String),
    
    #[error("Rate limiting error: {0}")]
    RateLimitError(String),
}

/// Result type for service mesh operations
pub type Result<T> = std::result::Result<T, ServiceMeshError>;

/// Service mesh configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServiceMeshConfig {
    /// Sidecar proxy configuration
    pub sidecar: SidecarConfig,
    
    /// Discovery configuration
    pub discovery: DiscoveryConfig,
    
    /// Traffic management configuration
    pub traffic: TrafficConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Kubernetes integration
    pub kubernetes: KubernetesConfig,
    
    /// Enable telemetry collection
    pub telemetry_enabled: bool,
    
    /// Log level (DEBUG, INFO, WARN, ERROR)
    pub log_level: String,
}

/// Sidecar proxy configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SidecarConfig {
    /// Image to use for sidecar container
    pub image: String,
    
    /// Resources for sidecar container
    pub resources: ResourceRequirements,
    
    /// Port for sidecar proxy
    pub proxy_port: u16,
    
    /// Port for admin interface
    pub admin_port: u16,
    
    /// Log level for sidecar
    pub log_level: String,
    
    /// Enable prometheus metrics
    pub metrics_enabled: bool,
}

/// Resource requirements for sidecar
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceRequirements {
    /// CPU requests
    pub cpu_requests: String,
    
    /// Memory requests
    pub memory_requests: String,
    
    /// CPU limits
    pub cpu_limits: String,
    
    /// Memory limits
    pub memory_limits: String,
}

/// Discovery configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscoveryConfig {
    /// Service mesh namespace
    pub mesh_namespace: String,
    
    /// DNS suffix for internal services
    pub dns_suffix: String,
    
    /// Enable mDNS for local discovery
    pub mdns_enabled: bool,
    
    /// Service discovery interval in seconds
    pub discovery_interval_secs: u64,
}

/// Traffic management configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrafficConfig {
    /// Default timeout in milliseconds
    pub default_timeout_ms: u64,
    
    /// Retry policy
    pub retry_policy: RetryPolicy,
    
    /// Timeout policy
    pub timeout_policy: TimeoutPolicy,
    
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    
    /// Circuit breaker settings
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Retry policy
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetryPolicy {
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Initial retry timeout in milliseconds
    pub initial_timeout_ms: u64,
    
    /// Maximum timeout in milliseconds
    pub max_timeout_ms: u64,
    
    /// Retry backoff multiplier
    pub backoff_multiplier: f64,
    
    /// Retryable status codes
    pub retryable_status_codes: Vec<String>,
}

/// Timeout policy
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeoutPolicy {
    /// Connect timeout in milliseconds
    pub connect_timeout_ms: u64,
    
    /// Idle timeout in milliseconds
    pub idle_timeout_ms: u64,
    
    /// Per-request timeout in milliseconds
    pub per_request_timeout_ms: u64,
    
    /// Stream idle timeout in milliseconds
    pub stream_idle_timeout_ms: u64,
}

/// Load balancing strategies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    /// Round robin
    RoundRobin,
    
    /// Least connections
    LeastConnections,
    
    /// Random
    Random,
    
    /// Ring hash
    RingHash,
    
    ///Maglev
    Maglev,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircuitBreakerConfig {
    /// Maximum number of connections
    pub max_connections: u32,
    
    /// Pending requests threshold
    pub pending_requests: u32,
    
    /// Concurrent requests threshold
    pub concurrent_requests: u32,
    
    /// Error threshold percentage
    pub error_threshold_percentage: u32,
    
    /// Recovery timeout in milliseconds
    pub recovery_timeout_ms: u64,
}

/// Security configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    /// Enable mTLS
    pub mtls_enabled: bool,
    
    /// Certificate rotation interval in days
    pub cert_rotation_days: u32,
    
    /// Root CA certificate path
    pub root_ca_cert_path: String,
    
    /// Issuer certificate path
    pub issuer_cert_path: String,
    
    /// Issuer key path
    pub issuer_key_path: String,
}

/// Kubernetes configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KubernetesConfig {
    /// Enable Kubernetes integration
    pub enabled: bool,
    
    /// Service account name
    pub service_account_name: String,
    
    /// Enable sidecar injection
    pub sidecar_injection_enabled: bool,
    
    /// Injection label selector
    pub injection_label_selector: String,
    
    /// CRD manifest path
    pub crd_manifest_path: String,
}

impl Default for ServiceMeshConfig {
    fn default() -> Self {
        Self {
            sidecar: SidecarConfig::default(),
            discovery: DiscoveryConfig::default(),
            traffic: TrafficConfig::default(),
            security: SecurityConfig::default(),
            kubernetes: KubernetesConfig::default(),
            telemetry_enabled: true,
            log_level: "INFO".to_string(),
        }
    }
}

impl Default for SidecarConfig {
    fn default() -> Self {
        Self {
            image: "widya/sidecar:latest".to_string(),
            resources: ResourceRequirements {
                cpu_requests: "100m".to_string(),
                memory_requests: "128Mi".to_string(),
                cpu_limits: "500m".to_string(),
                memory_limits: "512Mi".to_string(),
            },
            proxy_port: 15001,
            admin_port: 15000,
            log_level: "info".to_string(),
            metrics_enabled: true,
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            mesh_namespace: "widya-system".to_string(),
            dns_suffix: "svc.cluster.local".to_string(),
            mdns_enabled: false,
            discovery_interval_secs: 30,
        }
    }
}

impl Default for TrafficConfig {
    fn default() -> Self {
        Self {
            default_timeout_ms: 30000,
            retry_policy: RetryPolicy::default(),
            timeout_policy: TimeoutPolicy::default(),
            load_balancing: LoadBalancingStrategy::RoundRobin,
            circuit_breaker: CircuitBreakerConfig::default(),
        }
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_timeout_ms: 100,
            max_timeout_ms: 1000,
            backoff_multiplier: 2.0,
            retryable_status_codes: vec!["502".to_string(), "503".to_string(), "504".to_string()],
        }
    }
}

impl Default for TimeoutPolicy {
    fn default() -> Self {
        Self {
            connect_timeout_ms: 5000,
            idle_timeout_ms: 60000,
            per_request_timeout_ms: 30000,
            stream_idle_timeout_ms: 15000,
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            max_connections: 10000,
            pending_requests: 1000,
            concurrent_requests: 1000,
            error_threshold_percentage: 50,
            recovery_timeout_ms: 30000,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            mtls_enabled: true,
            cert_rotation_days: 30,
            root_ca_cert_path: "/etc/widya/ca.crt".to_string(),
            issuer_cert_path: "/etc/widya/cert.pem".to_string(),
            issuer_key_path: "/etc/widya/key.pem".to_string(),
        }
    }
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_account_name: "widya-proxy".to_string(),
            sidecar_injection_enabled: true,
            injection_label_selector: "widya.io/inject=enabled".to_string(),
            crd_manifest_path: "/etc/widya/crds".to_string(),
        }
    }
}

/// Initialize service mesh
pub fn init(config: ServiceMeshConfig) -> Result<ServiceMesh> {
    let mesh = ServiceMesh::new(config)?;
    Ok(mesh)
}

/// Service mesh instance
pub struct ServiceMesh {
    /// Configuration
    config: ServiceMeshConfig,
    
    /// Traffic manager
    traffic_manager: Option<TrafficManager>,
    
    /// Security engine
    security_engine: Option<mTLSEngine>,
    
    /// Service discovery
    discovery: Option<ServiceDiscovery>,
}

impl ServiceMesh {
    /// Create new service mesh
    pub fn new(config: ServiceMeshConfig) -> Result<Self> {
        // In production, initialize all components
        Ok(Self {
            config,
            traffic_manager: None,
            security_engine: None,
            discovery: None,
        })
    }
    
    /// Start service mesh components
    pub fn start(&mut self) -> Result<()> {
        // Initialize components based on configuration
        if self.config.traffic.circuit_breaker.error_threshold_percentage > 0 {
            self.traffic_manager = Some(TrafficManager::new());
        }
        
        if self.config.security.mtls_enabled {
            self.security_engine = Some(mTLSEngine::new(&self.config.security)?);
        }
        
        Ok(())
    }
    
    /// Stop service mesh components
    pub fn stop(&mut self) -> Result<()> {
        Ok(())
    }
    
    /// Get configuration
    pub fn config(&self) -> &ServiceMeshConfig {
        &self.config
    }
}

/// Service mesh statistics
#[derive(Debug, Clone)]
pub struct ServiceMeshStats {
    /// Active services
    pub active_services: usize,
    
    /// Active endpoints
    pub active_endpoints: usize,
    
    /// Connections established
    pub connections_established: u64,
    
    /// Connections closed
    pub connections_closed: u64,
    
    /// Circuit breakers triggered
    pub circuit_breakers_triggered: u64,
    
    /// Requests rate limited
    pub requests_rate_limited: u64,
    
    /// Total bandwidth in bytes
    pub total_bandwidth_bytes: u64,
}