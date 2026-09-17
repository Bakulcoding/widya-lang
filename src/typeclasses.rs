//! Type Classes/Traits system for Widya-Lang
//! Implementation of Haskell-style type classes and trait bounds

use crate::typesystem::{Type, TypeParameter, Span};
use std::collections::HashMap;

/// Trait definition
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDefinition {
    pub name: String,
    pub type_params: Vec<TypeParameter>,
    pub methods: Vec<TraitMethod>,
    pub super_traits: Vec<String>,
    pub span: Span,
}

/// Trait method signature
#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub name: String,
    pub params: Vec<String>,
    pub return_type: Type,
    pub default_impl: Option<String>,
    pub span: Span,
}

/// Trait implementation
#[derive(Debug, Clone, PartialEq)]
pub struct TraitImpl {
    pub trait_name: String,
    pub for_type: Type,
    pub implementations: Vec<String>,
    pub span: Span,
}

/// Type class system
pub struct TypeClassSystem {
    /// Registered traits
    traits: HashMap<String, TraitDefinition>,
    
    /// Registered trait implementations
    impls: Vec<TraitImpl>,
}

impl TypeClassSystem {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
            impls: Vec::new(),
        }
    }
    
    /// Register a new trait
    pub fn register_trait(&mut self, trait_def: TraitDefinition) {
        self.traits.insert(trait_def.name.clone(), trait_def);
    }
    
    /// Register a trait implementation
    pub fn register_impl(&mut self, impl_def: TraitImpl) {
        self.impls.push(impl_def);
    }
    
    /// Find trait implementation for a type
    pub fn find_impl(&self, trait_name: &str, ty: &Type) -> Option<&TraitImpl> {
        self.impls.iter().find(|impl_def| {
            impl_def.trait_name == trait_name && self.type_matches(&impl_def.for_type, ty)
        })
    }
    
    /// Check if type matches expected type (simplified)
    fn type_matches(&self, expected: &Type, actual: &Type) -> bool {
        expected == actual
    }
    
    /// Add default traits (like Rust's core traits)
    pub fn add_default_traits(&mut self) {
        // Add Show trait
        self.register_trait(TraitDefinition {
            name: "Tunjukkan".to_string(),
            type_params: vec![
                TypeParameter {
                    name: "T".to_string(),
                    constraints: Vec::new(),
                    span: Span::dummy(),
                },
            ],
            methods: vec![
                TraitMethod {
                    name: "to_string".to_string(),
                    params: vec!["self".to_string()],
                    return_type: Type::String,
                    default_impl: None,
                    span: Span::dummy(),
                },
            ],
            super_traits: Vec::new(),
            span: Span::dummy(),
        });
        
        // Add Eq trait
        self.register_trait(TraitDefinition {
            name: "Sama".to_string(),
            type_params: vec![
                TypeParameter {
                    name: "T".to_string(),
                    constraints: Vec::new(),
                    span: Span::dummy(),
                },
            ],
            methods: vec![
                TraitMethod {
                    name: "sama".to_string(),
                    params: vec!["self".to_string(), "other".to_string()],
                    return_type: Type::Boolean,
                    default_impl: None,
                    span: Span::dummy(),
                },
            ],
            super_traits: Vec::new(),
            span: Span::dummy(),
        });
    }
    
    /// Get trait definition
    pub fn get_trait(&self, name: &str) -> Option<&TraitDefinition> {
        self.traits.get(name)
    }
    
    /// Get all registered traits
    pub fn get_all_traits(&self) -> Vec<&TraitDefinition> {
        self.traits.values().collect()
    }
    
    /// Check if a type implements a trait
    pub fn type_implements(&self, ty: &Type, trait_name: &str) -> bool {
        self.find_impl(trait_name, ty).is_some()
    }
}

/// Helper functions for type class system
pub struct TypeClassUtils;

impl TypeClassUtils {
    /// Create a Show trait implementation for a type
    pub fn create_show_impl(ty: Type) -> TraitImpl {
        TraitImpl {
            trait_name: "Tunjukkan".to_string(),
            for_type: ty,
            implementations: vec!["to_string".to_string()],
            span: Span::dummy(),
        }
    }
    
    /// Create an Eq trait implementation for a type
    pub fn create_eq_impl(ty: Type) -> TraitImpl {
        TraitImpl {
            trait_name: "Sama".to_string(),
            for_type: ty,
            implementations: vec!["sama".to_string()],
            span: Span::dummy(),
        }
    }
    
    /// Check if trait is derived (built-in)
    pub fn is_derived_trait(trait_name: &str) -> bool {
        matches!(
            trait_name,
            "Tunjukkan" | "Sama"
        )
    }
    
    /// Get all derived traits
    pub fn get_derived_traits() -> Vec<String> {
        vec![
            "Tunjukkan".to_string(),
            "Sama".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_register_trait() {
        let mut system = TypeClassSystem::new();
        
        system.register_trait(TraitDefinition {
            name: "Show".to_string(),
            type_params: vec![
                TypeParameter {
                    name: "T".to_string(),
                    constraints: Vec::new(),
                    span: Span::dummy(),
                },
            ],
            methods: vec![TraitMethod {
                name: "show".to_string(),
                params: vec!["self".to_string()],
                return_type: Type::String,
                default_impl: None,
                span: Span::dummy(),
            }],
            super_traits: Vec::new(),
            span: Span::dummy(),
        });
        
        assert!(system.get_trait("Show").is_some());
    }
    
    #[test]
    fn test_add_default_traits() {
        let mut system = TypeClassSystem::new();
        system.add_default_traits();
        
        assert!(system.get_trait("Tunjukkan").is_some());
        assert!(system.get_trait("Sama").is_some());
        assert_eq!(system.get_all_traits().len(), 2);
    }
    
    #[test]
    fn test_register_and_find_impl() {
        let mut system = TypeClassSystem::new();
        system.add_default_traits();
        
        let impl_def = TypeClassUtils::create_show_impl(Type::Number);
        system.register_impl(impl_def);
        
        let found = system.find_impl("Tunjukkan", &Type::Number);
        assert!(found.is_some());
    }
    
    #[test]
    fn test_trait_derivation() {
        assert!(TypeClassUtils::is_derived_trait("Tunjukkan"));
        assert!(TypeClassUtils::is_derived_trait("Sama"));
        assert!(!TypeClassUtils::is_derived_trait("CustomTrait"));
    }
}
