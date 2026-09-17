//! Multi-tenancy module for Widya Enterprise Edition
//! Provides tenant isolation, resource quotas, and data partitioning for SaaS applications

pub mod tenant;
pub mod registry;
pub mod quota;
pub mod partition;
pub mod api;

#[cfg(test)]
pub mod test;

// Re-export commonly used types
pub use tenant::Tenant;
pub use registry::TenantRegistry;
pub use quota::{Quota, QuotaManager};
pub use partition::DataPartitioner;
pub use api::{MultiTenancyApi, create_default_api};

#[cfg(test)]
pub use test::{test_multi_tenancy_workflow, demonstrate_multi_tenancy, run_all_tests};

/// Multi-tenancy errors
#[derive(Debug, thiserror::Error)]
pub enum MultiTenancyError {
    #[error("Tenant not found: {0}")]
    TenantNotFound(String),
    
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
    
    #[error("Invalid tenant configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Tenant validation failed: {0}")]
    ValidationError(String),
    
    #[error("Resource allocation failed: {0}")]
    ResourceAllocationError(String),
    
    #[error("Data partitioning error: {0}")]
    PartitioningError(String),
    
    #[error("Registry operation failed: {0}")]
    RegistryError(String),
}

/// Result type for multi-tenancy operations
pub type Result<T> = std::result::Result<T, MultiTenancyError>;