use widya::typesystem::{Substitution, Type, TypeEnvironment, TypeInferrer};

#[test]
fn test_type_inferrer_primitive_unification() {
    let mut inferrer = TypeInferrer::new();
    let res = inferrer.unify(&Type::Number, &Type::Number);
    assert!(res.is_ok());

    let res_diff = inferrer.unify(&Type::Number, &Type::String);
    assert!(res_diff.is_err());
}

#[test]
fn test_type_inferrer_type_var_unification() {
    let mut inferrer = TypeInferrer::new();
    let var_t = inferrer.new_type_variable();
    let res = inferrer.unify(&var_t, &Type::Number);
    assert!(res.is_ok());

    inferrer.add_substitution(res.unwrap());
    let substituted = inferrer.apply_substitution(&var_t);
    assert_eq!(substituted, Type::Number);
}

#[test]
fn test_type_inferrer_function_type_inference() {
    let mut inferrer = TypeInferrer::new();
    let param_t = inferrer.new_type_variable();
    let return_t = inferrer.new_type_variable();
    
    let fn_type = Type::Function {
        params: vec![param_t.clone()],
        return_type: Box::new(return_t.clone()),
    };

    let target_fn_type = Type::Function {
        params: vec![Type::Number],
        return_type: Box::new(Type::Boolean),
    };

    let res = inferrer.unify(&fn_type, &target_fn_type);
    assert!(res.is_ok());
    inferrer.add_substitution(res.unwrap());
    assert_eq!(inferrer.apply_substitution(&param_t), Type::Number);
    assert_eq!(inferrer.apply_substitution(&return_t), Type::Boolean);
}

#[test]
fn test_type_inferrer_generic_instantiation_unification() {
    let mut inferrer = TypeInferrer::new();
    let t_arg = inferrer.new_type_variable();
    let generic_inst = Type::GenericInstantiation {
        base: "Opsi".to_string(),
        type_args: vec![t_arg.clone()],
    };

    let concrete_inst = Type::GenericInstantiation {
        base: "Opsi".to_string(),
        type_args: vec![Type::String],
    };

    let res = inferrer.unify(&generic_inst, &concrete_inst);
    assert!(res.is_ok());
    inferrer.add_substitution(res.unwrap());
    assert_eq!(inferrer.apply_substitution(&t_arg), Type::String);
}

#[test]
fn test_type_inferrer_occurs_check() {
    let mut inferrer = TypeInferrer::new();
    let var_t = inferrer.new_type_variable();
    let recursive_type = Type::Function {
        params: vec![var_t.clone()],
        return_type: Box::new(Type::Number),
    };

    let res = inferrer.unify(&var_t, &recursive_type);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Occurs check failed"));
}

#[test]
fn test_type_inferrer_nested_substitution() {
    let mut inferrer = TypeInferrer::new();
    let t1 = inferrer.new_type_variable();
    let t2 = inferrer.new_type_variable();

    let s1 = inferrer.unify(&t1, &t2).unwrap();
    inferrer.add_substitution(s1);

    let s2 = inferrer.unify(&t2, &Type::Boolean).unwrap();
    inferrer.add_substitution(s2);

    assert_eq!(inferrer.apply_substitution(&t1), Type::Boolean);
    assert_eq!(inferrer.apply_substitution(&t2), Type::Boolean);
}

#[test]
fn test_type_display_formatting() {
    assert_eq!(format!("{}", Type::Number), "Angka");
    assert_eq!(format!("{}", Type::String), "String");
    assert_eq!(format!("{}", Type::Boolean), "Boolean");
    assert_eq!(format!("{}", Type::Nil), "Nihil");
    assert_eq!(format!("{}", Type::Array(Box::new(Type::Number))), "Daftar[Angka]");
}

#[test]
fn test_type_environment_binding() {
    let mut env = TypeEnvironment::new();
    env.bind("x".to_string(), Type::Number);
    assert_eq!(env.lookup("x"), Some(&Type::Number));
    assert_eq!(env.lookup("y"), None);
}

#[test]
fn test_substitution_composition() {
    let mut s1 = Substitution::new();
    s1.bind("T".to_string(), Type::Generic("U".to_string()));

    let mut s2 = Substitution::new();
    s2.bind("U".to_string(), Type::Number);

    let s3 = s1.compose(&s2);
    let ty = Type::Generic("T".to_string());
    assert_eq!(s3.apply(&ty), Type::Number);
}
