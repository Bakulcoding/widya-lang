use std::sync::Arc;
use serde_json::json;
use uuid::Uuid;

use crate::multitenancy::{
    TenantRegistry, Tenant, MultiTenancyError, Result,
    QuotaManager, QuotaLimits,
    DataPartitioner, PartitionStrategy,
};

/// HTTP API handler for multi-tenancy
pub struct MultiTenancyApi {
    registry: Arc<TenantRegistry>,
    quota_manager: Arc<QuotaManager>,
    partitioner: Arc<DataPartitioner>,
}

impl MultiTenancyApi {
    /// Create a new API handler
    pub fn new(
        registry: Arc<TenantRegistry>,
        quota_manager: Arc<QuotaManager>,
        partitioner: Arc<DataPartitioner>,
    ) -> Self {
        Self {
            registry,
            quota_manager,
            partitioner,
        }
    }
    
    /// Handle tenant creation request
    pub fn handle_create_tenant(
        &self,
        name: String,
        domain: String,
        tier: crate::multitenancy::tenant::BillingTier,
    ) -> Result<serde_json::Value> {
        // Create tenant
        let tenant = self.registry.create_tenant(name, domain, tier)?;
        
        // Create quota for tenant
        let quota = self.quota_manager.get_or_create_quota(tenant.id, tenant.tier);
        
        // Create partitions for tenant
        let partition_config = self.partitioner.create_partitions(
            tenant.id,
            PartitionStrategy::Hash,
            4, // Default: 4 partitions
        )?;
        
        Ok(json!({
            "success": true,
            "tenant": {
                "id": tenant.id,
                "name": tenant.name,
                "domain": tenant.domain,
                "status": format!("{:?}", tenant.status),
                "tier": format!("{:?}", tenant.tier),
                "created_at": tenant.created_at.to_rfc3339(),
            },
            "quota": {
                "cpu_cores": quota.cpu_cores,
                "memory_bytes": quota.memory_bytes,
                "storage_bytes": quota.storage_bytes,
                "max_connections": quota.max_connections,
            },
            "partitions": {
                "strategy": format!("{:?}", partition_config.strategy),
                "count": partition_config.partition_count,
            },
        }))
    }
    
    /// Handle tenant retrieval by ID
    pub fn handle_get_tenant_by_id(&self, id: Uuid) -> Result<serde_json::Value> {
        let tenant = self.registry.get_tenant_by_id(id)?;
        let quota = self.quota_manager.get_quota(id);
        let partition_stats = self.partitioner.get_partition_stats(id);
        
        let mut response = json!({
            "tenant": {
                "id": tenant.id,
                "name": tenant.name,
                "domain": tenant.domain,
                "status": format!("{:?}", tenant.status),
                "tier": format!("{:?}", tenant.tier),
                "created_at": tenant.created_at.to_rfc3339(),
                "updated_at": tenant.updated_at.to_rfc3339(),
                "is_active": tenant.is_active(),
                "metadata": tenant.metadata,
            },
        });
        
        if let Some(quota) = quota {
            response["quota"] = json!({
                "cpu_cores": quota.cpu_cores,
                "cpu_usage": quota.cpu_usage,
                "cpu_usage_percentage": quota.cpu_usage_percentage(),
                "memory_bytes": quota.memory_bytes,
                "memory_usage": quota.memory_usage,
                "memory_usage_percentage": quota.memory_usage_percentage(),
                "storage_bytes": quota.storage_bytes,
                "storage_usage": quota.storage_usage,
                "storage_usage_percentage": quota.storage_usage_percentage(),
                "max_connections": quota.max_connections,
                "current_connections": quota.current_connections,
                "max_concurrent_requests": quota.max_concurrent_requests,
                "current_concurrent_requests": quota.current_concurrent_requests,
                "is_exceeded": quota.is_exceeded(),
                "period_start": quota.period_start.to_rfc3339(),
                "period_end": quota.period_end.to_rfc3339(),
            });
        }
        
        if let Ok(stats) = partition_stats {
            response["partitions"] = json!({
                "total_partitions": stats.total_partitions,
                "active_partitions": stats.active_partitions,
                "total_size_bytes": stats.total_size_bytes,
                "total_row_count": stats.total_row_count,
                "average_partition_size": stats.average_partition_size,
            });
        }
        
        Ok(response)
    }
    
    /// Handle tenant retrieval by domain
    pub fn handle_get_tenant_by_domain(&self, domain: &str) -> Result<serde_json::Value> {
        let tenant = self.registry.get_tenant_by_domain(domain)?;
        self.handle_get_tenant_by_id(tenant.id)
    }
    
    /// Handle tenant update request
    pub fn handle_update_tenant(
        &self,
        id: Uuid,
        updates: crate::multitenancy::registry::TenantUpdates,
    ) -> Result<serde_json::Value> {
        let tenant = self.registry.update_tenant(id, updates)?;
        
        // Update quota if tier changed
        if updates.tier.is_some() {
            self.quota_manager.get_or_create_quota(id, tenant.tier);
        }
        
        Ok(json!({
            "success": true,
            "tenant": {
                "id": tenant.id,
                "name": tenant.name,
                "domain": tenant.domain,
                "status": format!("{:?}", tenant.status),
                "tier": format!("{:?}", tenant.tier),
                "updated_at": tenant.updated_at.to_rfc3339(),
            },
        }))
    }
    
    /// Handle tenant deletion request
    pub fn handle_delete_tenant(&self, id: Uuid) -> Result<serde_json::Value> {
        self.registry.delete_tenant(id)?;
        
        Ok(json!({
            "success": true,
            "message": format!("Tenant {} deleted successfully", id),
        }))
    }
    
    /// Handle quota update request
    pub fn handle_update_quota(
        &self,
        tenant_id: Uuid,
        limits: QuotaLimits,
    ) -> Result<serde_json::Value> {
        let quota = self.quota_manager.update_quota(tenant_id, limits)?;
        
        Ok(json!({
            "success": true,
            "quota": {
                "tenant_id": quota.tenant_id,
                "cpu_cores": quota.cpu_cores,
                "memory_bytes": quota.memory_bytes,
                "storage_bytes": quota.storage_bytes,
                "network_bandwidth_bps": quota.network_bandwidth_bps,
                "max_connections": quota.max_connections,
                "max_concurrent_requests": quota.max_concurrent_requests,
            },
        }))
    }
    
    /// Handle quota usage reset
    pub fn handle_reset_quota_usage(&self, tenant_id: Uuid) -> Result<serde_json::Value> {
        self.quota_manager.reset_usage(tenant_id)?;
        
        Ok(json!({
            "success": true,
            "message": format!("Quota usage reset for tenant {}", tenant_id),
        }))
    }
    
    /// Handle list tenants request
    pub fn handle_list_tenants(&self, active_only: bool) -> Result<serde_json::Value> {
        let tenants = if active_only {
            self.registry.list_active_tenants()
        } else {
            self.registry.list_tenants()
        };
        
        let tenant_list: Vec<serde_json::Value> = tenants.iter()
            .map(|tenant| {
                json!({
                    "id": tenant.id,
                    "name": tenant.name,
                    "domain": tenant.domain,
                    "status": format!("{:?}", tenant.status),
                    "tier": format!("{:?}", tenant.tier),
                    "created_at": tenant.created_at.to_rfc3339(),
                    "is_active": tenant.is_active(),
                })
            })
            .collect();
        
        Ok(json!({
            "success": true,
            "count": tenant_list.len(),
            "tenants": tenant_list,
        }))
    }
    
    /// Handle usage statistics request
    pub fn handle_get_usage_stats(&self) -> Result<serde_json::Value> {
        let stats = self.quota_manager.get_usage_stats();
        
        let stats_list: Vec<serde_json::Value> = stats.iter()
            .map(|(tenant_id, stat)| {
                json!({
                    "tenant_id": tenant_id,
                    "cpu_usage_percentage": stat.cpu_usage_percentage,
                    "memory_usage_percentage": stat.memory_usage_percentage,
                    "storage_usage_percentage": stat.storage_usage_percentage,
                    "connection_usage_percentage": stat.connection_usage_percentage,
                    "concurrent_request_usage_percentage": stat.concurrent_request_usage_percentage,
                    "period_start": stat.period_start.to_rfc3339(),
                    "period_end": stat.period_end.to_rfc3339(),
                    "is_exceeded": stat.is_exceeded,
                })
            })
            .collect();
        
        Ok(json!({
            "success": true,
            "count": stats_list.len(),
            "stats": stats_list,
        }))
    }
    
    /// Handle RLS policy creation
    pub fn handle_create_rls_policy(
        &self,
        name: String,
        table: String,
        expression: String,
        tenant_ids: Vec<Uuid>,
    ) -> Result<serde_json::Value> {
        let policy = self.partitioner.create_rls_policy(name, table, expression, tenant_ids)?;
        
        Ok(json!({
            "success": true,
            "policy": {
                "name": policy.name,
                "table": policy.table,
                "expression": policy.expression,
                "tenant_ids": policy.tenant_ids,
                "active": policy.active,
            },
        }))
    }
    
    /// Handle partition statistics request
    pub fn handle_get_partition_stats(&self, tenant_id: Uuid) -> Result<serde_json::Value> {
        let stats = self.partitioner.get_partition_stats(tenant_id)?;
        
        Ok(json!({
            "success": true,
            "stats": {
                "tenant_id": stats.tenant_id,
                "total_partitions": stats.total_partitions,
                "active_partitions": stats.active_partitions,
                "total_size_bytes": stats.total_size_bytes,
                "total_row_count": stats.total_row_count,
                "average_partition_size": stats.average_partition_size,
            },
        }))
    }
    
    /// Handle health check
    pub fn handle_health_check(&self) -> serde_json::Value {
        let tenant_count = self.registry.count_tenants();
        let active_tenant_count = self.registry.count_active_tenants();
        
        json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "metrics": {
                "total_tenants": tenant_count,
                "active_tenants": active_tenant_count,
                "system_uptime": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            },
        })
    }
}

/// Create a default multi-tenancy API instance
pub fn create_default_api() -> MultiTenancyApi {
    let registry = Arc::new(TenantRegistry::new());
    let quota_manager = Arc::new(QuotaManager::new());
    let partitioner = Arc::new(DataPartitioner::new());
    
    MultiTenancyApi::new(registry, quota_manager, partitioner)
}