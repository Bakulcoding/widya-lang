//! Analytics engine for Widya Enterprise Edition
//! Provides in-memory data processing, SQL-based analytics, and aggregation

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::dataplatform::{DataPlatformError, Result, AnalyticsConfig};

/// Analytics engine for data processing
pub struct AnalyticsEngine {
    /// Configuration
    config: AnalyticsConfig,
    
    /// In-memory buffer
    buffer: Arc<RwLock<VecDeque<DataRow>>>,
    
    /// Query cache
    query_cache: Arc<RwLock<HashMap<String, CachedResult>>>,
    
    /// Execution statistics
    stats: Arc<RwLock<ExecutionStats>>,
}

impl AnalyticsEngine {
    /// Create new analytics engine
    pub fn new(config: &AnalyticsConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            buffer: Arc::new(RwLock::new(VecDeque::new())),
            query_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(ExecutionStats::default())),
        })
    }
    
    /// Load data into engine
    pub fn load(&self, data: Vec<DataRow>) -> Result<()> {
        let mut buffer = self.buffer.write().unwrap();
        buffer.extend(data);
        
        if buffer.len() > self.config.buffer_size {
            buffer.drain(..buffer.len() - self.config.buffer_size);
        }
        
        Ok(())
    }
    
    /// Execute query
    pub fn execute(&self, query: &QueryPlan) -> Result<QueryResult> {
        let start_time = current_time();
        
        let cache_key = query.cache_key();
        {
            let cache = self.query_cache.read().unwrap();
            if let Some(cached) = cache.get(&cache_key) {
                if !cached.is_expired() {
                    return Ok(cached.result.clone());
                }
            }
        }
        
        let result = self.execute_query(query)?;
        
        {
            let mut cache = self.query_cache.write().unwrap();
            cache.insert(cache_key, CachedResult {
                result: result.clone(),
                created_at: start_time,
                ttl_secs: 300,
            });
        }
        
        let mut stats = self.stats.write().unwrap();
        stats.queries_executed += 1;
        stats.total_execution_time_ms += current_time() - start_time;
        
        Ok(result)
    }
    
    /// Execute query plan
    fn execute_query(&self, query: &QueryPlan) -> Result<QueryResult> {
        let buffer = self.buffer.read().unwrap();
        let mut result_rows = Vec::new();
        
        match &query.operation {
            Operation::Scan { table } => {
                for row in buffer.iter() {
                    if row.table == *table {
                        result_rows.push(row.clone());
                    }
                }
            }
            
            Operation::Filter { predicate } => {
                for row in buffer.iter() {
                    if self.evaluate_predicate(row, predicate)? {
                        result_rows.push(row.clone());
                    }
                }
            }
            
            Operation::Project { columns } => {
                for row in buffer.iter() {
                    let mut projected = DataRow::new(row.table.clone());
                    for col in columns {
                        if let Some(value) = row.get(col) {
                            projected = projected.set(col.clone(), value.clone());
                        }
                    }
                    result_rows.push(projected);
                }
            }
            
            Operation::Aggregate { group_by, aggregations } => {
                result_rows = self.execute_aggregation(&buffer, group_by, aggregations)?;
            }
            
            Operation::Sort { column, ascending } => {
                result_rows = buffer.iter().cloned().collect();
                result_rows.sort_by(|a, b| {
                    let a_val = a.get(column);
                    let b_val = b.get(column);
                    match (a_val, b_val) {
                        (Some(DataValue::Number(a_n)), Some(DataValue::Number(b_n))) => {
                            if *ascending { a_n.partial_cmp(b_n).unwrap() } else { b_n.partial_cmp(a_n).unwrap() }
                        }
                        (Some(DataValue::String(a_s)), Some(DataValue::String(b_s))) => {
                            if *ascending { a_s.cmp(b_s) } else { b_s.cmp(a_s) }
                        }
                        _ => std::cmp::Ordering::Equal,
                    }
                });
            }
            
            Operation::Limit { count } => {
                result_rows = buffer.iter().take(*count).cloned().collect();
            }
        }
        
        Ok(QueryResult {
            rows: result_rows,
            columns: query.columns.clone(),
            execution_time_ms: 0,
        })
    }
    
    /// Execute aggregation
    fn execute_aggregation(
        &self,
        data: &VecDeque<DataRow>,
        group_by: &[String],
        aggregations: &[Aggregation],
    ) -> Result<Vec<DataRow>> {
        let mut groups: HashMap<String, AggregationState> = HashMap::new();
        
        for row in data.iter() {
            let key = if group_by.is_empty() {
                "__all__".to_string()
            } else {
                group_by.iter()
                    .filter_map(|col| row.get(col).map(|v| format!("{:?}", v)))
                    .collect::<Vec<_>>()
                    .join("|")
            };
            
            let state = groups.entry(key).or_insert_with(|| AggregationState::new(aggregations));
            state.update(row, aggregations)?;
        }
        
        let mut result = Vec::new();
        for (_, state) in groups {
            result.push(state.to_row());
        }
        
        Ok(result)
    }
    
    /// Evaluate predicate
    fn evaluate_predicate(&self, row: &DataRow, predicate: &Predicate) -> Result<bool> {
        match predicate {
            Predicate::Eq { column, value } => {
                if let Some(row_value) = row.get(column) {
                    Ok(row_value == value)
                } else {
                    Ok(false)
                }
            }
            
            Predicate::Gt { column, value } => {
                if let Some(row_value) = row.get(column) {
                    match (row_value, value) {
                        (DataValue::Number(r), DataValue::Number(v)) => Ok(r > v),
                        _ => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            
            Predicate::Lt { column, value } => {
                if let Some(row_value) = row.get(column) {
                    match (row_value, value) {
                        (DataValue::Number(r), DataValue::Number(v)) => Ok(r < v),
                        _ => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            
            Predicate::And(left, right) => {
                Ok(self.evaluate_predicate(row, left)? && self.evaluate_predicate(row, right)?)
            }
            
            Predicate::Or(left, right) => {
                Ok(self.evaluate_predicate(row, left)? || self.evaluate_predicate(row, right)?)
            }
            
            Predicate::Not(pred) => {
                Ok(!self.evaluate_predicate(row, pred)?)
            }
        }
    }
    
    /// Get statistics
    pub fn stats(&self) -> ExecutionStats {
        self.stats.read().unwrap().clone()
    }
    
    /// Clear buffer
    pub fn clear(&self) -> Result<()> {
        self.buffer.write().unwrap().clear();
        self.query_cache.write().unwrap().clear();
        Ok(())
    }
}

/// Query plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryPlan {
    /// Query ID
    pub id: String,
    
    /// Operation
    pub operation: Operation,
    
    /// Output columns
    pub columns: Vec<String>,
    
    /// Cache enabled
    pub cache_enabled: bool,
}

impl QueryPlan {
    /// Create scan query
    pub fn scan(table: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            operation: Operation::Scan { table },
            columns: Vec::new(),
            cache_enabled: false,
        }
    }
    
    /// Add filter
    pub fn filter(mut self, predicate: Predicate) -> Self {
        self.operation = Operation::Filter { predicate: Box::new(predicate) };
        self
    }
    
    /// Add projection
    pub fn project(mut self, columns: Vec<String>) -> Self {
        self.operation = Operation::Project { columns };
        self.columns = columns.clone();
        self
    }
    
    /// Add aggregation
    pub fn aggregate(mut self, group_by: Vec<String>, aggregations: Vec<Aggregation>) -> Self {
        self.operation = Operation::Aggregate { group_by, aggregations };
        self
    }
    
    /// Add sort
    pub fn sort(mut self, column: String, ascending: bool) -> Self {
        self.operation = Operation::Sort { column, ascending };
        self
    }
    
    /// Add limit
    pub fn limit(mut self, count: usize) -> Self {
        self.operation = Operation::Limit { count };
        self
    }
    
    /// Get cache key
    fn cache_key(&self) -> String {
        format!("{:?}", self.operation)
    }
}

/// Query operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    /// Table scan
    Scan { table: String },
    
    /// Filter rows
    Filter { predicate: Box<Predicate> },
    
    /// Project columns
    Project { columns: Vec<String> },
    
    /// Aggregate
    Aggregate { group_by: Vec<String>, aggregations: Vec<Aggregation> },
    
    /// Sort
    Sort { column: String, ascending: bool },
    
    /// Limit
    Limit { count: usize },
}

/// Predicate for filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Predicate {
    /// Equality
    Eq { column: String, value: DataValue },
    
    /// Greater than
    Gt { column: String, value: DataValue },
    
    /// Less than
    Lt { column: String, value: DataValue },
    
    /// And
    And(Box<Predicate>, Box<Predicate>),
    
    /// Or
    Or(Box<Predicate>, Box<Predicate>),
    
    /// Not
    Not(Box<Predicate>),
}

impl Predicate {
    /// Create equality predicate
    pub fn eq(column: String, value: DataValue) -> Self {
        Predicate::Eq { column, value }
    }
    
    /// Create greater than predicate
    pub fn gt(column: String, value: DataValue) -> Self {
        Predicate::Gt { column, value }
    }
    
    /// Create less than predicate
    pub fn lt(column: String, value: DataValue) -> Self {
        Predicate::Lt { column, value }
    }
    
    /// Combine with AND
    pub fn and(self, other: Predicate) -> Self {
        Predicate::And(Box::new(self), Box::new(other))
    }
    
    /// Combine with OR
    pub fn or(self, other: Predicate) -> Self {
        Predicate::Or(Box::new(self), Box::new(other))
    }
}

/// Aggregation specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Aggregation {
    /// Column to aggregate
    pub column: String,
    
    /// Aggregation function
    pub function: AggFunc,
    
    /// Output column name
    pub alias: String,
}

impl Aggregation {
    /// Create count aggregation
    pub fn count(column: String, alias: String) -> Self {
        Self {
            column,
            function: AggFunc::Count,
            alias,
        }
    }
    
    /// Create sum aggregation
    pub fn sum(column: String, alias: String) -> Self {
        Self {
            column,
            function: AggFunc::Sum,
            alias,
        }
    }
    
    /// Create average aggregation
    pub fn avg(column: String, alias: String) -> Self {
        Self {
            column,
            function: AggFunc::Avg,
            alias,
        }
    }
    
    /// Create min aggregation
    pub fn min(column: String, alias: String) -> Self {
        Self {
            column,
            function: AggFunc::Min,
            alias,
        }
    }
    
    /// Create max aggregation
    pub fn max(column: String, alias: String) -> Self {
        Self {
            column,
            function: AggFunc::Max,
            alias,
        }
    }
}

/// Aggregation functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggFunc {
    Count,
    Sum,
    Avg,
    Min,
    Max,
}

/// Aggregation state
struct AggregationState {
    count: u64,
    sum: f64,
    min: f64,
    max: f64,
    values: Vec<f64>,
}

impl AggregationState {
    fn new(_aggregations: &[Aggregation]) -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            values: Vec::new(),
        }
    }
    
    fn update(&mut self, row: &DataRow, aggregations: &[Aggregation]) -> Result<()> {
        for agg in aggregations {
            if let Some(DataValue::Number(n)) = row.get(&agg.column) {
                match agg.function {
                    AggFunc::Count => self.count += 1,
                    AggFunc::Sum => self.sum += n,
                    AggFunc::Min => self.min = self.min.min(n),
                    AggFunc::Max => self.max = self.max.max(n),
                    AggFunc::Avg => {
                        self.values.push(n);
                    }
                }
            }
        }
        Ok(())
    }
    
    fn to_row(&self) -> DataRow {
        let avg = if !self.values.is_empty() {
            self.values.iter().sum::<f64>() / self.values.len() as f64
        } else {
            0.0
        };
        
        DataRow::new("result".to_string())
            .set("count".to_string(), DataValue::Number(self.count as f64))
            .set("sum".to_string(), DataValue::Number(self.sum))
            .set("min".to_string(), DataValue::Number(self.min))
            .set("max".to_string(), DataValue::Number(self.max))
            .set("avg".to_string(), DataValue::Number(avg))
    }
}

/// Query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Result rows
    pub rows: Vec<DataRow>,
    
    /// Column names
    pub columns: Vec<String>,
    
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

impl QueryResult {
    /// Get row count
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}

/// Data row
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRow {
    /// Table name
    pub table: String,
    
    /// Column values
    pub values: HashMap<String, DataValue>,
}

impl DataRow {
    /// Create new row
    pub fn new(table: String) -> Self {
        Self {
            table,
            values: HashMap::new(),
        }
    }
    
    /// Set value
    pub fn set(mut self, column: String, value: DataValue) -> Self {
        self.values.insert(column, value);
        self
    }
    
    /// Get value
    pub fn get(&self, column: &str) -> Option<&DataValue> {
        self.values.get(column)
    }
}

/// Data value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataValue {
    /// Null
    Null,
    
    /// Boolean
    Boolean(bool),
    
    /// Number
    Number(f64),
    
    /// String
    String(String),
    
    /// Binary
    Binary(Vec<u8>),
}

/// Cached query result
struct CachedResult {
    result: QueryResult,
    created_at: u64,
    ttl_secs: u64,
}

impl CachedResult {
    fn is_expired(&self) -> bool {
        let now = current_time();
        now - self.created_at > self.ttl_secs * 1000
    }
}

/// Execution statistics
#[derive(Debug, Clone, Default)]
pub struct ExecutionStats {
    /// Queries executed
    pub queries_executed: u64,
    
    /// Total execution time
    pub total_execution_time_ms: u64,
    
    /// Cache hits
    pub cache_hits: u64,
    
    /// Cache misses
    pub cache_misses: u64,
    
    /// Rows processed
    pub rows_processed: u64,
}

/// Get current time in milliseconds
fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_query_plan() {
        let plan = QueryPlan::scan("users".to_string())
            .filter(Predicate::gt("age".to_string(), DataValue::Number(18.0)))
            .project(vec!["name".to_string(), "age".to_string()]);
        
        assert!(!plan.id.is_empty());
    }
    
    #[test]
    fn test_predicate() {
        let pred = Predicate::eq("status".to_string(), DataValue::String("active".to_string()))
            .and(Predicate::gt("age".to_string(), DataValue::Number(18.0)));
        
        let row = DataRow::new("users".to_string())
            .set("status".to_string(), DataValue::String("active".to_string()))
            .set("age".to_string(), DataValue::Number(25.0));
        
        let config = AnalyticsConfig {
            buffer_size: 1000,
            vectorized: true,
            spill_enabled: true,
        };
        let engine = AnalyticsEngine::new(&config).unwrap();
        let result = engine.evaluate_predicate(&row, &pred).unwrap();
        assert!(result);
    }
    
    #[test]
    fn test_aggregation() {
        let agg = Aggregation::count("id".to_string(), "total".to_string());
        assert_eq!(agg.function, AggFunc::Count);
    }
    
    #[test]
    fn test_data_row() {
        let row = DataRow::new("users".to_string())
            .set("id".to_string(), DataValue::Number(1.0))
            .set("name".to_string(), DataValue::String("test".to_string()));
        
        assert_eq!(row.get("id"), Some(&DataValue::Number(1.0)));
        assert_eq!(row.get("name"), Some(&DataValue::String("test".to_string())));
    }
}
