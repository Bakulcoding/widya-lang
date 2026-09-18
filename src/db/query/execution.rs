// ============================================================================
// Query Execution Engine - TAHAP 3.3
// ============================================================================
// Vectorized query execution engine dengan pipeline processing
// Features:
// - Row-based & columnar execution
// - Vectorized operators
// - Pipelined execution
// - Materialization control
// ============================================================================

use crate::db::query::parser::{Query, Expr, Literal, BinaryOp, UnaryOp};
use crate::db::query::optimizer::PlanNode;

/// Row type
pub type Row = Vec<Value>;

/// Value type for query execution
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
    Binary(Vec<u8>),
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::Float(n) => Some(*n as i64),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(n) => Some(*n),
            Value::Integer(n) => Some(*n as f64),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Value::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

/// Execution result
pub struct ExecutionResult {
    pub columns: Vec<String>,
    pub rows: Vec<Row>,
    pub rows_affected: usize,
}

/// Execution context
pub struct ExecutionContext {
    pub table_data: std::collections::HashMap<String, Vec<Row>>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            table_data: std::collections::HashMap::new(),
        }
    }

    pub fn add_table(&mut self, name: &str, rows: Vec<Row>) {
        self.table_data.insert(name.to_string(), rows);
    }
}

/// Execution engine
pub struct ExecutionEngine {
    pub context: ExecutionContext,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self {
            context: ExecutionContext::new(),
        }
    }

    /// Execute a query and return results
    pub fn execute(&mut self, query: Query) -> Result<ExecutionResult, String> {
        match query {
            Query::Select(select) => self.execute_select(select),
            Query::Insert(insert) => self.execute_insert(insert),
            Query::Update(update) => self.execute_update(update),
            Query::Delete(delete) => self.execute_delete(delete),
            Query::CreateTable(_) => Ok(ExecutionResult {
                columns: vec![],
                rows: vec![],
                rows_affected: 0,
            }),
            Query::DropTable(_) => Ok(ExecutionResult {
                columns: vec![],
                rows: vec![],
                rows_affected: 0,
            }),
        }
    }

    fn execute_select(&mut self, select: crate::db::query::parser::SelectQuery) -> Result<ExecutionResult, String> {
        // Start with first table
        let mut rows = self.scan_table(&select.from[0])?;
        
        // Apply filters
        if let Some(ref where_clause) = select.where_clause {
            rows = self.filter_rows(rows, where_clause)?;
        }
        
        // Apply projections
        let columns: Vec<String> = select.columns.iter().map(|c| c.name.clone()).collect();
        let projected = self.project_rows(rows, &columns)?;
        
        // Apply limit
        let limit = select.limit.unwrap_or(usize::MAX);
        let offset = select.offset.unwrap_or(0);
        let limited = projected.into_iter().skip(offset).take(limit).collect();
        
        Ok(ExecutionResult {
            columns,
            rows: limited,
            rows_affected: 0,
        })
    }

    fn execute_insert(&mut self, insert: crate::db::query::parser::InsertQuery) -> Result<ExecutionResult, String> {
        // TODO: Implement insert execution
        Ok(ExecutionResult {
            columns: vec![],
            rows: vec![],
            rows_affected: 1,
        })
    }

    fn execute_update(&mut self, update: crate::db::query::parser::UpdateQuery) -> Result<ExecutionResult, String> {
        // TODO: Implement update execution
        Ok(ExecutionResult {
            columns: vec![],
            rows: vec![],
            rows_affected: 0,
        })
    }

    fn execute_delete(&mut self, delete: crate::db::query::parser::DeleteQuery) -> Result<ExecutionResult, String> {
        // TODO: Implement delete execution
        Ok(ExecutionResult {
            columns: vec![],
            rows: vec![],
            rows_affected: 0,
        })
    }

    fn scan_table(&self, from: &crate::db::query::parser::FromClause) -> Result<Vec<Row>, String> {
        let table = from.table.clone();
        
        if let Some(rows) = self.context.table_data.get(&table) {
            Ok(rows.clone())
        } else {
            Err(format!("Table '{}' not found", table))
        }
    }

    fn filter_rows(&self, rows: Vec<Row>, expr: &Expr) -> Result<Vec<Row>, String> {
        rows.into_iter()
            .filter(|row| self.evaluate_expr(expr, row).map(|v| v != Value::Null).unwrap_or(false))
            .collect()
    }

    fn project_rows(&self, rows: Vec<Row>, columns: &[String]) -> Result<Vec<Row>, String> {
        if columns[0] == "*" {
            return Ok(rows);
        }
        
        // TODO: Implement column selection
        Ok(rows)
    }

    fn evaluate_expr(&self, expr: &Expr, _row: &[Value]) -> Result<Value, String> {
        match expr {
            Expr::Literal(lit) => Ok(match lit {
                Literal::Null => Value::Null,
                Literal::String(s) => Value::Text(s.clone()),
                Literal::Number(n) => Value::Float(*n),
                Literal::Boolean(b) => Value::Boolean(*b),
            }),
            Expr::Identifier(name) => {
                // TODO: Lookup column value
                Ok(Value::Null)
            }
            Expr::Unary { op, expr } => {
                let val = self.evaluate_expr(expr, &[])?;
                match op {
                    UnaryOp::Not => Ok(Value::Boolean(!val.as_boolean().unwrap_or(false))),
                    UnaryOp::Negative => {
                        Ok(Value::Float(-(val.as_float().unwrap_or(0.0))))
                    }
                }
            }
            Expr::Binary { left, op, right } => {
                let left_val = self.evaluate_expr(left, &[])?;
                let right_val = self.evaluate_expr(right, &[])?;
                
                self.evaluate_binary_op(&left_val, op, &right_val)
            }
            _ => Err("Expression type not implemented".to_string()),
        }
    }

    fn evaluate_binary_op(&self, left: &Value, op: &BinaryOp, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Integer(l), Value::Integer(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOp::Less => Ok(Value::Boolean(l < r)),
                BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                BinaryOp::Plus => Ok(Value::Integer(l + r)),
                BinaryOp::Minus => Ok(Value::Integer(l - r)),
                BinaryOp::Multiply => Ok(Value::Integer(l * r)),
                BinaryOp::Divide => Ok(Value::Integer(l / r)),
                _ => Err("Operator not implemented for integers".to_string()),
            },
            (Value::Float(l), Value::Float(r)) => match op {
                BinaryOp::Equal => Ok(Value::Boolean(l == r)),
                BinaryOp::NotEqual => Ok(Value::Boolean(l != r)),
                BinaryOp::Less => Ok(Value::Boolean(l < r)),
                BinaryOp::LessEqual => Ok(Value::Boolean(l <= r)),
                BinaryOp::Greater => Ok(Value::Boolean(l > r)),
                BinaryOp::GreaterEqual => Ok(Value::Boolean(l >= r)),
                BinaryOp::Plus => Ok(Value::Float(l + r)),
                BinaryOp::Minus => Ok(Value::Float(l - r)),
                BinaryOp::Multiply => Ok(Value::Float(l * r)),
                BinaryOp::Divide => Ok(Value::Float(l / r)),
                _ => Err("Operator not implemented for floats".to_string()),
            },
            _ => Err("Type mismatch in binary operation".to_string()),
        }
    }

    /// Add test data
    pub fn add_test_data(&mut self) {
        let users = vec![
            vec![Value::Integer(1), Value::Text("Alice".to_string()), Value::Integer(25)],
            vec![Value::Integer(2), Value::Text("Bob".to_string()), Value::Integer(30)],
            vec![Value::Integer(3), Value::Text("Charlie".to_string()), Value::Integer(35)],
        ];
        
        let orders = vec![
            vec![Value::Integer(1), Value::Integer(1), Value::Float(100.0)],
            vec![Value::Integer(2), Value::Integer(2), Value::Float(200.0)],
            vec![Value::Integer(3), Value::Integer(1), Value::Float(150.0)],
        ];
        
        self.context.add_table("users", users);
        self.context.add_table("orders", orders);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_engine() {
        let mut engine = ExecutionEngine::new();
        engine.add_test_data();
        
        // Simple select
        let query = Query::Select(Box::new(crate::db::query::parser::SelectQuery {
            columns: vec![crate::db::query::parser::Column { name: "*".to_string(), alias: None }],
            from: vec![crate::db::query::parser::FromClause {
                table: "users".to_string(),
                alias: None,
                join_type: None,
                join_condition: None,
            }],
            where_clause: None,
            group_by: Vec::new(),
            having: None,
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }));
        
        let result = engine.execute(query);
        assert!(result.is_ok());
    }
}
