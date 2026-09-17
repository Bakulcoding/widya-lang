use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

use crate::observability::{ObservabilityConfig, ObservabilityError, Result, DistributedContext};

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level - very detailed debugging information
    Trace,
    
    /// Debug level - debugging information
    Debug,
    
    /// Info level - informational messages
    Info,
    
    /// Warn level - warning messages
    Warn,
    
    /// Error level - error messages
    Error,
    
    /// Fatal level - critical errors (will terminate application)
    Fatal,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

impl From<&str> for LogLevel {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => LogLevel::Info, // Default
        }
    }
}

/// Log context for structured logging
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogContext {
    /// Correlation ID for distributed tracing
    pub correlation_id: Option<u128>,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<uuid::Uuid>,
    
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    
    /// Span ID (for tracing integration)
    pub span_id: Option<u64>,
    
    /// Trace ID (for tracing integration)
    pub trace_id: Option<u128>,
    
    /// Additional context fields
    pub fields: HashMap<String, serde_json::Value>,
    
    /// Error information (if applicable)
    pub error: Option<LogError>,
}

impl LogContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create from distributed context
    pub fn from_distributed_context(context: &DistributedContext) -> Self {
        let mut log_context = Self::new();
        
        log_context.correlation_id = Some(context.correlation_id.as_u128());
        log_context.tenant_id = context.tenant_id;
        log_context.trace_id = context.trace_id;
        log_context.span_id = context.parent_span_id;
        
        // Copy extra fields
        for (key, value) in &context.extra {
            log_context.fields.insert(key.clone(), serde_json::Value::String(value.clone()));
        }
        
        log_context
    }
    
    /// Add a field to the context
    pub fn with_field(mut self, key: &str, value: serde_json::Value) -> Self {
        self.fields.insert(key.to_string(), value);
        self
    }
    
    /// Add string field
    pub fn with_string(mut self, key: &str, value: &str) -> Self {
        self.fields.insert(key.to_string(), serde_json::Value::String(value.to_string()));
        self
    }
    
    /// Add number field
    pub fn with_number(mut self, key: &str, value: f64) -> Self {
        self.fields.insert(key.to_string(), serde_json::Value::Number(
            serde_json::Number::from_f64(value).unwrap_or_else(|| serde_json::Number::from(0))
        ));
        self
    }
    
    /// Add boolean field
    pub fn with_bool(mut self, key: &str, value: bool) -> Self {
        self.fields.insert(key.to_string(), serde_json::Value::Bool(value));
        self
    }
    
    /// Set correlation ID
    pub fn with_correlation_id(mut self, correlation_id: u128) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    /// Set tenant ID
    pub fn with_tenant_id(mut self, tenant_id: uuid::Uuid) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }
    
    /// Set user ID
    pub fn with_user_id(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }
    
    /// Set error information
    pub fn with_error(mut self, error: LogError) -> Self {
        self.error = Some(error);
        self
    }
}

/// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    /// Log level
    pub level: LogLevel,
    
    /// Log message
    pub message: String,
    
    /// Log context
    pub context: LogContext,
    
    /// Timestamp (microseconds since epoch)
    pub timestamp: u128,
    
    /// Thread ID (if available)
    pub thread_id: Option<u64>,
    
    /// File where log was emitted
    pub file: Option<String>,
    
    /// Line number where log was emitted
    pub line: Option<u32>,
    
    /// Module path
    pub module_path: Option<String>,
}

/// Log error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogError {
    /// Error type/kind
    pub error_type: String,
    
    /// Error message
    pub message: String,
    
    /// Error source (optional)
    pub source: Option<String>,
    
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
    
    /// Error code (if applicable)
    pub code: Option<String>,
}

impl LogError {
    /// Create a new log error
    pub fn new(error_type: &str, message: &str) -> Self {
        Self {
            error_type: error_type.to_string(),
            message: message.to_string(),
            source: None,
            stack_trace: None,
            code: None,
        }
    }
    
    /// Create from standard error
    pub fn from_std_error<E: std::error::Error>(error: &E) -> Self {
        Self {
            error_type: std::any::type_name::<E>().to_string(),
            message: error.to_string(),
            source: error.source().map(|e| e.to_string()),
            stack_trace: None,
            code: None,
        }
    }
}

/// Logger for structured logging
pub struct Logger {
    /// Logger configuration
    config: ObservabilityConfig,
    
    /// Log buffer for batch processing
    log_buffer: Arc<RwLock<Vec<StructuredLog>>>,
    
    /// File writer (if file output enabled)
    file_writer: Option<Arc<RwLock<std::fs::File>>>,
    
    /// Minimum log level
    min_level: LogLevel,
}

impl Logger {
    /// Create a new logger
    pub fn new(config: &ObservabilityConfig) -> Result<Self> {
        let min_level = LogLevel::from(config.logging.level.as_str());
        
        // Create file writer if file output is enabled
        let file_writer = if config.logging.destination.contains("file") {
            if let Some(file_path) = &config.logging.file_path {
                let file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .map_err(|e| ObservabilityError::LoggingError(
                        format!("Failed to open log file {}: {}", file_path, e)
                    ))?;
                
                Some(Arc::new(RwLock::new(file)))
            } else {
                None
            }
        } else {
            None
        };
        
        Ok(Self {
            config: config.clone(),
            log_buffer: Arc::new(RwLock::new(Vec::new())),
            file_writer,
            min_level,
        })
    }
    
    /// Log a message with context
    pub fn log(&self, level: LogLevel, message: &str, context: Option<LogContext>) -> Result<()> {
        // Check if log level is enabled
        if level < self.min_level {
            return Ok(());
        }
        
        let timestamp = current_time_micros();
        let thread_id = get_thread_id();
        
        let log_entry = StructuredLog {
            level,
            message: message.to_string(),
            context: context.unwrap_or_default(),
            timestamp,
            thread_id,
            file: None, // Would be set by macros
            line: None, // Would be set by macros
            module_path: None, // Would be set by macros
        };
        
        // Add to buffer
        {
            let mut buffer = self.log_buffer.write().unwrap();
            buffer.push(log_entry.clone());
            
            // Flush if buffer is large
            if buffer.len() >= 1000 {
                self.flush_buffer()?;
            }
        }
        
        // Output based on configuration
        self.output_log(&log_entry)?;
        
        Ok(())
    }
    
    /// Output log based on configuration
    fn output_log(&self, log: &StructuredLog) -> Result<()> {
        match self.config.logging.format.as_str() {
            "json" => self.output_json(log),
            "text" => self.output_text(log),
            _ => self.output_json(log), // Default to JSON
        }
    }
    
    /// Output log as JSON
    fn output_json(&self, log: &StructuredLog) -> Result<()> {
        let json_output = serde_json::to_string(log)
            .map_err(|e| ObservabilityError::LoggingError(format!("Failed to serialize log: {}", e)))?;
        
        self.write_output(&format!("{}\n", json_output))
    }
    
    /// Output log as text
    fn output_text(&self, log: &StructuredLog) -> Result<()> {
        let timestamp = if self.config.logging.include_timestamp {
            format_timestamp(log.timestamp)
        } else {
            String::new()
        };
        
        let thread_id = if self.config.logging.include_thread_id && log.thread_id.is_some() {
            format!("[T{}] ", log.thread_id.unwrap())
        } else {
            String::new()
        };
        
        let mut output = format!("{}{}{}: {}", timestamp, thread_id, log.level, log.message);
        
        // Add context if available
        if !log.context.fields.is_empty() || log.context.correlation_id.is_some() {
            output.push_str(" [");
            
            let mut context_parts = Vec::new();
            
            if let Some(correlation_id) = log.context.correlation_id {
                context_parts.push(format!("correlation_id={}", correlation_id));
            }
            
            if let Some(tenant_id) = log.context.tenant_id {
                context_parts.push(format!("tenant_id={}", tenant_id));
            }
            
            if let Some(user_id) = &log.context.user_id {
                context_parts.push(format!("user_id={}", user_id));
            }
            
            for (key, value) in &log.context.fields {
                context_parts.push(format!("{}={}", key, value));
            }
            
            output.push_str(&context_parts.join(", "));
            output.push(']');
        }
        
        // Add error if present
        if let Some(error) = &log.context.error {
            output.push_str(&format!(" Error: {} - {}", error.error_type, error.message));
        }
        
        output.push('\n');
        
        self.write_output(&output)
    }
    
    /// Write output to configured destinations
    fn write_output(&self, output: &str) -> Result<()> {
        // Output to stdout if configured
        if self.config.logging.destination.contains("stdout") {
            print!("{}", output);
        }
        
        // Output to file if configured
        if let Some(file_writer) = &self.file_writer {
            let mut file = file_writer.write().unwrap();
            use std::io::Write;
            file.write_all(output.as_bytes())
                .map_err(|e| ObservabilityError::LoggingError(format!("Failed to write to log file: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Flush log buffer
    fn flush_buffer(&self) -> Result<()> {
        let mut buffer = self.log_buffer.write().unwrap();
        
        if buffer.is_empty() {
            return Ok(());
        }
        
        // Process logs in buffer
        // In production, this might send logs to ELK, Splunk, etc.
        println!("[Log Export] Flushing {} logs", buffer.len());
        
        buffer.clear();
        
        Ok(())
    }
    
    /// Flush logger (write all pending logs)
    pub fn flush(&self) -> Result<()> {
        self.flush_buffer()
    }
    
    /// Log at trace level
    pub fn trace(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Trace, message, context)
    }
    
    /// Log at debug level
    pub fn debug(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Debug, message, context)
    }
    
    /// Log at info level
    pub fn info(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Info, message, context)
    }
    
    /// Log at warn level
    pub fn warn(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Warn, message, context)
    }
    
    /// Log at error level
    pub fn error(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Error, message, context)
    }
    
    /// Log at fatal level
    pub fn fatal(&self, message: &str, context: Option<LogContext>) -> Result<()> {
        self.log(LogLevel::Fatal, message, context)
    }
    
    /// Log error with error information
    pub fn error_with_error(
        &self,
        message: &str,
        error: &dyn std::error::Error,
        context: Option<LogContext>,
    ) -> Result<()> {
        let mut context = context.unwrap_or_default();
        context.error = Some(LogError::from_std_error(error));
        
        self.log(LogLevel::Error, message, Some(context))
    }
    
    /// Log with distributed context
    pub fn log_with_distributed_context(
        &self,
        level: LogLevel,
        message: &str,
        distributed_context: &DistributedContext,
    ) -> Result<()> {
        let log_context = LogContext::from_distributed_context(distributed_context);
        self.log(level, message, Some(log_context))
    }
}

/// Get current thread ID
fn get_thread_id() -> Option<u64> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            use windows::Win32::System::Threading::GetCurrentThreadId;
            Some(GetCurrentThreadId() as u64)
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // For non-Windows platforms, use thread ID
        Some(std::thread::current().id().as_u64().get())
    }
}

/// Format timestamp for human-readable output
fn format_timestamp(micros: u128) -> String {
    let seconds = (micros / 1_000_000) as i64;
    let micros_remainder = (micros % 1_000_000) as u32;
    
    let datetime = chrono::DateTime::from_timestamp(seconds, micros_remainder * 1000)
        .unwrap_or_else(|| chrono::Utc::now());
    
    datetime.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

/// Get current time in microseconds
fn current_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros()
}

/// Log macros for convenience
#[macro_export]
macro_rules! log_trace {
    ($logger:expr, $msg:expr) => {
        $logger.trace($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.trace($msg, Some($ctx))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($logger:expr, $msg:expr) => {
        $logger.debug($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.debug($msg, Some($ctx))
    };
}

#[macro_export]
macro_rules! log_info {
    ($logger:expr, $msg:expr) => {
        $logger.info($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.info($msg, Some($ctx))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($logger:expr, $msg:expr) => {
        $logger.warn($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.warn($msg, Some($ctx))
    };
}

#[macro_export]
macro_rules! log_error {
    ($logger:expr, $msg:expr) => {
        $logger.error($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.error($msg, Some($ctx))
    };
    ($logger:expr, $msg:expr, $err:expr) => {
        $logger.error_with_error($msg, $err, None)
    };
    ($logger:expr, $msg:expr, $err:expr, $ctx:expr) => {
        $logger.error_with_error($msg, $err, Some($ctx))
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($logger:expr, $msg:expr) => {
        $logger.fatal($msg, None)
    };
    ($logger:expr, $msg:expr, $ctx:expr) => {
        $logger.fatal($msg, Some($ctx))
    };
}