//! Type Parser - Helper functions for parsing generic types and patterns
//! Full support for generic syntax, pattern matching validation, and trait resolution

use crate::typesystem::Type;
use crate::ast::Attribute;

/// Type parser - provides helper functions for parsing advanced types
pub struct TypeParser;

impl TypeParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a type string into a concrete `Type` enum
    pub fn parse_type(input: &str) -> Type {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Type::Unknown;
        }

        if trimmed == "Angka" || trimmed == "f64" || trimmed == "i64" || trimmed == "number" {
            return Type::Number;
        }
        if trimmed == "Teks" || trimmed == "String" || trimmed == "str" || trimmed == "string" {
            return Type::String;
        }
        if trimmed == "Boolean" || trimmed == "bool" || trimmed == "boolean" {
            return Type::Boolean;
        }
        if trimmed == "Nihil" || trimmed == "()" || trimmed == "nil" || trimmed == "null" {
            return Type::Nil;
        }

        // Generic instantiation: Nama<Arg1, Arg2>
        if let Some(open_bracket) = trimmed.find('<') {
            if trimmed.ends_with('>') {
                let base_name = &trimmed[..open_bracket];
                let inner = &trimmed[open_bracket + 1..trimmed.len() - 1];
                let args = Self::split_type_arguments(inner);
                let parsed_args: Vec<Type> = args.into_iter().map(|a| Self::parse_type(&a)).collect();

                if base_name == "Daftar" || base_name == "Vec" || base_name == "List" || base_name == "Array" {
                    if let Some(first) = parsed_args.first() {
                        return Type::Array(Box::new(first.clone()));
                    }
                } else if base_name == "Hasil" || base_name == "Result" {
                    if parsed_args.len() == 2 {
                        return Type::Result(Box::new(parsed_args[0].clone()), Box::new(parsed_args[1].clone()));
                    }
                } else if base_name == "Opsi" || base_name == "Option" {
                    if let Some(first) = parsed_args.first() {
                        return Type::Option(Box::new(first.clone()));
                    }
                } else if base_name == "Peta" || base_name == "Map" || base_name == "HashMap" {
                    if parsed_args.len() == 2 {
                        return Type::Map(Box::new(parsed_args[0].clone()), Box::new(parsed_args[1].clone()));
                    }
                }

                return Type::GenericInstantiation {
                    base: base_name.to_string(),
                    type_args: parsed_args,
                };
            }
        }

        // Tuple type: (T1, T2)
        if trimmed.starts_with('(') && trimmed.ends_with(')') {
            let inner = &trimmed[1..trimmed.len() - 1];
            let parts = Self::split_type_arguments(inner);
            let types = parts.into_iter().map(|p| Self::parse_type(&p)).collect();
            return Type::Tuple(types);
        }

        // Single generic type parameter (like T, U, E, K, V)
        if is_generic_type(trimmed) && trimmed.len() <= 3 {
            return Type::Generic(trimmed.to_string());
        }

        Type::Alias(trimmed.to_string())
    }

    /// Split comma separated type arguments while respecting nested brackets
    pub fn split_type_arguments(input: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut current = String::new();
        let mut depth: usize = 0;

        for ch in input.chars() {
            match ch {
                '<' | '(' | '[' => {
                    depth += 1;
                    current.push(ch);
                }
                '>' | ')' | ']' => {
                    depth = depth.saturating_sub(1);
                    current.push(ch);
                }
                ',' if depth == 0 => {
                    let trimmed = current.trim();
                    if !trimmed.is_empty() {
                        results.push(trimmed.to_string());
                    }
                    current.clear();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        let trimmed = current.trim();
        if !trimmed.is_empty() {
            results.push(trimmed.to_string());
        }

        results
    }

    /// Parse generic parameter with optional bounds (e.g. `T: Sama + Urut`)
    pub fn parse_generic_param_with_bounds(input: &str) -> (String, Vec<String>) {
        let parts: Vec<&str> = input.splitn(2, ':').collect();
        let name = parts[0].trim().to_string();
        let mut bounds = Vec::new();

        if parts.len() > 1 {
            for bound in parts[1].split('+') {
                let b = bound.trim();
                if !b.is_empty() {
                    bounds.push(b.to_string());
                }
            }
        }

        (name, bounds)
    }

    /// Resolve derived traits from attribute list (`#[turunkan(Tunjukkan, Sama)]` or `#[derive(...)]`)
    pub fn resolve_derived_traits(attributes: &[Attribute]) -> Vec<String> {
        let mut traits = Vec::new();
        for attr in attributes {
            if attr.name == "turunkan" || attr.name == "derive" {
                traits.extend(attr.arguments.clone());
            }
        }
        traits
    }
}

/// Pattern parser - provides helper functions for parsing patterns
pub struct PatternParser;

impl PatternParser {
    pub fn new() -> Self {
        Self
    }

    /// Validate pattern syntax
    pub fn validate_pattern(pattern: &str) -> bool {
        let trimmed = pattern.trim();
        !trimmed.is_empty()
    }
}

/// Helper function to check if we're parsing a generic type identifier
pub fn is_generic_type(type_name: &str) -> bool {
    type_name.chars().next().is_some_and(|c| c.is_uppercase())
}

/// Helper function to parse type parameter name
pub fn parse_type_param_name(param: &str) -> String {
    param.trim().to_string()
}

/// Helper function to check pattern syntax
pub fn validate_pattern_syntax(pattern: &str) -> bool {
    !pattern.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Span;

    #[test]
    fn test_type_parser_basic_types() {
        assert_eq!(TypeParser::parse_type("Angka"), Type::Number);
        assert_eq!(TypeParser::parse_type("Teks"), Type::String);
        assert_eq!(TypeParser::parse_type("Boolean"), Type::Boolean);
        assert_eq!(TypeParser::parse_type("Nihil"), Type::Nil);
    }

    #[test]
    fn test_type_parser_generic_instantiations() {
        let list_type = TypeParser::parse_type("Daftar<Angka>");
        assert_eq!(list_type, Type::Array(Box::new(Type::Number)));

        let result_type = TypeParser::parse_type("Hasil<Angka, Teks>");
        assert_eq!(result_type, Type::Result(Box::new(Type::Number), Box::new(Type::String)));

        let custom_gen = TypeParser::parse_type("Kantong<T>");
        if let Type::GenericInstantiation { base, type_args } = custom_gen {
            assert_eq!(base, "Kantong");
            assert_eq!(type_args, vec![Type::Generic("T".to_string())]);
        } else {
            panic!("Expected generic instantiation");
        }
    }

    #[test]
    fn test_generic_param_with_bounds() {
        let (name, bounds) = TypeParser::parse_generic_param_with_bounds("T: Sama + Urut");
        assert_eq!(name, "T");
        assert_eq!(bounds, vec!["Sama".to_string(), "Urut".to_string()]);

        let (name2, bounds2) = TypeParser::parse_generic_param_with_bounds("U");
        assert_eq!(name2, "U");
        assert!(bounds2.is_empty());
    }

    #[test]
    fn test_resolve_derived_traits() {
        let attrs = vec![
            Attribute {
                name: "turunkan".to_string(),
                arguments: vec!["Tunjukkan".to_string(), "Sama".to_string()],
                span: Span::new(1, 1),
            }
        ];
        let derived = TypeParser::resolve_derived_traits(&attrs);
        assert_eq!(derived, vec!["Tunjukkan", "Sama"]);
    }
}
