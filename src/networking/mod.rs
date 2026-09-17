//! Networking module for Widya Enterprise Edition
//! Provides high-performance networking with zero-copy operations, UDP multicast, QUIC, and HTTP/2/3 support

pub mod socket;
pub mod udp;
pub mod quic;

// Re-export commonly used types
pub use socket::{SocketPool, ZeroCopySocket, SocketOptions};
pub use udp::{UdpSocket, MulticastGroup, BroadcastSocket};
pub use quic::{QuicClient, QuicServer, QuicStream};

/// Networking errors
#[derive(Debug, thiserror::Error)]
pub enum NetworkingError {
    #[error("Socket error: {0}")]
    SocketError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Protocol error: {0}")]
    ProtocolError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Timeout error")]
    TimeoutError,
}

/// Result type for networking operations
pub type Result<T> = std::result::Result<T, NetworkingError>;

/// Network configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NetworkConfig {
    /// Socket pool configuration
    pub socket_pool: SocketPoolConfig,
    
    /// UDP configuration
    pub udp: UdpConfig,
    
    /// QUIC configuration
    pub quic: QuicConfig,
    
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    
    /// Keep-alive interval in seconds
    pub keep_alive_interval_secs: u32,
    
    /// Max concurrent connections
    pub max_concurrent_connections: usize,
    
    /// Maximum transmission unit
    pub mtu: u32,
}

/// Socket pool configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SocketPoolConfig {
    /// Enable socket pooling
    pub enabled: bool,
    
    /// Pool size per tenant
    pub pool_size_per_tenant: usize,
    
    /// Connection reuse timeout in seconds
    pub reuse_timeout_secs: u64,
    
    /// Maximum connections per endpoint
    pub max_connections_per_endpoint: usize,
}

/// UDP configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UdpConfig {
    /// Enable multicast
    pub multicast_enabled: bool,
    
    /// Multicast TTL
    pub multicast_ttl: u8,
    
    /// Broadcast enabled
    pub broadcast_enabled: bool,
    
    /// Receive buffer size
    pub receive_buffer_size: usize,
    
    /// Send buffer size
    pub send_buffer_size: usize,
}

/// QUIC configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuicConfig {
    /// Enable QUIC
    pub enabled: bool,
    
    /// TLS configuration
    pub tls: TlsConfig,
    
    /// Congestion control algorithm
    pub congestion_control: CongestionControlAlgorithm,
    
    /// Initial connection window
    pub initial_window: u64,
    
    /// Max streams per connection
    pub max_streams_per_connection: u32,
    
    /// Max bidirectional streams
    pub max_bidirectional_streams: u32,
    
    /// Max unidirectional streams
    pub max_unidirectional_streams: u32,
}

/// TLS configuration for QUIC
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TlsConfig {
    /// Certificate path
    pub cert_path: String,
    
    /// Private key path
    pub key_path: String,
    
    /// ALPN protocols
    pub alpn_protocols: Vec<String>,
}

/// Congestion control algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CongestionControlAlgorithm {
    /// CUBIC (Linux default)
    Cubic,
    
    /// BBR (Google)
    Bbr,
    
    /// New Reno
    NewReno,
    
    /// Copa
    Copa,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            socket_pool: SocketPoolConfig::default(),
            udp: UdpConfig::default(),
            quic: QuicConfig::default(),
            connection_timeout_ms: 30000,
            keep_alive_interval_secs: 60,
            max_concurrent_connections: 10000,
            mtu: 1500,
        }
    }
}

impl Default for SocketPoolConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            pool_size_per_tenant: 100,
            reuse_timeout_secs: 300,
            max_connections_per_endpoint: 1000,
        }
    }
}

impl Default for UdpConfig {
    fn default() -> Self {
        Self {
            multicast_enabled: true,
            multicast_ttl: 64,
            broadcast_enabled: true,
            receive_buffer_size: 1024 * 1024,
            send_buffer_size: 1024 * 1024,
        }
    }
}

impl Default for QuicConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            tls: TlsConfig {
                cert_path: "/etc/widya/cert.pem".to_string(),
                key_path: "/etc/widya/key.pem".to_string(),
                alpn_protocols: vec!["h3".to_string(), "h2".to_string()],
            },
            congestion_control: CongestionControlAlgorithm::Cubic,
            initial_window: 15 * 1024 * 1024, // 15 MB
            max_streams_per_connection: 100,
            max_bidirectional_streams: 100,
            max_unidirectional_streams: 100,
        }
    }
}

/// Zero-copy socket operations
pub struct ZeroCopySocket {
    /// Socket file descriptor
    fd: i32,
    
    /// Socket options
    options: SocketOptions,
    
    /// Connection state
    state: SocketState,
}

/// Socket options
#[derive(Debug, Clone)]
pub struct SocketOptions {
    /// TCP no delay (disable Nagle's algorithm)
    pub tcp_nodelay: bool,
    
    /// Keep-alive enabled
    pub keep_alive: bool,
    
    /// Keep-alive interval in seconds
    pub keep_alive_interval: u32,
    
    /// Send buffer size
    pub send_buffer_size: usize,
    
    /// Receive buffer size
    pub receive_buffer_size: usize,
    
    /// Non-blocking mode
    pub non_blocking: bool,
    
    /// TCP congestion control algorithm
    pub congestion_control: Option<String>,
}

/// Socket state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    /// Socket is closed
    Closed,
    
    /// Socket is connecting
    Connecting,
    
    /// Socket is connected
    Connected,
    
    /// Socket is closing
    Closing,
}

/// Socket pool for connection reuse
pub struct SocketPool {
    /// Pool of idle connections
    idle_connections: std::sync::Arc<std::sync::RwLock<Vec<PoolEntry>>>,
    
    /// Configuration
    config: SocketPoolConfig,
    
    /// Per-tenant connection pools
    tenant_pools: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, Vec<PoolEntry>>>>,
}

/// Pool entry
struct PoolEntry {
    /// Socket file descriptor
    fd: i32,
    
    /// Endpoint identifier
    endpoint: String,
    
    /// Tenant ID
    tenant_id: String,
    
    /// Creation time
    created_at: std::time::Instant,
    
    /// Last used time
    last_used: std::time::Instant,
    
    /// Active usage count
    usage_count: usize,
}

impl SocketPool {
    /// Create new socket pool
    pub fn new(config: SocketPoolConfig) -> Self {
        Self {
            idle_connections: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
            config,
            tenant_pools: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Get or create socket for endpoint
    pub fn get_socket(&self, endpoint: &str, tenant_id: &str) -> Result<SocketPoolConnection> {
        // Try to get from idle pool
        if let Some(socket) = self.try_get_idle_socket(endpoint, tenant_id) {
            return Ok(socket);
        }
        
        // Create new socket
        self.create_socket(endpoint, tenant_id)
    }
    
    /// Try to get idle socket
    fn try_get_idle_socket(&self, endpoint: &str, tenant_id: &str) -> Option<SocketPoolConnection> {
        let mut idle = self.idle_connections.write().unwrap();
        
        for (i, entry) in idle.iter().enumerate() {
            if entry.endpoint == endpoint && entry.tenant_id == tenant_id {
                let entry = idle.remove(i);
                entry.last_used = std::time::Instant::now();
                entry.usage_count += 1;
                
                return Some(SocketPoolConnection {
                    fd: entry.fd,
                    endpoint: entry.endpoint,
                    usage_count: entry.usage_count,
                });
            }
        }
        
        None
    }
    
    /// Create new socket
    fn create_socket(&self, endpoint: &str, tenant_id: &str) -> Result<SocketPoolConnection> {
        // Create socket
        let fd = create_socket(endpoint)?;
        
        Ok(SocketPoolConnection {
            fd,
            endpoint: endpoint.to_string(),
            usage_count: 1,
        })
    }
    
    /// Return socket to pool
    pub fn return_socket(&self, connection: SocketPoolConnection) -> Result<()> {
        let entry = PoolEntry {
            fd: connection.fd,
            endpoint: connection.endpoint,
            tenant_id: "default".to_string(),
            created_at: std::time::Instant::now(),
            last_used: std::time::Instant::now(),
            usage_count: connection.usage_count,
        };
        
        let mut idle = self.idle_connections.write().unwrap();
        idle.push(entry);
        Ok(())
    }
    
    /// Cleanup expired connections
    pub fn cleanup(&self) -> Result<usize> {
        let mut idle = self.idle_connections.write().unwrap();
        let current_time = std::time::Instant::now();
        
        let original_len = idle.len();
        idle.retain(|entry| {
            current_time.duration_since(entry.last_used).as_secs() < self.config.reuse_timeout_secs as u64
        });
        
        Ok(original_len - idle.len())
    }
}

/// Socket pool connection
pub struct SocketPoolConnection {
    fd: i32,
    endpoint: String,
    usage_count: usize,
}

impl SocketPoolConnection {
    /// Get socket file descriptor
    pub fn fd(&self) -> i32 {
        self.fd
    }
    
    /// Get endpoint
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
    
    /// Get usage count
    pub fn usage_count(&self) -> usize {
        self.usage_count
    }
}

/// Create socket for endpoint
fn create_socket(endpoint: &str) -> Result<i32> {
    // In production, would use libc or socket2
    // For now, return mock file descriptor
    Ok(1) // Mock FD
}

/// Network metrics
#[derive(Debug, Clone)]
pub struct NetworkMetrics {
    /// Total connections established
    pub connections_established: u64,
    
    /// Total connections closed
    pub connections_closed: u64,
    
    /// Current active connections
    pub active_connections: u64,
    
    /// Total bytes sent
    pub bytes_sent: u64,
    
    /// Total bytes received
    pub bytes_received: u64,
    
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    
    /// Connection error count
    pub connection_errors: u64,
    
    /// Timeout count
    pub timeouts: u64,
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            connections_established: 0,
            connections_closed: 0,
            active_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
            average_latency_ms: 0.0,
            connection_errors: 0,
            timeouts: 0,
        }
    }
}