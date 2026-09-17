use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a tenant in the multi-tenant system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// Unique identifier for the tenant
    pub id: Uuid,
    
    /// Human-readable name for the tenant
    pub name: String,
    
    /// Tenant domain or subdomain (e.g., company.widya.app)
    pub domain: String,
    
    /// Tenant status
    pub status: TenantStatus,
    
    /// Billing tier for the tenant
    pub tier: BillingTier,
    
    /// Metadata key-value pairs for tenant-specific configuration
    pub metadata: HashMap<String, String>,
    
    /// When the tenant was created
    pub created_at: DateTime<Utc>,
    
    /// When the tenant was last updated
    pub updated_at: DateTime<Utc>,
    
    /// Optional date when tenant was deactivated
    pub deactivated_at: Option<DateTime<Utc>>,
}

/// Tenant status enum
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TenantStatus {
    /// Tenant is active and can use the system
    Active,
    
    /// Tenant is pending activation (e.g., after signup)
    Pending,
    
    /// Tenant is suspended (temporarily disabled)
    Suspended,
    
    /// Tenant is deactivated (permanently disabled)
    Deactivated,
}

/// Billing tier for tenants
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BillingTier {
    /// Free tier with basic features and limits
    Free,
    
    /// Basic tier with moderate limits
    Basic,
    
    /// Professional tier with high limits
    Professional,
    
    /// Enterprise tier with custom limits
    Enterprise,
}

impl Tenant {
    /// Create a new tenant with default metadata
    pub fn new(name: String, domain: String, tier: BillingTier) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            name,
            domain,
            status: TenantStatus::Pending,
            tier,
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
            deactivated_at: None,
        }
    }
    
    /// Activate the tenant
    pub fn activate(&mut self) -> Result<(), String> {
        if self.status == TenantStatus::Deactivated {
            return Err("Cannot activate a deactivated tenant".to_string());
        }
        
        self.status = TenantStatus::Active;
        self.updated_at = Utc::now();
        Ok(())
    }
    
    /// Suspend the tenant
    pub fn suspend(&mut self) {
        self.status = TenantStatus::Suspended;
        self.updated_at = Utc::now();
    }
    
    /// Deactivate the tenant
    pub fn deactivate(&mut self) {
        self.status = TenantStatus::Deactivated;
        self.deactivated_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Check if tenant is active
    pub fn is_active(&self) -> bool {
        self.status == TenantStatus::Active
    }
    
    /// Check if tenant can use resources
    pub fn can_use_resources(&self) -> bool {
        match self.status {
            TenantStatus::Active | TenantStatus::Pending => true,
            TenantStatus::Suspended | TenantStatus::Deactivated => false,
        }
    }
    
    /// Add metadata to the tenant
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
        self.updated_at = Utc::now();
    }
    
    /// Remove metadata from the tenant
    pub fn remove_metadata(&mut self, key: &str) -> Option<String> {
        self.updated_at = Utc::now();
        self.metadata.remove(key)
    }
    
    /// Get metadata value
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }
}

impl Default for Tenant {
    fn default() -> Self {
        Self::new(
            "Default Tenant".to_string(),
            "default.widya.app".to_string(),
            BillingTier::Free,
        )
    }
}

/// Validation rules for tenants
pub struct TenantValidator;

impl TenantValidator {
    /// Validate tenant name
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            return Err("Tenant name cannot be empty".to_string());
        }
        
        if name.len() > 100 {
            return Err("Tenant name cannot exceed 100 characters".to_string());
        }
        
        // Check for allowed characters
        if !name.chars().all(|c| c.is_alphanumeric() || c == ' ' || c == '-' || c == '_') {
            return Err("Tenant name can only contain alphanumeric characters, spaces, hyphens, and underscores".to_string());
        }
        
        Ok(())
    }
    
    /// Validate tenant domain
    pub fn validate_domain(domain: &str) -> Result<(), String> {
        if domain.is_empty() {
            return Err("Domain cannot be empty".to_string());
        }
        
        if domain.len() > 253 {
            return Err("Domain cannot exceed 253 characters".to_string());
        }
        
        // Basic domain format validation
        if !domain.contains('.') {
            return Err("Domain must contain at least one dot".to_string());
        }
        
        // Check each label
        for label in domain.split('.') {
            if label.is_empty() {
                return Err("Domain labels cannot be empty".to_string());
            }
            
            if label.len() > 63 {
                return Err("Domain labels cannot exceed 63 characters".to_string());
            }
            
            // Labels must start and end with alphanumeric
            if !label.chars().next().unwrap().is_alphanumeric() 
                || !label.chars().last().unwrap().is_alphanumeric() {
                return Err("Domain labels must start and end with alphanumeric characters".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Validate entire tenant
    pub fn validate_tenant(tenant: &Tenant) -> Result<(), String> {
        Self::validate_name(&tenant.name)?;
        Self::validate_domain(&tenant.domain)?;
        
        // Ensure domain doesn't conflict with reserved domains
        let reserved_domains = [
            "admin.widya.app",
            "app.widya.app",
            "api.widya.app",
            "docs.widya.app",
        ];
        
        if reserved_domains.contains(&tenant.domain.as_str()) {
            return Err(format!("Domain {} is reserved", tenant.domain));
        }
        
        Ok(())
    }
}