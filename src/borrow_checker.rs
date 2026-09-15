use crate::ast::*;
use crate::error::{Galat, Span};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum OwnershipState {
    Owned,
    Moved(Span),
    BorrowedImmutable { count: usize, span: Span },
    BorrowedMutable(Span),
}

/// Static Ownership & Borrow Checker
pub struct BorrowChecker {
    variables: HashMap<String, OwnershipState>,
    errors: Vec<Galat>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Galat> {
        self.variables.clear();
        self.errors.clear();

        for stmt in &program.statements {
            self.check_stmt(stmt);
        }

        if let Some(first_err) = self.errors.first() {
            Err(first_err.clone())
        } else {
            Ok(())
        }
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl { name, initializer, .. } => {
                if let Some(init) = initializer {
                    self.check_expr(init);
                }
                self.variables.insert(name.clone(), OwnershipState::Owned);
            }
            Stmt::Assignment { target, value, span, .. } => {
                self.check_expr(value);
                if let AssignTarget::Variable(name) = target {
                    if let Some(state) = self.variables.get(name) {
                        match state {
                            OwnershipState::BorrowedImmutable { .. } => {
                                self.errors.push(Galat::runtime(
                                    format!("Galat Keamanan Memori: Tidak dapat memutasi '{}' karena sedang dipinjam secara imutabel", name),
                                    span,
                                ));
                            }
                            OwnershipState::Moved(_) => {
                                self.errors.push(Galat::runtime(
                                    format!("Galat Keamanan Memori: Tidak dapat menggunakan variabel '{}' yang sudah dipindahkan (moved)", name),
                                    span,
                                ));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Stmt::FunctionDecl { params, body, .. } => {
                let mut inner_checker = BorrowChecker::new();
                for p in params {
                    inner_checker.variables.insert(p.clone(), OwnershipState::Owned);
                }
                for s in body {
                    inner_checker.check_stmt(s);
                }
                self.errors.extend(inner_checker.errors);
            }
            Stmt::Block(stmts, _) => {
                for s in stmts {
                    self.check_stmt(s);
                }
            }
            Stmt::If { condition, then_branch, elif_branches, else_branch, .. } => {
                self.check_expr(condition);
                self.check_stmt(then_branch);
                for (cond, b) in elif_branches {
                    self.check_expr(cond);
                    self.check_stmt(b);
                }
                if let Some(eb) = else_branch {
                    self.check_stmt(eb);
                }
            }
            Stmt::While { condition, body, .. } => {
                self.check_expr(condition);
                self.check_stmt(body);
            }
            Stmt::ForIn { iterable, body, .. } => {
                self.check_expr(iterable);
                self.check_stmt(body);
            }
            Stmt::Expression(expr) => {
                self.check_expr(expr);
            }
            _ => {}
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Identifier(name, span) => {
                if let Some(OwnershipState::Moved(_)) = self.variables.get(name) {
                    self.errors.push(Galat::runtime(
                        format!("Galat Keamanan Memori (Borrow Checker): Penggunaan variabel '{}' setelah dipindahkan (use-after-move)", name),
                        span,
                    ));
                }
            }
            Expr::Binary { left, right, .. } => {
                self.check_expr(left);
                self.check_expr(right);
            }
            Expr::Call { callee, arguments, .. } => {
                self.check_expr(callee);
                for arg in arguments {
                    self.check_expr(arg);
                }
            }
            Expr::Array(items, _) => {
                for item in items {
                    self.check_expr(item);
                }
            }
            _ => {}
        }
    }

    /// Explicitly mark a variable as moved (ownership transfer)
    pub fn mark_moved(&mut self, name: &str, span: Span) {
        self.variables.insert(name.to_string(), OwnershipState::Moved(span));
    }

    /// Borrow variable immutably
    pub fn borrow_immutable(&mut self, name: &str, span: Span) -> Result<(), Galat> {
        match self.variables.get(name) {
            Some(OwnershipState::BorrowedMutable(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Variabel '{}' tidak dapat dipinjam imutabel saat sedang dipinjam secara mutabel", name),
                    &span,
                ))
            }
            Some(OwnershipState::Moved(_)) => {
                Err(Galat::runtime(
                    format!("Galat Borrow Checker: Tidak dapat meminjam variabel '{}' yang nilainya sudah dipindahkan", name),
                    &span,
                ))
            }
            _ => {
                self.variables.insert(name.to_string(), OwnershipState::BorrowedImmutable { count: 1, span });
                Ok(())
            }
        }
    }

    /// Check that all functions with #[profil("X")] attribute are compatible with active_profile
    pub fn check_profile_context(&mut self, program: &Program, active_profile: Option<&str>) -> Result<(), Galat> {
        let active = match active_profile {
            Some(p) => p,
            None => return Ok(()),
        };

        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, attributes, span, .. } = stmt {
                for attr in attributes {
                    if attr.name == "profil" {
                        for required_profile in &attr.arguments {
                            if required_profile != active {
                                return Err(Galat::runtime(
                                    format!(
                                        "GALAT: Fungsi '{}' hanya tersedia di profil '{}', profil aktif '{}'",
                                        name, required_profile, active
                                    ),
                                    span,
                                ));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
