//! Advanced Type System for Widya-Lang
//! Implementation of generic types, type parameters, and algebraic data types

use std::fmt;

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
    pub variables: Vec<(String, String)>,
    pub generic_context: Vec<TypeParameter>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self { variables: Vec::new(), generic_context: Vec::new() }
    }
    
    pub fn enter_generic_context(&mut self, params: Vec<TypeParameter>) {
        self.generic_context.extend(params);
    }
    
    pub fn bind(&mut self, name: String, ty: String) {
        self.variables.push((name, ty));
    }
    
    pub fn lookup(&self, name: &str) -> Option<&String> {
        self.variables.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }
}

/// Type inference engine
pub struct TypeInferrer {
    env: TypeEnvironment,
    next_type_var_id: u64,
}

impl TypeInferrer {
    pub fn new() -> Self {
        Self { env: TypeEnvironment::new(), next_type_var_id: 0 }
    }
    
    pub fn new_type_variable(&mut self) -> Type {
        let id = self.next_type_var_id;
        self.next_type_var_id += 1;
        Type::Generic(format!("_T{}", id))
    }
    
    pub fn infer_expr(&mut self, _expr: &str) -> Type {
        Type::Unknown
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
        inferrer.env.bind("x".to_string(), "Angka".to_string());
        let x_type = inferrer.infer_expr("x");
        debug_assert_eq!(x_type, Type::Unknown);
    }
}
