//! Advanced Type System for Widya-Lang
//! Implementation of generic types, type parameters, and algebraic data types

use std::fmt;
use std::collections::HashMap;

/// Advanced type system for Widya-Lang
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Number,
    String,
    Boolean,
    Nil,
    Array(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Generic(String),
    GenericInstantiation {
        base: String,
        type_args: Vec<Type>,
    },
    EnumVariant {
        enum_name: String,
        variant_name: String,
        field_types: Vec<Type>,
    },
    Result(Box<Type>, Box<Type>),
    Option(Box<Type>),
    Union(Vec<Type>),
    Intersection(Vec<Type>),
    Alias(String),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParameter {
    pub name: String,
    pub constraints: Vec<String>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn dummy() -> Self {
        Self { start: 0, end: 0, line: 0, column: 0 }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "Angka"),
            Type::String => write!(f, "String"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Nil => write!(f, "Nihil"),
            Type::Array(inner) => write!(f, "Daftar[{}]", inner),
            Type::Map(key, value) => write!(f, "Peta[{} => {}]", key, value),
            Type::Tuple(types) => {
                write!(f, "(")?;
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", ty)?;
                }
                write!(f, ")")
            }
            Type::Function { params, return_type } => {
                write!(f, "fungsi(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", return_type)
            }
            Type::Generic(name) => write!(f, "{}", name),
            Type::GenericInstantiation { base, type_args } => {
                write!(f, "{}[", base)?;
                for (i, arg) in type_args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", arg)?;
                }
                write!(f, "]")
            }
            Type::EnumVariant { enum_name, variant_name, field_types } => {
                if field_types.is_empty() {
                    write!(f, "{}::{}", enum_name, variant_name)
                } else {
                    write!(f, "{}::{}(", enum_name, variant_name)?;
                    for (i, field_type) in field_types.iter().enumerate() {
                        if i > 0 { write!(f, ", ")?; }
                        write!(f, "{}", field_type)?;
                    }
                    write!(f, ")")
                }
            }
            Type::Result(ok_type, err_type) => write!(f, "Hasil[{}, {}]", ok_type, err_type),
            Type::Option(inner) => write!(f, "Opsi[{}]", inner),
            Type::Union(types) => {
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 { write!(f, " | ")?; }
                    write!(f, "{}", ty)?;
                }
                Ok(())
            }
            Type::Intersection(types) => {
                for (i, ty) in types.iter().enumerate() {
                    if i > 0 { write!(f, " & ")?; }
                    write!(f, "{}", ty)?;
                }
                Ok(())
            }
            Type::Alias(name) => write!(f, "{}", name),
            Type::Unknown => write!(f, "?"),
        }
    }
}

/// Type environment for inference
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    pub variables: Vec<(String, Type)>,
    pub generic_context: Vec<TypeParameter>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self { variables: Vec::new(), generic_context: Vec::new() }
    }
    
    pub fn enter_generic_context(&mut self, params: Vec<TypeParameter>) {
        self.generic_context.extend(params);
    }
    
    pub fn bind(&mut self, name: String, ty: Type) {
        self.variables.push((name, ty));
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.variables.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }
}

/// Type substitution for unification
#[derive(Debug, Clone)]
pub struct Substitution {
    mapping: HashMap<String, Type>,
}

impl Substitution {
    pub fn new() -> Self {
        Self { mapping: HashMap::new() }
    }
    
    pub fn apply(&self, ty: &Type) -> Type {
        match ty {
            Type::Generic(name) => {
                if let Some(subst_ty) = self.mapping.get(name) {
                    subst_ty.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Array(inner) => Type::Array(Box::new(self.apply(inner))),
            Type::Map(key, value) => Type::Map(Box::new(self.apply(key)), Box::new(self.apply(value))),
            Type::Tuple(types) => Type::Tuple(types.iter().map(|t| self.apply(t)).collect()),
            Type::Function { params, return_type } => Type::Function {
                params: params.iter().map(|t| self.apply(t)).collect(),
                return_type: Box::new(self.apply(return_type)),
            },
            Type::GenericInstantiation { base, type_args } => Type::GenericInstantiation {
                base: base.clone(),
                type_args: type_args.iter().map(|t| self.apply(t)).collect(),
            },
            Type::EnumVariant { enum_name, variant_name, field_types } => Type::EnumVariant {
                enum_name: enum_name.clone(),
                variant_name: variant_name.clone(),
                field_types: field_types.iter().map(|t| self.apply(t)).collect(),
            },
            Type::Result(ok_type, err_type) => Type::Result(
                Box::new(self.apply(ok_type)),
                Box::new(self.apply(err_type)),
            ),
            Type::Option(inner) => Type::Option(Box::new(self.apply(inner))),
            Type::Union(types) => Type::Union(types.iter().map(|t| self.apply(t)).collect()),
            Type::Intersection(types) => Type::Intersection(types.iter().map(|t| self.apply(t)).collect()),
            _ => ty.clone(),
        }
    }
    
    pub fn bind(&mut self, var_name: String, ty: Type) {
        self.mapping.insert(var_name, ty);
    }
    
    pub fn compose(&self, other: &Substitution) -> Substitution {
        let mut result = Substitution::new();
        
        // Apply other to self's mappings
        for (var, ty) in &self.mapping {
            result.bind(var.clone(), other.apply(ty));
        }
        
        // Add other's mappings that don't conflict
        for (var, ty) in &other.mapping {
            if !self.mapping.contains_key(var) {
                result.bind(var.clone(), ty.clone());
            }
        }
        
        result
    }
}

/// Type inference engine with Hindley-Milner algorithm
pub struct TypeInferrer {
    env: TypeEnvironment,
    next_type_var_id: u64,
    substitution: Substitution,
}

impl TypeInferrer {
    pub fn new() -> Self {
        Self { 
            env: TypeEnvironment::new(), 
            next_type_var_id: 0,
            substitution: Substitution::new(),
        }
    }
    
    pub fn new_type_variable(&mut self) -> Type {
        let id = self.next_type_var_id;
        self.next_type_var_id += 1;
        Type::Generic(format!("_T{}", id))
    }
    
    /// Unify two types, returning a substitution
    pub fn unify(&mut self, t1: &Type, t2: &Type) -> Result<Substitution, String> {
        let t1_applied = self.substitution.apply(t1);
        let t2_applied = self.substitution.apply(t2);
        
        match (&t1_applied, &t2_applied) {
            // Same base types
            (Type::Number, Type::Number) |
            (Type::String, Type::String) |
            (Type::Boolean, Type::Boolean) |
            (Type::Nil, Type::Nil) => Ok(Substitution::new()),
            
            // Type variables
            (Type::Generic(var_name), _) => {
                if self.occurs_check(var_name, &t2_applied) {
                    Err(format!("Occurs check failed: {} occurs in {:?}", var_name, t2_applied))
                } else {
                    let mut subst = Substitution::new();
                    subst.bind(var_name.clone(), t2_applied.clone());
                    Ok(subst)
                }
            }
            (_, Type::Generic(var_name)) => {
                if self.occurs_check(var_name, &t1_applied) {
                    Err(format!("Occurs check failed: {} occurs in {:?}", var_name, t1_applied))
                } else {
                    let mut subst = Substitution::new();
                    subst.bind(var_name.clone(), t1_applied.clone());
                    Ok(subst)
                }
            }
            
            // Arrays
            (Type::Array(inner1), Type::Array(inner2)) => {
                self.unify(inner1, inner2)
            }
            
            // Maps
            (Type::Map(key1, value1), Type::Map(key2, value2)) => {
                let subst1 = self.unify(key1, key2)?;
                let subst2 = self.unify(value1, value2)?;
                Ok(subst1.compose(&subst2))
            }
            
            // Tuples
            (Type::Tuple(types1), Type::Tuple(types2)) => {
                if types1.len() != types2.len() {
                    return Err(format!("Tuple length mismatch: {} vs {}", types1.len(), types2.len()));
                }
                
                let mut result = Substitution::new();
                for (t1, t2) in types1.iter().zip(types2.iter()) {
                    let subst = self.unify(t1, t2)?;
                    result = result.compose(&subst);
                }
                Ok(result)
            }
            
            // Functions
            (
                Type::Function { params: params1, return_type: ret1 },
                Type::Function { params: params2, return_type: ret2 }
            ) => {
                if params1.len() != params2.len() {
                    return Err(format!("Function parameter count mismatch: {} vs {}", params1.len(), params2.len()));
                }
                
                let mut result = Substitution::new();
                for (p1, p2) in params1.iter().zip(params2.iter()) {
                    let subst = self.unify(p1, p2)?;
                    result = result.compose(&subst);
                }
                
                let ret_subst = self.unify(ret1, ret2)?;
                Ok(result.compose(&ret_subst))
            }
            
            // Generic instantiation
            (
                Type::GenericInstantiation { base: base1, type_args: args1 },
                Type::GenericInstantiation { base: base2, type_args: args2 }
            ) => {
                if base1 != base2 {
                    return Err(format!("Generic base mismatch: {} vs {}", base1, base2));
                }
                if args1.len() != args2.len() {
                    return Err(format!("Generic argument count mismatch for {}: {} vs {}", base1, args1.len(), args2.len()));
                }
                
                let mut result = Substitution::new();
                for (a1, a2) in args1.iter().zip(args2.iter()) {
                    let subst = self.unify(a1, a2)?;
                    result = result.compose(&subst);
                }
                Ok(result)
            }
            
            // Incompatible types
            _ => Err(format!("Cannot unify {:?} with {:?}", t1_applied, t2_applied)),
        }
    }
    
    /// Check if a type variable occurs in a type
    fn occurs_check(&self, var_name: &str, ty: &Type) -> bool {
        match ty {
            Type::Generic(name) => name == var_name,
            Type::Array(inner) => self.occurs_check(var_name, inner),
            Type::Map(key, value) => 
                self.occurs_check(var_name, key) || self.occurs_check(var_name, value),
            Type::Tuple(types) => 
                types.iter().any(|t| self.occurs_check(var_name, t)),
            Type::Function { params, return_type } =>
                params.iter().any(|t| self.occurs_check(var_name, t)) ||
                self.occurs_check(var_name, return_type),
            Type::GenericInstantiation { type_args, .. } =>
                type_args.iter().any(|t| self.occurs_check(var_name, t)),
            Type::EnumVariant { field_types, .. } =>
                field_types.iter().any(|t| self.occurs_check(var_name, t)),
            Type::Result(ok_type, err_type) =>
                self.occurs_check(var_name, ok_type) || self.occurs_check(var_name, err_type),
            Type::Option(inner) => self.occurs_check(var_name, inner),
            Type::Union(types) => types.iter().any(|t| self.occurs_check(var_name, t)),
            Type::Intersection(types) => types.iter().any(|t| self.occurs_check(var_name, t)),
            _ => false,
        }
    }
    
    /// Infer type for an expression
    pub fn infer_expr(&mut self, expr: &str) -> Result<Type, String> {
        // Simple expression type inference
        match expr.trim() {
            "true" | "false" => Ok(Type::Boolean),
            "null" | "nil" => Ok(Type::Nil),
            _ if expr.chars().all(|c| c.is_digit(10)) => Ok(Type::Number),
            _ if expr.starts_with('"') && expr.ends_with('"') => Ok(Type::String),
            _ => {
                // Try to look up in environment
                if let Some(ty) = self.env.lookup(expr) {
                    Ok(ty.clone())
                } else {
                    // Create a new type variable for unknown expressions
                    Ok(self.new_type_variable())
                }
            }
        }
    }
    
    /// Apply current substitution to a type
    pub fn apply_substitution(&self, ty: &Type) -> Type {
        self.substitution.apply(ty)
    }
    
    /// Add a substitution to the current environment
    pub fn add_substitution(&mut self, subst: Substitution) {
        self.substitution = self.substitution.compose(&subst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_display() {
        assert_eq!(format!("{}", Type::Number), "Angka");
        assert_eq!(format!("{}", Type::String), "String");
        let array_type = Type::Array(Box::new(Type::Number));
        assert_eq!(format!("{}", array_type), "Daftar[Angka]");
        let result_type = Type::Result(Box::new(Type::String), Box::new(Type::Number));
        assert_eq!(format!("{}", result_type), "Hasil[String, Angka]");
    }
    
    #[test]
    fn test_generic_type() {
        let generic = Type::Generic("T".to_string());
        assert_eq!(format!("{}", generic), "T");
        let instantiation = Type::GenericInstantiation {
            base: "Kantong".to_string(),
            type_args: vec![Type::Number],
        };
        assert_eq!(format!("{}", instantiation), "Kantong[Angka]");
    }
    
    #[test]
    fn test_type_inference() {
        let mut inferrer = TypeInferrer::new();
        inferrer.env.bind("x".to_string(), Type::Number);
        
        // Test inference of bound variable
        let x_type = inferrer.infer_expr("x").unwrap();
        assert_eq!(x_type, Type::Number);
        
        // Test inference of literal
        let num_type = inferrer.infer_expr("42").unwrap();
        assert_eq!(num_type, Type::Number);
        
        // Test inference of string literal
        let str_type = inferrer.infer_expr("\"hello\"").unwrap();
        assert_eq!(str_type, Type::String);
        
        // Test inference of boolean
        let bool_type = inferrer.infer_expr("true").unwrap();
        assert_eq!(bool_type, Type::Boolean);
    }
    
    #[test]
    fn test_unification() {
        let mut inferrer = TypeInferrer::new();
        
        // Test unification of same types
        let result = inferrer.unify(&Type::Number, &Type::Number);
        assert!(result.is_ok());
        
        // Test unification of type variable with concrete type
        let t_var = inferrer.new_type_variable();
        let result = inferrer.unify(&t_var, &Type::Number);
        assert!(result.is_ok());
        
        // Apply substitution and check
        if let Ok(subst) = result {
            inferrer.add_substitution(subst);
            let applied = inferrer.apply_substitution(&t_var);
            assert_eq!(applied, Type::Number);
        }
        
        // Test occurs check
        let t1 = Type::Generic("T".to_string());
        let t2 = Type::Array(Box::new(Type::Generic("T".to_string())));
        let result = inferrer.unify(&t1, &t2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Occurs check failed"));
    }
    
    #[test]
    fn test_function_type_unification() {
        let mut inferrer = TypeInferrer::new();
        
        let func_type1 = Type::Function {
            params: vec![Type::Number, Type::String],
            return_type: Box::new(Type::Boolean),
        };
        
        let func_type2 = Type::Function {
            params: vec![Type::Number, Type::String],
            return_type: Box::new(Type::Boolean),
        };
        
        // Should unify successfully
        let result = inferrer.unify(&func_type1, &func_type2);
        assert!(result.is_ok());
        
        // Different return type should fail
        let func_type3 = Type::Function {
            params: vec![Type::Number, Type::String],
            return_type: Box::new(Type::Number),
        };
        
        let result = inferrer.unify(&func_type1, &func_type3);
        assert!(result.is_err());
    }
}
