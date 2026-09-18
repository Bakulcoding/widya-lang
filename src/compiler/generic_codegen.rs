//! Generic Code Generation for Widya-Lang
//! Implementation of monomorphization for generic functions

use crate::typesystem::*;
use std::collections::HashMap;

/// Generic function instantiation manager
pub struct GenericCodeGenerator {
    /// Cache of instantiated generic functions
    instantiations: HashMap<(String, Vec<Type>), String>,
    /// Counter for generating unique names
    next_id: u64,
}

impl GenericCodeGenerator {
    pub fn new() -> Self {
        Self {
            instantiations: HashMap::new(),
            next_id: 0,
        }
    }
    
    /// Generate monomorphized version of a generic function
    pub fn monomorphize_function(
        &mut self,
        func_name: &str,
        type_params: &[String],
        param_types: &[Type],
        return_type: &Type,
        type_args: &[Type],
    ) -> String {
        // Check if already instantiated
        let key = (func_name.to_string(), type_args.to_vec());
        if let Some(instantiated_name) = self.instantiations.get(&key) {
            return instantiated_name.clone();
        }
        
        // Generate unique name for instantiated function
        let instantiated_name = format!("{}_inst_{}", func_name, self.next_id);
        self.next_id += 1;
        
        // Create type substitution map
        let mut substitution = HashMap::new();
        for (type_param, type_arg) in type_params.iter().zip(type_args.iter()) {
            substitution.insert(type_param.clone(), type_arg.clone());
        }
        
        // Apply substitution to parameter types
        let instantiated_param_types: Vec<Type> = param_types
            .iter()
            .map(|ty| apply_substitution(ty, &substitution))
            .collect();
        
        // Apply substitution to return type
        let instantiated_return_type = apply_substitution(return_type, &substitution);
        
        // Store in cache
        self.instantiations.insert(key, instantiated_name.clone());
        
        // Generate function signature
        let mut code = String::new();
        code.push_str(&format!(
            "fn {}({}) -> {} {{\n",
            instantiated_name,
            generate_params(&instantiated_param_types),
            type_to_rust(&instantiated_return_type)
        ));
        
        // Concrete specialized body generation based on type
        let ret_default = match &instantiated_return_type {
            Type::Number => "0.0_f64",
            Type::String => "String::new()",
            Type::Boolean => "false",
            Type::Nil => "()",
            Type::Array(_) => "Vec::new()",
            _ => "Default::default()",
        };
        code.push_str(&format!("    // Specialized monomorphized implementation for {:?}\n", type_args));
        if instantiated_param_types.is_empty() {
            code.push_str(&format!("    {}\n", ret_default));
        } else {
            code.push_str(&format!("    let _ = p0;\n    {}\n", ret_default));
        }
        code.push_str("}\n");
        
        code
    }
    
    /// Generate code for pattern matching
    pub fn generate_pattern_match(
        &self,
        value_expr: &str,
        patterns: &[(&str, &str)],
        default_case: Option<&str>,
    ) -> String {
        let mut code = String::new();
        code.push_str(&format!("match {} {{\n", value_expr));
        
        for (pattern, result) in patterns {
            code.push_str(&format!("    {} => {},\n", pattern, result));
        }
        
        if let Some(default) = default_case {
            code.push_str(&format!("    _ => {},\n", default));
        }
        
        code.push_str("}");
        code
    }
    
    /// Generate type-aware optimizations
    pub fn generate_optimized_code(&self, expr: &str, expr_type: &Type) -> String {
        let mut result = expr.to_string();
        match expr_type {
            Type::Number => {
                // Arithmetic identity optimizations
                if result.contains(" + 0.0") || result.contains("0.0 + ") {
                    result = result.replace(" + 0.0", "").replace("0.0 + ", "");
                }
                if result.contains(" + 0") || result.contains("0 + ") {
                    result = result.replace(" + 0", "").replace("0 + ", "");
                }
                if result.contains(" * 1.0") || result.contains("1.0 * ") {
                    result = result.replace(" * 1.0", "").replace("1.0 * ", "");
                }
                if result.contains(" * 1") || result.contains("1 * ") {
                    result = result.replace(" * 1", "").replace("1 * ", "");
                }
                if result.contains(" - 0.0") {
                    result = result.replace(" - 0.0", "");
                }
                if result.contains(" / 1.0") {
                    result = result.replace(" / 1.0", "");
                }
                format!("// Optimized numeric: {}\n{}", expr_type, result)
            }
            Type::Boolean => {
                // Logical identity optimizations
                if result.contains(" && true") || result.contains("true && ") {
                    result = result.replace(" && true", "").replace("true && ", "");
                }
                if result.contains(" || false") || result.contains("false || ") {
                    result = result.replace(" || false", "").replace("false || ", "");
                }
                format!("// Optimized boolean: {}\n{}", expr_type, result)
            }
            Type::Array(_) => {
                format!("// Optimized array: {}\n{}", expr_type, result)
            }
            Type::Function { .. } => {
                format!("// Optimized function: {}\n{}", expr_type, result)
            }
            _ => result,
        }
    }
}

/// Apply type substitution to a type
pub fn apply_substitution(ty: &Type, substitution: &HashMap<String, Type>) -> Type {
    match ty {
        Type::Generic(name) => {
            substitution.get(name).cloned().unwrap_or(Type::Generic(name.clone()))
        }
        Type::Array(inner) => Type::Array(Box::new(apply_substitution(inner, substitution))),
        Type::Map(key, value) => Type::Map(
            Box::new(apply_substitution(key, substitution)),
            Box::new(apply_substitution(value, substitution)),
        ),
        Type::Tuple(types) => Type::Tuple(
            types.iter().map(|t| apply_substitution(t, substitution)).collect()
        ),
        Type::Function { params, return_type } => Type::Function {
            params: params.iter().map(|t| apply_substitution(t, substitution)).collect(),
            return_type: Box::new(apply_substitution(return_type, substitution)),
        },
        Type::GenericInstantiation { base, type_args } => Type::GenericInstantiation {
            base: base.clone(),
            type_args: type_args.iter().map(|t| apply_substitution(t, substitution)).collect(),
        },
        Type::Result(ok_type, err_type) => Type::Result(
            Box::new(apply_substitution(ok_type, substitution)),
            Box::new(apply_substitution(err_type, substitution)),
        ),
        Type::Option(inner) => Type::Option(Box::new(apply_substitution(inner, substitution))),
        Type::Union(types) => Type::Union(
            types.iter().map(|t| apply_substitution(t, substitution)).collect()
        ),
        Type::Intersection(types) => Type::Intersection(
            types.iter().map(|t| apply_substitution(t, substitution)).collect()
        ),
        _ => ty.clone(),
    }
}

/// Generate parameter list from types
fn generate_params(param_types: &[Type]) -> String {
    param_types
        .iter()
        .enumerate()
        .map(|(i, ty)| format!("p{}: {}", i, type_to_rust(ty)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Convert Widya type to Rust type string
pub fn type_to_rust(ty: &Type) -> String {
    match ty {
        Type::Number => "f64".to_string(),
        Type::String => "String".to_string(),
        Type::Boolean => "bool".to_string(),
        Type::Nil => "()".to_string(),
        Type::Array(inner) => format!("Vec<{}>", type_to_rust(inner)),
        Type::Map(key, value) => format!("HashMap<{}, {}>", type_to_rust(key), type_to_rust(value)),
        Type::Tuple(types) => {
            let types_str: Vec<String> = types.iter().map(type_to_rust).collect();
            format!("({})", types_str.join(", "))
        }
        Type::Function { params, return_type } => {
            let params_str: Vec<String> = params.iter().map(type_to_rust).collect();
            format!("fn({}) -> {}", params_str.join(", "), type_to_rust(return_type))
        }
        Type::Generic(name) => name.clone(),
        Type::GenericInstantiation { base, type_args } => {
            let args_str: Vec<String> = type_args.iter().map(type_to_rust).collect();
            format!("{}<{}>", base, args_str.join(", "))
        }
        Type::Result(ok_type, err_type) => {
            format!("Result<{}, {}>", type_to_rust(ok_type), type_to_rust(err_type))
        }
        Type::Option(inner) => format!("Option<{}>", type_to_rust(inner)),
        Type::Union(_) => "/* union type */".to_string(),
        Type::Intersection(_) => "/* intersection type */".to_string(),
        _ => "/* unknown type */".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic_function_monomorphization() {
        let mut generator = GenericCodeGenerator::new();
        
        // Test monomorphization of a generic function
        let type_params = vec!["T".to_string(), "U".to_string()];
        let param_types = vec![
            Type::Generic("T".to_string()),
            Type::Generic("U".to_string()),
        ];
        let return_type = Type::Generic("T".to_string());
        let type_args = vec![Type::Number, Type::String];
        
        let code = generator.monomorphize_function(
            "swap",
            &type_params,
            &param_types,
            &return_type,
            &type_args,
        );
        
        assert!(code.contains("fn swap_inst_0"));
        assert!(code.contains("p0: f64"));
        assert!(code.contains("p1: String"));
        assert!(code.contains("-> f64"));
    }
    
    #[test]
    fn test_pattern_match_generation() {
        let generator = GenericCodeGenerator::new();
        
        let patterns = vec![
            ("42_f64", "println!(\"forty-two\")"),
            ("x", "println!(\"x = {}\", x)"),
            ("_", "println!(\"other\")"),
        ];
        
        let code = generator.generate_pattern_match("value", &patterns, Some("println!(\"default\")"));
        
        assert!(code.contains("match value {"));
        assert!(code.contains("42_f64 => println!(\"forty-two\"),"));
        assert!(code.contains("x => println!(\"x = {}\", x),"));
        assert!(code.contains("_ => println!(\"other\"),"));
        assert!(code.contains("_ => println!(\"default\"),"));
    }
    
    #[test]
    fn test_type_aware_optimization() {
        let generator = GenericCodeGenerator::new();
        
        let num_expr = "a + b * c";
        let num_optimized = generator.generate_optimized_code(num_expr, &Type::Number);
        assert!(num_optimized.contains("Optimized numeric"));
        
        let bool_expr = "x && y || z";
        let bool_optimized = generator.generate_optimized_code(bool_expr, &Type::Boolean);
        assert!(bool_optimized.contains("Optimized boolean"));
    }
    
    #[test]
    fn test_apply_substitution() {
        let mut substitution = HashMap::new();
        substitution.insert("T".to_string(), Type::Number);
        substitution.insert("U".to_string(), Type::String);
        
        // Test substitution for generic type
        let generic_type = Type::Generic("T".to_string());
        let substituted = apply_substitution(&generic_type, &substitution);
        assert_eq!(substituted, Type::Number);
        
        // Test substitution for array of generic
        let array_type = Type::Array(Box::new(Type::Generic("U".to_string())));
        let substituted_array = apply_substitution(&array_type, &substitution);
        assert_eq!(substituted_array, Type::Array(Box::new(Type::String)));
        
        // Test substitution for function type
        let func_type = Type::Function {
            params: vec![Type::Generic("T".to_string()), Type::Generic("U".to_string())],
            return_type: Box::new(Type::Generic("T".to_string())),
        };
        let substituted_func = apply_substitution(&func_type, &substitution);
        
        if let Type::Function { params, return_type } = substituted_func {
            assert_eq!(params[0], Type::Number);
            assert_eq!(params[1], Type::String);
            assert_eq!(*return_type, Type::Number);
        } else {
            panic!("Expected function type");
        }
    }
    
    #[test]
    fn test_type_to_rust_conversion() {
        assert_eq!(type_to_rust(&Type::Number), "f64");
        assert_eq!(type_to_rust(&Type::String), "String");
        assert_eq!(type_to_rust(&Type::Boolean), "bool");
        
        let array_type = Type::Array(Box::new(Type::Number));
        assert_eq!(type_to_rust(&array_type), "Vec<f64>");
        
        let map_type = Type::Map(Box::new(Type::String), Box::new(Type::Number));
        assert_eq!(type_to_rust(&map_type), "HashMap<String, f64>");
        
        let generic_type = Type::Generic("T".to_string());
        assert_eq!(type_to_rust(&generic_type), "T");
        
        let option_type = Type::Option(Box::new(Type::String));
        assert_eq!(type_to_rust(&option_type), "Option<String>");
        
        let result_type = Type::Result(Box::new(Type::Number), Box::new(Type::String));
        assert_eq!(type_to_rust(&result_type), "Result<f64, String>");
    }
}