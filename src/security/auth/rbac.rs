/// RBAC (Role-Based Access Control) engine with hierarchical permissions
pub struct RbacEngine {
    /// Roles hierarchy
    roles_hierarchy: Arc<RwLock<HashMap<String, Vec<String>>>>,
    
    /// Role permissions
    role_permissions: Arc<RwLock<HashMap<String, HashSet<String>>>>,
    
    /// Resource permissions
    resource_permissions: Arc<RwLock<HashMap<String, ResourcePermission>>>,
}

impl RbacEngine {
    /// Create new RBAC engine
    pub fn new() -> Self {
        Self {
            roles_hierarchy: Arc::new(RwLock::new(HashMap::new())),
            role_permissions: Arc::new(RwLock::new(HashMap::new())),
            resource_permissions: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Add role with parent roles
    pub fn add_role(&self, role: &str, parent_roles: Vec<String>) -> Result<()> {
        let mut hierarchy = self.roles_hierarchy.write().unwrap();
        hierarchy.insert(role.to_string(), parent_roles);
        Ok(())
    }
    
    /// Add permission to role
    pub fn add_permission_to_role(&self, role: &str, permission: &str) -> Result<()> {
        let mut permissions = self.role_permissions.write().unwrap();
        let role_perms = permissions.entry(role.to_string())
            .or_insert_with(HashSet::new);
        role_perms.insert(permission.to_string());
        Ok(())
    }
    
    /// Add resource with specific permissions
    pub fn add_resource(&self, resource: &str, actions: Vec<String>) -> Result<()> {
        let mut resources = self.resource_permissions.write().unwrap();
        resources.insert(resource.to_string(), ResourcePermission {
            actions: actions.into_iter().collect(),
        });
        Ok(())
    }
    
    /// Check if user has permission
    pub fn check_permission(&self, context: &AuthorizationContext) -> Result<bool> {
        let principal = &context.principal;
        let resource = &context.resource;
        let action = &context.action;
        
        // Get all roles for user (including inherited roles)
        let user_roles = self.get_all_user_roles(principal);
        
        // Check if any role has the required permission
        for role in user_roles {
            if self.role_has_permission(&role, resource, action)? {
                return Ok(true);
            }
        }
        
        // Check direct permissions
        if principal.permissions.contains(&format!("{}:{}", resource, action)) {
            return Ok(true);
        }
        
        Ok(false)
    }
    
    /// Get all roles for user (including inherited)
    fn get_all_user_roles(&self, principal: &UserPrincipal) -> Vec<String> {
        let mut all_roles = principal.roles.clone();
        let hierarchy = self.roles_hierarchy.read().unwrap();
        
        // Get inherited roles
        let mut queue = principal.roles.clone();
        while let Some(role) = queue.pop() {
            if let Some(parents) = hierarchy.get(&role) {
                for parent in parents {
                    if !all_roles.contains(parent) {
                        all_roles.push(parent.clone());
                        queue.push(parent.clone());
                    }
                }
            }
        }
        
        all_roles
    }
    
    /// Check if role has permission for resource action
    fn role_has_permission(&self, role: &str, resource: &str, action: &str) -> Result<bool> {
        let permissions = self.role_permissions.read().unwrap();
        
        if let Some(role_perms) = permissions.get(role) {
            // Check exact permission
            if role_perms.contains(&format!("{}:{}", resource, action)) {
                return Ok(true);
            }
            
            // Check wildcard permissions
            if role_perms.contains(&format!("{}:*", resource)) {
                return Ok(true);
            }
            
            if role_perms.contains(&format!("*:{}", action)) {
                return Ok(true);
            }
            
            if role_perms.contains("*:*") {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Get all permissions for user
    pub fn get_user_permissions(&self, principal: &UserPrincipal) -> Vec<String> {
        let user_roles = self.get_all_user_roles(principal);
        let permissions = self.role_permissions.read().unwrap();
        
        let mut all_permissions: HashSet<String> = HashSet::new();
        
        // Add role permissions
        for role in user_roles {
            if let Some(role_perms) = permissions.get(&role) {
                all_permissions.extend(role_perms.iter().cloned());
            }
        }
        
        // Add direct permissions
        all_permissions.extend(principal.permissions.iter().cloned());
        
        all_permissions.into_iter().collect()
    }
    
    /// Validate permission string format
    pub fn validate_permission_format(&self, permission: &str) -> Result<()> {
        let parts: Vec<&str> = permission.split(':').collect();
        
        if parts.len() != 2 {
            return Err(SecurityError::ValidationError(
                "Permission must be in format 'resource:action'".to_string()
            ));
        }
        
        if parts[0].is_empty() || parts[1].is_empty() {
            return Err(SecurityError::ValidationError(
                "Resource and action cannot be empty".to_string()
            ));
        }
        
        Ok(())
    }
}

/// Resource permission definition
#[derive(Debug, Clone)]
struct ResourcePermission {
    actions: HashSet<String>,
}

/// Default RBAC roles and permissions
pub struct DefaultRbacConfig;

impl DefaultRbacConfig {
    /// Create default RBAC configuration
    pub fn configure(engine: &RbacEngine) -> Result<()> {
        // Define role hierarchy
        engine.add_role("super_admin", vec![])?;
        engine.add_role("admin", vec!["super_admin".to_string()])?;
        engine.add_role("manager", vec!["admin".to_string()])?;
        engine.add_role("user", vec!["manager".to_string()])?;
        engine.add_role("guest", vec!["user".to_string()])?;
        
        // Define permissions for each role
        Self::configure_super_admin_permissions(engine)?;
        Self::configure_admin_permissions(engine)?;
        Self::configure_manager_permissions(engine)?;
        Self::configure_user_permissions(engine)?;
        Self::configure_guest_permissions(engine)?;
        
        // Define common resources
        engine.add_resource("users", vec![
            "create".to_string(),
            "read".to_string(),
            "update".to_string(),
            "delete".to_string(),
            "list".to_string(),
        ])?;
        
        engine.add_resource("documents", vec![
            "create".to_string(),
            "read".to_string(),
            "update".to_string(),
            "delete".to_string(),
            "share".to_string(),
            "download".to_string(),
        ])?;
        
        engine.add_resource("settings", vec![
            "read".to_string(),
            "update".to_string(),
        ])?;
        
        Ok(())
    }
    
    fn configure_super_admin_permissions(engine: &RbacEngine) -> Result<()> {
        engine.add_permission_to_role("super_admin", "*:*")?;
        Ok(())
    }
    
    fn configure_admin_permissions(engine: &RbacEngine) -> Result<()> {
        engine.add_permission_to_role("admin", "users:*")?;
        engine.add_permission_to_role("admin", "documents:*")?;
        engine.add_permission_to_role("admin", "settings:*")?;
        engine.add_permission_to_role("admin", "reports:*")?;
        Ok(())
    }
    
    fn configure_manager_permissions(engine: &RbacEngine) -> Result<()> {
        engine.add_permission_to_role("manager", "users:read")?;
        engine.add_permission_to_role("manager", "users:update")?;
        engine.add_permission_to_role("manager", "documents:*")?;
        engine.add_permission_to_role("manager", "reports:read")?;
        Ok(())
    }
    
    fn configure_user_permissions(engine: &RbacEngine) -> Result<()> {
        engine.add_permission_to_role("user", "documents:create")?;
        engine.add_permission_to_role("user", "documents:read")?;
        engine.add_permission_to_role("user", "documents:update")?;
        engine.add_permission_to_role("user", "documents:delete")?;
        engine.add_permission_to_role("user", "settings:read")?;
        Ok(())
    }
    
    fn configure_guest_permissions(engine: &RbacEngine) -> Result<()> {
        engine.add_permission_to_role("guest", "documents:read")?;
        engine.add_permission_to_role("guest", "settings:read")?;
        Ok(())
    }
}

/// Authorization helper functions
pub struct Authorization;

impl Authorization {
    /// Authorize user action
    pub fn authorize(context: &AuthorizationContext, engine: &RbacEngine) -> Result<()> {
        if engine.check_permission(context)? {
            Ok(())
        } else {
            Err(SecurityError::AuthorizationError(
                format!("User {} not authorized for {}:{}", 
                    context.principal.user_id, context.resource, context.action)
            ))
        }
    }
    
    /// Create authorization context from user principal
    pub fn create_context(principal: &UserPrincipal, resource: &str, action: &str) -> AuthorizationContext {
        AuthorizationContext::new(principal.clone(), resource.to_string(), action.to_string())
    }
    
    /// Check multiple permissions
    pub fn check_multiple_permissions(
        principal: &UserPrincipal,
        permissions: &[(&str, &str)],
        engine: &RbacEngine,
    ) -> Result<Vec<bool>> {
        let mut results = Vec::new();
        
        for (resource, action) in permissions {
            let context = Self::create_context(principal, resource, action);
            results.push(engine.check_permission(&context)?);
        }
        
        Ok(results)
    }
    
    /// Check if user has all permissions
    pub fn has_all_permissions(
        principal: &UserPrincipal,
        permissions: &[(&str, &str)],
        engine: &RbacEngine,
    ) -> Result<bool> {
        let results = Self::check_multiple_permissions(principal, permissions, engine)?;
        Ok(results.iter().all(|&r| r))
    }
    
    /// Check if user has any permission
    pub fn has_any_permission(
        principal: &UserPrincipal,
        permissions: &[(&str, &str)],
        engine: &RbacEngine,
    ) -> Result<bool> {
        let results = Self::check_multiple_permissions(principal, permissions, engine)?;
        Ok(results.iter().any(|&r| r))
    }
}