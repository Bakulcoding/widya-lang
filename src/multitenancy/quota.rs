use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::multitenancy::{MultiTenancyError, Result};

/// Resource quotas for tenants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quota {
    /// Tenant ID this quota belongs to
    pub tenant_id: Uuid,
    
    /// CPU quota in CPU cores (can be fractional)
    pub cpu_cores: f64,
    
    /// Current CPU usage
    pub cpu_usage: f64,
    
    /// Memory quota in bytes
    pub memory_bytes: u64,
    
    /// Current memory usage
    pub memory_usage: u64,
    
    /// Storage quota in bytes
    pub storage_bytes: u64,
    
    /// Current storage usage
    pub storage_usage: u64,
    
    /// Network bandwidth quota in bytes per second
    pub network_bandwidth_bps: u64,
    
    /// Current network usage (bytes transferred in current period)
    pub network_usage: u64,
    
    /// Maximum number of database connections
    pub max_connections: u32,
    
    /// Current number of connections
    pub current_connections: u32,
    
    /// Maximum number of concurrent requests
    pub max_concurrent_requests: u32,
    
    /// Current number of concurrent requests
    pub current_concurrent_requests: u32,
    
    /// When the quota period started
    pub period_start: chrono::DateTime<chrono::Utc>,
    
    /// When the quota period ends
    pub period_end: chrono::DateTime<chrono::Utc>,
    
    /// Whether to reset usage at end of period
    pub reset_on_period_end: bool,
}

impl Quota {
    /// Create a new quota for a tenant
    pub fn new(tenant_id: Uuid, tier: crate::multitenancy::tenant::BillingTier) -> Self {
        let now = chrono::Utc::now();
        let period_end = now + chrono::Duration::days(30); // Monthly quotas by default
        
        let (cpu_cores, memory_bytes, storage_bytes, network_bandwidth_bps, max_connections, max_concurrent_requests) = match tier {
            crate::multitenancy::tenant::BillingTier::Free => (0.1, 128 * 1024 * 1024, 1 * 1024 * 1024 * 1024, 10 * 1024 * 1024, 10, 100),
            crate::multitenancy::tenant::BillingTier::Basic => (1.0, 1 * 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024, 100 * 1024 * 1024, 50, 1000),
            crate::multitenancy::tenant::BillingTier::Professional => (4.0, 8 * 1024 * 1024 * 1024, 100 * 1024 * 1024 * 1024, 1 * 1024 * 1024 * 1024, 200, 10000),
            crate::multitenancy::tenant::BillingTier::Enterprise => (16.0, 64 * 1024 * 1024 * 1024, 1 * 1024 * 1024 * 1024 * 1024, 10 * 1024 * 1024 * 1024, 1000, 100000),
        };
        
        Self {
            tenant_id,
            cpu_cores,
            cpu_usage: 0.0,
            memory_bytes,
            memory_usage: 0,
            storage_bytes,
            storage_usage: 0,
            network_bandwidth_bps,
            network_usage: 0,
            max_connections,
            current_connections: 0,
            max_concurrent_requests,
            current_concurrent_requests: 0,
            period_start: now,
            period_end,
            reset_on_period_end: true,
        }
    }
    
    /// Check if quota has been exceeded
    pub fn is_exceeded(&self) -> bool {
        self.cpu_usage > self.cpu_cores ||
        self.memory_usage > self.memory_bytes ||
        self.storage_usage > self.storage_bytes ||
        self.current_connections > self.max_connections ||
        self.current_concurrent_requests > self.max_concurrent_requests
    }
    
    /// Check CPU quota
    pub fn check_cpu(&self, requested_cpu: f64) -> Result<()> {
        if self.cpu_usage + requested_cpu > self.cpu_cores {
            Err(MultiTenancyError::QuotaExceeded(
                format!("CPU quota exceeded. Available: {:.2}, Requested: {:.2}", 
                    self.cpu_cores - self.cpu_usage, requested_cpu)
            ))
        } else {
            Ok(())
        }
    }
    
    /// Check memory quota
    pub fn check_memory(&self, requested_memory: u64) -> Result<()> {
        if self.memory_usage + requested_memory > self.memory_bytes {
            Err(MultiTenancyError::QuotaExceeded(
                format!("Memory quota exceeded. Available: {}, Requested: {}", 
                    self.memory_bytes - self.memory_usage, requested_memory)
            ))
        } else {
            Ok(())
        }
    }
    
    /// Check storage quota
    pub fn check_storage(&self, requested_storage: u64) -> Result<()> {
        if self.storage_usage + requested_storage > self.storage_bytes {
            Err(MultiTenancyError::QuotaExceeded(
                format!("Storage quota exceeded. Available: {}, Requested: {}", 
                    self.storage_bytes - self.storage_usage, requested_storage)
            ))
        } else {
            Ok(())
        }
    }
    
    /// Check connections quota
    pub fn check_connection(&self) -> Result<()> {
        if self.current_connections >= self.max_connections {
            Err(MultiTenancyError::QuotaExceeded(
                format!("Connection quota exceeded. Max: {}, Current: {}", 
                    self.max_connections, self.current_connections)
            ))
        } else {
            Ok(())
        }
    }
    
    /// Check concurrent requests quota
    pub fn check_concurrent_request(&self) -> Result<()> {
        if self.current_concurrent_requests >= self.max_concurrent_requests {
            Err(MultiTenancyError::QuotaExceeded(
                format!("Concurrent request quota exceeded. Max: {}, Current: {}", 
                    self.max_concurrent_requests, self.current_concurrent_requests)
            ))
        } else {
            Ok(())
        }
    }
    
    /// Allocate CPU resources
    pub fn allocate_cpu(&mut self, cpu: f64) -> Result<()> {
        self.check_cpu(cpu)?;
        self.cpu_usage += cpu;
        Ok(())
    }
    
    /// Allocate memory resources
    pub fn allocate_memory(&mut self, memory: u64) -> Result<()> {
        self.check_memory(memory)?;
        self.memory_usage += memory;
        Ok(())
    }
    
    /// Allocate storage resources
    pub fn allocate_storage(&mut self, storage: u64) -> Result<()> {
        self.check_storage(storage)?;
        self.storage_usage += storage;
        Ok(())
    }
    
    /// Add a connection
    pub fn add_connection(&mut self) -> Result<()> {
        self.check_connection()?;
        self.current_connections += 1;
        Ok(())
    }
    
    /// Remove a connection
    pub fn remove_connection(&mut self) {
        if self.current_connections > 0 {
            self.current_connections -= 1;
        }
    }
    
    /// Add a concurrent request
    pub fn add_concurrent_request(&mut self) -> Result<()> {
        self.check_concurrent_request()?;
        self.current_concurrent_requests += 1;
        Ok(())
    }
    
    /// Remove a concurrent request
    pub fn remove_concurrent_request(&mut self) {
        if self.current_concurrent_requests > 0 {
            self.current_concurrent_requests -= 1;
        }
    }
    
    /// Get CPU usage percentage
    pub fn cpu_usage_percentage(&self) -> f64 {
        if self.cpu_cores == 0.0 {
            0.0
        } else {
            (self.cpu_usage / self.cpu_cores) * 100.0
        }
    }
    
    /// Get memory usage percentage
    pub fn memory_usage_percentage(&self) -> f64 {
        if self.memory_bytes == 0 {
            0.0
        } else {
            (self.memory_usage as f64 / self.memory_bytes as f64) * 100.0
        }
    }
    
    /// Get storage usage percentage
    pub fn storage_usage_percentage(&self) -> f64 {
        if self.storage_bytes == 0 {
            0.0
        } else {
            (self.storage_usage as f64 / self.storage_bytes as f64) * 100.0
        }
    }
    
    /// Check if period has ended
    pub fn is_period_ended(&self) -> bool {
        chrono::Utc::now() >= self.period_end
    }
    
    /// Reset usage if period has ended
    pub fn reset_if_period_ended(&mut self) -> bool {
        if self.is_period_ended() && self.reset_on_period_end {
            self.reset_usage();
            self.period_start = chrono::Utc::now();
            self.period_end = self.period_start + chrono::Duration::days(30);
            true
        } else {
            false
        }
    }
    
    /// Reset all usage counters
    pub fn reset_usage(&mut self) {
        self.cpu_usage = 0.0;
        self.memory_usage = 0;
        self.storage_usage = 0;
        self.network_usage = 0;
        self.current_connections = 0;
        self.current_concurrent_requests = 0;
    }
    
    /// Update quota limits
    pub fn update_limits(&mut self, limits: QuotaLimits) {
        if let Some(cpu_cores) = limits.cpu_cores {
            self.cpu_cores = cpu_cores;
        }
        if let Some(memory_bytes) = limits.memory_bytes {
            self.memory_bytes = memory_bytes;
        }
        if let Some(storage_bytes) = limits.storage_bytes {
            self.storage_bytes = storage_bytes;
        }
        if let Some(network_bandwidth_bps) = limits.network_bandwidth_bps {
            self.network_bandwidth_bps = network_bandwidth_bps;
        }
        if let Some(max_connections) = limits.max_connections {
            self.max_connections = max_connections;
        }
        if let Some(max_concurrent_requests) = limits.max_concurrent_requests {
            self.max_concurrent_requests = max_concurrent_requests;
        }
        if let Some(reset_on_period_end) = limits.reset_on_period_end {
            self.reset_on_period_end = reset_on_period_end;
        }
    }
}

/// Builder for quota limit updates
#[derive(Debug, Clone, Default)]
pub struct QuotaLimits {
    pub cpu_cores: Option<f64>,
    pub memory_bytes: Option<u64>,
    pub storage_bytes: Option<u64>,
    pub network_bandwidth_bps: Option<u64>,
    pub max_connections: Option<u32>,
    pub max_concurrent_requests: Option<u32>,
    pub reset_on_period_end: Option<bool>,
}

impl QuotaLimits {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn cpu_cores(mut self, cpu_cores: f64) -> Self {
        self.cpu_cores = Some(cpu_cores);
        self
    }
    
    pub fn memory_bytes(mut self, memory_bytes: u64) -> Self {
        self.memory_bytes = Some(memory_bytes);
        self
    }
    
    pub fn storage_bytes(mut self, storage_bytes: u64) -> Self {
        self.storage_bytes = Some(storage_bytes);
        self
    }
    
    pub fn network_bandwidth_bps(mut self, network_bandwidth_bps: u64) -> Self {
        self.network_bandwidth_bps = Some(network_bandwidth_bps);
        self
    }
    
    pub fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = Some(max_connections);
        self
    }
    
    pub fn max_concurrent_requests(mut self, max_concurrent_requests: u32) -> Self {
        self.max_concurrent_requests = Some(max_concurrent_requests);
        self
    }
    
    pub fn reset_on_period_end(mut self, reset_on_period_end: bool) -> Self {
        self.reset_on_period_end = Some(reset_on_period_end);
        self
    }
}

/// Manager for tenant quotas
#[derive(Clone)]
pub struct QuotaManager {
    quotas: Arc<RwLock<HashMap<Uuid, Quota>>>,
}

impl QuotaManager {
    /// Create a new quota manager
    pub fn new() -> Self {
        Self {
            quotas: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create or get quota for tenant
    pub fn get_or_create_quota(
        &self,
        tenant_id: Uuid,
        tier: crate::multitenancy::tenant::BillingTier,
    ) -> Quota {
        let mut quotas = self.quotas.write().unwrap();
        
        if let Some(quota) = quotas.get(&tenant_id) {
            // Check if period needs reset
            let mut quota = quota.clone();
            if quota.reset_if_period_ended() {
                quotas.insert(tenant_id, quota.clone());
            }
            quota
        } else {
            // Create new quota
            let quota = Quota::new(tenant_id, tier);
            quotas.insert(tenant_id, quota.clone());
            quota
        }
    }
    
    /// Get quota for tenant
    pub fn get_quota(&self, tenant_id: Uuid) -> Option<Quota> {
        let quotas = self.quotas.read().unwrap();
        quotas.get(&tenant_id).cloned()
    }
    
    /// Update quota for tenant
    pub fn update_quota(&self, tenant_id: Uuid, limits: QuotaLimits) -> Result<Quota> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.update_limits(limits);
        Ok(quota.clone())
    }
    
    /// Reset usage for tenant
    pub fn reset_usage(&self, tenant_id: Uuid) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.reset_usage();
        Ok(())
    }
    
    /// Allocate CPU for tenant
    pub fn allocate_cpu(&self, tenant_id: Uuid, cpu: f64) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.allocate_cpu(cpu)
    }
    
    /// Allocate memory for tenant
    pub fn allocate_memory(&self, tenant_id: Uuid, memory: u64) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.allocate_memory(memory)
    }
    
    /// Allocate storage for tenant
    pub fn allocate_storage(&self, tenant_id: Uuid, storage: u64) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.allocate_storage(storage)
    }
    
    /// Add connection for tenant
    pub fn add_connection(&self, tenant_id: Uuid) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.add_connection()
    }
    
    /// Remove connection for tenant
    pub fn remove_connection(&self, tenant_id: Uuid) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.remove_connection();
        Ok(())
    }
    
    /// Add concurrent request for tenant
    pub fn add_concurrent_request(&self, tenant_id: Uuid) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.add_concurrent_request()
    }
    
    /// Remove concurrent request for tenant
    pub fn remove_concurrent_request(&self, tenant_id: Uuid) -> Result<()> {
        let mut quotas = self.quotas.write().unwrap();
        let quota = quotas.get_mut(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        quota.remove_concurrent_request();
        Ok(())
    }
    
    /// Check if tenant has exceeded quota
    pub fn check_quota(&self, tenant_id: Uuid) -> Result<bool> {
        let quotas = self.quotas.read().unwrap();
        let quota = quotas.get(&tenant_id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(tenant_id.to_string()))?;
        
        Ok(quota.is_exceeded())
    }
    
    /// Get usage statistics for all tenants
    pub fn get_usage_stats(&self) -> HashMap<Uuid, QuotaUsageStats> {
        let quotas = self.quotas.read().unwrap();
        quotas.iter()
            .map(|(tenant_id, quota)| {
                (*tenant_id, QuotaUsageStats::from_quota(quota))
            })
            .collect()
    }
}

/// Usage statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaUsageStats {
    pub tenant_id: Uuid,
    pub cpu_usage_percentage: f64,
    pub memory_usage_percentage: f64,
    pub storage_usage_percentage: f64,
    pub connection_usage_percentage: f64,
    pub concurrent_request_usage_percentage: f64,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub is_exceeded: bool,
}

impl QuotaUsageStats {
    fn from_quota(quota: &Quota) -> Self {
        let connection_usage_percentage = if quota.max_connections == 0 {
            0.0
        } else {
            (quota.current_connections as f64 / quota.max_connections as f64) * 100.0
        };
        
        let concurrent_request_usage_percentage = if quota.max_concurrent_requests == 0 {
            0.0
        } else {
            (quota.current_concurrent_requests as f64 / quota.max_concurrent_requests as f64) * 100.0
        };
        
        Self {
            tenant_id: quota.tenant_id,
            cpu_usage_percentage: quota.cpu_usage_percentage(),
            memory_usage_percentage: quota.memory_usage_percentage(),
            storage_usage_percentage: quota.storage_usage_percentage(),
            connection_usage_percentage,
            concurrent_request_usage_percentage,
            period_start: quota.period_start,
            period_end: quota.period_end,
            is_exceeded: quota.is_exceeded(),
        }
    }
}