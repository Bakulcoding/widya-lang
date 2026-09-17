//! UDP networking module for Widya networking
//! Provides UDP socket operations, multicast, and broadcast support

use std::collections::HashSet;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

use crate::networking::{NetworkingError, Result, UdpConfig};

/// UDP socket with enhanced features
pub struct UdpSocket {
    /// Underlying socket
    socket: std::net::UdpSocket,
    
    /// Configuration
    config: UdpConfig,
    
    /// Multicast groups joined
    multicast_groups: std::sync::Arc<std::sync::RwLock<HashSet<String>>>,
    
    /// Broadcast address
    broadcast_address: Option<SocketAddr>,
}

impl UdpSocket {
    /// Create new UDP socket
    pub fn bind(address: &str, config: UdpConfig) -> Result<Self> {
        let socket = std::net::UdpSocket::bind(address)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        // Set socket options
        socket.set_broadcast(config.broadcast_enabled)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        socket.set_multicast_loop_v4(config.multicast_enabled)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        socket.set_multicast_ttl_v4(config.multicast_ttl)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        Ok(Self {
            socket,
            config,
            multicast_groups: std::sync::Arc::new(std::sync::RwLock::new(HashSet::new())),
            broadcast_address: None,
        })
    }
    
    /// Join multicast group
    pub fn join_multicast_group(&mut self, multicast_addr: &str, interface: Option<&str>) -> Result<()> {
        let multicast_ip: IpAddr = multicast_addr.parse()
            .map_err(|e| NetworkingError::NetworkError(format!("Invalid multicast address: {}", e)))?;
        
        match multicast_ip {
            IpAddr::V4(addr) => {
                use std::net::ToSocketAddrs;
                
                let interface_addr = if let Some(if_name) = interface {
                    // Get interface IP from name
                    let interface_ip = Self::get_interface_ip(if_name)?;
                    interface_ip.to_string()
                } else {
                    "0.0.0.0".to_string()
                };
                
                let multicast_addr_v4: Ipv4Addr = addr;
                
                // Use setsockopt for multicast
                // This is a simplified implementation
                let group = format!("{}/{}", multicast_addr, interface_addr);
                
                let mut groups = self.multicast_groups.write().unwrap();
                groups.insert(group);
                
                Ok(())
            }
            IpAddr::V6(_) => {
                Err(NetworkingError::NetworkError("IPv6 multicast not implemented".to_string()))
            }
        }
    }
    
    /// Leave multicast group
    pub fn leave_multicast_group(&mut self, multicast_addr: &str) -> Result<()> {
        let group = format!("{}", multicast_addr);
        let mut groups = self.multicast_groups.write().unwrap();
        groups.remove(&group);
        Ok(())
    }
    
    /// Send to multicast group
    pub fn send_to_multicast(&self, data: &[u8], group: &str, port: u16) -> Result<usize> {
        let addr = format!("{}:{}", group, port)
            .parse::<SocketAddr>()
            .map_err(|e| NetworkingError::NetworkError(e.to_string()))?;
        
        self.socket.send_to(data, &addr)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))
    }
    
    /// Broadcast data
    pub fn broadcast(&self, data: &[u8], port: u16) -> Result<usize> {
        if !self.config.broadcast_enabled {
            return Err(NetworkingError::ConfigurationError("Broadcast disabled".to_string()));
        }
        
        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), port);
        
        self.socket.send_to(data, &addr)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))
    }
    
    /// Set broadcast address
    pub fn set_broadcast_address(&mut self, addr: SocketAddr) {
        self.broadcast_address = Some(addr);
    }
    
    /// Get interface IP by name
    fn get_interface_ip(interface_name: &str) -> Result<Ipv4Addr> {
        // In production, would use netlink or similar
        // For now, return localhost
        Ok(Ipv4Addr::LOCALHOST)
    }
    
    /// Receive data with timeout
    pub fn receive_with_timeout(&self, timeout: Duration) -> Result<(usize, SocketAddr)> {
        self.socket.set_read_timeout(Some(timeout))
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        let mut buffer = [0u8; 65535];
        let (len, addr) = self.socket.recv_from(&mut buffer)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        Ok((len, addr))
    }
    
    /// Get socket statistics
    pub fn stats(&self) -> UdpStats {
        UdpStats {
            multicast_groups: self.multicast_groups.read().unwrap().len(),
            broadcast_enabled: self.config.broadcast_enabled,
            multicast_enabled: self.config.multicast_enabled,
        }
    }
}

/// UDP statistics
#[derive(Debug, Clone)]
pub struct UdpStats {
    pub multicast_groups: usize,
    pub broadcast_enabled: bool,
    pub multicast_enabled: bool,
}

/// Multicast group manager
pub struct MulticastGroupManager {
    /// Active multicast groups
    groups: std::sync::Arc<std::sync::RwLock<HashSet<String>>>,
    
    /// Interface to multicast mapping
    interface_groups: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<String, Vec<String>>>>,
}

impl MulticastGroupManager {
    /// Create new multicast group manager
    pub fn new() -> Self {
        Self {
            groups: std::sync::Arc::new(std::sync::RwLock::new(HashSet::new())),
            interface_groups: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    
    /// Join multicast group on interface
    pub fn join_group(&self, group: &str, interface: &str) -> Result<()> {
        let mut groups = self.groups.write().unwrap();
        groups.insert(group.to_string());
        
        let mut interface_groups = self.interface_groups.write().unwrap();
        interface_groups.entry(interface.to_string())
            .or_insert_with(Vec::new)
            .push(group.to_string());
        
        Ok(())
    }
    
    /// Leave multicast group
    pub fn leave_group(&self, group: &str) -> Result<()> {
        let mut groups = self.groups.write().unwrap();
        groups.remove(group);
        
        Ok(())
    }
    
    /// Get active groups
    pub fn active_groups(&self) -> Vec<String> {
        let groups = self.groups.read().unwrap();
        groups.iter().cloned().collect()
    }
    
    /// Get groups on interface
    pub fn groups_for_interface(&self, interface: &str) -> Vec<String> {
        let interface_groups = self.interface_groups.read().unwrap();
        interface_groups.get(interface)
            .map(|g| g.iter().cloned().collect())
            .unwrap_or_default()
    }
}

/// Broadcast socket
pub struct BroadcastSocket {
    /// UDP socket
    socket: std::net::UdpSocket,
    
    /// Broadcast address
    address: SocketAddr,
}

impl BroadcastSocket {
    /// Create new broadcast socket
    pub fn bind(port: u16) -> Result<Self> {
        let socket = std::net::UdpSocket::bind(format!("0.0.0.0:{}", port))
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        socket.set_broadcast(true)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))?;
        
        Ok(Self {
            socket,
            address: SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), port),
        })
    }
    
    /// Send broadcast message
    pub fn send(&self, data: &[u8]) -> Result<usize> {
        self.socket.send_to(data, &self.address)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))
    }
    
    /// Receive broadcast message
    pub fn receive(&self, buffer: &mut [u8]) -> Result<usize> {
        self.socket.recv_from(buffer)
            .map(|(len, _)| len)
            .map_err(|e| NetworkingError::SocketError(e.to_string()))
    }
    
    /// Get broadcast address
    pub fn address(&self) -> &SocketAddr {
        &self.address
    }
}