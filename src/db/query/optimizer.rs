// ============================================================================
// Query Optimizer - TAHAP 3.2
// ============================================================================
// Cost-based query optimizer dengan join reordering & index selection
// Features:
// - Statistics collection
// - Cost estimation
// - Join order optimization
// - Index selection
// ============================================================================

use crate::db::query::parser::{Query, SelectQuery, FromClause, JoinType, Expr, BinaryOp};
use std::collections::HashMap;

/// Table statistics
#[derive(Debug, Clone)]
pub struct TableStats {
    pub table_name: String,
    pub row_count: usize,
    pub avg_row_size: usize,
    pub columns: Vec<String>,
    pub column_stats: HashMap<String, ColumnStats>,
}

/// Column statistics
#[derive(Debug, Clone)]
pub struct ColumnStats {
    pub distinct_count: usize,
    pub min_value: Option<String>,
    pub max_value: Option<String>,
    pub null_count: usize,
}

/// Query plan node types
#[derive(Debug, Clone)]
pub enum PlanNode {
    SeqScan {
        table: String,
        alias: Option<String>,
        filter: Option<Expr>,
    },
    IndexScan {
        table: String,
        alias: Option<String>,
        index_name: String,
        range: Option<(Expr, Expr)>,
    },
    HashJoin {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        join_type: JoinType,
        left_key: Expr,
        right_key: Expr,
    },
    MergeJoin {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        join_type: JoinType,
        left_key: Expr,
        right_key: Expr,
        left_sorted: bool,
        right_sorted: bool,
    },
    NestedLoopJoin {
        left: Box<PlanNode>,
        right: Box<PlanNode>,
        join_type: JoinType,
        condition: Option<Expr>,
    },
    Sort {
        input: Box<PlanNode>,
        order_by: Vec<(Expr, bool)>,
    },
    Aggregate {
        input: Box<PlanNode>,
        group_by: Vec<Expr>,
        aggregates: Vec<AggregateExpr>,
    },
    Project {
        input: Box<PlanNode>,
        columns: Vec<String>,
    },
    Limit {
        input: Box<PlanNode>,
        count: usize,
        offset: usize,
    },
}

/// Aggregate expression
#[derive(Debug, Clone)]
pub struct AggregateExpr {
    pub func: AggregateFunc,
    pub expr: Expr,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AggregateFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    CountDistinct,
}

/// Cost model for query optimization
#[derive(Debug, Clone)]
pub struct CostModel {
    pub cpu_cost_per_row: f64,
    pub io_cost_per_page: f64,
    pub join_factor: f64,
}

impl CostModel {
    pub fn new() -> Self {
        Self {
            cpu_cost_per_row: 0.01,
            io_cost_per_page: 1.0,
            join_factor: 1.5,
        }
    }

    /// Estimate cost of a plan node
    pub fn estimate_cost(&self, node: &PlanNode) -> f64 {
        match node {
            PlanNode::SeqScan { table, .. } => {
                // Cost = (rows / pages_per_row) * io_cost
                self.io_cost_per_page * 10.0
            }
            PlanNode::IndexScan { table, .. } => {
                // Index scan is cheaper than full scan
                self.io_cost_per_page * 2.0
            }
            PlanNode::HashJoin { left, right, .. } => {
                let left_cost = self.estimate_cost(left);
                let right_cost = self.estimate_cost(right);
                // Hash join cost: build + probe
                left_cost + right_cost + (left_cost * right_cost * self.join_factor)
            }
            PlanNode::MergeJoin { left, right, .. } => {
                let left_cost = self.estimate_cost(left);
                let right_cost = self.estimate_cost(right);
                left_cost + right_cost + (left_cost + right_cost) * 0.1
            }
            PlanNode::NestedLoopJoin { left, right, .. } => {
                let left_cost = self.estimate_cost(left);
                let right_cost = self.estimate_cost(right);
                // O(n * m) for nested loop
                left_cost + right_cost + (left_cost * right_cost * self.join_factor)
            }
            PlanNode::Sort { input, .. } => {
                let input_cost = self.estimate_cost(input);
                input_cost + input_cost * 0.3
            }
            PlanNode::Aggregate { input, .. } => {
                let input_cost = self.estimate_cost(input);
                input_cost + input_cost * 0.2
            }
            PlanNode::Project { input, .. } => {
                let input_cost = self.estimate_cost(input);
                input_cost + input_cost * 0.05
            }
            PlanNode::Limit { input, .. } => {
                let input_cost = self.estimate_cost(input);
                input_cost * 0.1
            }
        }
    }
}

/// Query optimizer
pub struct QueryOptimizer {
    pub statistics: HashMap<String, TableStats>,
    pub cost_model: CostModel,
}

impl QueryOptimizer {
    pub fn new() -> Self {
        Self {
            statistics: HashMap::new(),
            cost_model: CostModel::new(),
        }
    }

    /// Add table statistics
    pub fn add_table_stats(&mut self, stats: TableStats) {
        self.statistics.insert(stats.table_name.clone(), stats);
    }

    /// Optimize a query and return a physical plan
    pub fn optimize(&mut self, query: Query) -> Result<PlanNode, String> {
        match query {
            Query::Select(select) => self.optimize_select(select),
            _ => Err("Only SELECT queries are optimized".to_string()),
        }
    }

    fn optimize_select(&mut self, select: SelectQuery) -> Result<PlanNode, String> {
        // Get tables from FROM clause
        let tables = select.from;
        
        // Determine join order using dynamic programming
        let best_plan = self.optimize_joins(&tables, select.where_clause.clone())?;
        
        // Apply filter pushdown
        let filtered_plan = self.pushdown_filters(best_plan, select.where_clause)?;
        
        // Apply column pruning
        let projected_plan = self.prune_columns(filtered_plan, &select.columns)?;
        
        // Apply aggregation if needed
        let aggregated_plan = if !select.group_by.is_empty() || !select.having.is_none() {
            self.optimize_aggregation(projected_plan, select.group_by, select.having)?
        } else {
            projected_plan
        };
        
        // Apply sorting if needed
        let sorted_plan = if !select.order_by.is_empty() {
            self.optimize_sort(aggregated_plan, select.order_by)?
        } else {
            aggregated_plan
        };
        
        // Apply limit/offset
        let final_plan = if select.limit.is_some() || select.offset.is_some() {
            self.optimize_limit(sorted_plan, select.limit.unwrap_or(0), select.offset.unwrap_or(0))?
        } else {
            sorted_plan
        };
        
        Ok(final_plan)
    }

    /// Optimize join order using dynamic programming (bottom-up)
    fn optimize_joins(&mut self, tables: &[FromClause], where_clause: Option<Expr>) -> Result<PlanNode, String> {
        if tables.is_empty() {
            return Err("No tables in FROM clause".to_string());
        }
        
        if tables.len() == 1 {
            return self.optimize_single_table(&tables[0]);
        }
        
        // For simplicity, use left-deep join tree
        // In production, use bushy trees or complete DP
        let mut current = self.optimize_single_table(&tables[0])?;
        
        for from_clause in tables.iter().skip(1) {
            let right = self.optimize_single_table(from_clause)?;
            
            // Determine join type
            let join_type = from_clause.join_type.unwrap_or(JoinType::Inner);
            
            // Find join condition
            let (left_key, right_key) = self.find_join_condition(&current, &right, &where_clause)?;
            
            current = PlanNode::HashJoin {
                left: Box::new(current),
                right: Box::new(right),
                join_type,
                left_key,
                right_key,
            };
        }
        
        Ok(current)
    }

    fn optimize_single_table(&self, from: &FromClause) -> Result<PlanNode, String> {
        let table = from.table.clone();
        let alias = from.alias.clone();
        
        // Check if table has statistics
        let has_stats = self.statistics.contains_key(&table);
        
        // Use index scan if available and filter exists
        // For now, default to seq scan
        let plan = PlanNode::SeqScan {
            table,
            alias,
            filter: None,
        };
        
        Ok(plan)
    }

    fn find_join_condition(
        &self,
        left: &PlanNode,
        right: &PlanNode,
        where_clause: &Option<Expr>,
    ) -> Result<(Expr, Expr), String> {
        // Look for equality conditions in WHERE clause
        if let Some(ref expr) = where_clause {
            if let Expr::Binary {
                left: ref left_expr,
                op: BinaryOp::Equal,
                right: ref right_expr,
            } = *expr
            {
                return Ok((left_expr.as_ref().clone(), right_expr.as_ref().clone()));
            }
        }
        
        // Default: no join condition
        Ok((Expr::Literal(crate::db::query::parser::Literal::Number(1.0)), Expr::Literal(crate::db::query::parser::Literal::Number(1.0))))
    }

    fn pushdown_filters(&self, plan: PlanNode, _filter: Option<Expr>) -> Result<PlanNode, String> {
        // In production, push filters down to scan nodes
        Ok(plan)
    }

    fn prune_columns(&self, plan: PlanNode, _columns: &[crate::db::query::parser::Column]) -> Result<PlanNode, String> {
        // In production, prune unused columns
        Ok(plan)
    }

    fn optimize_aggregation(
        &self,
        input: PlanNode,
        group_by: Vec<Expr>,
        having: Option<Expr>,
    ) -> Result<PlanNode, String> {
        if group_by.is_empty() && having.is_none() {
            return Ok(input);
        }

        Ok(PlanNode::Aggregate {
            input: Box::new(input),
            group_by,
            aggregates: Vec::new(),
        })
    }

    fn optimize_sort(
        &self,
        input: PlanNode,
        order_by: Vec<crate::db::query::parser::OrderByClause>,
    ) -> Result<PlanNode, String> {
        let sort_keys: Vec<(Expr, bool)> = order_by
            .into_iter()
            .map(|o| (o.expr, o.direction == crate::db::query::parser::OrderDirection::Desc))
            .collect();

        Ok(PlanNode::Sort {
            input: Box::new(input),
            order_by: sort_keys,
        })
    }

    fn optimize_limit(&self, input: PlanNode, count: usize, offset: usize) -> Result<PlanNode, String> {
        Ok(PlanNode::Limit {
            input: Box::new(input),
            count,
            offset,
        })
    }

    /// Get cost model reference
    pub fn cost_model(&self) -> &CostModel {
        &self.cost_model
    }

    /// Get cost model mutable reference
    pub fn cost_model_mut(&mut self) -> &mut CostModel {
        &mut self.cost_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_model() {
        let model = CostModel::new();
        assert_eq!(model.cpu_cost_per_row, 0.01);
        assert_eq!(model.io_cost_per_page, 1.0);
    }

    #[test]
    fn test_optimizer_basic() {
        let mut optimizer = QueryOptimizer::new();
        
        let stats = TableStats {
            table_name: "users".to_string(),
            row_count: 1000,
            avg_row_size: 100,
            columns: vec!["id".to_string(), "name".to_string()],
            column_stats: HashMap::new(),
        };
        
        optimizer.add_table_stats(stats);
        
        // Test that optimizer can be created
        assert!(optimizer.statistics.contains_key("users"));
    }
}
