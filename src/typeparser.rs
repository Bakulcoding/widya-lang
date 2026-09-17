//! Type Parser - Helper functions for parsing generic types and patterns
//! (Internal use only - not meant to be a standalone module)

/// Type parser - provides helper functions for parsing advanced types
pub struct TypeParser;

impl TypeParser {
    pub fn new() -> Self {
        Self
    }
}

/// Pattern parser - provides helper functions for parsing patterns
pub struct PatternParser;

impl PatternParser {
    pub fn new() -> Self {
        Self
    }
}

/// Helper function to check if we're parsing a generic type
pub fn is_generic_type(type_name: &str) -> bool {
    type_name.chars().next().map_or(false, |c| c.is_uppercase())
}

/// Helper function to parse type parameter name
pub fn parse_type_param_name(param: &str) -> String {
    param.trim().to_string()
}

/// Helper function to check pattern syntax
pub fn validate_pattern_syntax(pattern: &str) -> bool {
    !pattern.is_empty()
}
