//! Test module for multi-tenancy features
//! Demonstrates the core functionality of the multi-tenancy system

use std::sync::Arc;
use uuid::Uuid;
use crate::multitenancy::{
    TenantRegistry, QuotaManager, DataPartitioner, MultiTenancyApi,
    tenant::{Tenant, BillingTier, TenantStatus},
    registry::TenantUpdates,
    quota::QuotaLimits,
    partition::PartitionStrategy,
};

/// Test the complete multi-tenancy workflow
pub fn test_multi_tenancy_workflow() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing Multi-tenancy Workflow...");
    
    // Create instances
    let registry = Arc::new(TenantRegistry::new());
    let quota_manager = Arc::new(QuotaManager::new());
    let partitioner = Arc::new(DataPartitioner::new());
    let api = MultiTenancyApi::new(registry.clone(), quota_manager.clone(), partitioner.clone());
    
    // Test 1: Create tenants
    println!("  Test 1: Creating tenants...");
    
    let tenant1 = registry.create_tenant(
        "Acme Corp".to_string(),
        "acme.widya.app".to_string(),
        BillingTier::Professional,
    )?;
    
    let tenant2 = registry.create_tenant(
        "Beta Inc".to_string(),
        "beta.widya.app".to_string(),
        BillingTier::Basic,
    )?;
    
    println!("    ✅ Created tenants: {} and {}", tenant1.name, tenant2.name);
    
    // Test 2: Get tenants by different methods
    println!("  Test 2: Retrieving tenants...");
    
    let retrieved_by_id = registry.get_tenant_by_id(tenant1.id)?;
    let retrieved_by_domain = registry.get_tenant_by_domain("beta.widya.app")?;
    let retrieved_by_name = registry.get_tenant_by_name("Acme Corp")?;
    
    assert_eq!(retrieved_by_id.id, tenant1.id);
    assert_eq!(retrieved_by_domain.id, tenant2.id);
    assert_eq!(retrieved_by_name.id, tenant1.id);
    
    println!("    ✅ Tenant retrieval works correctly");
    
    // Test 3: Update tenant
    println!("  Test 3: Updating tenant...");
    
    let updates = TenantUpdates::new()
        .status(TenantStatus::Active)
        .metadata([("contact_email".to_string(), "admin@acme.com".to_string())].into());
    
    let updated_tenant = registry.update_tenant(tenant1.id, updates)?;
    assert_eq!(updated_tenant.status, TenantStatus::Active);
    assert_eq!(updated_tenant.get_metadata("contact_email"), Some(&"admin@acme.com".to_string()));
    
    println!("    ✅ Tenant update works correctly");
    
    // Test 4: Quota management
    println!("  Test 4: Managing quotas...");
    
    let quota1 = quota_manager.get_or_create_quota(tenant1.id, tenant1.tier);
    let quota2 = quota_manager.get_or_create_quota(tenant2.id, tenant2.tier);
    
    // Professional tier should have more resources than Basic
    assert!(quota1.cpu_cores > quota2.cpu_cores);
    assert!(quota1.memory_bytes > quota2.memory_bytes);
    
    println!("    ✅ Quota creation works correctly");
    
    // Test 5: Allocate resources
    println!("  Test 5: Allocating resources...");
    
    quota_manager.allocate_memory(tenant1.id, 100 * 1024 * 1024)?; // 100MB
    quota_manager.add_connection(tenant1.id)?;
    quota_manager.add_concurrent_request(tenant1.id)?;
    
    let updated_quota = quota_manager.get_quota(tenant1.id).unwrap();
    assert!(updated_quota.memory_usage > 0);
    assert_eq!(updated_quota.current_connections, 1);
    assert_eq!(updated_quota.current_concurrent_requests, 1);
    
    println!("    ✅ Resource allocation works correctly");
    
    // Test 6: Quota limits
    println!("  Test 6: Testing quota limits...");
    
    // Try to exceed quota (should fail)
    let large_memory = quota2.memory_bytes + 1;
    let result = quota_manager.allocate_memory(tenant2.id, large_memory);
    assert!(result.is_err());
    
    println!("    ✅ Quota limits enforced correctly");
    
    // Test 7: Data partitioning
    println!("  Test 7: Data partitioning...");
    
    let partition_config = partitioner.create_partitions(
        tenant1.id,
        PartitionStrategy::Hash,
        4,
    )?;
    
    assert_eq!(partition_config.partition_count, 4);
    assert_eq!(partition_config.assignments.len(), 4);
    
    let partition = partitioner.get_partition_for_insert(tenant1.id, "user_123")?;
    assert!(partition.active);
    
    println!("    ✅ Data partitioning works correctly");
    
    // Test 8: Encryption
    println!("  Test 8: Data encryption...");
    
    let plaintext = b"secret tenant data";
    let encrypted = partitioner.encrypt_data(tenant1.id, plaintext)?;
    let decrypted = partitioner.decrypt_data(tenant1.id, &encrypted)?;
    
    assert_ne!(plaintext, &encrypted[..]);
    assert_eq!(plaintext, &decrypted[..]);
    
    println!("    ✅ Encryption/decryption works correctly");
    
    // Test 9: RLS policies
    println!("  Test 9: Row-level security...");
    
    let policy = partitioner.create_rls_policy(
        "tenant_data_policy".to_string(),
        "users".to_string(),
        "tenant_id = $1".to_string(),
        vec![tenant1.id],
    )?;
    
    let policies = partitioner.get_rls_policies_for_tenant(tenant1.id);
    assert!(!policies.is_empty());
    assert_eq!(policies[0].name, policy.name);
    
    println!("    ✅ RLS policies work correctly");
    
    // Test 10: API layer
    println!("  Test 10: API layer...");
    
    let api_response = api.handle_get_tenant_by_id(tenant1.id)?;
    assert!(api_response["success"].as_bool().unwrap());
    assert_eq!(api_response["tenant"]["name"].as_str().unwrap(), "Acme Corp");
    
    let list_response = api.handle_list_tenants(false)?;
    assert!(list_response["success"].as_bool().unwrap());
    assert_eq!(list_response["count"].as_u64().unwrap(), 2);
    
    println!("    ✅ API layer works correctly");
    
    // Test 11: Cleanup
    println!("  Test 11: Cleanup...");
    
    registry.delete_tenant(tenant1.id)?;
    registry.delete_tenant(tenant2.id)?;
    
    assert!(registry.get_tenant_by_id(tenant1.id).is_err());
    assert!(registry.get_tenant_by_id(tenant2.id).is_err());
    
    println!("    ✅ Cleanup works correctly");
    
    println!("🎉 All tests passed!");
    Ok(())
}

/// Demonstrate the multi-tenancy system
pub fn demonstrate_multi_tenancy() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Demonstrating Widya Multi-tenancy System");
    println!("===========================================\n");
    
    // Create API instance
    let api = crate::multitenancy::create_default_api();
    
    // Create some example tenants
    println!("1. Creating example tenants...");
    
    let tenant1_result = api.handle_create_tenant(
        "Tech Startup Inc".to_string(),
        "techstartup.widya.app".to_string(),
        BillingTier::Professional,
    )?;
    
    let tenant2_result = api.handle_create_tenant(
        "Local Business Ltd".to_string(),
        "localbusiness.widya.app".to_string(),
        BillingTier::Basic,
    )?;
    
    let tenant1_id: Uuid = serde_json::from_str(&format!("\"{}\"", tenant1_result["tenant"]["id"].as_str().unwrap()))?;
    let tenant2_id: Uuid = serde_json::from_str(&format!("\"{}\"", tenant2_result["tenant"]["id"].as_str().unwrap()))?;
    
    println!("   ✅ Created Tech Startup Inc (Professional Tier)");
    println!("   ✅ Created Local Business Ltd (Basic Tier)");
    
    // Show tenant details
    println!("\n2. Tenant details...");
    
    let tenant1_details = api.handle_get_tenant_by_id(tenant1_id)?;
    let tenant2_details = api.handle_get_tenant_by_id(tenant2_id)?;
    
    println!("   Tech Startup Inc:");
    println!("     - Domain: {}", tenant1_details["tenant"]["domain"].as_str().unwrap());
    println!("     - Tier: {}", tenant1_details["tenant"]["tier"].as_str().unwrap());
    println!("     - Status: {}", tenant1_details["tenant"]["status"].as_str().unwrap());
    println!("     - CPU Quota: {:.1} cores", tenant1_details["quota"]["cpu_cores"].as_f64().unwrap());
    println!("     - Memory Quota: {} GB", tenant1_details["quota"]["memory_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    
    println!("\n   Local Business Ltd:");
    println!("     - Domain: {}", tenant2_details["tenant"]["domain"].as_str().unwrap());
    println!("     - Tier: {}", tenant2_details["tenant"]["tier"].as_str().unwrap());
    println!("     - Status: {}", tenant2_details["tenant"]["status"].as_str().unwrap());
    println!("     - CPU Quota: {:.1} cores", tenant2_details["quota"]["cpu_cores"].as_f64().unwrap());
    println!("     - Memory Quota: {} MB", tenant2_details["quota"]["memory_bytes"].as_u64().unwrap() / 1024 / 1024);
    
    // Demonstrate quota usage
    println!("\n3. Simulating resource usage...");
    
    // Allocate some resources
    api.handle_update_quota(
        tenant1_id,
        QuotaLimits::new()
            .cpu_cores(2.0)
            .memory_bytes(2 * 1024 * 1024 * 1024), // 2GB
    )?;
    
    println!("   ✅ Updated Tech Startup quota to 2 CPU cores and 2GB memory");
    
    // List all tenants
    println!("\n4. Listing all tenants...");
    
    let all_tenants = api.handle_list_tenants(false)?;
    println!("   Total tenants: {}", all_tenants["count"].as_u64().unwrap());
    
    for tenant in all_tenants["tenants"].as_array().unwrap() {
        println!("   - {} ({}) - {}", 
            tenant["name"].as_str().unwrap(),
            tenant["domain"].as_str().unwrap(),
            tenant["status"].as_str().unwrap());
    }
    
    // Show usage statistics
    println!("\n5. System health check...");
    
    let health = api.handle_health_check();
    println!("   Status: {}", health["status"].as_str().unwrap());
    println!("   Total tenants: {}", health["metrics"]["total_tenants"].as_u64().unwrap());
    println!("   Active tenants: {}", health["metrics"]["active_tenants"].as_u64().unwrap());
    
    println!("\n🎯 Demonstration complete!");
    println!("The multi-tenancy system provides:");
    println!("   • Tenant isolation & resource quotas");
    println!("   • Data partitioning & encryption");
    println!("   • Row-level security policies");
    println!("   • RESTful API for management");
    println!("   • Usage tracking & reporting");
    
    Ok(())
}

/// Run all tests
pub fn run_all_tests() {
    println!("Running Multi-tenancy Tests");
    println!("===========================\n");
    
    match test_multi_tenancy_workflow() {
        Ok(_) => println!("\n✅ All tests passed successfully!"),
        Err(e) => println!("\n❌ Test failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tenant_creation() {
        let registry = TenantRegistry::new();
        
        let tenant = registry.create_tenant(
            "Test Tenant".to_string(),
            "test.widya.app".to_string(),
            BillingTier::Free,
        ).unwrap();
        
        assert_eq!(tenant.name, "Test Tenant");
        assert_eq!(tenant.domain, "test.widya.app");
        assert_eq!(tenant.tier, BillingTier::Free);
    }
    
    #[test]
    fn test_quota_management() {
        let quota_manager = QuotaManager::new();
        let tenant_id = Uuid::new_v4();
        
        let quota = quota_manager.get_or_create_quota(tenant_id, BillingTier::Professional);
        
        assert!(quota.cpu_cores > 0.0);
        assert!(quota.memory_bytes > 0);
        assert!(quota.storage_bytes > 0);
    }
    
    #[test]
    fn test_data_partitioning() {
        let partitioner = DataPartitioner::new();
        let tenant_id = Uuid::new_v4();
        
        let config = partitioner.create_partitions(
            tenant_id,
            PartitionStrategy::Hash,
            4,
        ).unwrap();
        
        assert_eq!(config.partition_count, 4);
        assert_eq!(config.assignments.len(), 4);
    }
}