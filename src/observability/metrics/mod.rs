use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

use crate::observability::{ObservabilityConfig, ObservabilityError, Result};

/// Metric type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter metric (monotonically increasing)
    Counter,
    
    /// Gauge metric (can go up and down)
    Gauge,
    
    /// Histogram metric (with buckets)
    Histogram,
    
    /// Summary metric (with quantiles)
    Summary,
}

/// Metric value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    /// Counter value
    Counter(u64),
    
    /// Gauge value
    Gauge(f64),
    
    /// Histogram values with counts per bucket
    Histogram {
        sum: f64,
        count: u64,
        buckets: Vec<(f64, u64)>, // (upper_bound, count)
    },
    
    /// Summary values with quantiles
    Summary {
        sum: f64,
        count: u64,
        quantiles: Vec<(f64, f64)>, // (quantile, value)
    },
}

/// Metric with labels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric name
    pub name: String,
    
    /// Metric type
    pub metric_type: MetricType,
    
    /// Metric value
    pub value: MetricValue,
    
    /// Metric labels (key-value pairs)
    pub labels: HashMap<String, String>,
    
    /// Timestamp when metric was recorded
    pub timestamp: u128,
    
    /// Tenant ID (for multi-tenancy)
    pub tenant_id: Option<uuid::Uuid>,
    
    /// Correlation ID (for tracing integration)
    pub correlation_id: Option<u128>,
}

impl Metric {
    /// Create a new counter metric
    pub fn counter(name: String, value: u64, labels: HashMap<String, String>) -> Self {
        Self {
            name,
            metric_type: MetricType::Counter,
            value: MetricValue::Counter(value),
            labels,
            timestamp: current_time_micros(),
            tenant_id: None,
            correlation_id: None,
        }
    }
    
    /// Create a new gauge metric
    pub fn gauge(name: String, value: f64, labels: HashMap<String, String>) -> Self {
        Self {
            name,
            metric_type: MetricType::Gauge,
            value: MetricValue::Gauge(value),
            labels,
            timestamp: current_time_micros(),
            tenant_id: None,
            correlation_id: None,
        }
    }
    
    /// Create a new histogram metric
    pub fn histogram(
        name: String,
        sum: f64,
        count: u64,
        buckets: Vec<(f64, u64)>,
        labels: HashMap<String, String>,
    ) -> Self {
        Self {
            name,
            metric_type: MetricType::Histogram,
            value: MetricValue::Histogram { sum, count, buckets },
            labels,
            timestamp: current_time_micros(),
            tenant_id: None,
            correlation_id: None,
        }
    }
    
    /// Create a new summary metric
    pub fn summary(
        name: String,
        sum: f64,
        count: u64,
        quantiles: Vec<(f64, f64)>,
        labels: HashMap<String, String>,
    ) -> Self {
        Self {
            name,
            metric_type: MetricType::Summary,
            value: MetricValue::Summary { sum, count, quantiles },
            labels,
            timestamp: current_time_micros(),
            tenant_id: None,
            correlation_id: None,
        }
    }
    
    /// Set tenant ID
    pub fn with_tenant_id(mut self, tenant_id: uuid::Uuid) -> Self {
        self.tenant_id = Some(tenant_id);
        self
    }
    
    /// Set correlation ID
    pub fn with_correlation_id(mut self, correlation_id: u128) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }
    
    /// Get metric value as float (if applicable)
    pub fn as_f64(&self) -> Option<f64> {
        match &self.value {
            MetricValue::Counter(v) => Some(*v as f64),
            MetricValue::Gauge(v) => Some(*v),
            MetricValue::Histogram { sum, .. } => Some(*sum),
            MetricValue::Summary { sum, .. } => Some(*sum),
        }
    }
    
    /// Get metric value as integer (if applicable)
    pub fn as_u64(&self) -> Option<u64> {
        match &self.value {
            MetricValue::Counter(v) => Some(*v),
            MetricValue::Gauge(v) => Some(*v as u64),
            MetricValue::Histogram { count, .. } => Some(*count),
            MetricValue::Summary { count, .. } => Some(*count),
        }
    }
}

/// Metrics collector
pub struct MetricsCollector {
    /// Collector configuration
    config: ObservabilityConfig,
    
    /// Collected metrics (ring buffer for retention)
    metrics: Arc<RwLock<VecDeque<Metric>>>,
    
    /// Metric aggregation windows
    aggregations: Arc<RwLock<HashMap<String, MetricAggregation>>>,
    
    /// Prometheus exporter state
    prometheus_exporter: Option<PrometheusExporter>,
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: &ObservabilityConfig) -> Result<Self> {
        let prometheus_exporter = if config.metrics.prometheus_enabled {
            Some(PrometheusExporter::new(config)?)
        } else {
            None
        };
        
        Ok(Self {
            config: config.clone(),
            metrics: Arc::new(RwLock::new(VecDeque::new())),
            aggregations: Arc::new(RwLock::new(HashMap::new())),
            prometheus_exporter,
        })
    }
    
    /// Record a metric
    pub fn record(&self, metric: Metric) -> Result<()> {
        let mut metrics = self.metrics.write().unwrap();
        
        // Add to metrics buffer
        metrics.push_back(metric.clone());
        
        // Apply retention policy
        if metrics.len() > 100_000 { // Keep last 100k metrics
            metrics.pop_front();
        }
        
        // Update aggregations
        self.update_aggregations(&metric)?;
        
        // Export if configured
        if let Some(exporter) = &self.prometheus_exporter {
            exporter.export_metric(&metric)?;
        }
        
        Ok(())
    }
    
    /// Update metric aggregations
    fn update_aggregations(&self, metric: &Metric) -> Result<()> {
        let mut aggregations = self.aggregations.write().unwrap();
        
        let key = format!("{}:{}", metric.name, self.labels_key(&metric.labels));
        
        match metric.metric_type {
            MetricType::Counter => {
                let entry = aggregations.entry(key).or_insert_with(|| 
                    MetricAggregation::new(metric.metric_type)
                );
                
                if let MetricValue::Counter(value) = metric.value {
                    if let Some(agg_value) = entry.value.as_u64() {
                        entry.value = MetricValue::Counter(agg_value + value);
                    } else {
                        entry.value = MetricValue::Counter(value);
                    }
                }
            }
            
            MetricType::Gauge => {
                let entry = aggregations.entry(key).or_insert_with(|| 
                    MetricAggregation::new(metric.metric_type)
                );
                
                if let MetricValue::Gauge(value) = metric.value {
                    entry.value = MetricValue::Gauge(value);
                }
            }
            
            MetricType::Histogram => {
                // Histograms are not aggregated across calls
                // They're aggregated within each metric
            }
            
            MetricType::Summary => {
                // Summaries are not aggregated across calls
                // They're aggregated within each metric
            }
        }
        
        Ok(())
    }
    
    /// Get metrics by name
    pub fn get_metrics_by_name(&self, name: &str) -> Vec<Metric> {
        let metrics = self.metrics.read().unwrap();
        metrics.iter()
            .filter(|m| m.name == name)
            .cloned()
            .collect()
    }
    
    /// Get aggregated metrics
    pub fn get_aggregated_metrics(&self) -> HashMap<String, MetricAggregation> {
        let aggregations = self.aggregations.read().unwrap();
        aggregations.clone()
    }
    
    /// Get metrics for Prometheus export
    pub fn get_prometheus_metrics(&self) -> String {
        let mut output = String::new();
        
        // Get aggregated metrics
        let aggregations = self.aggregations.read().unwrap();
        
        for (key, aggregation) in aggregations.iter() {
            // Parse key back to name and labels
            let parts: Vec<&str> = key.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let name = parts[0];
            let labels_str = parts[1];
            
            // Format for Prometheus
            match aggregation.value {
                MetricValue::Counter(value) => {
                    output.push_str(&format!(
                        "# TYPE {} counter\n",
                        sanitize_prometheus_name(name)
                    ));
                    
                    if labels_str.is_empty() {
                        output.push_str(&format!("{} {}\n", name, value));
                    } else {
                        output.push_str(&format!("{}{{{}}} {}\n", name, labels_str, value));
                    }
                }
                
                MetricValue::Gauge(value) => {
                    output.push_str(&format!(
                        "# TYPE {} gauge\n",
                        sanitize_prometheus_name(name)
                    ));
                    
                    if labels_str.is_empty() {
                        output.push_str(&format!("{} {}\n", name, value));
                    } else {
                        output.push_str(&format!("{}{{{}}} {}\n", name, labels_str, value));
                    }
                }
                
                MetricValue::Histogram { sum, count, ref buckets } => {
                    output.push_str(&format!(
                        "# TYPE {} histogram\n",
                        sanitize_prometheus_name(name)
                    ));
                    
                    // Buckets
                    for (upper_bound, bucket_count) in buckets {
                        let bucket_labels = if labels_str.is_empty() {
                            format!("le=\"{}\"", upper_bound)
                        } else {
                            format!("{},le=\"{}\"", labels_str, upper_bound)
                        };
                        
                        output.push_str(&format!(
                            "{}_bucket{{{}}} {}\n",
                            name, bucket_labels, bucket_count
                        ));
                    }
                    
                    // Sum and count
                    if labels_str.is_empty() {
                        output.push_str(&format!("{}_sum {}\n", name, sum));
                        output.push_str(&format!("{}_count {}\n", name, count));
                    } else {
                        output.push_str(&format!("{}_sum{{{}}} {}\n", name, labels_str, sum));
                        output.push_str(&format!("{}_count{{{}}} {}\n", name, labels_str, count));
                    }
                }
                
                MetricValue::Summary { sum, count, ref quantiles } => {
                    output.push_str(&format!(
                        "# TYPE {} summary\n",
                        sanitize_prometheus_name(name)
                    ));
                    
                    // Quantiles
                    for (quantile, value) in quantiles {
                        let quantile_labels = if labels_str.is_empty() {
                            format!("quantile=\"{}\"", quantile)
                        } else {
                            format!("{},quantile=\"{}\"", labels_str, quantile)
                        };
                        
                        output.push_str(&format!(
                            "{}{{{}}} {}\n",
                            name, quantile_labels, value
                        ));
                    }
                    
                    // Sum and count
                    if labels_str.is_empty() {
                        output.push_str(&format!("{}_sum {}\n", name, sum));
                        output.push_str(&format!("{}_count {}\n", name, count));
                    } else {
                        output.push_str(&format!("{}_sum{{{}}} {}\n", name, labels_str, sum));
                        output.push_str(&format!("{}_count{{{}}} {}\n", name, labels_str, count));
                    }
                }
            }
        }
        
        output
    }
    
    /// Increment a counter
    pub fn increment_counter(&self, name: &str, labels: HashMap<String, String>) -> Result<()> {
        let metric = Metric::counter(name.to_string(), 1, labels);
        self.record(metric)
    }
    
    /// Set a gauge value
    pub fn set_gauge(&self, name: &str, value: f64, labels: HashMap<String, String>) -> Result<()> {
        let metric = Metric::gauge(name.to_string(), value, labels);
        self.record(metric)
    }
    
    /// Observe a histogram value
    pub fn observe_histogram(
        &self,
        name: &str,
        value: f64,
        labels: HashMap<String, String>,
        buckets: Option<Vec<f64>>,
    ) -> Result<()> {
        let default_buckets = vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0];
        let buckets = buckets.unwrap_or(default_buckets);
        
        // Create bucket counts
        let mut bucket_counts = Vec::new();
        for &bucket in &buckets {
            let count = if value <= bucket { 1 } else { 0 };
            bucket_counts.push((bucket, count));
        }
        
        let metric = Metric::histogram(
            name.to_string(),
            value,
            1,
            bucket_counts,
            labels,
        );
        
        self.record(metric)
    }
    
    /// Observe a summary value
    pub fn observe_summary(
        &self,
        name: &str,
        value: f64,
        labels: HashMap<String, String>,
        quantiles: Option<Vec<f64>>,
    ) -> Result<()> {
        let default_quantiles = vec![0.5, 0.9, 0.95, 0.99];
        let quantiles = quantiles.unwrap_or(default_quantiles);
        
        // Create quantile values (simplified - in production, use reservoir sampling)
        let mut quantile_values = Vec::new();
        for &quantile in &quantiles {
            quantile_values.push((quantile, value));
        }
        
        let metric = Metric::summary(
            name.to_string(),
            value,
            1,
            quantile_values,
            labels,
        );
        
        self.record(metric)
    }
    
    /// Flush metrics to exporters
    pub fn flush(&self) -> Result<()> {
        // Export to Prometheus if enabled
        if let Some(exporter) = &self.prometheus_exporter {
            exporter.flush()?;
        }
        
        Ok(())
    }
    
    /// Create labels key for aggregation
    fn labels_key(&self, labels: &HashMap<String, String>) -> String {
        let mut sorted_labels: Vec<(&String, &String)> = labels.iter().collect();
        sorted_labels.sort_by_key(|(k, _)| *k);
        
        sorted_labels.iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<String>>()
            .join(",")
    }
}

/// Metric aggregation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregation {
    /// Metric type
    pub metric_type: MetricType,
    
    /// Aggregated value
    pub value: MetricValue,
    
    /// Last update timestamp
    pub last_update: u128,
    
    /// Number of samples
    pub sample_count: u64,
}

impl MetricAggregation {
    /// Create new aggregation
    pub fn new(metric_type: MetricType) -> Self {
        Self {
            metric_type,
            value: match metric_type {
                MetricType::Counter => MetricValue::Counter(0),
                MetricType::Gauge => MetricValue::Gauge(0.0),
                MetricType::Histogram => MetricValue::Histogram {
                    sum: 0.0,
                    count: 0,
                    buckets: Vec::new(),
                },
                MetricType::Summary => MetricValue::Summary {
                    sum: 0.0,
                    count: 0,
                    quantiles: Vec::new(),
                },
            },
            last_update: current_time_micros(),
            sample_count: 0,
        }
    }
}

/// Prometheus exporter
struct PrometheusExporter {
    /// Export endpoint
    endpoint: String,
    
    /// Metrics buffer for batch export
    metrics_buffer: Arc<RwLock<Vec<Metric>>>,
}

impl PrometheusExporter {
    /// Create new Prometheus exporter
    fn new(config: &ObservabilityConfig) -> Result<Self> {
        let endpoint = config.metrics.prometheus_endpoint
            .clone()
            .unwrap_or_else(|| "/metrics".to_string());
        
        Ok(Self {
            endpoint,
            metrics_buffer: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Export metric to Prometheus
    fn export_metric(&self, metric: &Metric) -> Result<()> {
        let mut buffer = self.metrics_buffer.write().unwrap();
        buffer.push(metric.clone());
        
        // Flush if buffer is large
        if buffer.len() >= 1000 {
            self.flush_buffer()?;
        }
        
        Ok(())
    }
    
    /// Flush metrics buffer
    fn flush(&self) -> Result<()> {
        self.flush_buffer()
    }
    
    /// Actually flush the buffer
    fn flush_buffer(&self) -> Result<()> {
        let mut buffer = self.metrics_buffer.write().unwrap();
        if buffer.is_empty() {
            return Ok(());
        }
        
        // Convert metrics to Prometheus format
        let prometheus_text = self.format_metrics(&buffer);
        
        // In production, this would HTTP POST to Prometheus pushgateway
        // or expose an HTTP endpoint for Prometheus to scrape
        println!("[Prometheus Export] {} metrics via endpoint {}", 
            buffer.len(), self.endpoint);
        
        // Clear buffer
        buffer.clear();
        
        Ok(())
    }
    
    /// Format metrics for Prometheus
    fn format_metrics(&self, metrics: &[Metric]) -> String {
        // Simplified formatting
        // In production, use proper Prometheus text format
        let mut output = String::new();
        
        for metric in metrics {
            output.push_str(&format!("{:?}\n", metric));
        }
        
        output
    }
}

/// Sanitize metric name for Prometheus
fn sanitize_prometheus_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

/// Get current time in microseconds
fn current_time_micros() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros()
}

/// Predefined metric names
pub mod metric_names {
    /// HTTP request metrics
    pub const HTTP_REQUESTS_TOTAL: &str = "http_requests_total";
    pub const HTTP_REQUEST_DURATION_SECONDS: &str = "http_request_duration_seconds";
    pub const HTTP_REQUEST_SIZE_BYTES: &str = "http_request_size_bytes";
    pub const HTTP_RESPONSE_SIZE_BYTES: &str = "http_response_size_bytes";
    
    /// Database metrics
    pub const DB_QUERIES_TOTAL: &str = "db_queries_total";
    pub const DB_QUERY_DURATION_SECONDS: &str = "db_query_duration_seconds";
    pub const DB_CONNECTIONS_TOTAL: &str = "db_connections_total";
    pub const DB_CONNECTIONS_ACTIVE: &str = "db_connections_active";
    
    /// System metrics
    pub const CPU_USAGE_PERCENT: &str = "cpu_usage_percent";
    pub const MEMORY_USAGE_BYTES: &str = "memory_usage_bytes";
    pub const DISK_USAGE_BYTES: &str = "disk_usage_bytes";
    pub const NETWORK_BYTES_TOTAL: &str = "network_bytes_total";
    
    /// Business metrics
    pub const USERS_TOTAL: &str = "users_total";
    pub const ACTIVE_USERS: &str = "active_users";
    pub const TRANSACTIONS_TOTAL: &str = "transactions_total";
    pub const REVENUE_TOTAL: &str = "revenue_total";
    
    /// Tenant metrics (for multi-tenancy)
    pub const TENANT_REQUESTS_TOTAL: &str = "tenant_requests_total";
    pub const TENANT_RESOURCE_USAGE: &str = "tenant_resource_usage";
    pub const TENANT_QUOTA_USAGE_PERCENT: &str = "tenant_quota_usage_percent";
}