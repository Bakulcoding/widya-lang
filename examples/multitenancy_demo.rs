//! Example demonstrating Widya Enterprise Multi-tenancy features
//! This shows how to use the multi-tenancy system for SaaS applications

use std::sync::Arc;
use widya::multitenancy::{
    TenantRegistry, QuotaManager, DataPartitioner, MultiTenancyApi,
    tenant::{BillingTier, TenantStatus},
    registry::TenantUpdates,
    quota::QuotaLimits,
    partition::PartitionStrategy,
};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Widya Enterprise - Multi-tenancy Demo");
    println!("========================================\n");
    
    // Create the multi-tenancy system components
    let registry = Arc::new(TenantRegistry::new());
    let quota_manager = Arc::new(QuotaManager::new());
    let partitioner = Arc::new(DataPartitioner::new());
    let api = MultiTenancyApi::new(registry.clone(), quota_manager.clone(), partitioner.clone());
    
    // Demo 1: Create SaaS tenants
    println!("1. Creating SaaS Tenants:");
    println!("-------------------------");
    
    let acme_response = api.handle_create_tenant(
        "Acme Corporation".to_string(),
        "acme.saas.widya.app".to_string(),
        BillingTier::Enterprise,
    )?;
    
    let beta_response = api.handle_create_tenant(
        "Beta Solutions".to_string(),
        "beta.saas.widya.app".to_string(),
        BillingTier::Professional,
    )?;
    
    let gamma_response = api.handle_create_tenant(
        "Gamma Technologies".to_string(),
        "gamma.saas.widya.app".to_string(),
        BillingTier::Basic,
    )?;
    
    println!("   ✅ Acme Corporation (Enterprise Tier)");
    println!("   ✅ Beta Solutions (Professional Tier)");
    println!("   ✅ Gamma Technologies (Basic Tier)");
    
    // Demo 2: Show tenant isolation
    println!("\n2. Tenant Isolation & Quotas:");
    println!("------------------------------");
    
    let acme_id = serde_json::from_str(&format!("\"{}\"", acme_response["tenant"]["id"].as_str().unwrap()))?;
    let beta_id = serde_json::from_str(&format!("\"{}\"", beta_response["tenant"]["id"].as_str().unwrap()))?;
    let gamma_id = serde_json::from_str(&format!("\"{}\"", gamma_response["tenant"]["id"].as_str().unwrap()))?;
    
    // Activate tenants
    let acme_updates = TenantUpdates::new().status(TenantStatus::Active);
    let beta_updates = TenantUpdates::new().status(TenantStatus::Active);
    let gamma_updates = TenantUpdates::new().status(TenantStatus::Active);
    
    api.handle_update_tenant(acme_id, acme_updates)?;
    api.handle_update_tenant(beta_id, beta_updates)?;
    api.handle_update_tenant(gamma_id, gamma_updates)?;
    
    // Show quota differences based on tier
    let acme_details = api.handle_get_tenant_by_id(acme_id)?;
    let beta_details = api.handle_get_tenant_by_id(beta_id)?;
    let gamma_details = api.handle_get_tenant_by_id(gamma_id)?;
    
    println!("   Acme (Enterprise):");
    println!("     - CPU: {:.1} cores", acme_details["quota"]["cpu_cores"].as_f64().unwrap());
    println!("     - Memory: {} GB", acme_details["quota"]["memory_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    println!("     - Storage: {} GB", acme_details["quota"]["storage_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    
    println!("\n   Beta (Professional):");
    println!("     - CPU: {:.1} cores", beta_details["quota"]["cpu_cores"].as_f64().unwrap());
    println!("     - Memory: {} GB", beta_details["quota"]["memory_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    println!("     - Storage: {} GB", beta_details["quota"]["storage_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    
    println!("\n   Gamma (Basic):");
    println!("     - CPU: {:.1} cores", gamma_details["quota"]["cpu_cores"].as_f64().unwrap());
    println!("     - Memory: {} MB", gamma_details["quota"]["memory_bytes"].as_u64().unwrap() / 1024 / 1024);
    println!("     - Storage: {} GB", gamma_details["quota"]["storage_bytes"].as_u64().unwrap() / 1024 / 1024 / 1024);
    
    // Demo 3: Resource allocation simulation
    println!("\n3. Simulating Resource Usage:");
    println!("------------------------------");
    
    // Simulate Acme using resources
    println!("   Simulating Acme Corporation workload:");
    
    // Allocate resources
    quota_manager.allocate_cpu(acme_id, 1.5)?;
    quota_manager.allocate_memory(acme_id, 4 * 1024 * 1024 * 1024)?; // 4GB
    quota_manager.allocate_storage(acme_id, 50 * 1024 * 1024 * 1024)?; // 50GB
    
    for _ in 0..50 {
        quota_manager.add_connection(acme_id)?;
    }
    
    for _ in 0..500 {
        quota_manager.add_concurrent_request(acme_id)?;
    }
    
    let updated_acme_quota = quota_manager.get_quota(acme_id).unwrap();
    println!("     - CPU Usage: {:.1}/{:.1} cores ({:.1}%)", 
        updated_acme_quota.cpu_usage, 
        updated_acme_quota.cpu_cores,
        updated_acme_quota.cpu_usage_percentage());
    println!("     - Memory Usage: {}/{} GB ({:.1}%)", 
        updated_acme_quota.memory_usage / 1024 / 1024 / 1024,
        updated_acme_quota.memory_bytes / 1024 / 1024 / 1024,
        updated_acme_quota.memory_usage_percentage());
    println!("     - Connections: {}/{} ({:.1}%)",
        updated_acme_quota.current_connections,
        updated_acme_quota.max_connections,
        (updated_acme_quota.current_connections as f64 / updated_acme_quota.max_connections as f64) * 100.0);
    
    // Demo 4: Data partitioning & encryption
    println!("\n4. Data Partitioning & Security:");
    println!("----------------------------------");
    
    // Create partitions for Beta tenant
    let beta_partitions = partitioner.create_partitions(beta_id, PartitionStrategy::Hash, 8)?;
    println!("   Beta Solutions data partitioned into {} shards", beta_partitions.partition_count);
    
    // Demonstrate encryption
    let sensitive_data = b"Customer PII: John Doe, SSN: 123-45-6789";
    let encrypted = partitioner.encrypt_data(beta_id, sensitive_data)?;
    let decrypted = partitioner.decrypt_data(beta_id, &encrypted)?;
    
    println!("   Data encryption working:");
    println!("     - Original: {}", String::from_utf8_lossy(sensitive_data));
    println!("     - Encrypted: {} bytes", encrypted.len());
    println!("     - Decrypted matches: {}", sensitive_data == &decrypted[..]);
    
    // Demo 5: Row-level security
    println!("\n5. Row-Level Security (RLS):");
    println!("------------------------------");
    
    // Create RLS policies
    let customer_policy = partitioner.create_rls_policy(
        "customer_data_policy".to_string(),
        "customers".to_string(),
        "tenant_id = $1 AND active = true".to_string(),
        vec![beta_id],
    )?;
    
    println!("   Created RLS policy '{}' for table 'customers'", customer_policy.name);
    println!("   Policy expression: {}", customer_policy.expression);
    println!("   Applies to tenant: Beta Solutions");
    
    // Demo 6: API Management
    println!("\n6. Management API:");
    println!("-------------------");
    
    // List all tenants
    let all_tenants = api.handle_list_tenants(true)?;
    println!("   Active tenants: {}", all_tenants["count"].as_u64().unwrap());
    
    // Usage statistics
    let usage_stats = api.handle_get_usage_stats()?;
    println!("   System usage statistics collected for {} tenants", usage_stats["count"].as_u64().unwrap());
    
    // Health check
    let health = api.handle_health_check();
    println!("   System status: {}", health["status"].as_str().unwrap());
    
    // Demo 7: Quota enforcement
    println!("\n7. Quota Enforcement:");
    println!("----------------------");
    
    // Try to exceed Gamma's basic tier quota
    println!("   Testing Gamma Technologies quota limits (Basic Tier):");
    
    // This should fail - Basic tier has only 0.1 CPU cores
    match quota_manager.allocate_cpu(gamma_id, 0.2) {
        Ok(_) => println!("     ❌ ERROR: Should have failed CPU quota check!"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    // Demo 8: Cross-tenant query prevention
    println!("\n8. Cross-Tenant Query Prevention:");
    println!("-----------------------------------");
    
    let malicious_query = "SELECT * FROM users WHERE tenant_id != 'acme-id' OR tenant_id IS NULL";
    match partitioner.prevent_cross_tenant_query(malicious_query, beta_id) {
        Ok(_) => println!("     ❌ ERROR: Should have rejected cross-tenant query!"),
        Err(e) => println!("     ✅ Correctly rejected: {}", e),
    }
    
    let safe_query = "SELECT * FROM users WHERE status = 'active'";
    match partitioner.prevent_cross_tenant_query(safe_query, beta_id) {
        Ok(secured_query) => {
            println!("     ✅ Safe query secured:");
            println!("       Original: {}", safe_query);
            println!("       Secured: {}", secured_query);
        }
        Err(e) => println!("     ❌ ERROR: {}", e),
    }
    
    println!("\n🎉 Multi-tenancy Demo Complete!");
    println!("================================");
    println!("\nKey Features Demonstrated:");
    println!("1. ✅ Tenant creation & isolation");
    println!("2. ✅ Tier-based resource quotas");
    println!("3. ✅ Resource allocation & tracking");
    println!("4. ✅ Data partitioning & sharding");
    println!("5. ✅ Encryption at rest");
    println!("6. ✅ Row-level security policies");
    println!("7. ✅ Management APIs");
    println!("8. ✅ Quota enforcement");
    println!("9. ✅ Cross-tenant query prevention");
    println!("10. ✅ Usage monitoring & reporting");
    
    println!("\n📊 Production Ready For:");
    println!("• SaaS platforms");
    println!("• Multi-tenant databases");
    println!("• Enterprise applications");
    println!("• Cloud-native services");
    
    Ok(())
}