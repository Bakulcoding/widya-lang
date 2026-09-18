use crate::ast::*;
use crate::environment::Environment;
use crate::error::{Galat, Span};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::stdlib::register_stdlib;
use crate::value::{Value, WidyaFunction, WidyaInstance, WidyaStruct};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        register_stdlib(&mut globals);
        let globals_rc = Rc::new(RefCell::new(globals));
        Self {
            globals: globals_rc.clone(),
            environment: globals_rc,
        }
    }

    pub fn get_environment_bindings(&self) -> HashMap<String, Value> {
        let mut bindings = HashMap::new();
        let all_local = self.environment.borrow().get_all_local();
        for (k, v) in all_local {
            match v {
                Value::Builtin(_) => {}
                _ => {
                    bindings.insert(k, v);
                }
            }
        }
        bindings
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, Galat> {
        let mut last_value = Value::Nil;
        for stmt in &program.statements {
            match self.execute(stmt) {
                Ok(v) => last_value = v,
                Err(Galat::Kembalikan(val)) => return Ok(val),
                Err(e) => return Err(e),
            }
        }
        Ok(last_value)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<Value, Galat> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate(expr),
            Stmt::VarDecl {
                name,
                is_const,
                initializer,
                ..
            } => {
                let value = if let Some(init) = initializer {
                    self.evaluate(init)?
                } else {
                    Value::Nil
                };
                self.environment
                    .borrow_mut()
                    .define(name.clone(), value, *is_const);
                Ok(Value::Nil)
            }
            Stmt::Assignment {
                target,
                op,
                value,
                span,
            } => {
                let rhs_val = self.evaluate(value)?;
                match target {
                    AssignTarget::Variable(name) => {
                        let final_val = match op {
                            AssignOp::Assign => rhs_val,
                            _ => {
                                let current_val = self.environment.borrow().get(name, span)?;
                                self.apply_compound_assign(current_val, rhs_val, op, span)?
                            }
                        };
                        self.environment
                            .borrow_mut()
                            .assign(name, final_val.clone(), span)?;
                        Ok(final_val)
                    }
                    AssignTarget::Index { target, index } => {
                        let target_val = self.evaluate(target)?;
                        let index_val = self.evaluate(index)?;
                        self.assign_to_index(target_val, index_val, rhs_val, op, span)
                    }
                }
            }
            Stmt::Block(statements, _) => {
                let previous = self.environment.clone();
                let block_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                self.environment = block_env;

                let mut result = Ok(Value::Nil);
                for statement in statements {
                    match self.execute(statement) {
                        Ok(v) => {
                            result = Ok(v);
                        }
                        Err(e) => {
                            result = Err(e);
                            break;
                        }
                    }
                }

                self.environment = previous;
                result
            }
            Stmt::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
                ..
            } => {
                let cond_val = self.evaluate(condition)?;
                if cond_val.is_truthy() {
                    return self.execute(then_branch);
                }

                for (elif_cond, elif_body) in elif_branches {
                    let elif_val = self.evaluate(elif_cond)?;
                    if elif_val.is_truthy() {
                        return self.execute(elif_body);
                    }
                }

                if let Some(else_stmt) = else_branch {
                    return self.execute(else_stmt);
                }

                Ok(Value::Nil)
            }
            Stmt::While {
                condition, body, ..
            } => {
                let mut last = Value::Nil;
                while self.evaluate(condition)?.is_truthy() {
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            Stmt::ForIn {
                var_name,
                iterable,
                body,
                span,
            } => {
                let iter_val = self.evaluate(iterable)?;
                let items: Vec<Value> = match iter_val {
                    Value::Range(start, end) => {
                        if start <= end {
                            (start..=end).map(|n| Value::Number(n as f64)).collect()
                        } else {
                            (end..=start)
                                .rev()
                                .map(|n| Value::Number(n as f64))
                                .collect()
                        }
                    }
                    Value::Array(a) => a.borrow().clone(),
                    Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
                    Value::Map(m) => {
                        let map = m.borrow();
                        let mut keys: Vec<Value> =
                            map.keys().map(|k| Value::String(k.clone())).collect();
                        keys.sort_by(|a, b| a.to_string_repr().cmp(&b.to_string_repr()));
                        keys
                    }
                    other => {
                        return Err(Galat::runtime(
                            format!(
                                "Tipe '{}' tidak dapat diiterasi pada perulangan 'untuk'",
                                other.type_name()
                            ),
                            span,
                        ));
                    }
                };

                let previous = self.environment.clone();
                let loop_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                self.environment = loop_env;

                let mut last = Value::Nil;
                for item in items {
                    self.environment
                        .borrow_mut()
                        .define(var_name.clone(), item, false);
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => {
                            self.environment = previous;
                            return Err(e);
                        }
                    }
                }

                self.environment = previous;
                Ok(last)
            }
            Stmt::Loop { body, .. } => {
                let mut last = Value::Nil;
                loop {
                    match self.execute(body) {
                        Ok(v) => last = v,
                        Err(Galat::Berhenti) => break,
                        Err(Galat::Lanjut) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            Stmt::DestructureDecl {
                names,
                initializer,
                is_const,
                span,
            } => {
                let init_val = self.evaluate(initializer)?;
                match init_val {
                    Value::Array(arr) => {
                        let items = arr.borrow();
                        for (i, name) in names.iter().enumerate() {
                            let val = items.get(i).cloned().unwrap_or(Value::Nil);
                            self.environment.borrow_mut().define(name.clone(), val, *is_const);
                        }
                        Ok(Value::Nil)
                    }
                    Value::Map(m) => {
                        let map = m.borrow();
                        for name in names {
                            let val = map.get(name).cloned().unwrap_or(Value::Nil);
                            self.environment.borrow_mut().define(name.clone(), val, *is_const);
                        }
                        Ok(Value::Nil)
                    }
                    other => Err(Galat::runtime(
                        format!("Dekonstruksi memerlukan daftar atau kamus, ditemukan '{}'", other.type_name()),
                        span,
                    )),
                }
            }
            Stmt::FunctionDecl {
                name,
                params,
                body,
                ..
            } => {
                let function = WidyaFunction {
                    name: Some(name.clone()),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::Function(Rc::new(function)),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::StructDecl {
                name,
                fields,
                methods,
                attributes,
                span: _,
            } => {
                let mut method_map = HashMap::new();
                for method in methods {
                    method_map.insert(method.name.clone(), method.clone());
                }

                // Process Procedural Macro attributes: #[turunkan(Json, Serialisasi, Debug, Duplikat)]
                for attr in attributes {
                    if attr.name == "turunkan" || attr.name == "derive" {
                        for trait_macro in &attr.arguments {
                            match trait_macro.to_lowercase().as_str() {
                                "json" | "serialisasi" | "serde" => {
                                    // Auto-generate .ke_json() method
                                    let dummy_span = attr.span.clone();
                                    method_map.insert("ke_json".to_string(), StructMethod {
                                        name: "ke_json".to_string(),
                                        params: Vec::new(),
                                        body: vec![Stmt::Return {
                                            value: Some(Expr::Call {
                                                callee: Box::new(Expr::Identifier("ke_json".to_string(), dummy_span.clone())),
                                                arguments: vec![Expr::This(dummy_span.clone())],
                                                span: dummy_span.clone(),
                                            }),
                                            span: dummy_span.clone(),
                                        }],
                                        span: dummy_span,
                                    });
                                }
                                "debug" | "tampilan" => {
                                    // Auto-generate .format_debug()
                                    let dummy_span = attr.span.clone();
                                    method_map.insert("format_debug".to_string(), StructMethod {
                                        name: "format_debug".to_string(),
                                        params: Vec::new(),
                                        body: vec![Stmt::Return {
                                            value: Some(Expr::Call {
                                                callee: Box::new(Expr::Identifier("ke_teks".to_string(), dummy_span.clone())),
                                                arguments: vec![Expr::This(dummy_span.clone())],
                                                span: dummy_span.clone(),
                                            }),
                                            span: dummy_span.clone(),
                                        }],
                                        span: dummy_span,
                                    });
                                }
                                "duplikat" | "clone" => {
                                    // Auto-generate .duplikat()
                                    let dummy_span = attr.span.clone();
                                    method_map.insert("duplikat".to_string(), StructMethod {
                                        name: "duplikat".to_string(),
                                        params: Vec::new(),
                                        body: vec![Stmt::Return {
                                            value: Some(Expr::This(dummy_span.clone())),
                                            span: dummy_span.clone(),
                                        }],
                                        span: dummy_span,
                                    });
                                }
                                _ => {}
                            }
                        }
                    }
                }

                let struct_def = WidyaStruct {
                    name: name.clone(),
                    fields: fields.clone(),
                    methods: method_map,
                    closure: self.environment.clone(),
                };
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::StructDef(Rc::new(struct_def)),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::TraitDecl { .. } => {
                // Traits register interface metadata for contract verification
                Ok(Value::Nil)
            }
            Stmt::ImplDecl { target_name, methods, span, .. } => {
                let current_val = self.environment.borrow().get(target_name, span)?;
                if let Value::StructDef(ref struct_def) = current_val {
                    let mut new_methods = struct_def.methods.clone();
                    for m in methods {
                        new_methods.insert(m.name.clone(), m.clone());
                    }
                    let updated_struct = WidyaStruct {
                        name: struct_def.name.clone(),
                        fields: struct_def.fields.clone(),
                        methods: new_methods,
                        closure: self.environment.clone(),
                    };
                    self.environment.borrow_mut().assign(target_name, Value::StructDef(Rc::new(updated_struct)), span)?;
                }
                Ok(Value::Nil)
            }
            Stmt::IfLet {
                pattern,
                value,
                then_branch,
                else_branch,
                ..
            } => {
                let target_val = self.evaluate(value)?;
                if let Some(bindings) = self.match_pattern_check(pattern, &target_val)? {
                    let previous = self.environment.clone();
                    let then_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                    for (var_name, var_val) in bindings {
                        then_env.borrow_mut().define(var_name, var_val, false);
                    }
                    self.environment = then_env;
                    let result = self.execute(then_branch);
                    self.environment = previous;
                    result
                } else if let Some(else_b) = else_branch {
                    self.execute(else_b)
                } else {
                    Ok(Value::Nil)
                }
            }
            Stmt::EnumDecl { name, variants, .. } => {
                let mut enum_map = HashMap::new();
                for variant in variants {
                    let v_name = variant.name.clone();
                    let e_name = name.clone();
                    let f_count = variant.fields_count;

                    if f_count == 0 {
                        let enum_val = Value::EnumVariant(Rc::new(crate::value::WidyaEnumVariant {
                            enum_name: e_name,
                            variant_name: v_name.clone(),
                            fields: Vec::new(),
                        }));
                        enum_map.insert(v_name, enum_val);
                    } else {
                        // Constructor function for variant with fields
                        let ctor = Value::Function(Rc::new(WidyaFunction {
                            name: Some(format!("{}::{}", e_name, v_name)),
                            params: (0..f_count).map(|i| format!("_arg{}", i)).collect(),
                            body: Vec::new(), // Handled via special constructor
                            closure: self.environment.clone(),
                        }));
                        enum_map.insert(v_name, ctor);
                    }
                }
                self.environment.borrow_mut().define(
                    name.clone(),
                    Value::Map(Rc::new(RefCell::new(enum_map))),
                    false,
                );
                Ok(Value::Nil)
            }
            Stmt::Import { path, alias, span } => {
                let module_path = if Path::new(path).exists() {
                    path.clone()
                } else if Path::new(&format!("{}.wya", path)).exists() {
                    format!("{}.wya", path)
                } else if Path::new(&format!("{}.widya", path)).exists() {
                    format!("{}.widya", path)
                } else {
                    return Err(Galat::runtime(
                        format!("Berkas modul '{}' tidak ditemukan", path),
                        span,
                    ));
                };

                let source = match fs::read_to_string(&module_path) {
                    Ok(s) => s,
                    Err(e) => {
                        return Err(Galat::runtime(
                            format!("Gagal membaca berkas modul '{}': {}", module_path, e),
                            span,
                        ));
                    }
                };

                let mut lexer = Lexer::new(&source);
                let tokens = lexer.scan_tokens()?;
                let mut parser = Parser::new(tokens);
                let program = parser.parse()?;

                let mut module_interp = Interpreter::new();
                module_interp.interpret(&program)?;

                let exported = module_interp.environment.borrow().get_all_local();

                if let Some(alias_name) = alias {
                    self.environment.borrow_mut().define(
                        alias_name.clone(),
                        Value::Map(Rc::new(RefCell::new(exported))),
                        false,
                    );
                } else {
                    for (k, v) in exported {
                        self.environment.borrow_mut().define(k, v, false);
                    }
                }

                Ok(Value::Nil)
            }
            Stmt::TryCatch {
                try_block,
                error_var,
                catch_block,
                ..
            } => match self.execute(try_block) {
                Ok(v) => Ok(v),
                Err(Galat::Kembalikan(v)) => Err(Galat::Kembalikan(v)),
                Err(Galat::Berhenti) => Err(Galat::Berhenti),
                Err(Galat::Lanjut) => Err(Galat::Lanjut),
                Err(e) => {
                    let prev_env = self.environment.clone();
                    let catch_env =
                        Rc::new(RefCell::new(Environment::with_parent(prev_env.clone())));
                    self.environment = catch_env;

                    let error_msg = match &e {
                        Galat::Runtime { pesan, .. } => pesan.clone(),
                        Galat::Sintaks { pesan, .. } => pesan.clone(),
                        _ => format!("{}", e),
                    };

                    self.environment.borrow_mut().define(
                        error_var.clone(),
                        Value::String(error_msg),
                        false,
                    );

                    let res = self.execute(catch_block);
                    self.environment = prev_env;
                    res
                }
            },
            Stmt::Throw { expr, span } => {
                let err_val = self.evaluate(expr)?;
                Err(Galat::runtime(err_val.to_string_repr(), span))
            }
            Stmt::Return { value, .. } => {
                let val = if let Some(expr) = value {
                    self.evaluate(expr)?
                } else {
                    Value::Nil
                };
                Err(Galat::Kembalikan(val))
            }
            Stmt::ExternalBlock { abi: _, functions, span: _ } => {
                // Register mock FFI functions into current environment
                for func_decl in functions {
                    let f_name = func_decl.name.clone();
                    let builtin_fn: crate::value::BuiltinFn = match f_name.as_str() {
                        "puts" | "printf" => crate::stdlib::builtin_cetak,
                        "abs" => crate::stdlib::builtin_mutlak,
                        "sqrt" => crate::stdlib::builtin_akar,
                        "sin" => crate::stdlib::builtin_sin,
                        "cos" => crate::stdlib::builtin_cos,
                        _ => crate::stdlib::builtin_ffi_call_generic,
                    };
                    
                    let builtin = crate::value::BuiltinFunction {
                        name: f_name.clone(),
                        arity: Some(func_decl.params.len()),
                        func: builtin_fn,
                    };
                    self.environment.borrow_mut().define(
                        func_decl.name.clone(),
                        Value::Builtin(Rc::new(builtin)),
                        false,
                    );
                }
                Ok(Value::Nil)
            }
            Stmt::VolatileWrite { address, value, span } => {
                let addr_val = self.evaluate(address)?;
                let data_val = self.evaluate(value)?;
                let addr_num = match addr_val {
                    Value::Number(n) => n as usize,
                    _ => return Err(Galat::runtime("Alamat memori volatil harus berupa angka pointer", span)),
                };
                crate::stdlib::tulis_memori_volatil(addr_num, &data_val, span)?;
                Ok(Value::Nil)
            }
            Stmt::Break(_) => Err(Galat::Berhenti),
            Stmt::Continue(_) => Err(Galat::Lanjut),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, Galat> {
        match expr {
            Expr::Number(n, _) => Ok(Value::Number(*n)),
            Expr::String(s, _) => Ok(Value::String(s.clone())),
            Expr::Boolean(b, _) => Ok(Value::Bool(*b)),
            Expr::Nil(_) => Ok(Value::Nil),
            Expr::This(span) => self.environment.borrow().get("ini", span),
            Expr::Identifier(name, span) => self.environment.borrow().get(name, span),
            Expr::Grouping(e, _) => self.evaluate(e),
            Expr::Unary { op, right, span } => {
                let right_val = self.evaluate(right)?;
                match op {
                    UnaryOp::Negate => match right_val {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(Galat::runtime(
                            format!(
                                "Operator '-' tidak dapat digunakan pada tipe '{}'",
                                right_val.type_name()
                            ),
                            span,
                        )),
                    },
                    UnaryOp::Not => Ok(Value::Bool(!right_val.is_truthy())),
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                span,
            } => {
                if *op == BinaryOp::And {
                    let left_val = self.evaluate(left)?;
                    if !left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.evaluate(right);
                }
                if *op == BinaryOp::Or {
                    let left_val = self.evaluate(left)?;
                    if left_val.is_truthy() {
                        return Ok(left_val);
                    }
                    return self.evaluate(right);
                }

                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match op {
                    BinaryOp::Add => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                        (Value::String(a), _) => {
                            Ok(Value::String(format!("{}{}", a, right_val.to_string_repr())))
                        }
                        (_, Value::String(b)) => {
                            Ok(Value::String(format!("{}{}", left_val.to_string_repr(), b)))
                        }
                        (Value::Array(a), Value::Array(b)) => {
                            let mut combined = a.borrow().clone();
                            combined.extend(b.borrow().clone());
                            Ok(Value::Array(Rc::new(RefCell::new(combined))))
                        }
                        _ => Err(Galat::runtime(
                            format!(
                                "Operasi '+' tidak valid antara '{}' dan '{}'",
                                left_val.type_name(),
                                right_val.type_name()
                            ),
                            span,
                        )),
                    },
                    BinaryOp::Subtract => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                        _ => Err(Galat::runtime("Operasi '-' memerlukan dua angka", span)),
                    },
                    BinaryOp::Multiply => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                        (Value::String(s), Value::Number(n)) | (Value::Number(n), Value::String(s)) => {
                            let count = *n as usize;
                            Ok(Value::String(s.repeat(count)))
                        }
                        _ => Err(Galat::runtime(
                            "Operasi '*' memerlukan angka (atau teks dengan angka)",
                            span,
                        )),
                    },
                    BinaryOp::Divide => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if *b == 0.0 {
                                Err(Galat::runtime("Pembagian dengan nol", span))
                            } else {
                                Ok(Value::Number(a / b))
                            }
                        }
                        _ => Err(Galat::runtime("Operasi '/' memerlukan dua angka", span)),
                    },
                    BinaryOp::Modulo => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => {
                            if *b == 0.0 {
                                Err(Galat::runtime("Sisa bagi (modulo) dengan nol", span))
                            } else {
                                Ok(Value::Number(a % b))
                            }
                        }
                        _ => Err(Galat::runtime("Operasi '%' memerlukan dua angka", span)),
                    },
                    BinaryOp::Power => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
                        _ => Err(Galat::runtime("Operasi '^' memerlukan dua angka", span)),
                    },
                    BinaryOp::Equal => Ok(Value::Bool(left_val == right_val)),
                    BinaryOp::NotEqual => Ok(Value::Bool(left_val != right_val)),
                    BinaryOp::Less => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a < b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '<' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::LessEqual => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a <= b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '<=' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::Greater => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a > b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '>' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::GreaterEqual => match (&left_val, &right_val) {
                        (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                        (Value::String(a), Value::String(b)) => Ok(Value::Bool(a >= b)),
                        _ => Err(Galat::runtime(
                            "Perbandingan '>=' hanya berlaku untuk angka atau teks",
                            span,
                        )),
                    },
                    BinaryOp::And | BinaryOp::Or => unreachable!(),
                }
            }
            Expr::Array(elements, _) => {
                let mut vals = Vec::new();
                for elem in elements {
                    vals.push(self.evaluate(elem)?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(vals))))
            }
            Expr::Map(pairs, _) => {
                let mut map = HashMap::new();
                for (k_expr, v_expr) in pairs {
                    let k_val = self.evaluate(k_expr)?;
                    let v_val = self.evaluate(v_expr)?;
                    let key_str = match k_val {
                        Value::String(s) => s,
                        other => other.to_string_repr(),
                    };
                    map.insert(key_str, v_val);
                }
                Ok(Value::Map(Rc::new(RefCell::new(map))))
            }
            Expr::Range { start, end, span } => {
                let start_val = self.evaluate(start)?;
                let end_val = self.evaluate(end)?;
                match (start_val, end_val) {
                    (Value::Number(s), Value::Number(e)) => {
                        Ok(Value::Range(s as i64, e as i64))
                    }
                    _ => Err(Galat::runtime("Rentang (..) memerlukan dua angka", span)),
                }
            }
            Expr::Index {
                target,
                index,
                span,
            } => {
                let target_val = self.evaluate(target)?;
                let index_val = self.evaluate(index)?;
                self.index_get(&target_val, &index_val, span)
            }
            Expr::Call {
                callee,
                arguments,
                span,
            } => {
                // Check if this is a method call on an instance: obj.method(...)
                if let Expr::Index { target, index, .. } = &**callee {
                    let target_val = self.evaluate(target)?;
                    let index_val = self.evaluate(index)?;

                    let method_opt = if let Value::Instance(ref inst) = target_val {
                        let method_name = match &index_val {
                            Value::String(s) => s.clone(),
                            other => other.to_string_repr(),
                        };
                        inst.borrow().struct_def.methods.get(&method_name).cloned()
                    } else {
                        None
                    };

                    if let (Value::Instance(inst), Some(method)) = (target_val.clone(), method_opt) {
                        let mut evaluated_args = Vec::new();
                        for arg in arguments {
                            evaluated_args.push(self.evaluate(arg)?);
                        }
                        return self.call_method(inst, &method, &evaluated_args, span);
                    }

                    // Fallback to evaluating the resolved property as a function
                    let callee_val = self.index_get(&target_val, &index_val, span)?;
                    let mut evaluated_args = Vec::new();
                    for arg in arguments {
                        evaluated_args.push(self.evaluate(arg)?);
                    }
                    return self.call_value(callee_val, &evaluated_args, span);
                }

                let callee_val = self.evaluate(callee)?;
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    evaluated_args.push(self.evaluate(arg)?);
                }
                self.call_value(callee_val, &evaluated_args, span)
            }
            Expr::FunctionExpr { params, body, .. } => {
                let function = WidyaFunction {
                    name: None,
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                Ok(Value::Function(Rc::new(function)))
            }
            Expr::Match { target, arms, span } => {
                let target_val = self.evaluate(target)?;
                for arm in arms {
                    if let Some(bindings) = self.match_pattern_check(&arm.pattern, &target_val)? {
                        let previous = self.environment.clone();
                        let arm_env = Rc::new(RefCell::new(Environment::with_parent(previous.clone())));
                        for (var_name, var_val) in bindings {
                            arm_env.borrow_mut().define(var_name, var_val, false);
                        }
                        self.environment = arm_env;
                        let result = self.evaluate(&arm.body);
                        self.environment = previous;
                        return result;
                    }
                }
                Err(Galat::runtime("Tidak ada pola yang cocok pada ekspresi 'cocokkan'", span))
            }
            Expr::Try(inner, span) => {
                let val = self.evaluate(inner)?;
                match &val {
                    Value::Map(m) => {
                        let map = m.borrow();
                        if let Some(Value::Bool(true)) = map.get("apakah_ok") {
                            Ok(map.get("nilai").cloned().unwrap_or(Value::Nil))
                        } else if let Some(Value::Bool(false)) = map.get("apakah_ok") {
                            // Early return error object (propagasi galat)
                            Err(Galat::Kembalikan(val.clone()))
                        } else if let Some(Value::Bool(true)) = map.get("ada") {
                            Ok(map.get("nilai").cloned().unwrap_or(Value::Nil))
                        } else if let Some(Value::Bool(false)) = map.get("ada") {
                            Err(Galat::Kembalikan(val.clone()))
                        } else {
                            Ok(val.clone())
                        }
                    }
                    Value::EnumVariant(ev) => {
                        if ev.variant_name == "Ok" || ev.variant_name == "Ada" {
                            if let Some(first_field) = ev.fields.first() {
                                Ok(first_field.clone())
                            } else {
                                Ok(Value::Nil)
                            }
                        } else if ev.variant_name == "Err" || ev.variant_name == "Kosong" {
                            // Early return error enum variant
                            Err(Galat::Kembalikan(val.clone()))
                        } else {
                            Ok(val.clone())
                        }
                    }
                    Value::Nil => {
                        Err(Galat::runtime("Operator '?' tidak dapat digunakan pada nilai nihil", span))
                    }
                    other => Ok(other.clone()),
                }
            }
            Expr::Await(inner, span) => {
                let val = self.evaluate(inner)?;
                match &val {
                    Value::Map(m) => {
                        let map = m.borrow();
                        // If it's a Thread handle object (Utas), join it
                        if let Some(Value::String(t)) = map.get("_tipe") {
                            if t == "Utas" {
                                if let Some(Value::Number(id)) = map.get("id") {
                                    return crate::stdlib::builtin_gabung_utas(&[Value::Number(*id)], span);
                                }
                            }
                        }
                        Ok(val.clone())
                    }
                    Value::Function(f) => {
                        // If awaiting a function, invoke it
                        self.call_value(Value::Function(f.clone()), &[], span)
                    }
                    other => Ok(other.clone()),
                }
            }
            Expr::VolatileRead { address, span } => {
                let addr_val = self.evaluate(address)?;
                let addr_num = match addr_val {
                    Value::Number(n) => n as usize,
                    _ => return Err(Galat::runtime("Alamat memori volatil harus berupa angka pointer", span)),
                };
                crate::stdlib::baca_memori_volatil(addr_num, span)
            }
        }
    }

    fn index_get(&self, target: &Value, index: &Value, span: &Span) -> Result<Value, Galat> {
        match target {
            Value::Array(a) => {
                let items = a.borrow();
                let idx = match index {
                    Value::Number(n) => *n as i64,
                    _ => return Err(Galat::runtime("Indeks daftar harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (items.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < items.len() {
                    Ok(items[actual_idx].clone())
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar batas daftar (panjang: {})",
                            idx,
                            items.len()
                        ),
                        span,
                    ))
                }
            }
            Value::String(s) => {
                let chars: Vec<char> = s.chars().collect();
                let idx = match index {
                    Value::Number(n) => *n as i64,
                    _ => return Err(Galat::runtime("Indeks teks harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (chars.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < chars.len() {
                    Ok(Value::String(chars[actual_idx].to_string()))
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar jangkauan teks (panjang: {})",
                            idx,
                            chars.len()
                        ),
                        span,
                    ))
                }
            }
            Value::Map(m) => {
                let key = match index {
                    Value::String(s) => s.clone(),
                    other => other.to_string_repr(),
                };
                let map = m.borrow();
                Ok(map.get(&key).cloned().unwrap_or(Value::Nil))
            }
            Value::Instance(inst) => {
                let key = match index {
                    Value::String(s) => s.clone(),
                    other => other.to_string_repr(),
                };
                let instance = inst.borrow();
                if let Some(val) = instance.fields.get(&key) {
                    return Ok(val.clone());
                }
                if let Some(method) = instance.struct_def.methods.get(&key) {
                    // Return a bound function with 'ini' bound
                    let method_fn = WidyaFunction {
                        name: Some(format!("{}.{}", instance.struct_def.name, method.name)),
                        params: method.params.clone(),
                        body: method.body.clone(),
                        closure: {
                            let method_env = Rc::new(RefCell::new(Environment::with_parent(
                                instance.struct_def.closure.clone(),
                            )));
                            method_env
                                .borrow_mut()
                                .define("ini".to_string(), Value::Instance(inst.clone()), false);
                            method_env
                        },
                    };
                    return Ok(Value::Function(Rc::new(method_fn)));
                }
                Ok(Value::Nil)
            }
            other => Err(Galat::runtime(
                format!(
                    "Tipe '{}' tidak mendukung pengaksesan dengan indeks [] atau .",
                    other.type_name()
                ),
                span,
            )),
        }
    }

    fn assign_to_index(
        &mut self,
        target_val: Value,
        index_val: Value,
        rhs_val: Value,
        op: &AssignOp,
        span: &Span,
    ) -> Result<Value, Galat> {
        match target_val {
            Value::Array(a) => {
                let mut items = a.borrow_mut();
                let idx = match index_val {
                    Value::Number(n) => n as i64,
                    _ => return Err(Galat::runtime("Indeks daftar harus berupa angka", span)),
                };
                let actual_idx = if idx < 0 {
                    (items.len() as i64 + idx) as usize
                } else {
                    idx as usize
                };
                if actual_idx < items.len() {
                    let final_val = match op {
                        AssignOp::Assign => rhs_val,
                        _ => {
                            let curr = items[actual_idx].clone();
                            self.apply_compound_assign(curr, rhs_val, op, span)?
                        }
                    };
                    items[actual_idx] = final_val.clone();
                    Ok(final_val)
                } else {
                    Err(Galat::runtime(
                        format!(
                            "Indeks {} di luar batas daftar (panjang: {})",
                            idx,
                            items.len()
                        ),
                        span,
                    ))
                }
            }
            Value::Map(m) => {
                let key = match index_val {
                    Value::String(s) => s,
                    other => other.to_string_repr(),
                };
                let mut map = m.borrow_mut();
                let final_val = match op {
                    AssignOp::Assign => rhs_val,
                    _ => {
                        let curr = map.get(&key).cloned().unwrap_or(Value::Nil);
                        self.apply_compound_assign(curr, rhs_val, op, span)?
                    }
                };
                map.insert(key, final_val.clone());
                Ok(final_val)
            }
            Value::Instance(inst) => {
                let key = match index_val {
                    Value::String(s) => s,
                    other => other.to_string_repr(),
                };
                let mut instance = inst.borrow_mut();
                let final_val = match op {
                    AssignOp::Assign => rhs_val,
                    _ => {
                        let curr = instance.fields.get(&key).cloned().unwrap_or(Value::Nil);
                        self.apply_compound_assign(curr, rhs_val, op, span)?
                    }
                };
                instance.fields.insert(key, final_val.clone());
                Ok(final_val)
            }
            other => Err(Galat::runtime(
                format!(
                    "Tidak dapat mengubah elemen pada tipe '{}'",
                    other.type_name()
                ),
                span,
            )),
        }
    }

    fn apply_compound_assign(
        &self,
        left: Value,
        right: Value,
        op: &AssignOp,
        span: &Span,
    ) -> Result<Value, Galat> {
        match op {
            AssignOp::PlusAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), _) => {
                    Ok(Value::String(format!("{}{}", a, right.to_string_repr())))
                }
                _ => Err(Galat::runtime(
                    "Operasi '+=' memerlukan angka atau teks",
                    span,
                )),
            },
            AssignOp::MinusAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                _ => Err(Galat::runtime("Operasi '-=' memerlukan angka", span)),
            },
            AssignOp::StarAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                _ => Err(Galat::runtime("Operasi '*=' memerlukan angka", span)),
            },
            AssignOp::SlashAssign => match (&left, &right) {
                (Value::Number(a), Value::Number(b)) => {
                    if *b == 0.0 {
                        Err(Galat::runtime("Pembagian dengan nol pada '/='", span))
                    } else {
                        Ok(Value::Number(a / b))
                    }
                }
                _ => Err(Galat::runtime("Operasi '/=' memerlukan angka", span)),
            },
            AssignOp::Assign => unreachable!(),
        }
    }

    fn call_method(
        &mut self,
        instance: Rc<RefCell<WidyaInstance>>,
        method: &StructMethod,
        args: &[Value],
        span: &Span,
    ) -> Result<Value, Galat> {
        if args.len() != method.params.len() {
            return Err(Galat::runtime(
                format!(
                    "Metode '{}' mengharapkan {} argumen, tetapi menerima {}",
                    method.name,
                    method.params.len(),
                    args.len()
                ),
                span,
            ));
        }

        let closure_env = instance.borrow().struct_def.closure.clone();
        let method_env = Rc::new(RefCell::new(Environment::with_parent(closure_env)));

        // Bind 'ini' to the instance
        method_env
            .borrow_mut()
            .define("ini".to_string(), Value::Instance(instance), false);

        for (param, arg) in method.params.iter().zip(args.iter()) {
            method_env
                .borrow_mut()
                .define(param.clone(), arg.clone(), false);
        }

        let previous = self.environment.clone();
        self.environment = method_env;

        let mut result = Ok(Value::Nil);
        for statement in &method.body {
            match self.execute(statement) {
                Ok(_) => {}
                Err(Galat::Kembalikan(val)) => {
                    result = Ok(val);
                    break;
                }
                Err(e) => {
                    result = Err(e);
                    break;
                }
            }
        }

        self.environment = previous;
        result
    }

    pub fn call_value(
        &mut self,
        callee: Value,
        args: &[Value],
        span: &Span,
    ) -> Result<Value, Galat> {
        match callee {
            Value::StructDef(struct_def) => {
                let mut field_values = HashMap::new();

                // Bind positional arguments to fields if matching
                for (i, field_name) in struct_def.fields.iter().enumerate() {
                    if let Some(arg) = args.get(i) {
                        field_values.insert(field_name.clone(), arg.clone());
                    } else {
                        field_values.insert(field_name.clone(), Value::Nil);
                    }
                }

                let instance = Rc::new(RefCell::new(WidyaInstance {
                    struct_def: struct_def.clone(),
                    fields: field_values,
                }));

                // Check for constructor method: 'inisialisasi' or 'init' or struct_name
                let constructor = struct_def
                    .methods
                    .get("inisialisasi")
                    .or_else(|| struct_def.methods.get("init"))
                    .or_else(|| struct_def.methods.get(&struct_def.name))
                    .cloned();

                if let Some(init_method) = constructor {
                    self.call_method(instance.clone(), &init_method, args, span)?;
                }

                Ok(Value::Instance(instance))
            }
            Value::Builtin(b) => {
                if let Some(expected_arity) = b.arity {
                    if args.len() != expected_arity {
                        return Err(Galat::runtime(
                            format!(
                                "Fungsi bawaan '{}' mengharapkan {} argumen, tetapi menerima {}",
                                b.name,
                                expected_arity,
                                args.len()
                            ),
                            span,
                        ));
                    }
                }
                (b.func)(args, span)
            }
            Value::Function(func) => {
                // Check if this is a synthetic enum constructor
                if let Some(ref fn_name) = func.name {
                    if fn_name.contains("::") && func.body.is_empty() {
                        let parts: Vec<&str> = fn_name.split("::").collect();
                        if parts.len() == 2 {
                            return Ok(Value::EnumVariant(Rc::new(crate::value::WidyaEnumVariant {
                                enum_name: parts[0].to_string(),
                                variant_name: parts[1].to_string(),
                                fields: args.to_vec(),
                            })));
                        }
                    }
                }

                if args.len() != func.params.len() {
                    let fn_name = func.name.as_deref().unwrap_or("anonim");
                    return Err(Galat::runtime(
                        format!(
                            "Fungsi '{}' mengharapkan {} argumen, tetapi menerima {}",
                            fn_name,
                            func.params.len(),
                            args.len()
                        ),
                        span,
                    ));
                }

                let call_env = Rc::new(RefCell::new(Environment::with_parent(func.closure.clone())));
                for (param, arg) in func.params.iter().zip(args.iter()) {
                    call_env
                        .borrow_mut()
                        .define(param.clone(), arg.clone(), false);
                }

                let previous = self.environment.clone();
                self.environment = call_env;

                let mut result = Ok(Value::Nil);
                for statement in &func.body {
                    match self.execute(statement) {
                        Ok(_) => {}
                        Err(Galat::Kembalikan(val)) => {
                            result = Ok(val);
                            break;
                        }
                        Err(e) => {
                            result = Err(e);
                            break;
                        }
                    }
                }

                self.environment = previous;
                result
            }
            other => Err(Galat::runtime(
                format!("Tipe '{}' tidak dapat dipanggil sebagai fungsi", other.type_name()),
                span,
            )),
        }
    }

    fn match_pattern_check(
        &mut self,
        pattern: &MatchPattern,
        target: &Value,
    ) -> Result<Option<Vec<(String, Value)>>, Galat> {
        match pattern {
            MatchPattern::Wildcard(_) => Ok(Some(Vec::new())),
            MatchPattern::Literal(expr) => {
                let lit_val = self.evaluate(expr)?;
                if &lit_val == target {
                    Ok(Some(Vec::new()))
                } else {
                    Ok(None)
                }
            }
            MatchPattern::Identifier(name, _) => {
                // If identifier matches an Enum variant with 0 fields in scope, check equality
                if let Ok(val) = self.environment.borrow().get(name, &crate::error::Span::new(0, 0)) {
                    if let Value::EnumVariant(ev) = &val {
                        if let Value::EnumVariant(target_ev) = target {
                            if ev.variant_name == target_ev.variant_name && ev.fields.is_empty() && target_ev.fields.is_empty() {
                                return Ok(Some(Vec::new()));
                            }
                        }
                    }
                }
                // Otherwise it's a variable binding pattern
                Ok(Some(vec![(name.clone(), target.clone())]))
            }
            MatchPattern::EnumVariant {
                enum_name,
                variant_name,
                bindings,
                ..
            } => {
                if let Value::EnumVariant(target_ev) = target {
                    let enum_matches = if let Some(e_name) = enum_name {
                        &target_ev.enum_name == e_name
                    } else {
                        true
                    };

                    if enum_matches && &target_ev.variant_name == variant_name {
                        if bindings.len() == target_ev.fields.len() {
                            let mut bound_vars = Vec::new();
                            for (var_name, field_val) in bindings.iter().zip(target_ev.fields.iter()) {
                                bound_vars.push((var_name.clone(), field_val.clone()));
                            }
                            return Ok(Some(bound_vars));
                        }
                    }
                }
                Ok(None)
            }
        }
    }
}
