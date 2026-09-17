//! Instrumentation for HTTP, database, and other services
//! Provides automatic tracing, metrics, and logging for common operations

use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::observability::{
    ObservabilitySystem, DistributedContext, LogContext,
    tracing::{Span, SpanKind, SpanStatus},
    metrics::{Metric, metric_names},
    logging::{LogLevel, Logger},
};

/// HTTP request information
#[derive(Debug, Clone)]
pub struct HttpRequestInfo {
    /// HTTP method
    pub method: String,
    
    /// Request path
    pub path: String,
    
    /// Query string
    pub query: Option<String>,
    
    /// Request headers
    pub headers: HashMap<String, String>,
    
    /// Client IP address
    pub client_ip: Option<String>,
    
    /// Request body size in bytes
    pub body_size: Option<usize>,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<Uuid>,
    
    /// User ID (if authenticated)
    pub user_id: Option<String>,
}

/// HTTP response information
#[derive(Debug, Clone)]
pub struct HttpResponseInfo {
    /// HTTP status code
    pub status_code: u16,
    
    /// Response headers
    pub headers: HashMap<String, String>,
    
    /// Response body size in bytes
    pub body_size: Option<usize>,
    
    /// Response time in milliseconds
    pub response_time_ms: u64,
}

/// Database query information
#[derive(Debug, Clone)]
pub struct DatabaseQueryInfo {
    /// Query type (SELECT, INSERT, UPDATE, DELETE, etc.)
    pub query_type: String,
    
    /// Table name
    pub table: Option<String>,
    
    /// Query text (may be truncated)
    pub query_text: String,
    
    /// Parameters count
    pub params_count: usize,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<Uuid>,
    
    /// Query plan (if available)
    pub query_plan: Option<String>,
}

/// Database query result
#[derive(Debug, Clone)]
pub struct DatabaseQueryResult {
    /// Rows affected/returned
    pub rows_affected: Option<u64>,
    
    /// Query execution time in milliseconds
    pub execution_time_ms: u64,
    
    /// Error (if any)
    pub error: Option<String>,
    
    /// Whether query was successful
    pub success: bool,
}

/// Instrumentation for HTTP requests
pub struct HttpInstrumentation {
    /// Observability system
    observability: ObservabilitySystem,
}

impl HttpInstrumentation {
    /// Create new HTTP instrumentation
    pub fn new(observability: ObservabilitySystem) -> Self {
        Self { observability }
    }
    
    /// Instrument an HTTP request
    pub fn instrument_request(
        &self,
        request: &HttpRequestInfo,
        context: Option<DistributedContext>,
    ) -> InstrumentedHttpRequest {
        let start_time = Instant::now();
        let correlation_id = context.as_ref()
            .map(|c| c.correlation_id.as_u128())
            .unwrap_or_else(|| uuid::Uuid::new_v4().as_u128());
        
        // Create span for the request
        let span_name = format!("HTTP {} {}", request.method, request.path);
        let span = if let Some(context) = context {
            self.observability.tracer()
                .map(|tracer| tracer.start_span_from_context(&span_name, &context, SpanKind::Server))
        } else {
            self.observability.tracer()
                .map(|tracer| tracer.start_span(&span_name))
        };
        
        // Add span attributes
        if let Some(span) = span.as_ref() {
            let mut span = span.clone();
            span.add_attribute("http.method".to_string(), request.method.clone());
            span.add_attribute("http.path".to_string(), request.path.clone());
            
            if let Some(query) = &request.query {
                span.add_attribute("http.query".to_string(), query.clone());
            }
            
            if let Some(tenant_id) = request.tenant_id {
                span.set_tenant_id(tenant_id);
            }
            
            if let Some(client_ip) = &request.client_ip {
                span.add_attribute("http.client_ip".to_string(), client_ip.clone());
            }
            
            if let Some(body_size) = request.body_size {
                span.add_attribute("http.request.size".to_string(), body_size.to_string());
            }
        }
        
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("method".to_string(), request.method.clone());
            labels.insert("path".to_string(), request.path.clone());
            
            if let Some(tenant_id) = request.tenant_id {
                labels.insert("tenant_id".to_string(), tenant_id.to_string());
            }
            
            let _ = collector.increment_counter(metric_names::HTTP_REQUESTS_TOTAL, labels);
        }
        
        // Log request
        if let Some(logger) = self.observability.logger() {
            let log_context = context.map(|c| {
                let mut ctx = LogContext::from_distributed_context(&c);
                if let Some(tenant_id) = request.tenant_id {
                    ctx = ctx.with_tenant_id(tenant_id);
                }
                if let Some(user_id) = &request.user_id {
                    ctx = ctx.with_user_id(user_id);
                }
                ctx = ctx.with_string("http_method", &request.method)
                    .with_string("http_path", &request.path);
                
                if let Some(query) = &request.query {
                    ctx = ctx.with_string("http_query", query);
                }
                
                ctx
            });
            
            let _ = logger.info(
                &format!("HTTP request started: {} {}", request.method, request.path),
                log_context,
            );
        }
        
        InstrumentedHttpRequest {
            observability: self.observability.clone(),
            span,
            start_time,
            correlation_id,
            request: request.clone(),
            context: context.cloned(),
        }
    }
}

/// Instrumented HTTP request
pub struct InstrumentedHttpRequest {
    observability: ObservabilitySystem,
    span: Option<Span>,
    start_time: Instant,
    correlation_id: u128,
    request: HttpRequestInfo,
    context: Option<DistributedContext>,
}

impl InstrumentedHttpRequest {
    /// Complete the HTTP request with response
    pub fn complete(self, response: HttpResponseInfo) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        // Update span
        if let Some(mut span) = self.span {
            span.add_attribute("http.status_code".to_string(), response.status_code.to_string());
            
            if let Some(body_size) = response.body_size {
                span.add_attribute("http.response.size".to_string(), body_size.to_string());
            }
            
            span.add_attribute("http.duration_ms".to_string(), duration_ms.to_string());
            
            let status = if response.status_code >= 400 {
                SpanStatus::Error
            } else {
                SpanStatus::Ok
            };
            
            span.end(status);
            
            // End span through tracer
            if let Some(tracer) = self.observability.tracer() {
                let _ = tracer.end_span(span.id, status);
            }
        }
        
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("method".to_string(), self.request.method.clone());
            labels.insert("path".to_string(), self.request.path.clone());
            labels.insert("status".to_string(), response.status_code.to_string());
            
            if let Some(tenant_id) = self.request.tenant_id {
                labels.insert("tenant_id".to_string(), tenant_id.to_string());
            }
            
            let _ = collector.observe_histogram(
                metric_names::HTTP_REQUEST_DURATION_SECONDS,
                duration.as_secs_f64(),
                labels.clone(),
                None,
            );
            
            if let Some(request_size) = self.request.body_size {
                let _ = collector.observe_histogram(
                    metric_names::HTTP_REQUEST_SIZE_BYTES,
                    request_size as f64,
                    labels.clone(),
                    None,
                );
            }
            
            if let Some(response_size) = response.body_size {
                let _ = collector.observe_histogram(
                    metric_names::HTTP_RESPONSE_SIZE_BYTES,
                    response_size as f64,
                    labels,
                    None,
                );
            }
        }
        
        // Log response
        if let Some(logger) = self.observability.logger() {
            let level = if response.status_code >= 400 {
                LogLevel::Warn
            } else {
                LogLevel::Info
            };
            
            let mut log_context = self.context.map(|c| LogContext::from_distributed_context(&c))
                .unwrap_or_else(LogContext::new);
            
            log_context = log_context.with_correlation_id(self.correlation_id)
                .with_number("duration_ms", duration_ms as f64)
                .with_number("status_code", response.status_code as f64);
            
            if let Some(tenant_id) = self.request.tenant_id {
                log_context = log_context.with_tenant_id(tenant_id);
            }
            
            let _ = logger.log(
                level,
                &format!("HTTP request completed: {} {} -> {}", 
                    self.request.method, self.request.path, response.status_code),
                Some(log_context),
            );
        }
    }
    
    /// Get correlation ID
    pub fn correlation_id(&self) -> u128 {
        self.correlation_id
    }
    
    /// Get distributed context for propagation
    pub fn distributed_context(&self) -> DistributedContext {
        self.context.clone().unwrap_or_else(|| {
            let mut context = DistributedContext::new();
            context.correlation_id = crate::observability::CorrelationId::from_u128(self.correlation_id);
            
            if let Some(tenant_id) = self.request.tenant_id {
                context.tenant_id = Some(tenant_id);
            }
            
            if let Some(user_id) = &self.request.user_id {
                context.user_id = Some(user_id.clone());
            }
            
            // Extract trace/span from span if available
            if let Some(span) = &self.span {
                context.trace_id = Some(span.trace_id);
                context.parent_span_id = Some(span.id);
            }
            
            context
        })
    }
}

/// Instrumentation for database operations
pub struct DatabaseInstrumentation {
    /// Observability system
    observability: ObservabilitySystem,
}

impl DatabaseInstrumentation {
    /// Create new database instrumentation
    pub fn new(observability: ObservabilitySystem) -> Self {
        Self { observability }
    }
    
    /// Instrument a database query
    pub fn instrument_query(
        &self,
        query: &DatabaseQueryInfo,
        context: Option<DistributedContext>,
    ) -> InstrumentedDatabaseQuery {
        let start_time = Instant::now();
        
        // Create span for the query
        let span_name = format!("DB {} {}", query.query_type, query.table.as_deref().unwrap_or("query"));
        let span = if let Some(context) = context {
            self.observability.tracer()
                .map(|tracer| tracer.start_span_from_context(&span_name, &context, SpanKind::Internal))
        } else {
            self.observability.tracer()
                .map(|tracer| tracer.start_span(&span_name))
        };
        
        // Add span attributes
        if let Some(span) = span.as_ref() {
            let mut span = span.clone();
            span.add_attribute("db.operation".to_string(), query.query_type.clone());
            
            if let Some(table) = &query.table {
                span.add_attribute("db.table".to_string(), table.clone());
            }
            
            span.add_attribute("db.query".to_string(), query.query_text.clone());
            span.add_attribute("db.params_count".to_string(), query.params_count.to_string());
            
            if let Some(tenant_id) = query.tenant_id {
                span.set_tenant_id(tenant_id);
            }
            
            if let Some(query_plan) = &query.query_plan {
                span.add_attribute("db.query_plan".to_string(), query_plan.clone());
            }
        }
        
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("operation".to_string(), query.query_type.clone());
            
            if let Some(table) = &query.table {
                labels.insert("table".to_string(), table.clone());
            }
            
            if let Some(tenant_id) = query.tenant_id {
                labels.insert("tenant_id".to_string(), tenant_id.to_string());
            }
            
            let _ = collector.increment_counter(metric_names::DB_QUERIES_TOTAL, labels);
        }
        
        // Log query start
        if let Some(logger) = self.observability.logger() {
            let log_context = context.map(|c| {
                let mut ctx = LogContext::from_distributed_context(&c);
                if let Some(tenant_id) = query.tenant_id {
                    ctx = ctx.with_tenant_id(tenant_id);
                }
                ctx = ctx.with_string("db_operation", &query.query_type);
                
                if let Some(table) = &query.table {
                    ctx = ctx.with_string("db_table", table);
                }
                
                ctx
            });
            
            let _ = logger.debug(
                &format!("Database query started: {} ({})", 
                    query.query_type, query.table.as_deref().unwrap_or("unknown")),
                log_context,
            );
        }
        
        InstrumentedDatabaseQuery {
            observability: self.observability.clone(),
            span,
            start_time,
            query: query.clone(),
            context: context.cloned(),
        }
    }
}

/// Instrumented database query
pub struct InstrumentedDatabaseQuery {
    observability: ObservabilitySystem,
    span: Option<Span>,
    start_time: Instant,
    query: DatabaseQueryInfo,
    context: Option<DistributedContext>,
}

impl InstrumentedDatabaseQuery {
    /// Complete the database query with result
    pub fn complete(self, result: DatabaseQueryResult) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        // Update span
        if let Some(mut span) = self.span {
            if let Some(rows_affected) = result.rows_affected {
                span.add_attribute("db.rows_affected".to_string(), rows_affected.to_string());
            }
            
            span.add_attribute("db.duration_ms".to_string(), duration_ms.to_string());
            
            let status = if result.success {
                SpanStatus::Ok
            } else {
                SpanStatus::Error
            };
            
            if let Some(error) = &result.error {
                span.add_attribute("db.error".to_string(), error.clone());
            }
            
            span.end(status);
            
            // End span through tracer
            if let Some(tracer) = self.observability.tracer() {
                let _ = tracer.end_span(span.id, status);
            }
        }
        
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("operation".to_string(), self.query.query_type.clone());
            labels.insert("success".to_string(), result.success.to_string());
            
            if let Some(table) = &self.query.table {
                labels.insert("table".to_string(), table.clone());
            }
            
            if let Some(tenant_id) = self.query.tenant_id {
                labels.insert("tenant_id".to_string(), tenant_id.to_string());
            }
            
            let _ = collector.observe_histogram(
                metric_names::DB_QUERY_DURATION_SECONDS,
                duration.as_secs_f64(),
                labels,
                None,
            );
        }
        
        // Log query completion
        if let Some(logger) = self.observability.logger() {
            let level = if result.success {
                LogLevel::Debug
            } else {
                LogLevel::Error
            };
            
            let mut log_context = self.context.map(|c| LogContext::from_distributed_context(&c))
                .unwrap_or_else(LogContext::new);
            
            log_context = log_context.with_number("duration_ms", duration_ms as f64)
                .with_bool("success", result.success);
            
            if let Some(tenant_id) = self.query.tenant_id {
                log_context = log_context.with_tenant_id(tenant_id);
            }
            
            if let Some(rows_affected) = result.rows_affected {
                log_context = log_context.with_number("rows_affected", rows_affected as f64);
            }
            
            if let Some(error) = &result.error {
                log_context = log_context.with_string("error", error);
            }
            
            let message = if result.success {
                format!("Database query completed: {} ({}) in {}ms", 
                    self.query.query_type, 
                    self.query.table.as_deref().unwrap_or("unknown"),
                    duration_ms)
            } else {
                format!("Database query failed: {} ({}) - {}", 
                    self.query.query_type, 
                    self.query.table.as_deref().unwrap_or("unknown"),
                    result.error.as_deref().unwrap_or("unknown error"))
            };
            
            let _ = logger.log(level, &message, Some(log_context));
        }
    }
}

/// Cache instrumentation
pub struct CacheInstrumentation {
    /// Observability system
    observability: ObservabilitySystem,
}

impl CacheInstrumentation {
    /// Create new cache instrumentation
    pub fn new(observability: ObservabilitySystem) -> Self {
        Self { observability }
    }
    
    /// Instrument cache operation
    pub fn instrument_cache_operation(
        &self,
        operation: &str,
        key: &str,
        hit: bool,
        context: Option<DistributedContext>,
    ) {
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("operation".to_string(), operation.to_string());
            labels.insert("hit".to_string(), hit.to_string());
            
            if let Some(context) = &context {
                if let Some(tenant_id) = context.tenant_id {
                    labels.insert("tenant_id".to_string(), tenant_id.to_string());
                }
            }
            
            let _ = collector.increment_counter("cache_operations_total", labels);
        }
        
        // Log cache operation
        if let Some(logger) = self.observability.logger() {
            let level = if hit { LogLevel::Debug } else { LogLevel::Info };
            
            let log_context = context.map(|c| {
                let mut ctx = LogContext::from_distributed_context(&c);
                ctx = ctx.with_string("cache_operation", operation)
                    .with_string("cache_key", key)
                    .with_bool("cache_hit", hit);
                ctx
            });
            
            let _ = logger.log(
                level,
                &format!("Cache {}: {} (hit: {})", operation, key, hit),
                log_context,
            );
        }
    }
}

/// Message queue instrumentation
pub struct MessageQueueInstrumentation {
    /// Observability system
    observability: ObservabilitySystem,
}

impl MessageQueueInstrumentation {
    /// Create new message queue instrumentation
    pub fn new(observability: ObservabilitySystem) -> Self {
        Self { observability }
    }
    
    /// Instrument message publish
    pub fn instrument_publish(
        &self,
        queue: &str,
        message_size: usize,
        context: Option<DistributedContext>,
    ) {
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("queue".to_string(), queue.to_string());
            
            if let Some(context) = &context {
                if let Some(tenant_id) = context.tenant_id {
                    labels.insert("tenant_id".to_string(), tenant_id.to_string());
                }
            }
            
            let _ = collector.increment_counter("mq_messages_published_total", labels.clone());
            
            let _ = collector.observe_histogram(
                "mq_message_size_bytes",
                message_size as f64,
                labels,
                None,
            );
        }
        
        // Log message publish
        if let Some(logger) = self.observability.logger() {
            let log_context = context.map(|c| {
                let mut ctx = LogContext::from_distributed_context(&c);
                ctx = ctx.with_string("mq_queue", queue)
                    .with_number("mq_message_size", message_size as f64);
                ctx
            });
            
            let _ = logger.debug(
                &format!("Message published to {} (size: {} bytes)", queue, message_size),
                log_context,
            );
        }
    }
    
    /// Instrument message consumption
    pub fn instrument_consume(
        &self,
        queue: &str,
        processing_time_ms: u64,
        success: bool,
        context: Option<DistributedContext>,
    ) {
        // Record metrics
        if let Some(collector) = self.observability.metrics_collector() {
            let mut labels = HashMap::new();
            labels.insert("queue".to_string(), queue.to_string());
            labels.insert("success".to_string(), success.to_string());
            
            if let Some(context) = &context {
                if let Some(tenant_id) = context.tenant_id {
                    labels.insert("tenant_id".to_string(), tenant_id.to_string());
                }
            }
            
            let _ = collector.increment_counter("mq_messages_consumed_total", labels.clone());
            
            let _ = collector.observe_histogram(
                "mq_message_processing_time_ms",
                processing_time_ms as f64,
                labels,
                None,
            );
        }
        
        // Log message consumption
        if let Some(logger) = self.observability.logger() {
            let level = if success { LogLevel::Debug } else { LogLevel::Error };
            
            let log_context = context.map(|c| {
                let mut ctx = LogContext::from_distributed_context(&c);
                ctx = ctx.with_string("mq_queue", queue)
                    .with_number("mq_processing_time_ms", processing_time_ms as f64)
                    .with_bool("mq_success", success);
                ctx
            });
            
            let message = if success {
                format!("Message consumed from {} in {}ms", queue, processing_time_ms)
            } else {
                format!("Message consumption failed from {} after {}ms", queue, processing_time_ms)
            };
            
            let _ = logger.log(level, &message, log_context);
        }
    }
}