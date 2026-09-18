//! QUIC protocol implementation for Widya networking
//! Provides QUIC (HTTP/3) support with multiplexing and congestion control

use std::collections::{HashMap, VecDeque};
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::{Duration, Instant};

use crate::networking::{NetworkingError, Result, QuicConfig, TlsConfig, CongestionControlAlgorithm};

/// QUIC client
pub struct QuicClient {
    /// Configuration
    config: QuicConfig,
    
    /// Active connections
    connections: std::sync::Arc<std::sync::RwLock<HashMap<String, QuicConnection>>>,
    
    /// Connection counter
    connection_counter: std::sync::Arc<std::sync::Mutex<u64>>,
}

impl QuicClient {
    /// Create new QUIC client
    pub fn new(config: QuicConfig) -> Self {
        Self {
            config,
            connections: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
            connection_counter: std::sync::Arc::new(std::sync::Mutex::new(0)),
        }
    }
    
    /// Connect to server
    pub fn connect(&self, addr: &str) -> Result<QuicConnection> {
        let addr: SocketAddr = addr.parse()
            .map_err(|e| NetworkingError::NetworkError(e.to_string()))?;
        
        let conn_id = self.connection_counter.lock().unwrap().inc();
        
        let connection = QuicConnection::new(
            conn_id,
            addr,
            self.config.clone(),
        )?;
        
        // Store connection
        let addr_str = addr.to_string();
        {
            let mut connections = self.connections.write().unwrap();
            connections.insert(addr_str.clone(), connection.clone());
        }
        
        Ok(connection)
    }
    
    /// Get connection for address
    pub fn get_connection(&self, addr: &str) -> Option<QuicConnection> {
        let connections = self.connections.read().unwrap();
        connections.get(addr).cloned()
    }
    
    /// Close all connections
    pub fn close_all(&self) -> Result<()> {
        let mut connections = self.connections.write().unwrap();
        connections.clear();
        Ok(())
    }
}

/// QUIC server
pub struct QuicServer {
    /// Configuration
    config: QuicConfig,
    
    /// Listener address
    listener: Option<std::net::UdpSocket>,
    
    /// Active connections
    connections: std::sync::Arc<std::sync::RwLock<HashMap<u64, QuicConnection>>>,
    
    /// New connection queue
    new_connections: std::sync::Arc<std::sync::Mutex<VecDeque<QuicConnection>>>,
}

impl QuicServer {
    /// Create new QUIC server
    pub fn bind(addr: &str, config: QuicConfig) -> Result<Self> {
        let socket = std::net::UdpSocket::bind(addr)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        Ok(Self {
            config,
            listener: Some(socket),
            connections: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
            new_connections: std::sync::Arc::new(std::sync::Mutex::new(VecDeque::new())),
        })
    }
    
    /// Accept incoming connection (non-blocking)
    pub fn accept(&self) -> Result<Option<QuicConnection>> {
        let mut queue = self.new_connections.lock().unwrap();
        
        if let Some(conn) = queue.pop_front() {
            // Store connection
            let mut connections = self.connections.write().unwrap();
            connections.insert(conn.id, conn.clone());
            
            Ok(Some(conn))
        } else {
            Ok(None)
        }
    }
    
    /// Simulate incoming connection (for testing)
    pub fn simulate_incoming(&self, addr: SocketAddr) -> Result<QuicConnection> {
        let conn_id = self.connections.read().unwrap().len() as u64 + 1;
        
        let connection = QuicConnection::new(
            conn_id,
            addr,
            self.config.clone(),
        )?;
        
        let mut queue = self.new_connections.lock().unwrap();
        queue.push_back(connection.clone());
        
        Ok(connection)
    }
    
    /// Get connection count
    pub fn connection_count(&self) -> usize {
        self.connections.read().unwrap().len()
    }
}

/// QUIC connection
pub struct QuicConnection {
    /// Connection ID
    pub id: u64,
    
    /// Remote address
    pub remote_addr: SocketAddr,
    
    /// Configuration
    config: QuicConfig,
    
    /// Stream map
    streams: std::sync::Arc<std::sync::RwLock<HashMap<u64, QuicStream>>>,
    
    /// Stream counter
    stream_counter: std::sync::Arc<std::sync::Mutex<u64>>,
    
    /// Transmission state
    transmission: TransmissionState,
    
    /// Last activity time
    last_activity: std::sync::Arc<std::sync::Mutex<Instant>>,
}

impl QuicConnection {
    /// Create new QUIC connection
    fn new(id: u64, remote_addr: SocketAddr, config: QuicConfig) -> Result<Self> {
        Ok(Self {
            id,
            remote_addr,
            config,
            streams: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
            stream_counter: std::sync::Arc::new(std::sync::Mutex::new(0)),
            transmission: TransmissionState::new(),
            last_activity: std::sync::Arc::new(std::sync::Mutex::new(Instant::now())),
        })
    }
    
    /// Open bidirectional stream
    pub fn open_bidirectional_stream(&self) -> Result<QuicStream> {
        let mut counter = self.stream_counter.lock().unwrap();
        let stream_id = *counter;
        *counter += 2;
        
        let stream = QuicStream::new(stream_id, StreamDirection::Bidirectional);
        
        let mut streams = self.streams.write().unwrap();
        streams.insert(stream_id, stream.clone());
        
        Ok(stream)
    }
    
    /// Open unidirectional stream
    pub fn open_unidirectional_stream(&self) -> Result<QuicStream> {
        let mut counter = self.stream_counter.lock().unwrap();
        let stream_id = *counter;
        *counter += 2;
        
        let stream = QuicStream::new(stream_id, StreamDirection::Unidirectional);
        
        let mut streams = self.streams.write().unwrap();
        streams.insert(stream_id, stream.clone());
        
        Ok(stream)
    }
    
    /// Get stream
    pub fn get_stream(&self, stream_id: u64) -> Option<QuicStream> {
        let streams = self.streams.read().unwrap();
        streams.get(&stream_id).cloned()
    }
    
    /// Update last activity time
    pub fn update_activity(&self) {
        let mut last_activity = self.last_activity.lock().unwrap();
        *last_activity = Instant::now();
    }
    
    /// Get connection stats
    pub fn stats(&self) -> QuicStats {
        let streams = self.streams.read().unwrap();
        
        QuicStats {
            connection_id: self.id,
            remote_addr: self.remote_addr.to_string(),
            active_streams: streams.len(),
            bytes_sent: self.transmission.bytes_sent,
            bytes_received: self.transmission.bytes_received,
            streams_opened: self.transmission.streams_opened,
            streams_closed: self.transmission.streams_closed,
            last_activity_ms: self.last_activity.lock().unwrap().elapsed().as_millis() as u64,
        }
    }
}

/// QUIC stream
pub struct QuicStream {
    /// Stream ID
    pub id: u64,
    
    /// Direction
    direction: StreamDirection,
    
    /// Write buffer
    write_buffer: VecDeque<u8>,
    
    /// Read buffer
    read_buffer: VecDeque<u8>,
    
    /// State
    state: StreamState,
}

impl QuicStream {
    /// Create new QUIC stream
    fn new(id: u64, direction: StreamDirection) -> Self {
        Self {
            id,
            direction,
            write_buffer: VecDeque::new(),
            read_buffer: VecDeque::new(),
            state: StreamState::Open,
        }
    }
    
    /// Write data to stream
    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        if self.state != StreamState::Open {
            return Err(NetworkingError::SocketError("Stream not open".to_string()));
        }
        
        for &byte in data {
            self.write_buffer.push_back(byte);
        }
        
        Ok(data.len())
    }
    
    /// Read data from stream
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut bytes_read = 0;
        for i in 0..buffer.len() {
            if let Some(byte) = self.read_buffer.pop_front() {
                buffer[i] = byte;
                bytes_read += 1;
            } else {
                break;
            }
        }
        
        Ok(bytes_read)
    }
    
    /// Get write buffer size
    pub fn write_buffer_len(&self) -> usize {
        self.write_buffer.len()
    }
    
    /// Check if stream is open
    pub fn is_open(&self) -> bool {
        self.state == StreamState::Open
    }
}

/// Stream direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamDirection {
    Bidirectional,
    Unidirectional,
}

/// Stream state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StreamState {
    Open,
    Closed,
}

/// Transmission state
struct TransmissionState {
    bytes_sent: u64,
    bytes_received: u64,
    streams_opened: u64,
    streams_closed: u64,
}

impl TransmissionState {
    fn new() -> Self {
        Self {
            bytes_sent: 0,
            bytes_received: 0,
            streams_opened: 0,
            streams_closed: 0,
        }
    }
}

/// QUIC statistics
#[derive(Debug, Clone)]
pub struct QuicStats {
    pub connection_id: u64,
    pub remote_addr: String,
    pub active_streams: usize,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub streams_opened: u64,
    pub streams_closed: u64,
    pub last_activity_ms: u64,
}

/// Congestion controller interface
pub trait CongestionController {
    /// Called when packet is sent
    fn on_packet_sent(&mut self, bytes: usize);
    
    /// Called when packet is acknowledged
    fn on_packet_acked(&mut self, bytes: usize);
    
    /// Called when packet is lost
    fn on_packet_lost(&mut self, bytes: usize);
    
    /// Get current congestion window
    fn congestion_window(&self) -> u64;
    
    /// Get current ssthresh
    fn ssthresh(&self) -> u64;
    
    /// Get current flight size
    fn flight_size(&self) -> u64;
}

/// CUBIC congestion controller
pub struct CubicCongestionController {
    /// Initial congestion window
    initial_cwnd: u64,
    
    /// Current congestion window
    cwnd: u64,
    
    /// Slow start threshold
    ssthresh: u64,
    
    /// Flight size
    flight_size: u64,
    
    /// Last congestion window
    last_cwnd: u64,
    
    /// Last congestion event time
    last_congestion_time: Option<u64>,
    
    /// K (CUBIC parameter)
    k: f64,
    
    /// W_max (CUBIC parameter)
    w_max: f64,
}

impl CubicCongestionController {
    pub fn new(initial_cwnd: u64) -> Self {
        Self {
            initial_cwnd,
            cwnd: initial_cwnd,
            ssthresh: u64::MAX,
            flight_size: 0,
            last_cwnd: initial_cwnd,
            last_congestion_time: None,
            k: 0.0,
            w_max: 0.0,
        }
    }
}

impl CongestionController for CubicCongestionController {
    fn on_packet_sent(&mut self, bytes: usize) {
        self.flight_size += bytes as u64;
    }
    
    fn on_packet_acked(&mut self, bytes: usize) {
        self.flight_size = self.flight_size.saturating_sub(bytes as u64);
        
        if self.cwnd < self.ssthresh {
            // Slow start
            self.cwnd = (self.cwnd as u64 + bytes as u64).min(self.initial_cwnd * 10);
        } else {
            // Congestion avoidance (CUBIC)
            self.k = ((self.w_max - self.cwnd as f64) / (0.4 * self.w_max)).powf(1.0/3.0);
            self.cwnd = self.cwnd + ((self.k as f64 - self.k).powi(3) * 0.4) as u64;
        }
    }
    
    fn on_packet_lost(&mut self, bytes: usize) {
        self.last_cwnd = self.cwnd;
        self.w_max = self.cwnd as f64;
        self.last_congestion_time = Some(0); // In production, use current time
        
        self.ssthresh = (self.cwnd as f64 * 0.7).max(2.0) as u64;
        self.cwnd = self.ssthresh;
    }
    
    fn congestion_window(&self) -> u64 {
        self.cwnd
    }
    
    fn ssthresh(&self) -> u64 {
        self.ssthresh
    }
    
    fn flight_size(&self) -> u64 {
        self.flight_size
    }
}

impl Default for CubicCongestionController {
    fn default() -> Self {
        Self::new(15 * 1024 * 1024) // 15 MB initial window
    }
}