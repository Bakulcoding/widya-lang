//! Observability module for Widya Enterprise Edition
//! Provides distributed tracing, metrics collection, and structured logging

pub mod tracing;
pub mod metrics;
pub mod logging;
pub mod instrumentation;
pub mod opentelemetry;
pub mod debugging;

// Re-export commonly used types
pub use tracing::{Trace, Span, Tracer, SpanContext};
pub use metrics::{MetricsCollector, Metric, MetricType, MetricValue};
pub use logging::{Logger, LogLevel, StructuredLog, LogContext};
pub use instrumentation::{
    HttpInstrumentation, DatabaseInstrumentation, CacheInstrumentation, MessageQueueInstrumentation,
    HttpRequestInfo, HttpResponseInfo, DatabaseQueryInfo, DatabaseQueryResult,
    InstrumentedHttpRequest, InstrumentedDatabaseQuery,
};
pub use opentelemetry::{
    OtelSdk, OtelTracer, OtelSpan, OtelSpanContext, OtelMetricsExporter, OtelLogger,
    OtelSeverity, OtelResource, OtelLogRecord,
};
pub use debugging::{TraceAnalyzer, TraceVisualizer, TraceDebugger, TraceAnalysisReport};

/// Observability errors
#[derive(Debug, thiserror::Error)]
pub enum ObservabilityError {
    #[error("Tracing error: {0}")]
    TracingError(String),
    
    #[error("Metrics error: {0}")]
    MetricsError(String),
    
    #[error("Logging error: {0}")]
    LoggingError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Export error: {0}")]
    ExportError(String),
    
    #[error("Instrumentation error: {0}")]
    InstrumentationError(String),
}

/// Result type for observability operations
pub type Result<T> = std::result::Result<T, ObservabilityError>;

/// Observability configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObservabilityConfig {
    /// Tracing configuration
    pub tracing: TracingConfig,
    
    /// Metrics configuration
    pub metrics: MetricsConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
    
    /// Service name for telemetry
    pub service_name: String,
    
    /// Service version
    pub service_version: String,
    
    /// Environment (dev, staging, prod)
    pub environment: String,
}

/// Tracing configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TracingConfig {
    /// Enable tracing
    pub enabled: bool,
    
    /// Sampling rate (0.0 to 1.0)
    pub sampling_rate: f64,
    
    /// Export to Jaeger
    pub jaeger_enabled: bool,
    
    /// Jaeger endpoint
    pub jaeger_endpoint: Option<String>,
    
    /// Export to Zipkin
    pub zipkin_enabled: bool,
    
    /// Zipkin endpoint
    pub zipkin_endpoint: Option<String>,
}

/// Metrics configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsConfig {
    /// Enable metrics
    pub enabled: bool,
    
    /// Prometheus export enabled
    pub prometheus_enabled: bool,
    
    /// Prometheus scrape endpoint
    pub prometheus_endpoint: Option<String>,
    
    /// Metrics collection interval in seconds
    pub collection_interval_secs: u64,
    
    /// Metric retention period in days
    pub retention_days: u32,
}

/// Logging configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoggingConfig {
    /// Enable logging
    pub enabled: bool,
    
    /// Log level (TRACE, DEBUG, INFO, WARN, ERROR)
    pub level: String,
    
    /// Output format (json, text)
    pub format: String,
    
    /// Output destination (stdout, file, both)
    pub destination: String,
    
    /// Log file path (if file output enabled)
    pub file_path: Option<String>,
    
    /// Enable structured logging
    pub structured: bool,
    
    /// Include timestamp in logs
    pub include_timestamp: bool,
    
    /// Include thread ID in logs
    pub include_thread_id: bool,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            service_name: "widya-service".to_string(),
            service_version: "0.1.0".to_string(),
            environment: "development".to_string(),
            tracing: TracingConfig::default(),
            metrics: MetricsConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sampling_rate: 1.0, // Sample all traces
            jaeger_enabled: false,
            jaeger_endpoint: None,
            zipkin_enabled: false,
            zipkin_endpoint: None,
        }
    }
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            prometheus_enabled: true,
            prometheus_endpoint: Some("/metrics".to_string()),
            collection_interval_secs: 15,
            retention_days: 30,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level: "INFO".to_string(),
            format: "json".to_string(),
            destination: "stdout".to_string(),
            file_path: None,
            structured: true,
            include_timestamp: true,
            include_thread_id: true,
        }
    }
}

/// Initialize observability system
pub fn init(config: ObservabilityConfig) -> Result<ObservabilitySystem> {
    let system = ObservabilitySystem::new(config)?;
    Ok(system)
}

/// Observability system
pub struct ObservabilitySystem {
    config: ObservabilityConfig,
    tracer: Option<tracing::Tracer>,
    metrics_collector: Option<metrics::MetricsCollector>,
    logger: Option<logging::Logger>,
}

impl ObservabilitySystem {
    /// Create a new observability system
    pub fn new(config: ObservabilityConfig) -> Result<Self> {
        let tracer = if config.tracing.enabled {
            Some(tracing::Tracer::new(&config)?)
        } else {
            None
        };
        
        let metrics_collector = if config.metrics.enabled {
            Some(metrics::MetricsCollector::new(&config)?)
        } else {
            None
        };
        
        let logger = if config.logging.enabled {
            Some(logging::Logger::new(&config)?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            tracer,
            metrics_collector,
            logger,
        })
    }
    
    /// Get tracer
    pub fn tracer(&self) -> Option<&tracing::Tracer> {
        self.tracer.as_ref()
    }
    
    /// Get metrics collector
    pub fn metrics_collector(&self) -> Option<&metrics::MetricsCollector> {
        self.metrics_collector.as_ref()
    }
    
    /// Get logger
    pub fn logger(&self) -> Option<&logging::Logger> {
        self.logger.as_ref()
    }
    
    /// Get configuration
    pub fn config(&self) -> &ObservabilityConfig {
        &self.config
    }
    
    /// Start a span
    pub fn start_span(&self, name: &str) -> Option<tracing::Span> {
        self.tracer.as_ref().map(|tracer| tracer.start_span(name))
    }
    
    /// Record a metric
    pub fn record_metric(&self, metric: metrics::Metric) -> Result<()> {
        if let Some(collector) = &self.metrics_collector {
            collector.record(metric)
        } else {
            Ok(())
        }
    }
    
    /// Log a message
    pub fn log(&self, level: logging::LogLevel, message: &str, context: Option<LogContext>) -> Result<()> {
        if let Some(logger) = &self.logger {
            logger.log(level, message, context)
        } else {
            Ok(())
        }
    }
    
    /// Shutdown observability system
    pub fn shutdown(&self) -> Result<()> {
        // Flush any pending exports
        if let Some(tracer) = &self.tracer {
            tracer.flush()?;
        }
        
        if let Some(collector) = &self.metrics_collector {
            collector.flush()?;
        }
        
        if let Some(logger) = &self.logger {
            logger.flush()?;
        }
        
        Ok(())
    }
}

/// Correlation ID for distributed tracing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CorrelationId(u128);

impl CorrelationId {
    /// Generate a new correlation ID
    pub fn new() -> Self {
        use uuid::Uuid;
        let uuid = Uuid::new_v4();
        Self(uuid.as_u128())
    }
    
    /// Create from existing value
    pub fn from_u128(value: u128) -> Self {
        Self(value)
    }
    
    /// Get the value
    pub fn as_u128(&self) -> u128 {
        self.0
    }
    
    /// Format as hex string
    pub fn to_hex(&self) -> String {
        format!("{:032x}", self.0)
    }
}

impl Default for CorrelationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for CorrelationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

/// Context for distributed operations
#[derive(Debug, Clone)]
pub struct DistributedContext {
    /// Correlation ID
    pub correlation_id: CorrelationId,
    
    /// Parent span ID (if any)
    pub parent_span_id: Option<u64>,
    
    /// Trace ID
    pub trace_id: Option<u128>,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<uuid::Uuid>,
    
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    
    /// Additional context
    pub extra: std::collections::HashMap<String, String>,
}

impl DistributedContext {
    /// Create a new context
    pub fn new() -> Self {
        Self {
            correlation_id: CorrelationId::new(),
            parent_span_id: None,
            trace_id: None,
            tenant_id: None,
            user_id: None,
            extra: std::collections::HashMap::new(),
        }
    }
    
    /// Create with correlation ID
    pub fn with_correlation_id(correlation_id: CorrelationId) -> Self {
        Self {
            correlation_id,
            parent_span_id: None,
            trace_id: None,
            tenant_id: None,
            user_id: None,
            extra: std::collections::HashMap::new(),
        }
    }
    
    /// Add extra context
    pub fn with_extra(mut self, key: String, value: String) -> Self {
        self.extra.insert(key, value);
        self
    }
    
    /// Set tenant ID
    pub fn with_tenant_id(mut self, tenant_id: uuid::Uuid) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
    
    /// Set trace ID
    pub fn with_trace_id(mut self, trace_id: u128) -> Self {
        self.trace_id = Some(trace_id);
        self
    }
    
    /// Set parent span ID
    pub fn with_parent_span_id(mut self, parent_span_id: u64) -> Self {
        self.parent_span_id = Some(parent_span_id);
        self
    }
    
    /// Get correlation ID as hex string
    pub fn correlation_id_hex(&self) -> String {
        self.correlation_id.to_hex()
    }
}

impl Default for DistributedContext {
    fn default() -> Self {
        Self::new()
    }
}