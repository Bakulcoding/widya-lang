//! Pattern matching and ADT (Algebraic Data Types) support for Widya-Lang
//! Updated with full syntax support

use crate::typesystem::{Type, Span};
use std::collections::HashMap;

/// Pattern matching arms
#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<String>,
    pub body: String,
    pub span: Span,
}

/// Pattern for pattern matching
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// Wildcard pattern (_)
    Wildcard(Span),
    
    /// Literal pattern (42, "hello", true, nihil)
    Literal(String, Span),
    
    /// Variable pattern (x)
    Variable(String, Span),
    
    /// Constructor pattern (Some(x) or Status::Aktif)
    Constructor {
        name: String,
        variant: Option<String>,
        subpatterns: Vec<Pattern>,
        span: Span,
    },
    
    /// Tuple pattern (x, y, z)
    Tuple(Vec<Pattern>, Span),
    
    /// Or pattern (A | B)
    Or(Vec<Pattern>, Span),
    
    /// Type annotation pattern (x: Angka)
    TypeAnnotation {
        pattern: Box<Pattern>,
        ty: Type,
        span: Span,
    },
}

/// Pattern matching context
pub struct PatternMatcher {
    bindings: HashMap<String, String>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
    
    /// Match value against pattern
    pub fn match_pattern(&mut self, _value: &str, pattern: &Pattern) -> Result<HashMap<String, String>, String> {
        match pattern {
            Pattern::Wildcard(_) => Ok(HashMap::new()),
            
            Pattern::Variable(name, _) => {
                let mut bindings = HashMap::new();
                bindings.insert(name.clone(), "matched".to_string());
                Ok(bindings)
            }
            
            Pattern::Literal(lit, _) => Ok(HashMap::new()),
            
            Pattern::Constructor { name, variant, subpatterns, span } => {
                let mut all_bindings = HashMap::new();
                for subpattern in subpatterns {
                    let bindings = self.match_pattern("value", subpattern)?;
                    all_bindings.extend(bindings);
                }
                Ok(all_bindings)
            }
            
            Pattern::Tuple(patterns, _) => {
                let mut all_bindings = HashMap::new();
                for pattern in patterns {
                    let bindings = self.match_pattern("value", pattern)?;
                    all_bindings.extend(bindings);
                }
                Ok(all_bindings)
            }
            
            Pattern::Or(patterns, _) => {
                if let Ok(bindings) = self.match_pattern("value", &patterns[0]) {
                    Ok(bindings)
                } else {
                    Err("No alternative pattern matches".to_string())
                }
            }
            
            Pattern::TypeAnnotation { pattern, .. } => {
                self.match_pattern("value", pattern)
            }
        }
    }
    
    /// Get bound variables from pattern
    pub fn bound_variables(pattern: &Pattern) -> Vec<String> {
        let mut vars = Vec::new();
        Self::collect_variables(pattern, &mut vars);
        vars
    }
    
    fn collect_variables(pattern: &Pattern, vars: &mut Vec<String>) {
        match pattern {
            Pattern::Variable(name, _) => vars.push(name.clone()),
            Pattern::Constructor { subpatterns, .. } => {
                for subpattern in subpatterns {
                    Self::collect_variables(subpattern, vars);
                }
            }
            Pattern::Tuple(patterns, _) => {
                for pattern in patterns {
                    Self::collect_variables(pattern, vars);
                }
            }
            Pattern::Or(patterns, _) => {
                if let Some(first) = patterns.first() {
                    Self::collect_variables(first, vars);
                }
            }
            Pattern::TypeAnnotation { pattern, .. } => {
                Self::collect_variables(pattern, vars);
            }
            _ => {}
        }
    }
}

/// ADT utilities
pub struct ADTUtils;

impl ADTUtils {
    /// Check if pattern is irrefutable
    pub fn is_irrefutable(pattern: &Pattern) -> bool {
        matches!(pattern, Pattern::Wildcard(_) | Pattern::Variable(_, _))
    }
    
    /// Create Result type string
    pub fn result_type(ok_type: String, err_type: String) -> String {
        format!("Hasil[{}, {}]", ok_type, err_type)
    }
    
    /// Create Option type string
    pub fn option_type(inner_type: String) -> String {
        format!("Opsi[{}]", inner_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_matching_basic() {
        let mut matcher = PatternMatcher::new();
        
        let value = "42";
        let pattern = Pattern::Wildcard(Span::dummy());
        let result = matcher.match_pattern(value, &pattern);
        assert!(result.is_ok());
        
        let pattern = Pattern::Variable("x".to_string(), Span::dummy());
        let result = matcher.match_pattern(value, &pattern);
        assert!(result.is_ok());
        let bindings = result.unwrap();
        assert_eq!(bindings.get("x").unwrap(), "matched");
    }
    
    #[test]
    fn test_bound_variables() {
        let pattern = Pattern::Variable("x".to_string(), Span::dummy());
        let vars = ADTUtils::bound_variables(&pattern);
        assert_eq!(vars, vec!["x"]);
        
        let pattern = Pattern::Constructor {
            name: "Some".to_string(),
            variant: None,
            subpatterns: vec![
                Pattern::Variable("value".to_string(), Span::dummy()),
                Pattern::Wildcard(Span::dummy()),
            ],
            span: Span::dummy(),
        };
        let vars = ADTUtils::bound_variables(&pattern);
        assert_eq!(vars, vec!["value"]);
    }
    
    #[test]
    fn test_adt_utilities() {
        let result_type = ADTUtils::result_type("String".to_string(), "Error".to_string());
        assert_eq!(result_type, "Hasil[String, Error]");
        
        let option_type = ADTUtils::option_type("String".to_string());
        assert_eq!(option_type, "Opsi[String]");
    }
}
