use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::observability::{ObservabilityConfig, ObservabilityError, Result, DistributedContext};

/// Distributed tracing span
#[derive(Debug, Clone)]
pub struct Span {
    /// Span ID
    pub id: u64,
    
    /// Trace ID
    pub trace_id: u128,
    
    /// Parent span ID (if any)
    pub parent_id: Option<u64>,
    
    /// Span name
    pub name: String,
    
    /// Span start time (microseconds since epoch)
    pub start_time: u128,
    
    /// Span end time (microseconds since epoch)
    pub end_time: Option<u128>,
    
    /// Span duration (microseconds)
    pub duration: Option<u128>,
    
    /// Span attributes (key-value pairs)
    pub attributes: HashMap<String, String>,
    
    /// Span events (timeline of events within span)
    pub events: Vec<SpanEvent>,
    
    /// Span status (OK, ERROR, etc.)
    pub status: SpanStatus,
    
    /// Span kind (SERVER, CLIENT, INTERNAL, etc.)
    pub kind: SpanKind,
    
    /// Service name
    pub service_name: String,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<Uuid>,
    
    /// Correlation ID
    pub correlation_id: u128,
}

/// Span event (timeline event within a span)
#[derive(Debug, Clone)]
pub struct SpanEvent {
    /// Event name
    pub name: String,
    
    /// Event timestamp (microseconds since epoch)
    pub timestamp: u128,
    
    /// Event attributes
    pub attributes: HashMap<String, String>,
}

/// Span status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStatus {
    /// Span completed successfully
    Ok,
    
    /// Span completed with error
    Error,
    
    /// Span is unset
    Unset,
}

/// Span kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanKind {
    /// Server span (incoming request)
    Server,
    
    /// Client span (outgoing request)
    Client,
    
    /// Internal span (internal operation)
    Internal,
    
    /// Producer span (message producer)
    Producer,
    
    /// Consumer span (message consumer)
    Consumer,
}

/// Span context for propagation
#[derive(Debug, Clone, Copy)]
pub struct SpanContext {
    /// Trace ID
    pub trace_id: u128,
    
    /// Span ID
    pub span_id: u64,
    
    /// Trace flags (sampling, etc.)
    pub trace_flags: u8,
    
    /// Is remote context
    pub is_remote: bool,
}

impl Span {
    /// Create a new span
    pub fn new(
        trace_id: u128,
        parent_id: Option<u64>,
        name: String,
        kind: SpanKind,
        service_name: String,
        correlation_id: u128,
    ) -> Self {
        let start_time = current_time_micros();
        
        Self {
            id: generate_span_id(),
            trace_id,
            parent_id,
            name,
            start_time,
            end_time: None,
            duration: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            status: SpanStatus::Unset,
            kind,
            service_name,
            tenant_id: None,
            correlation_id,
        }
    }
    
    /// Add attribute to span
    pub fn add_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }
    
    /// Add event to span
    pub fn add_event(&mut self, name: String, attributes: HashMap<String, String>) {
        let event = SpanEvent {
            name,
            timestamp: current_time_micros(),
            attributes,
        };
        self.events.push(event);
    }
    
    /// End the span
    pub fn end(&mut self, status: SpanStatus) {
        let end_time = current_time_micros();
        self.end_time = Some(end_time);
        self.duration = Some(end_time.saturating_sub(self.start_time));
        self.status = status;
    }
    
    /// Check if span is ended
    pub fn is_ended(&self) -> bool {
        self.end_time.is_some()
    }
    
    /// Set tenant ID
    pub fn set_tenant_id(&mut self, tenant_id: Uuid) {
        self.tenant_id = Some(tenant_id);
    }
    
    /// Get span context for propagation
    pub fn context(&self) -> SpanContext {
        SpanContext {
            trace_id: self.trace_id,
            span_id: self.id,
            trace_flags: 1, // Sampled
            is_remote: false,
        }
    }
}

/// Tracer for distributed tracing
pub struct Tracer {
    /// Tracer configuration
    config: ObservabilityConfig,
    
    /// Active spans
    active_spans: Arc<RwLock<HashMap<u64, Span>>>,
    
    /// Completed spans for export
    completed_spans: Arc<RwLock<Vec<Span>>>,
    
    /// Span ID generator
    span_id_counter: Arc<RwLock<u64>>,
    
    /// Trace ID generator
    trace_id_counter: Arc<RwLock<u128>>,
}

impl Tracer {
    /// Create a new tracer
    pub fn new(config: &ObservabilityConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            completed_spans: Arc::new(RwLock::new(Vec::new())),
            span_id_counter: Arc::new(RwLock::new(1)),
            trace_id_counter: Arc::new(RwLock::new(1)),
        })
    }
    
    /// Start a new span
    pub fn start_span(&self, name: &str) -> Span {
        let trace_id = self.generate_trace_id();
        let correlation_id = uuid::Uuid::new_v4().as_u128();
        
        let span = Span::new(
            trace_id,
            None,
            name.to_string(),
            SpanKind::Internal,
            self.config.service_name.clone(),
            correlation_id,
        );
        
        let span_id = span.id;
        
        {
            let mut active_spans = self.active_spans.write().unwrap();
            active_spans.insert(span_id, span.clone());
        }
        
        span
    }
    
    /// Start a child span
    pub fn start_child_span(&self, parent_span_id: u64, name: &str, kind: SpanKind) -> Result<Span> {
        let parent_span = {
            let active_spans = self.active_spans.read().unwrap();
            active_spans.get(&parent_span_id).cloned()
        };
        
        let parent_span = parent_span.ok_or_else(|| 
            ObservabilityError::TracingError(format!("Parent span {} not found", parent_span_id))
        )?;
        
        let span = Span::new(
            parent_span.trace_id,
            Some(parent_span_id),
            name.to_string(),
            kind,
            self.config.service_name.clone(),
            parent_span.correlation_id,
        );
        
        let span_id = span.id;
        
        {
            let mut active_spans = self.active_spans.write().unwrap();
            active_spans.insert(span_id, span.clone());
        }
        
        Ok(span)
    }
    
    /// End a span
    pub fn end_span(&self, span_id: u64, status: SpanStatus) -> Result<()> {
        let mut span = {
            let mut active_spans = self.active_spans.write().unwrap();
            active_spans.remove(&span_id)
        };
        
        let mut span = span.ok_or_else(|| 
            ObservabilityError::TracingError(format!("Span {} not found", span_id))
        )?;
        
        span.end(status);
        
        // Move to completed spans
        {
            let mut completed_spans = self.completed_spans.write().unwrap();
            completed_spans.push(span);
        }
        
        Ok(())
    }
    
    /// Get current span by ID
    pub fn get_span(&self, span_id: u64) -> Option<Span> {
        let active_spans = self.active_spans.read().unwrap();
        active_spans.get(&span_id).cloned()
    }
    
    /// Add attribute to span
    pub fn add_span_attribute(&self, span_id: u64, key: String, value: String) -> Result<()> {
        let mut active_spans = self.active_spans.write().unwrap();
        let span = active_spans.get_mut(&span_id)
            .ok_or_else(|| ObservabilityError::TracingError(format!("Span {} not found", span_id)))?;
        
        span.add_attribute(key, value);
        Ok(())
    }
    
    /// Add event to span
    pub fn add_span_event(&self, span_id: u64, name: String, attributes: HashMap<String, String>) -> Result<()> {
        let mut active_spans = self.active_spans.write().unwrap();
        let span = active_spans.get_mut(&span_id)
            .ok_or_else(|| ObservabilityError::TracingError(format!("Span {} not found", span_id)))?;
        
        span.add_event(name, attributes);
        Ok(())
    }
    
    /// Start span from distributed context
    pub fn start_span_from_context(
        &self,
        name: &str,
        context: &DistributedContext,
        kind: SpanKind,
    ) -> Span {
        let trace_id = context.trace_id.unwrap_or_else(|| self.generate_trace_id());
        let parent_span_id = context.parent_span_id;
        
        let span = Span::new(
            trace_id,
            parent_span_id,
            name.to_string(),
            kind,
            self.config.service_name.clone(),
            context.correlation_id.as_u128(),
        );
        
        // Set tenant ID if present
        if let Some(tenant_id) = context.tenant_id {
            let mut span = span.clone();
            span.set_tenant_id(tenant_id);
        }
        
        let span_id = span.id;
        
        {
            let mut active_spans = self.active_spans.write().unwrap();
            active_spans.insert(span_id, span.clone());
        }
        
        span
    }
    
    /// Extract span context for propagation
    pub fn extract_context(&self, span_id: u64) -> Option<DistributedContext> {
        let span = self.get_span(span_id)?;
        
        let mut context = DistributedContext::with_correlation_id(
            crate::observability::CorrelationId::from_u128(span.correlation_id)
        );
        
        context.trace_id = Some(span.trace_id);
        context.parent_span_id = Some(span.id);
        
        if let Some(tenant_id) = span.tenant_id {
            context.tenant_id = Some(tenant_id);
        }
        
        Some(context)
    }
    
    /// Inject span context into headers (for HTTP propagation)
    pub fn inject_to_headers(&self, span_id: u64) -> Result<HashMap<String, String>> {
        let span = self.get_span(span_id)
            .ok_or_else(|| ObservabilityError::TracingError(format!("Span {} not found", span_id)))?;
        
        let mut headers = HashMap::new();
        
        // W3C Trace Context format
        headers.insert("traceparent".to_string(), format!(
            "00-{:032x}-{:016x}-{:02x}",
            span.trace_id,
            span.id,
            1 // sampled
        ));
        
        // Custom headers for Widya
        headers.insert("x-correlation-id".to_string(), span.correlation_id.to_string());
        headers.insert("x-trace-id".to_string(), format!("{:032x}", span.trace_id));
        headers.insert("x-span-id".to_string(), format!("{:016x}", span.id));
        
        if let Some(tenant_id) = span.tenant_id {
            headers.insert("x-tenant-id".to_string(), tenant_id.to_string());
        }
        
        Ok(headers)
    }
    
    /// Extract span context from headers
    pub fn extract_from_headers(&self, headers: &HashMap<String, String>) -> Option<DistributedContext> {
        // Try W3C Trace Context first
        if let Some(traceparent) = headers.get("traceparent") {
            if let Some(context) = self.parse_traceparent(traceparent) {
                return Some(context);
            }
        }
        
        // Fall back to custom Widya headers
        let correlation_id = headers.get("x-correlation-id")
            .and_then(|s| s.parse::<u128>().ok())
            .map(crate::observability::CorrelationId::from_u128)
            .unwrap_or_else(crate::observability::CorrelationId::new);
        
        let mut context = DistributedContext::with_correlation_id(correlation_id);
        
        if let Some(trace_id_str) = headers.get("x-trace-id") {
            if let Ok(trace_id) = u128::from_str_radix(trace_id_str, 16) {
                context.trace_id = Some(trace_id);
            }
        }
        
        if let Some(span_id_str) = headers.get("x-span-id") {
            if let Ok(span_id) = u64::from_str_radix(span_id_str, 16) {
                context.parent_span_id = Some(span_id);
            }
        }
        
        if let Some(tenant_id_str) = headers.get("x-tenant-id") {
            if let Ok(tenant_id) = Uuid::parse_str(tenant_id_str) {
                context.tenant_id = Some(tenant_id);
            }
        }
        
        Some(context)
    }
    
    /// Parse W3C Trace Context header
    fn parse_traceparent(&self, traceparent: &str) -> Option<DistributedContext> {
        let parts: Vec<&str> = traceparent.split('-').collect();
        if parts.len() != 4 {
            return None;
        }
        
        if parts[0] != "00" {
            return None; // Unknown version
        }
        
        let trace_id = u128::from_str_radix(parts[1], 16).ok()?;
        let span_id = u64::from_str_radix(parts[2], 16).ok()?;
        let trace_flags = u8::from_str_radix(parts[3], 16).ok()?;
        
        let is_sampled = (trace_flags & 1) == 1;
        if !is_sampled {
            return None; // Not sampled
        }
        
        let mut context = DistributedContext::new();
        context.trace_id = Some(trace_id);
        context.parent_span_id = Some(span_id);
        
        Some(context)
    }
    
    /// Flush completed spans to exporters
    pub fn flush(&self) -> Result<()> {
        let completed_spans = {
            let mut completed_spans = self.completed_spans.write().unwrap();
            let spans = completed_spans.clone();
            completed_spans.clear();
            spans
        };
        
        if completed_spans.is_empty() {
            return Ok(());
        }
        
        // Export to configured endpoints
        if self.config.tracing.jaeger_enabled {
            self.export_to_jaeger(&completed_spans)?;
        }
        
        if self.config.tracing.zipkin_enabled {
            self.export_to_zipkin(&completed_spans)?;
        }
        
        Ok(())
    }
    
    /// Export spans to Jaeger
    fn export_to_jaeger(&self, spans: &[Span]) -> Result<()> {
        // Simplified Jaeger export
        // In production, use proper Jaeger Thrift/Proto format
        println!("[Jaeger Export] {} spans", spans.len());
        Ok(())
    }
    
    /// Export spans to Zipkin
    fn export_to_zipkin(&self, spans: &[Span]) -> Result<()> {
        // Simplified Zipkin export
        // In production, use proper Zipkin JSON format
        println!("[Zipkin Export] {} spans", spans.len());
        Ok(())
    }
    
    /// Generate new trace ID
    fn generate_trace_id(&self) -> u128 {
        let mut counter = self.trace_id_counter.write().unwrap();
        let trace_id = *counter;
        *counter = counter.wrapping_add(1);
        trace_id
    }
}

/// Generate new span ID
fn generate_span_id() -> u64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen()
}

/// Get current time in microseconds
fn current_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros()
}

/// Trace for grouping related spans
#[derive(Debug, Clone)]
pub struct Trace {
    /// Trace ID
    pub id: u128,
    
    /// Spans in this trace
    pub spans: Vec<Span>,
    
    /// Trace start time
    pub start_time: u128,
    
    /// Trace end time
    pub end_time: Option<u128>,
    
    /// Trace duration
    pub duration: Option<u128>,
    
    /// Service name
    pub service_name: String,
    
    /// Root span (if any)
    pub root_span_id: Option<u64>,
}

impl Trace {
    /// Create a new trace
    pub fn new(trace_id: u128, service_name: String) -> Self {
        Self {
            id: trace_id,
            spans: Vec::new(),
            start_time: current_time_micros(),
            end_time: None,
            duration: None,
            service_name,
            root_span_id: None,
        }
    }
    
    /// Add span to trace
    pub fn add_span(&mut self, span: Span) {
        if span.parent_id.is_none() && self.root_span_id.is_none() {
            self.root_span_id = Some(span.id);
        }
        self.spans.push(span);
    }
    
    /// End trace
    pub fn end(&mut self) {
        let end_time = current_time_micros();
        self.end_time = Some(end_time);
        self.duration = Some(end_time.saturating_sub(self.start_time));
    }
    
    /// Get trace statistics
    pub fn statistics(&self) -> TraceStatistics {
        let mut total_spans = 0;
        let mut error_spans = 0;
        let mut total_duration = 0;
        let mut max_duration = 0;
        
        for span in &self.spans {
            total_spans += 1;
            
            if span.status == SpanStatus::Error {
                error_spans += 1;
            }
            
            if let Some(duration) = span.duration {
                total_duration += duration;
                if duration > max_duration {
                    max_duration = duration;
                }
            }
        }
        
        let avg_duration = if total_spans > 0 {
            total_duration / total_spans as u128
        } else {
            0
        };
        
        TraceStatistics {
            total_spans,
            error_spans,
            avg_duration_micros: avg_duration,
            max_duration_micros: max_duration,
            error_rate: if total_spans > 0 {
                error_spans as f64 / total_spans as f64
            } else {
                0.0
            },
        }
    }
}

/// Trace statistics
#[derive(Debug, Clone)]
pub struct TraceStatistics {
    pub total_spans: usize,
    pub error_spans: usize,
    pub avg_duration_micros: u128,
    pub max_duration_micros: u128,
    pub error_rate: f64,
}