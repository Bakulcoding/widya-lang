//! Discovery module for Widya service mesh
//! Provides service discovery and endpoint management

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use crate::servicemesh::{ServiceMeshError, Result};

/// Service discovery for managing service endpoints
pub struct ServiceDiscovery {
    /// Service registry
    services: Arc<RwLock<HashMap<String, Service>>>,
    
    /// Endpoint registry
    endpoints: Arc<RwLock<HashMap<String, Vec<Endpoint>>>>,
    
    /// Health check results
    health: Arc<RwLock<HashMap<String, HealthStatus>>>,
    
    /// Last discovery time
    last_discovery: Arc<RwLock<Instant>>,
    
    /// DNS suffix for service resolution
    dns_suffix: String,
}

impl ServiceDiscovery {
    /// Create new service discovery
    pub fn new(dns_suffix: &str) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            health: Arc::new(RwLock::new(HashMap::new())),
            last_discovery: Arc::new(RwLock::new(Instant::now())),
            dns_suffix: dns_suffix.to_string(),
        }
    }
    
    /// Register service
    pub fn register_service(&self, service: Service) -> Result<()> {
        let mut services = self.services.write().unwrap();
        services.insert(service.name.clone(), service);
        Ok(())
    }
    
    /// Register endpoint for service
    pub fn register_endpoint(&self, service: &str, endpoint: Endpoint) -> Result<()> {
        let mut endpoints = self.endpoints.write().unwrap();
        
        endpoints.entry(service.to_string())
            .or_insert_with(Vec::new)
            .push(endpoint);
        
        Ok(())
    }
    
    /// Get service by name
    pub fn get_service(&self, name: &str) -> Option<Service> {
        let services = self.services.read().unwrap();
        services.get(name).cloned()
    }
    
    /// Get endpoints for service
    pub fn get_endpoints(&self, name: &str) -> Vec<Endpoint> {
        let endpoints = self.endpoints.read().unwrap();
        endpoints.get(name).cloned().unwrap_or_default()
    }
    
    /// Get healthy endpoints for service
    pub fn get_healthy_endpoints(&self, name: &str) -> Vec<Endpoint> {
        let endpoints = self.endpoints.read().unwrap();
        
        endpoints.get(name)
            .map(|eps| {
                eps.iter()
                    .filter(|e| e.health == HealthStatus::Healthy || e.health == HealthStatus::Unknown)
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }
    
    /// Update health status
    pub fn update_health(&self, service: &str, endpoint: &str, status: HealthStatus) -> Result<()> {
        let mut health = self.health.write().unwrap();
        let key = format!("{}:{}", service, endpoint);
        health.insert(key, status);
        
        // Update endpoint health
        let mut endpoints = self.endpoints.write().unwrap();
        if let Some(endpoints_list) = endpoints.get_mut(service) {
            for ep in endpoints_list.iter_mut() {
                if ep.address == endpoint {
                    ep.health = status;
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Discover services (poll from external source)
    pub fn discover(&self) -> Result<usize> {
        // In production, query from xDS server or DNS
        // For now, return 0 (no new services discovered)
        
        let mut last_discovery = self.last_discovery.write().unwrap();
        *last_discovery = Instant::now();
        
        Ok(0)
    }
    
    /// Resolve service name to address
    pub fn resolve(&self, name: &str) -> Result<String> {
        let endpoints = self.endpoints.read().unwrap();
        
        if let Some(eps) = endpoints.get(name) {
            if !eps.is_empty() {
                return Ok(format!("{}:{}", eps[0].address, eps[0].port));
            }
        }
        
        // Try with DNS suffix
        let with_suffix = format!("{}.{}", name, self.dns_suffix);
        Ok(with_suffix)
    }
    
    /// Get discovery statistics
    pub fn stats(&self) -> DiscoveryStats {
        let services = self.services.read().unwrap();
        let endpoints = self.endpoints.read().unwrap();
        
        let mut total_endpoints = 0;
        let mut healthy_endpoints = 0;
        
        for eps in endpoints.values() {
            total_endpoints += eps.len();
            healthy_endpoints += eps.iter().filter(|e| e.health == HealthStatus::Healthy).count();
        }
        
        DiscoveryStats {
            total_services: services.len(),
            total_endpoints,
            healthy_endpoints,
        }
    }
}

/// Service information
#[derive(Debug, Clone)]
pub struct Service {
    /// Service name
    pub name: String,
    
    /// Service namespace
    pub namespace: String,
    
    /// Service labels
    pub labels: HashMap<String, String>,
    
    /// Service annotations
    pub annotations: HashMap<String, String>,
    
    /// Ports
    pub ports: Vec<ServicePort>,
    
    /// Cluster IP
    pub cluster_ip: Option<String>,
    
    /// Type (ClusterIP, NodePort, LoadBalancer)
    pub service_type: ServiceType,
}

/// Service port
#[derive(Debug, Clone)]
pub struct ServicePort {
    /// Port name
    pub name: String,
    
    /// Port number
    pub port: u16,
    
    /// Target port
    pub target_port: u16,
    
    /// Protocol
    pub protocol: Protocol,
}

/// Service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    /// ClusterIP (internal only)
    ClusterIP,
    
    /// NodePort (external via node port)
    NodePort,
    
    /// LoadBalancer (external via load balancer)
    LoadBalancer,
}

/// Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// TCP
    Tcp,
    
    /// UDP
    Udp,
}

/// Endpoint information
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// Endpoint address (IP or hostname)
    pub address: String,
    
    /// Port
    pub port: u16,
    
    /// Hostname
    pub hostname: Option<String>,
    
    /// Health status
    pub health: HealthStatus,
    
    /// Weight for load balancing
    pub weight: u32,
    
    /// Metadata
    pub metadata: HashMap<String, String>,
    
    /// Zone (for zone-aware routing)
    pub zone: Option<String>,
    
    /// Local locality
    pub locality: Option<String>,
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// Healthy
    Healthy,
    
    /// Unhealthy
    Unhealthy,
    
    /// Unknown
    Unknown,
}

/// Discovery statistics
#[derive(Debug, Clone)]
pub struct DiscoveryStats {
    pub total_services: usize,
    pub total_endpoints: usize,
    pub healthy_endpoints: usize,
}

/// Health checker for endpoints
pub struct HealthChecker {
    /// Health check intervals
    check_intervals: HashMap<String, Duration>,
    
    /// Health check results
    results: Arc<RwLock<HashMap<String, HealthCheckResult>>>,
    
    /// Active health checkers
    active_checkers: HashSet<String>,
}

impl HealthChecker {
    /// Create new health checker
    pub fn new() -> Self {
        Self {
            check_intervals: HashMap::new(),
            results: Arc::new(RwLock::new(HashMap::new())),
            active_checkers: HashSet::new(),
        }
    }
    
    /// Register health check for endpoint
    pub fn register_health_check(
        &mut self,
        service: &str,
        endpoint: &str,
        interval: Duration,
        timeout: Duration,
    ) -> Result<()> {
        let key = format!("{}:{}", service, endpoint);
        self.check_intervals.insert(key.clone(), interval);
        
        // In production, spawn health check task
        self.active_checkers.insert(key);
        
        Ok(())
    }
    
    /// Record health check result
    pub fn record_health_result(
        &self,
        service: &str,
        endpoint: &str,
        success: bool,
        latency_ms: u64,
    ) -> Result<()> {
        let key = format!("{}:{}", service, endpoint);
        let mut results = self.results.write().unwrap();
        
        results.insert(key, HealthCheckResult {
            success,
            latency_ms,
            checked_at: chrono::Utc::now(),
            consecutive_successes: if success { 1 } else { 0 },
            consecutive_failures: if !success { 1 } else { 0 },
        });
        
        Ok(())
    }
    
    /// Get health check result
    pub fn get_health_result(&self, service: &str, endpoint: &str) -> Option<HealthCheckResult> {
        let results = self.results.read().unwrap();
        let key = format!("{}:{}", service, endpoint);
        results.get(&key).cloned()
    }
}

/// Health check result
#[derive(Debug, Clone)]
pub struct HealthCheckResult {
    /// Whether check was successful
    pub success: bool,
    
    /// Latency in milliseconds
    pub latency_ms: u64,
    
    /// When check was performed
    pub checked_at: chrono::DateTime<chrono::Utc>,
    
    /// Consecutive successes
    pub consecutive_successes: u32,
    
    /// Consecutive failures
    pub consecutive_failures: u32,
}