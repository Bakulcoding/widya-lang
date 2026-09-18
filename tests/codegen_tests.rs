use std::collections::HashMap;
use widya::compiler::generic_codegen::{apply_substitution, type_to_rust, GenericCodeGenerator};
use widya::typesystem::Type;

#[test]
fn test_codegen_monomorphize_function_generation() {
    let mut codegen = GenericCodeGenerator::new();

    let func_name = "identitas";
    let type_params = vec!["T".to_string()];
    let param_types = vec![Type::Generic("T".to_string())];
    let return_type = Type::Generic("T".to_string());
    let type_args = vec![Type::Number];

    let generated_code = codegen.monomorphize_function(
        func_name,
        &type_params,
        &param_types,
        &return_type,
        &type_args,
    );

    assert!(generated_code.contains("fn identitas_inst_0(p0: f64) -> f64"));
    assert!(generated_code.contains("0.0_f64"));
}

#[test]
fn test_codegen_pattern_match_generation() {
    let codegen = GenericCodeGenerator::new();

    let patterns = vec![
        ("1", "\"satu\""),
        ("2", "\"dua\""),
    ];

    let generated = codegen.generate_pattern_match("angka", &patterns, Some("\"lainnya\""));
    assert!(generated.contains("match angka {"));
    assert!(generated.contains("1 => \"satu\","));
    assert!(generated.contains("2 => \"dua\","));
    assert!(generated.contains("_ => \"lainnya\","));
}

#[test]
fn test_codegen_type_aware_arithmetic_optimizations() {
    let codegen = GenericCodeGenerator::new();

    let optimized_num = codegen.generate_optimized_code("x + 0.0", &Type::Number);
    assert!(optimized_num.contains("Optimized numeric"));
    assert!(optimized_num.contains("x"));

    let optimized_mul = codegen.generate_optimized_code("y * 1.0", &Type::Number);
    assert!(optimized_mul.contains("y"));

    let optimized_bool = codegen.generate_optimized_code("flag && true", &Type::Boolean);
    assert!(optimized_bool.contains("Optimized boolean"));
    assert!(optimized_bool.contains("flag"));
}

#[test]
fn test_codegen_type_to_rust_mapping() {
    assert_eq!(type_to_rust(&Type::Number), "f64");
    assert_eq!(type_to_rust(&Type::String), "String");
    assert_eq!(type_to_rust(&Type::Boolean), "bool");
    assert_eq!(type_to_rust(&Type::Nil), "()");
    assert_eq!(type_to_rust(&Type::Array(Box::new(Type::Number))), "Vec<f64>");
}

#[test]
fn test_codegen_substitution_application() {
    let mut subst = HashMap::new();
    subst.insert("T".to_string(), Type::Number);

    let ty = Type::Generic("T".to_string());
    assert_eq!(apply_substitution(&ty, &subst), Type::Number);

    let arr = Type::Array(Box::new(Type::Generic("T".to_string())));
    assert_eq!(apply_substitution(&arr, &subst), Type::Array(Box::new(Type::Number)));
}
