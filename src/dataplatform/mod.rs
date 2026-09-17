//! Data Platform module for Widya Enterprise Edition
//! Provides CDC, Parquet, Delta Lake, stream processing, and analytics

pub mod cdc;
pub mod parquet;
pub mod stream;
pub mod analytics;

// Re-export commonly used types
pub use cdc::{CDCSource, CDCConnector, ChangeRecord, ChangeOperation};
pub use parquet::{ParquetReader, ParquetWriter, ParquetOptions, ParquetSchema, ParquetRow, ParquetValue};
pub use stream::{StreamProcessor, WindowType, StreamState, StreamEvent};
pub use analytics::{AnalyticsEngine, QueryPlan, ExecutionStats, DataRow, DataValue};

/// Data platform errors
#[derive(Debug, thiserror::Error)]
pub enum DataPlatformError {
    #[error("CDC error: {0}")]
    CdcError(String),
    
    #[error("Parquet error: {0}")]
    ParquetError(String),
    
    #[error("Stream processing error: {0}")]
    StreamError(String),
    
    #[error("Analytics error: {0}")]
    AnalyticsError(String),
    
    #[error("Delta Lake error: {0}")]
    DeltaLakeError(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Result type for data platform operations
pub type Result<T> = std::result::Result<T, DataPlatformError>;

/// Data platform configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataPlatformConfig {
    /// CDC configuration
    pub cdc: CdcConfig,
    
    /// Parquet configuration
    pub parquet: ParquetConfig,
    
    /// Delta Lake configuration
    pub delta: DeltaLakeConfig,
    
    /// Stream processing configuration
    pub stream: StreamConfig,
    
    /// Analytics configuration
    pub analytics: AnalyticsConfig,
}

/// CDC configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CdcConfig {
    /// Enable CDC
    pub enabled: bool,
    
    /// Database type (PostgreSQL, MySQL)
    pub database_type: CdcDatabaseType,
    
    /// Connection string
    pub connection_string: String,
    
    /// Start from beginning
    pub start_from_beginning: bool,
    
    /// Checkpoint interval in seconds
    pub checkpoint_interval_secs: u64,
}

/// Stream processing configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StreamConfig {
    /// Batch size for micro-batch processing
    pub batch_size: usize,
    
    /// Batch interval in milliseconds
    pub batch_interval_ms: u64,
    
    /// Window slide interval in milliseconds
    pub window_slide_ms: u64,
    
    /// State retention in days
    pub state_retention_days: u32,
}

/// Parquet configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParquetConfig {
    /// Default compression
    pub compression: CompressionAlgorithm,
    
    /// Row group size in bytes
    pub row_group_size: usize,
    
    /// Data page size in bytes
    pub data_page_size: usize,
    
    /// Dictionary encoding enabled
    pub dictionary_enabled: bool,
    
    /// Statistics enabled
    pub statistics_enabled: bool,
}

/// Delta Lake configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeltaLakeConfig {
    /// Enable Delta Lake
    pub enabled: bool,
    
    /// Log retention in days
    pub log_retention_days: u32,
    
    /// Auto compact enabled
    pub auto_compact: bool,
    
    /// Vacuum retention in hours
    pub vacuum_retention_hours: u32,
}

/// Analytics configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyticsConfig {
    /// In-memory buffer size
    pub buffer_size: usize,
    
    /// Enable vectorized execution
    pub vectorized: bool,
    
    /// Aggregation spill to disk
    pub spill_enabled: bool,
}

/// CDC database types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CdcDatabaseType {
    PostgreSQL,
    MySQL,
    Oracle,
    SQLServer,
}

/// Compression algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionAlgorithm {
    Snappy,
    Gzip,
    Zstd,
    Uncompressed,
}

/// Initialize data platform
pub fn init(config: DataPlatformConfig) -> Result<DataPlatform> {
    let platform = DataPlatform::new(config)?;
    Ok(platform)
}

/// Data platform instance
pub struct DataPlatform {
    /// Configuration
    config: DataPlatformConfig,
    
    /// CDC connector
    cdc: Option<CDCConnector>,
    
    /// Stream processor
    stream_processor: Option<StreamProcessor>,
    
    /// Analytics engine
    analytics: Option<AnalyticsEngine>,
}

impl DataPlatform {
    /// Create new data platform
    pub fn new(config: DataPlatformConfig) -> Result<Self> {
        let cdc = if config.cdc.enabled {
            Some(CDCConnector::new(&config.cdc)?)
        } else {
            None
        };
        
        let stream_processor = if config.stream.batch_size > 0 {
            Some(StreamProcessor::new(&config.stream)?)
        } else {
            None
        };
        
        let analytics = if config.analytics.buffer_size > 0 {
            Some(AnalyticsEngine::new(&config.analytics)?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            cdc,
            stream_processor,
            analytics,
        })
    }
    
    /// Get CDC connector
    pub fn cdc(&self) -> Option<&CDCConnector> {
        self.cdc.as_ref()
    }
    
    /// Get stream processor
    pub fn stream_processor(&self) -> Option<&StreamProcessor> {
        self.stream_processor.as_ref()
    }
    
    /// Get analytics engine
    pub fn analytics(&self) -> Option<&AnalyticsEngine> {
        self.analytics.as_ref()
    }
}

/// Time travel support for data history
#[derive(Debug, Clone)]
pub struct TimeTravelVersion {
    /// Version number
    pub version: u64,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Data snapshot
    pub data_snapshot: String,
}

impl TimeTravelVersion {
    /// Create new version
    pub fn new(version: u64, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            version,
            timestamp,
            data_snapshot: format!("snapshot-v{}", version),
        }
    }
}