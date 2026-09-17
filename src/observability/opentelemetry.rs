//! OpenTelemetry compatibility for Widya observability
//! Provides OpenTelemetry-compatible interfaces for tracing, metrics, and logging

use std::collections::HashMap;
use uuid::Uuid;

use crate::observability::{
    ObservabilitySystem, DistributedContext, LogContext,
    tracing::{Span, SpanKind, SpanStatus, Tracer},
    metrics::{MetricsCollector, Metric, MetricType, MetricValue},
    logging::{Logger, LogLevel},
};

/// OpenTelemetry span wrapper
pub struct OtelSpan {
    /// Internal Widya span
    widya_span: Span,
    
    /// Tracer reference
    tracer: Option<Tracer>,
}

impl OtelSpan {
    /// Create new OpenTelemetry span from Widya span
    pub fn new(widya_span: Span, tracer: Option<Tracer>) -> Self {
        Self {
            widya_span,
            tracer,
        }
    }
    
    /// Get span ID
    pub fn span_id(&self) -> u64 {
        self.widya_span.id
    }
    
    /// Get trace ID
    pub fn trace_id(&self) -> u128 {
        self.widya_span.trace_id
    }
    
    /// Set attribute
    pub fn set_attribute(&mut self, key: String, value: String) {
        self.widya_span.add_attribute(key, value);
    }
    
    /// Add event
    pub fn add_event(&mut self, name: String, attributes: HashMap<String, String>) {
        self.widya_span.add_event(name, attributes);
    }
    
    /// Set status
    pub fn set_status(&mut self, status: SpanStatus) {
        if let Some(tracer) = &self.tracer {
            let _ = tracer.end_span(self.widya_span.id, status);
        }
    }
    
    /// End span
    pub fn end(self) {
        // Span is automatically ended when dropped
        // or when set_status is called
    }
}

/// OpenTelemetry tracer wrapper
pub struct OtelTracer {
    /// Internal Widya tracer
    widya_tracer: Tracer,
    
    /// Service name
    service_name: String,
}

impl OtelTracer {
    /// Create new OpenTelemetry tracer
    pub fn new(widya_tracer: Tracer, service_name: String) -> Self {
        Self {
            widya_tracer,
            service_name,
        }
    }
    
    /// Start a new span
    pub fn start(&self, name: &str) -> OtelSpan {
        let span = self.widya_tracer.start_span(name);
        OtelSpan::new(span, Some(self.widya_tracer.clone()))
    }
    
    /// Start a span with parent context
    pub fn start_with_context(&self, name: &str, context: &OtelSpanContext) -> OtelSpan {
        let distributed_context = DistributedContext::new()
            .with_trace_id(context.trace_id)
            .with_parent_span_id(context.span_id);
        
        let span = self.widya_tracer.start_span_from_context(
            name,
            &distributed_context,
            SpanKind::Internal,
        );
        
        OtelSpan::new(span, Some(self.widya_tracer.clone()))
    }
    
    /// Extract context from headers (W3C Trace Context)
    pub fn extract_from_headers(&self, headers: &HashMap<String, String>) -> Option<OtelSpanContext> {
        self.widya_tracer.extract_from_headers(headers)
            .map(|context| OtelSpanContext {
                trace_id: context.trace_id.unwrap_or(0),
                span_id: context.parent_span_id.unwrap_or(0),
            })
    }
    
    /// Inject context into headers (W3C Trace Context)
    pub fn inject_to_headers(&self, span: &OtelSpan) -> Result<HashMap<String, String>, String> {
        self.widya_tracer.inject_to_headers(span.span_id())
            .map_err(|e| e.to_string())
    }
}

/// OpenTelemetry span context
#[derive(Debug, Clone, Copy)]
pub struct OtelSpanContext {
    /// Trace ID
    pub trace_id: u128,
    
    /// Span ID
    pub span_id: u64,
}

/// OpenTelemetry metrics exporter
pub struct OtelMetricsExporter {
    /// Internal Widya metrics collector
    widya_collector: MetricsCollector,
}

impl OtelMetricsExporter {
    /// Create new OpenTelemetry metrics exporter
    pub fn new(widya_collector: MetricsCollector) -> Self {
        Self { widya_collector }
    }
    
    /// Export counter metric
    pub fn export_counter(
        &self,
        name: &str,
        value: u64,
        attributes: HashMap<String, String>,
    ) -> Result<(), String> {
        let metric = Metric::counter(name.to_string(), value, attributes);
        self.widya_collector.record(metric)
            .map_err(|e| e.to_string())
    }
    
    /// Export gauge metric
    pub fn export_gauge(
        &self,
        name: &str,
        value: f64,
        attributes: HashMap<String, String>,
    ) -> Result<(), String> {
        let metric = Metric::gauge(name.to_string(), value, attributes);
        self.widya_collector.record(metric)
            .map_err(|e| e.to_string())
    }
    
    /// Export histogram metric
    pub fn export_histogram(
        &self,
        name: &str,
        sum: f64,
        count: u64,
        bucket_counts: Vec<(f64, u64)>,
        attributes: HashMap<String, String>,
    ) -> Result<(), String> {
        let metric = Metric::histogram(name.to_string(), sum, count, bucket_counts, attributes);
        self.widya_collector.record(metric)
            .map_err(|e| e.to_string())
    }
    
    /// Export summary metric
    pub fn export_summary(
        &self,
        name: &str,
        sum: f64,
        count: u64,
        quantile_values: Vec<(f64, f64)>,
        attributes: HashMap<String, String>,
    ) -> Result<(), String> {
        let metric = Metric::summary(name.to_string(), sum, count, quantile_values, attributes);
        self.widya_collector.record(metric)
            .map_err(|e| e.to_string())
    }
    
    /// Get metrics in OpenTelemetry Proto format (simplified)
    pub fn get_metrics_proto(&self) -> String {
        // Simplified Proto representation
        // In production, generate proper OpenTelemetry Proto
        let prometheus_text = self.widya_collector.get_prometheus_metrics();
        format!("OpenTelemetry Proto Metrics (converted from Prometheus):\n{}", prometheus_text)
    }
}

/// OpenTelemetry logger wrapper
pub struct OtelLogger {
    /// Internal Widya logger
    widya_logger: Logger,
}

impl OtelLogger {
    /// Create new OpenTelemetry logger
    pub fn new(widya_logger: Logger) -> Self {
        Self { widya_logger }
    }
    
    /// Log with OpenTelemetry severity
    pub fn log(
        &self,
        severity: OtelSeverity,
        message: &str,
        attributes: HashMap<String, String>,
        trace_id: Option<u128>,
        span_id: Option<u64>,
    ) -> Result<(), String> {
        let level = match severity {
            OtelSeverity::Trace => LogLevel::Trace,
            OtelSeverity::Debug => LogLevel::Debug,
            OtelSeverity::Info => LogLevel::Info,
            OtelSeverity::Warn => LogLevel::Warn,
            OtelSeverity::Error => LogLevel::Error,
            OtelSeverity::Fatal => LogLevel::Fatal,
        };
        
        let mut context = LogContext::new();
        
        // Add attributes
        for (key, value) in attributes {
            context = context.with_string(&key, &value);
        }
        
        // Add trace/span context
        if let Some(trace_id) = trace_id {
            context = context.with_correlation_id(trace_id);
        }
        
        if let Some(span_id) = span_id {
            context = context.with_number("span_id", span_id as f64);
        }
        
        self.widya_logger.log(level, message, Some(context))
            .map_err(|e| e.to_string())
    }
    
    /// Log with error
    pub fn log_error(
        &self,
        message: &str,
        error: &dyn std::error::Error,
        attributes: HashMap<String, String>,
    ) -> Result<(), String> {
        let mut context = LogContext::new();
        
        // Add attributes
        for (key, value) in attributes {
            context = context.with_string(&key, &value);
        }
        
        // Add error
        context.error = Some(crate::observability::logging::LogError::from_std_error(error));
        
        self.widya_logger.log(LogLevel::Error, message, Some(context))
            .map_err(|e| e.to_string())
    }
}

/// OpenTelemetry severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtelSeverity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// OpenTelemetry resource attributes
pub struct OtelResource {
    /// Service name
    pub service_name: String,
    
    /// Service version
    pub service_version: String,
    
    /// Service namespace
    pub service_namespace: Option<String>,
    
    /// Service instance ID
    pub service_instance_id: Option<String>,
    
    /// Deployment environment
    pub deployment_environment: Option<String>,
    
    /// Additional attributes
    pub attributes: HashMap<String, String>,
}

impl OtelResource {
    /// Create new OpenTelemetry resource
    pub fn new(service_name: String, service_version: String) -> Self {
        Self {
            service_name,
            service_version,
            service_namespace: None,
            service_instance_id: None,
            deployment_environment: None,
            attributes: HashMap::new(),
        }
    }
    
    /// Add attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
    
    /// Set deployment environment
    pub fn with_environment(mut self, environment: String) -> Self {
        self.deployment_environment = Some(environment);
        self
    }
    
    /// Get attributes as HashMap
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = self.attributes.clone();
        
        map.insert("service.name".to_string(), self.service_name.clone());
        map.insert("service.version".to_string(), self.service_version.clone());
        
        if let Some(namespace) = &self.service_namespace {
            map.insert("service.namespace".to_string(), namespace.clone());
        }
        
        if let Some(instance_id) = &self.service_instance_id {
            map.insert("service.instance.id".to_string(), instance_id.clone());
        }
        
        if let Some(environment) = &self.deployment_environment {
            map.insert("deployment.environment".to_string(), environment.clone());
        }
        
        map
    }
}

/// OpenTelemetry SDK for Widya
pub struct OtelSdk {
    /// Tracer
    pub tracer: OtelTracer,
    
    /// Metrics exporter
    pub metrics_exporter: OtelMetricsExporter,
    
    /// Logger
    pub logger: OtelLogger,
    
    /// Resource
    pub resource: OtelResource,
}

impl OtelSdk {
    /// Create new OpenTelemetry SDK from Widya observability system
    pub fn from_widya(
        observability: &ObservabilitySystem,
        resource: OtelResource,
    ) -> Option<Self> {
        let tracer = observability.tracer()
            .map(|t| OtelTracer::new(t.clone(), resource.service_name.clone()))?;
        
        let metrics_exporter = observability.metrics_collector()
            .map(|c| OtelMetricsExporter::new(c.clone()))?;
        
        let logger = observability.logger()
            .map(|l| OtelLogger::new(l.clone()))?;
        
        Some(Self {
            tracer,
            metrics_exporter,
            logger,
            resource,
        })
    }
    
    /// Create a tracer provider
    pub fn tracer_provider(&self) -> OtelTracerProvider {
        OtelTracerProvider {
            tracer: self.tracer.clone(),
            resource: self.resource.clone(),
        }
    }
    
    /// Create a meter provider
    pub fn meter_provider(&self) -> OtelMeterProvider {
        OtelMeterProvider {
            exporter: self.metrics_exporter.clone(),
            resource: self.resource.clone(),
        }
    }
    
    /// Create a logger provider
    pub fn logger_provider(&self) -> OtelLoggerProvider {
        OtelLoggerProvider {
            logger: self.logger.clone(),
            resource: self.resource.clone(),
        }
    }
}

/// OpenTelemetry tracer provider
#[derive(Clone)]
pub struct OtelTracerProvider {
    tracer: OtelTracer,
    resource: OtelResource,
}

impl OtelTracerProvider {
    /// Get a tracer
    pub fn tracer(&self, name: &str) -> OtelTracer {
        // Name is ignored in this implementation
        // In full implementation, would create named tracers
        self.tracer.clone()
    }
    
    /// Shutdown tracer provider
    pub fn shutdown(&self) {
        // Nothing to shutdown in this implementation
    }
}

/// OpenTelemetry meter provider
#[derive(Clone)]
pub struct OtelMeterProvider {
    exporter: OtelMetricsExporter,
    resource: OtelResource,
}

impl OtelMeterProvider {
    /// Get a meter
    pub fn meter(&self, name: &str) -> OtelMeter {
        OtelMeter {
            name: name.to_string(),
            exporter: self.exporter.clone(),
            resource: self.resource.clone(),
        }
    }
    
    /// Force flush
    pub fn force_flush(&self) -> Result<(), String> {
        Ok(()) // Flush is handled internally
    }
    
    /// Shutdown meter provider
    pub fn shutdown(&self) {
        // Nothing to shutdown in this implementation
    }
}

/// OpenTelemetry meter
pub struct OtelMeter {
    name: String,
    exporter: OtelMetricsExporter,
    resource: OtelResource,
}

impl OtelMeter {
    /// Create a counter
    pub fn create_counter(&self, name: &str) -> OtelCounter {
        OtelCounter {
            name: format!("{}.{}", self.name, name),
            exporter: self.exporter.clone(),
            resource: self.resource.clone(),
        }
    }
    
    /// Create a gauge
    pub fn create_gauge(&self, name: &str) -> OtelGauge {
        OtelGauge {
            name: format!("{}.{}", self.name, name),
            exporter: self.exporter.clone(),
            resource: self.resource.clone(),
        }
    }
    
    /// Create a histogram
    pub fn create_histogram(&self, name: &str) -> OtelHistogram {
        OtelHistogram {
            name: format!("{}.{}", self.name, name),
            exporter: self.exporter.clone(),
            resource: self.resource.clone(),
        }
    }
}

/// OpenTelemetry counter
pub struct OtelCounter {
    name: String,
    exporter: OtelMetricsExporter,
    resource: OtelResource,
}

impl OtelCounter {
    /// Add to counter
    pub fn add(&self, value: u64, attributes: HashMap<String, String>) -> Result<(), String> {
        let mut all_attributes = self.resource.to_hashmap();
        all_attributes.extend(attributes);
        
        self.exporter.export_counter(&self.name, value, all_attributes)
    }
}

/// OpenTelemetry gauge
pub struct OtelGauge {
    name: String,
    exporter: OtelMetricsExporter,
    resource: OtelResource,
}

impl OtelGauge {
    /// Record gauge value
    pub fn record(&self, value: f64, attributes: HashMap<String, String>) -> Result<(), String> {
        let mut all_attributes = self.resource.to_hashmap();
        all_attributes.extend(attributes);
        
        self.exporter.export_gauge(&self.name, value, all_attributes)
    }
}

/// OpenTelemetry histogram
pub struct OtelHistogram {
    name: String,
    exporter: OtelMetricsExporter,
    resource: OtelResource,
}

impl OtelHistogram {
    /// Record histogram value
    pub fn record(&self, value: f64, attributes: HashMap<String, String>) -> Result<(), String> {
        let mut all_attributes = self.resource.to_hashmap();
        all_attributes.extend(attributes);
        
        // Simplified histogram with single bucket
        let bucket_counts = vec![(value, 1)];
        
        self.exporter.export_histogram(&self.name, value, 1, bucket_counts, all_attributes)
    }
}

/// OpenTelemetry logger provider
#[derive(Clone)]
pub struct OtelLoggerProvider {
    logger: OtelLogger,
    resource: OtelResource,
}

impl OtelLoggerProvider {
    /// Get a logger
    pub fn logger(&self, name: &str) -> OtelLoggerBridge {
        OtelLoggerBridge {
            name: name.to_string(),
            logger: self.logger.clone(),
            resource: self.resource.clone(),
        }
    }
}

/// OpenTelemetry logger bridge
pub struct OtelLoggerBridge {
    name: String,
    logger: OtelLogger,
    resource: OtelResource,
}

impl OtelLoggerBridge {
    /// Emit log record
    pub fn emit(&self, record: OtelLogRecord) -> Result<(), String> {
        let mut attributes = self.resource.to_hashmap();
        attributes.insert("logger.name".to_string(), self.name.clone());
        attributes.extend(record.attributes);
        
        self.logger.log(
            record.severity,
            &record.body,
            attributes,
            record.trace_id,
            record.span_id,
        )
    }
}

/// OpenTelemetry log record
pub struct OtelLogRecord {
    /// Log severity
    pub severity: OtelSeverity,
    
    /// Log body/message
    pub body: String,
    
    /// Timestamp (nanoseconds since Unix epoch)
    pub timestamp: Option<u128>,
    
    /// Trace ID
    pub trace_id: Option<u128>,
    
    /// Span ID
    pub span_id: Option<u64>,
    
    /// Attributes
    pub attributes: HashMap<String, String>,
}

impl OtelLogRecord {
    /// Create new log record
    pub fn new(severity: OtelSeverity, body: String) -> Self {
        Self {
            severity,
            body,
            timestamp: None,
            trace_id: None,
            span_id: None,
            attributes: HashMap::new(),
        }
    }
    
    /// Add attribute
    pub fn with_attribute(mut self, key: String, value: String) -> Self {
        self.attributes.insert(key, value);
        self
    }
    
    /// Set trace context
    pub fn with_trace_context(mut self, trace_id: u128, span_id: u64) -> Self {
        self.trace_id = Some(trace_id);
        self.span_id = Some(span_id);
        self
    }
}