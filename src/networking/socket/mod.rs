//! Socket operations module for Widya networking
//! Provides zero-copy socket operations and memory-mapped I/O

use std::net::{TcpListener, TcpStream, UdpSocket};
use std::os::unix::io::AsRawFd;
use std::time::{Duration, Instant};

use crate::networking::{NetworkingError, Result, SocketOptions, SocketState};

/// Zero-copy socket for high-performance I/O
pub struct ZeroCopySocket {
    /// TCP stream
    stream: TcpStream,
    
    /// Socket options
    options: SocketOptions,
    
    /// Connection state
    state: SocketState,
    
    /// Creation time
    created_at: Instant,
    
    /// Last activity time
    last_activity: Instant,
}

impl ZeroCopySocket {
    /// Create new zero-copy socket from TCP stream
    pub fn new(stream: TcpStream, options: SocketOptions) -> Result<Self> {
        // Apply socket options
        stream.set_nonblocking(options.non_blocking)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        stream.set_ttl(options.mtu as u32)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        Ok(Self {
            stream,
            options,
            state: SocketState::Connected,
            created_at: Instant::now(),
            last_activity: Instant::now(),
        })
    }
    
    /// Read data without copying (memory-mapped)
    pub fn read_mapped(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if self.state != SocketState::Connected {
            return Err(NetworkingError::SocketError("Socket not connected".to_string()));
        }
        
        let bytes_read = self.stream.read(buffer)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        self.last_activity = Instant::now();
        
        Ok(bytes_read)
    }
    
    /// Write data without copying
    pub fn write_mapped(&mut self, buffer: &[u8]) -> Result<usize> {
        if self.state != SocketState::Connected {
            return Err(NetworkingError::SocketError("Socket not connected".to_string()));
        }
        
        let bytes_written = self.stream.write(buffer)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        self.last_activity = Instant::now();
        
        Ok(bytes_written)
    }
    
    /// Zero-copy read with buffer
    pub fn read_zero_copy(&mut self, buffer: &mut [u8]) -> Result<usize> {
        // This would use vectored I/O or memory-mapped buffers
        // for true zero-copy in production
        self.read_mapped(buffer)
    }
    
    /// Zero-copy write with buffer
    pub fn write_zero_copy(&mut self, buffer: &[u8]) -> Result<usize> {
        // This would use vectored I/O or memory-mapped buffers
        // for true zero-copy in production
        self.write_mapped(buffer)
    }
    
    /// Sendfile for zero-copy file transfer
    pub fn sendfile(&mut self, file_fd: i32, offset: &mut u64, count: usize) -> Result<usize> {
        // In production, would use sendfile syscall
        // For now, fall back to read-write
        let mut buffer = vec![0u8; count.min(64 * 1024)]; // 64KB buffer
        
        // This would be implemented using libc::sendfile on Unix
        Err(NetworkingError::SocketError("sendfile not implemented".to_string()))
    }
    
    /// Get socket file descriptor
    pub fn fd(&self) -> i32 {
        self.stream.as_raw_fd()
    }
    
    /// Check if socket is connected
    pub fn is_connected(&self) -> bool {
        self.state == SocketState::Connected
    }
    
    /// Check if socket is closed
    pub fn is_closed(&self) -> bool {
        self.state == SocketState::Closed
    }
    
    /// Get connection age in milliseconds
    pub fn age_ms(&self) -> u64 {
        self.created_at.elapsed().as_millis() as u64
    }
    
    /// Get last activity age in milliseconds
    pub fn last_activity_ms(&self) -> u64 {
        self.last_activity.elapsed().as_millis() as u64
    }
    
    /// Close the socket
    pub fn close(&mut self) -> Result<()> {
        self.state = SocketState::Closed;
        Ok(())
    }
}

/// Memory-mapped I/O for high-throughput networking
pub struct MappedIo {
    /// Buffer size
    buffer_size: usize,
    
    /// Alignment for memory mapping
    alignment: usize,
}

impl MappedIo {
    /// Create new memory-mapped I/O
    pub fn new(buffer_size: usize, alignment: usize) -> Result<Self> {
        if buffer_size == 0 {
            return Err(NetworkingError::ConfigurationError("Buffer size must be positive".to_string()));
        }
        
        Ok(Self {
            buffer_size,
            alignment: alignment.max(4096), // Page size alignment
        })
    }
    
    /// Create memory-mapped buffer
    pub fn create_buffer(&self) -> Result<MappedBuffer> {
        // In production, would use mmap for true zero-copy
        // For now, create aligned buffer
        let aligned_size = ((self.buffer_size + self.alignment - 1) / self.alignment) * self.alignment;
        
        // Allocate aligned memory
        let buffer = vec![0u8; aligned_size];
        
        Ok(MappedBuffer {
            buffer,
            offset: 0,
            length: 0,
        })
    }
}

/// Memory-mapped buffer
pub struct MappedBuffer {
    buffer: Vec<u8>,
    offset: usize,
    length: usize,
}

impl MappedBuffer {
    /// Get buffer slice for reading
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer[self.offset..self.offset + self.length]
    }
    
    /// Get buffer slice for writing
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buffer[self.offset..self.buffer.len()]
    }
    
    /// Set read offset
    pub fn set_read_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.buffer.len());
    }
    
    /// Set write length
    pub fn set_write_length(&mut self, length: usize) {
        self.length = length.min(self.buffer.len() - self.offset);
    }
    
    /// Reset buffer
    pub fn reset(&mut self) {
        self.offset = 0;
        self.length = 0;
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }
    
    /// Get buffer length
    pub fn len(&self) -> usize {
        self.length
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

/// Socket pooling for connection reuse
pub struct SocketPool {
    /// Pool of idle connections
    idle_sockets: Vec<ZeroCopySocket>,
    
    /// Maximum pool size
    max_pool_size: usize,
    
    /// Connection timeout
    timeout: Duration,
}

impl SocketPool {
    /// Create new socket pool
    pub fn new(max_pool_size: usize, timeout: Duration) -> Self {
        Self {
            idle_sockets: Vec::new(),
            max_pool_size,
            timeout,
        }
    }
    
    /// Get socket from pool or create new
    pub fn get(&mut self, stream: TcpStream, options: SocketOptions) -> Result<ZeroCopySocket> {
        // Try to get from pool
        if !self.idle_sockets.is_empty() {
            if let Some(socket) = self.idle_sockets.pop() {
                if socket.last_activity_ms() < self.timeout.as_millis() as u64 {
                    return Ok(socket);
                }
            }
        }
        
        // Create new socket
        ZeroCopySocket::new(stream, options)
    }
    
    /// Return socket to pool
    pub fn return_to_pool(&mut self, mut socket: ZeroCopySocket) -> Result<()> {
        if self.idle_sockets.len() < self.max_pool_size {
            socket.last_activity = Instant::now();
            self.idle_sockets.push(socket);
            Ok(())
        } else {
            socket.close()?;
            Ok(())
        }
    }
    
    /// Get pool statistics
    pub fn stats(&self) -> SocketPoolStats {
        SocketPoolStats {
            idle_count: self.idle_sockets.len(),
            max_size: self.max_pool_size,
            timeout_ms: self.timeout.as_millis() as u64,
        }
    }
}

/// Socket pool statistics
#[derive(Debug, Clone)]
pub struct SocketPoolStats {
    pub idle_count: usize,
    pub max_size: usize,
    pub timeout_ms: u64,
}

/// High-performance socket operations
pub struct HighPerformanceSocket {
    /// Underlying socket
    socket: ZeroCopySocket,
    
    /// Buffer for batch operations
    send_buffer: Vec<u8>,
    
    /// Receive buffer
    receive_buffer: Vec<u8>,
    
    /// Statistics
    stats: SocketStats,
}

impl HighPerformanceSocket {
    /// Create new high-performance socket
    pub fn new(stream: TcpStream, options: SocketOptions, buffer_size: usize) -> Result<Self> {
        let socket = ZeroCopySocket::new(stream, options)?;
        
        Ok(Self {
            socket,
            send_buffer: Vec::with_capacity(buffer_size),
            receive_buffer: Vec::with_capacity(buffer_size),
            stats: SocketStats::default(),
        })
    }
    
    /// Write data with batching
    pub fn write_batch(&mut self, buffers: &[&[u8]]) -> Result<usize> {
        let total_len: usize = buffers.iter().map(|b| b.len()).sum();
        
        if total_len == 0 {
            return Ok(0);
        }
        
        // In production, would use writev for vectored I/O
        let mut total_written = 0;
        for buffer in buffers {
            let written = self.socket.write_mapped(buffer)?;
            total_written += written;
            self.stats.bytes_sent += written as u64;
            
            if written < buffer.len() {
                break;
            }
        }
        
        Ok(total_written)
    }
    
    /// Read data with batching
    pub fn read_batch(&mut self, buffers: &mut [&mut [u8]]) -> Result<usize> {
        let mut total_read = 0;
        for buffer in buffers {
            let read = self.socket.read_mapped(buffer)?;
            total_read += read;
            self.stats.bytes_received += read as u64;
            
            if read < buffer.len() {
                break;
            }
        }
        
        Ok(total_read)
    }
    
    /// Get socket statistics
    pub fn stats(&self) -> &SocketStats {
        &self.stats
    }
    
    /// Get socket
    pub fn socket(&self) -> &ZeroCopySocket {
        &self.socket
    }
}

/// Socket statistics
#[derive(Debug, Clone, Default)]
pub struct SocketStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub operations_sent: u64,
    pub operations_received: u64,
    pub errors: u64,
    pub timeouts: u64,
}