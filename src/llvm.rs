use crate::ast::*;
use crate::error::Galat;

pub struct LlvmEmitter {
    buffer: String,
    temp_counter: usize,
    label_counter: usize,
    string_constants: Vec<(String, String)>,
}

impl LlvmEmitter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            temp_counter: 1,
            label_counter: 1,
            string_constants: Vec::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    #[allow(dead_code)]
    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    pub fn emit_llvm_ir(&mut self, program: &Program) -> Result<String, Galat> {
        self.buffer.clear();
        self.temp_counter = 1;
        self.label_counter = 1;
        self.string_constants.clear();

        let mut body_buffer = String::new();

        // Standard LLVM Declarations
        let header = r#"; ModuleID = 'widya_module'
source_filename = "widya_source.wya"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@.str_fmt_num = private unnamed_addr constant [6 x i8] c"%.2f\0A\00", align 1
@.str_fmt_str = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1

declare i32 @printf(i8*, ...)
declare double @sqrt(double)
declare double @pow(double, double)
declare double @sin(double)
declare double @cos(double)
declare double @fabs(double)
declare i32 @puts(i8*)
declare i8* @malloc(i64)
declare void @free(i8*)

"#;

        // Collect custom function definitions and statements
        let mut functions_code = String::new();

        for stmt in &program.statements {
            match stmt {
                Stmt::FunctionDecl { name, params, body, .. } => {
                    let mut fn_code = format!("define double @widya_fn_{}(", name);
                    let mut param_defs = Vec::new();
                    for (i, p) in params.iter().enumerate() {
                        param_defs.push(format!("double %p_{}_{}", p, i));
                    }
                    fn_code.push_str(&param_defs.join(", "));
                    fn_code.push_str(") {\nentry:\n");
                    
                    for s in body {
                        if let Stmt::Return { value: Some(ret_expr), .. } = s {
                            let (val, code) = self.emit_expr(ret_expr)?;
                            fn_code.push_str(&code);
                            fn_code.push_str(&format!("  ret double {}\n", val));
                        } else if let Stmt::Return { value: None, .. } = s {
                            fn_code.push_str("  ret double 0.0\n");
                        }
                    }
                    fn_code.push_str("  ret double 0.0\n}\n\n");
                    functions_code.push_str(&fn_code);
                }
                Stmt::ExternalBlock { abi, functions, .. } => {
                    functions_code.push_str(&format!("; --- External ABI \"{}\" ---\n", abi));
                    for f in functions {
                        let ret_type = match f.return_type.as_deref() {
                            Some("Angka") => "double",
                            Some("Teks") => "i8*",
                            _ => "void",
                        };
                        functions_code.push_str(&format!("declare {} @{}(...)\n", ret_type, f.name));
                    }
                    functions_code.push_str("\n");
                }
                _ => {
                    let stmt_code = self.emit_main_stmt(stmt)?;
                    body_buffer.push_str(&stmt_code);
                }
            }
        }

        // Main entry
        let mut main_func = String::new();
        main_func.push_str("define i32 @main() {\nentry:\n");
        main_func.push_str(&body_buffer);
        main_func.push_str("  ret i32 0\n}\n");

        // String constants
        let mut consts = String::new();
        for (id, val) in &self.string_constants {
            let len = val.len() + 1;
            consts.push_str(&format!(
                "@{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1\n",
                id, len, val
            ));
        }

        self.buffer.push_str(header);
        self.buffer.push_str(&consts);
        if !self.string_constants.is_empty() {
            self.buffer.push_str("\n");
        }
        self.buffer.push_str(&functions_code);
        self.buffer.push_str(&main_func);

        Ok(self.buffer.clone())
    }

    fn emit_main_stmt(&mut self, stmt: &Stmt) -> Result<String, Galat> {
        let mut code = String::new();
        match stmt {
            Stmt::VarDecl { name, initializer, .. } => {
                code.push_str(&format!("  %var_{} = alloca double, align 8\n", name));
                if let Some(init) = initializer {
                    let (val, val_code) = self.emit_expr(init)?;
                    code.push_str(&val_code);
                    code.push_str(&format!("  store double {}, double* %var_{}, align 8\n", val, name));
                }
            }
            Stmt::VolatileWrite { address, value, .. } => {
                let (addr_val, addr_code) = self.emit_expr(address)?;
                let (data_val, data_code) = self.emit_expr(value)?;
                code.push_str(&addr_code);
                code.push_str(&data_code);
                let ptr_temp = self.new_temp();
                code.push_str(&format!("  {} = inttoptr i64 (fptosi double {} to i64) to double*\n", ptr_temp, addr_val));
                code.push_str(&format!("  store volatile double {}, double* {}, align 8\n", data_val, ptr_temp));
            }
            Stmt::Expression(Expr::Call { callee, arguments, .. }) => {
                if let Expr::Identifier(ref name, _) = **callee {
                    if name == "cetak" || name == "tulis" {
                        for arg in arguments {
                            let (val, val_code) = self.emit_expr(arg)?;
                            code.push_str(&val_code);
                            let t = self.new_temp();
                            code.push_str(&format!(
                                "  {} = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @.str_fmt_num, i32 0, i32 0), double {})\n",
                                t, val
                            ));
                        }
                    } else {
                        let (_res, call_code) = self.emit_call_expr(name, arguments)?;
                        code.push_str(&call_code);
                    }
                }
            }
            Stmt::Expression(expr) => {
                let (_, expr_code) = self.emit_expr(expr)?;
                code.push_str(&expr_code);
            }
            _ => {}
        }
        Ok(code)
    }

    fn emit_expr(&mut self, expr: &Expr) -> Result<(String, String), Galat> {
        let mut code = String::new();
        match expr {
            Expr::Number(n, _) => Ok((format!("{:.6}", n), code)),
            Expr::Identifier(name, _) => {
                let t = self.new_temp();
                code.push_str(&format!("  {} = load double, double* %var_{}, align 8\n", t, name));
                Ok((t, code))
            }
            Expr::VolatileRead { address, .. } => {
                let (addr_val, addr_code) = self.emit_expr(address)?;
                code.push_str(&addr_code);
                let ptr_temp = self.new_temp();
                let load_temp = self.new_temp();
                code.push_str(&format!("  {} = inttoptr i64 (fptosi double {} to i64) to double*\n", ptr_temp, addr_val));
                code.push_str(&format!("  {} = load volatile double, double* {}, align 8\n", load_temp, ptr_temp));
                Ok((load_temp, code))
            }
            Expr::Binary { left, op, right, .. } => {
                let (l_val, l_code) = self.emit_expr(left)?;
                let (r_val, r_code) = self.emit_expr(right)?;
                code.push_str(&l_code);
                code.push_str(&r_code);
                let t = self.new_temp();
                let op_str = match op {
                    BinaryOp::Add => "fadd",
                    BinaryOp::Subtract => "fsub",
                    BinaryOp::Multiply => "fmul",
                    BinaryOp::Divide => "fdiv",
                    _ => "fadd",
                };
                code.push_str(&format!("  {} = {} double {}, {}\n", t, op_str, l_val, r_val));
                Ok((t, code))
            }
            Expr::Call { callee, arguments, .. } => {
                if let Expr::Identifier(ref name, _) = **callee {
                    self.emit_call_expr(name, arguments)
                } else {
                    Ok(("0.0".to_string(), code))
                }
            }
            _ => Ok(("0.0".to_string(), code)),
        }
    }

    fn emit_call_expr(&mut self, name: &str, arguments: &[Expr]) -> Result<(String, String), Galat> {
        let mut code = String::new();
        let mut arg_vals = Vec::new();
        for arg in arguments {
            let (v, c) = self.emit_expr(arg)?;
            code.push_str(&c);
            arg_vals.push(format!("double {}", v));
        }

        let ret_temp = self.new_temp();
        match name {
            "akar" | "sqrt" => {
                let first = arg_vals.first().cloned().unwrap_or_else(|| "double 0.0".to_string());
                code.push_str(&format!("  {} = call double @sqrt({})\n", ret_temp, first));
            }
            "sin" => {
                let first = arg_vals.first().cloned().unwrap_or_else(|| "double 0.0".to_string());
                code.push_str(&format!("  {} = call double @sin({})\n", ret_temp, first));
            }
            "cos" => {
                let first = arg_vals.first().cloned().unwrap_or_else(|| "double 0.0".to_string());
                code.push_str(&format!("  {} = call double @cos({})\n", ret_temp, first));
            }
            _ => {
                code.push_str(&format!(
                    "  {} = call double @widya_fn_{}({})\n",
                    ret_temp,
                    name,
                    arg_vals.join(", ")
                ));
            }
        }

        Ok((ret_temp, code))
    }
}
