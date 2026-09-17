use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::multitenancy::{Tenant, MultiTenancyError, Result};

/// Tenant registry for storing and managing tenants
#[derive(Clone)]
pub struct TenantRegistry {
    /// In-memory storage of tenants
    tenants: Arc<RwLock<HashMap<Uuid, Tenant>>>,
    
    /// Index for domain to tenant ID lookup
    domain_index: Arc<RwLock<HashMap<String, Uuid>>>,
    
    /// Index for name to tenant ID lookup (case-insensitive)
    name_index: Arc<RwLock<HashMap<String, Uuid>>>,
}

impl TenantRegistry {
    /// Create a new tenant registry
    pub fn new() -> Self {
        Self {
            tenants: Arc::new(RwLock::new(HashMap::new())),
            domain_index: Arc::new(RwLock::new(HashMap::new())),
            name_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Create a new tenant and add it to the registry
    pub fn create_tenant(
        &self,
        name: String,
        domain: String,
        tier: crate::multitenancy::tenant::BillingTier,
    ) -> Result<Tenant> {
        // Validate inputs
        crate::multitenancy::tenant::TenantValidator::validate_name(&name)
            .map_err(|e| MultiTenancyError::ValidationError(e))?;
        crate::multitenancy::tenant::TenantValidator::validate_domain(&domain)
            .map_err(|e| MultiTenancyError::ValidationError(e))?;
        
        // Check for duplicate domain
        let domain_lower = domain.to_lowercase();
        if self.domain_index.read().unwrap().contains_key(&domain_lower) {
            return Err(MultiTenancyError::ValidationError(
                format!("Domain {} is already registered", domain)
            ));
        }
        
        // Check for duplicate name (case-insensitive)
        let name_lower = name.to_lowercase();
        if self.name_index.read().unwrap().contains_key(&name_lower) {
            return Err(MultiTenancyError::ValidationError(
                format!("Tenant name {} is already taken", name)
            ));
        }
        
        // Create tenant
        let mut tenant = Tenant::new(name.clone(), domain.clone(), tier);
        crate::multitenancy::tenant::TenantValidator::validate_tenant(&tenant)
            .map_err(|e| MultiTenancyError::ValidationError(e))?;
        
        let tenant_id = tenant.id;
        
        // Add to registry
        {
            let mut tenants = self.tenants.write().unwrap();
            tenants.insert(tenant_id, tenant.clone());
        }
        
        // Update indices
        {
            let mut domain_index = self.domain_index.write().unwrap();
            domain_index.insert(domain_lower, tenant_id);
        }
        
        {
            let mut name_index = self.name_index.write().unwrap();
            name_index.insert(name_lower, tenant_id);
        }
        
        Ok(tenant)
    }
    
    /// Get tenant by ID
    pub fn get_tenant_by_id(&self, id: Uuid) -> Result<Tenant> {
        let tenants = self.tenants.read().unwrap();
        tenants.get(&id)
            .cloned()
            .ok_or_else(|| MultiTenancyError::TenantNotFound(id.to_string()))
    }
    
    /// Get tenant by domain
    pub fn get_tenant_by_domain(&self, domain: &str) -> Result<Tenant> {
        let domain_lower = domain.to_lowercase();
        let domain_index = self.domain_index.read().unwrap();
        let id = domain_index.get(&domain_lower)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(domain.to_string()))?;
        
        self.get_tenant_by_id(*id)
    }
    
    /// Get tenant by name (case-insensitive)
    pub fn get_tenant_by_name(&self, name: &str) -> Result<Tenant> {
        let name_lower = name.to_lowercase();
        let name_index = self.name_index.read().unwrap();
        let id = name_index.get(&name_lower)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(name.to_string()))?;
        
        self.get_tenant_by_id(*id)
    }
    
    /// Update an existing tenant
    pub fn update_tenant(&self, id: Uuid, updates: TenantUpdates) -> Result<Tenant> {
        let mut tenants = self.tenants.write().unwrap();
        let tenant = tenants.get_mut(&id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(id.to_string()))?;
        
        // Handle name change if needed
        if let Some(new_name) = updates.name {
            let name_lower = tenant.name.to_lowercase();
            let new_name_lower = new_name.to_lowercase();
            
            if name_lower != new_name_lower {
                // Check if new name is available
                if self.name_index.read().unwrap().contains_key(&new_name_lower) {
                    return Err(MultiTenancyError::ValidationError(
                        format!("Tenant name {} is already taken", new_name)
                    ));
                }
                
                // Remove old name index
                {
                    let mut name_index = self.name_index.write().unwrap();
                    name_index.remove(&name_lower);
                }
                
                // Update tenant name
                tenant.name = new_name.clone();
                
                // Add new name index
                {
                    let mut name_index = self.name_index.write().unwrap();
                    name_index.insert(new_name_lower, id);
                }
            }
        }
        
        // Handle domain change if needed
        if let Some(new_domain) = updates.domain {
            let domain_lower = tenant.domain.to_lowercase();
            let new_domain_lower = new_domain.to_lowercase();
            
            if domain_lower != new_domain_lower {
                // Check if new domain is available
                if self.domain_index.read().unwrap().contains_key(&new_domain_lower) {
                    return Err(MultiTenancyError::ValidationError(
                        format!("Domain {} is already registered", new_domain)
                    ));
                }
                
                // Validate new domain
                crate::multitenancy::tenant::TenantValidator::validate_domain(&new_domain)
                    .map_err(|e| MultiTenancyError::ValidationError(e))?;
                
                // Remove old domain index
                {
                    let mut domain_index = self.domain_index.write().unwrap();
                    domain_index.remove(&domain_lower);
                }
                
                // Update tenant domain
                tenant.domain = new_domain.clone();
                
                // Add new domain index
                {
                    let mut domain_index = self.domain_index.write().unwrap();
                    domain_index.insert(new_domain_lower, id);
                }
            }
        }
        
        // Handle tier change if needed
        if let Some(tier) = updates.tier {
            tenant.tier = tier;
        }
        
        // Handle status change if needed
        if let Some(status) = updates.status {
            match status {
                crate::multitenancy::tenant::TenantStatus::Active => {
                    tenant.activate()
                        .map_err(|e| MultiTenancyError::ValidationError(e))?;
                }
                crate::multitenancy::tenant::TenantStatus::Suspended => {
                    tenant.suspend();
                }
                crate::multitenancy::tenant::TenantStatus::Deactivated => {
                    tenant.deactivate();
                }
                _ => {
                    tenant.status = status;
                }
            }
        }
        
        // Update metadata
        if let Some(metadata) = updates.metadata {
            tenant.metadata = metadata;
        }
        
        tenant.updated_at = chrono::Utc::now();
        
        Ok(tenant.clone())
    }
    
    /// Delete a tenant
    pub fn delete_tenant(&self, id: Uuid) -> Result<()> {
        let mut tenants = self.tenants.write().unwrap();
        let tenant = tenants.remove(&id)
            .ok_or_else(|| MultiTenancyError::TenantNotFound(id.to_string()))?;
        
        // Remove from indices
        {
            let mut domain_index = self.domain_index.write().unwrap();
            domain_index.remove(&tenant.domain.to_lowercase());
        }
        
        {
            let mut name_index = self.name_index.write().unwrap();
            name_index.remove(&tenant.name.to_lowercase());
        }
        
        Ok(())
    }
    
    /// List all tenants
    pub fn list_tenants(&self) -> Vec<Tenant> {
        let tenants = self.tenants.read().unwrap();
        tenants.values().cloned().collect()
    }
    
    /// List active tenants
    pub fn list_active_tenants(&self) -> Vec<Tenant> {
        let tenants = self.tenants.read().unwrap();
        tenants.values()
            .filter(|t| t.is_active())
            .cloned()
            .collect()
    }
    
    /// Search tenants by metadata
    pub fn search_by_metadata(&self, key: &str, value: &str) -> Vec<Tenant> {
        let tenants = self.tenants.read().unwrap();
        tenants.values()
            .filter(|t| t.get_metadata(key) == Some(&value.to_string()))
            .cloned()
            .collect()
    }
    
    /// Count total tenants
    pub fn count_tenants(&self) -> usize {
        self.tenants.read().unwrap().len()
    }
    
    /// Count active tenants
    pub fn count_active_tenants(&self) -> usize {
        self.tenants.read().unwrap().values()
            .filter(|t| t.is_active())
            .count()
    }
}

/// Struct for updating tenant properties
pub struct TenantUpdates {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub tier: Option<crate::multitenancy::tenant::BillingTier>,
    pub status: Option<crate::multitenancy::tenant::TenantStatus>,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl Default for TenantUpdates {
    fn default() -> Self {
        Self {
            name: None,
            domain: None,
            tier: None,
            status: None,
            metadata: None,
        }
    }
}

impl TenantUpdates {
    /// Create a new TenantUpdates builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set name
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    /// Set domain
    pub fn domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }
    
    /// Set tier
    pub fn tier(mut self, tier: crate::multitenancy::tenant::BillingTier) -> Self {
        self.tier = Some(tier);
        self
    }
    
    /// Set status
    pub fn status(mut self, status: crate::multitenancy::tenant::TenantStatus) -> Self {
        self.status = Some(status);
        self
    }
    
    /// Set metadata
    pub fn metadata(mut self, metadata: std::collections::HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}