//! Stream processing module for Widya Enterprise Edition
//! Provides real-time stream processing with windowing and state management

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::dataplatform::{DataPlatformError, Result, StreamConfig};

/// Stream processor for real-time data processing
pub struct StreamProcessor {
    /// Configuration
    config: StreamConfig,
    
    /// Event buffer
    event_buffer: Arc<RwLock<VecDeque<StreamEvent>>>,
    
    /// Window manager
    window_manager: WindowManager,
    
    /// State store
    state_store: StateStore,
    
    /// Processing metrics
    metrics: Arc<RwLock<StreamMetrics>>,
}

impl StreamProcessor {
    /// Create new stream processor
    pub fn new(config: &StreamConfig) -> Result<Self> {
        Ok(Self {
            config: config.clone(),
            event_buffer: Arc::new(RwLock::new(VecDeque::new())),
            window_manager: WindowManager::new(config.window_slide_ms),
            state_store: StateStore::new(config.state_retention_days),
            metrics: Arc::new(RwLock::new(StreamMetrics::default())),
        })
    }
    
    /// Push event into stream
    pub fn push(&self, event: StreamEvent) -> Result<()> {
        let mut buffer = self.event_buffer.write().unwrap();
        buffer.push_back(event);
        
        if buffer.len() >= self.config.batch_size {
            self.process_batch()?;
        }
        
        Ok(())
    }
    
    /// Push multiple events
    pub fn push_batch(&self, events: Vec<StreamEvent>) -> Result<()> {
        let mut buffer = self.event_buffer.write().unwrap();
        buffer.extend(events);
        
        if buffer.len() >= self.config.batch_size {
            self.process_batch()?;
        }
        
        Ok(())
    }
    
    /// Process a batch of events
    fn process_batch(&self) -> Result<()> {
        let mut buffer = self.event_buffer.write().unwrap();
        let events: Vec<StreamEvent> = buffer.drain(..).collect();
        
        for event in events {
            self.process_event(event)?;
        }
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.batches_processed += 1;
        
        Ok(())
    }
    
    /// Process single event
    fn process_event(&self, event: StreamEvent) -> Result<()> {
        self.window_manager.add_event(event.clone())?;
        
        let mut metrics = self.metrics.write().unwrap();
        metrics.events_processed += 1;
        
        Ok(())
    }
    
    /// Get events in window
    pub fn get_window(&self, window_type: WindowType) -> Result<Vec<StreamEvent>> {
        self.window_manager.get_events(window_type)
    }
    
    /// Get state for key
    pub fn get_state(&self, key: &str) -> Option<StreamState> {
        self.state_store.get(key)
    }
    
    /// Set state for key
    pub fn set_state(&self, key: String, state: StreamState) -> Result<()> {
        self.state_store.set(key, state);
        Ok(())
    }
    
    /// Get metrics
    pub fn metrics(&self) -> StreamMetrics {
        self.metrics.read().unwrap().clone()
    }
    
    /// Flush all pending events
    pub fn flush(&self) -> Result<()> {
        if !self.event_buffer.read().unwrap().is_empty() {
            self.process_batch()?;
        }
        Ok(())
    }
}

/// Stream event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Event ID
    pub id: String,
    
    /// Event timestamp (event time)
    pub timestamp: u64,
    
    /// Processing timestamp (processing time)
    pub processing_time: u64,
    
    /// Event key
    pub key: Option<String>,
    
    /// Event payload
    pub payload: HashMap<String, String>,
    
    /// Event headers
    pub headers: HashMap<String, String>,
}

impl StreamEvent {
    /// Create new event
    pub fn new(payload: HashMap<String, String>) -> Self {
        let now = current_time();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: now,
            processing_time: now,
            key: None,
            payload,
            headers: HashMap::new(),
        }
    }
    
    /// Set key
    pub fn with_key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }
    
    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
    
    /// Add header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// Window types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WindowType {
    /// Tumbling window (non-overlapping)
    Tumbling { size_ms: u64 },
    
    /// Sliding window (overlapping)
    Sliding { size_ms: u64, slide_ms: u64 },
    
    /// Session window (gap-based)
    Session { gap_ms: u64 },
    
    /// Global window (all events)
    Global,
}

impl WindowType {
    /// Create tumbling window
    pub fn tumbling(size_ms: u64) -> Self {
        WindowType::Tumbling { size_ms }
    }
    
    /// Create sliding window
    pub fn sliding(size_ms: u64, slide_ms: u64) -> Self {
        WindowType::Sliding { size_ms, slide_ms }
    }
    
    /// Create session window
    pub fn session(gap_ms: u64) -> Self {
        WindowType::Session { gap_ms }
    }
}

/// Window manager
struct WindowManager {
    /// Window slide interval
    slide_ms: u64,
    
    /// Active windows
    windows: Arc<RwLock<Vec<Window>>>,
}

impl WindowManager {
    /// Create new window manager
    fn new(slide_ms: u64) -> Self {
        Self {
            slide_ms,
            windows: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Add event to windows
    fn add_event(&self, event: StreamEvent) -> Result<()> {
        let mut windows = self.windows.write().unwrap();
        
        for window in windows.iter_mut() {
            if window.contains(event.timestamp) {
                window.events.push(event.clone());
            }
        }
        
        Ok(())
    }
    
    /// Get events from window
    fn get_events(&self, window_type: WindowType) -> Result<Vec<StreamEvent>> {
        let windows = self.windows.read().unwrap();
        
        for window in windows.iter() {
            if window.window_type == window_type {
                return Ok(window.events.clone());
            }
        }
        
        Ok(Vec::new())
    }
}

/// Window
#[derive(Debug, Clone)]
struct Window {
    /// Window type
    window_type: WindowType,
    
    /// Start timestamp
    start: u64,
    
    /// End timestamp
    end: u64,
    
    /// Events in window
    events: Vec<StreamEvent>,
}

impl Window {
    /// Check if timestamp is in window
    fn contains(&self, timestamp: u64) -> bool {
        timestamp >= self.start && timestamp < self.end
    }
}

/// State store for stateful processing
struct StateStore {
    /// Retention in days
    retention_days: u32,
    
    /// State entries
    entries: Arc<RwLock<HashMap<String, StateEntry>>>,
}

impl StateStore {
    /// Create new state store
    fn new(retention_days: u32) -> Self {
        Self {
            retention_days,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Get state
    fn get(&self, key: &str) -> Option<StreamState> {
        let entries = self.entries.read().unwrap();
        entries.get(key).map(|e| e.state.clone())
    }
    
    /// Set state
    fn set(&self, key: String, state: StreamState) {
        let mut entries = self.entries.write().unwrap();
        entries.insert(key, StateEntry {
            state,
            created_at: current_time(),
            updated_at: current_time(),
        });
    }
}

/// State entry
struct StateEntry {
    /// State value
    state: StreamState,
    
    /// Creation timestamp
    created_at: u64,
    
    /// Last update timestamp
    updated_at: u64,
}

/// Stream state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamState {
    /// Value state
    Value(String),
    
    /// List state
    List(Vec<String>),
    
    /// Map state
    Map(HashMap<String, String>),
    
    /// Aggregation state
    Aggregate {
        count: u64,
        sum: f64,
        min: f64,
        max: f64,
        avg: f64,
    },
}

impl StreamState {
    /// Create value state
    pub fn value(v: String) -> Self {
        StreamState::Value(v)
    }
    
    /// Create aggregate state
    pub fn aggregate() -> Self {
        StreamState::Aggregate {
            count: 0,
            sum: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            avg: 0.0,
        }
    }
    
    /// Update aggregate with value
    pub fn update_aggregate(&mut self, value: f64) {
        if let StreamState::Aggregate { count, sum, min, max, avg } = self {
            *count += 1;
            *sum += value;
            *min = (*min).min(value);
            *max = (*max).max(value);
            *avg = *sum / *count as f64;
        }
    }
}

/// Stream metrics
#[derive(Debug, Clone, Default)]
pub struct StreamMetrics {
    /// Total events processed
    pub events_processed: u64,
    
    /// Total batches processed
    pub batches_processed: u64,
    
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    
    /// Throughput (events per second)
    pub throughput: f64,
    
    /// Errors count
    pub errors: u64,
}

/// Watermark for event time processing
#[derive(Debug, Clone)]
pub struct Watermark {
    /// Watermark timestamp
    pub timestamp: u64,
    
    /// Stream identifier
    pub stream_id: String,
}

impl Watermark {
    /// Create new watermark
    pub fn new(stream_id: String, timestamp: u64) -> Self {
        Self { timestamp, stream_id }
    }
}

/// Get current Unix timestamp in milliseconds
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
    fn test_stream_processor_creation() {
        let config = StreamConfig {
            batch_size: 100,
            batch_interval_ms: 1000,
            window_slide_ms: 100,
            state_retention_days: 7,
        };
        
        let processor = StreamProcessor::new(&config).unwrap();
        let metrics = processor.metrics();
        assert_eq!(metrics.events_processed, 0);
    }
    
    #[test]
    fn test_stream_event() {
        let mut payload = HashMap::new();
        payload.insert("key".to_string(), "value".to_string());
        
        let event = StreamEvent::new(payload)
            .with_key("partition-1".to_string());
        
        assert!(event.key.is_some());
    }
    
    #[test]
    fn test_window_types() {
        let tumbling = WindowType::tumbling(60000);
        assert!(matches!(tumbling, WindowType::Tumbling { size_ms: 60000 }));
        
        let sliding = WindowType::sliding(60000, 10000);
        assert!(matches!(sliding, WindowType::Sliding { .. }));
    }
    
    #[test]
    fn test_stream_state() {
        let mut state = StreamState::aggregate();
        state.update_aggregate(10.0);
        state.update_aggregate(20.0);
        
        if let StreamState::Aggregate { count, avg, .. } = state {
            assert_eq!(count, 2);
            assert_eq!(avg, 15.0);
        }
    }
}
