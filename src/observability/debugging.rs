//! Trace-based debugging tools for Widya observability
//! Provides tools for analyzing traces, identifying bottlenecks, and debugging distributed systems

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use crate::observability::tracing::{Trace, Span, SpanStatus, TraceStatistics};

/// Trace analyzer for performance analysis
pub struct TraceAnalyzer {
    /// Traces to analyze
    traces: Vec<Trace>,
    
    /// Service name filter (optional)
    service_filter: Option<String>,
    
    /// Time range filter (optional)
    time_range: Option<(u128, u128)>, // (start_time, end_time) in microseconds
}

impl TraceAnalyzer {
    /// Create new trace analyzer
    pub fn new(traces: Vec<Trace>) -> Self {
        Self {
            traces,
            service_filter: None,
            time_range: None,
        }
    }
    
    /// Filter traces by service name
    pub fn with_service_filter(mut self, service_name: &str) -> Self {
        self.service_filter = Some(service_name.to_string());
        self
    }
    
    /// Filter traces by time range
    pub fn with_time_range(mut self, start_time: u128, end_time: u128) -> Self {
        self.time_range = Some((start_time, end_time));
        self
    }
    
    /// Analyze traces and generate report
    pub fn analyze(&self) -> TraceAnalysisReport {
        let filtered_traces = self.filter_traces();
        
        if filtered_traces.is_empty() {
            return TraceAnalysisReport::empty();
        }
        
        let mut report = TraceAnalysisReport::new();
        
        // Calculate basic statistics
        report.total_traces = filtered_traces.len();
        report.total_spans = filtered_traces.iter()
            .map(|t| t.spans.len())
            .sum();
        
        // Calculate duration statistics
        let durations: Vec<u128> = filtered_traces.iter()
            .filter_map(|t| t.duration)
            .collect();
        
        if !durations.is_empty() {
            report.avg_trace_duration_micros = durations.iter().sum::<u128>() / durations.len() as u128;
            report.min_trace_duration_micros = *durations.iter().min().unwrap();
            report.max_trace_duration_micros = *durations.iter().max().unwrap();
            
            // Calculate percentile durations
            let mut sorted_durations = durations.clone();
            sorted_durations.sort_unstable();
            
            report.p50_trace_duration_micros = percentile(&sorted_durations, 0.5);
            report.p90_trace_duration_micros = percentile(&sorted_durations, 0.9);
            report.p95_trace_duration_micros = percentile(&sorted_durations, 0.95);
            report.p99_trace_duration_micros = percentile(&sorted_durations, 0.99);
        }
        
        // Calculate error rate
        let error_traces = filtered_traces.iter()
            .filter(|t| {
                t.spans.iter().any(|s| s.status == SpanStatus::Error)
            })
            .count();
        
        report.error_rate = if report.total_traces > 0 {
            error_traces as f64 / report.total_traces as f64
        } else {
            0.0
        };
        
        // Analyze spans
        self.analyze_spans(&filtered_traces, &mut report);
        
        // Identify bottlenecks
        self.identify_bottlenecks(&filtered_traces, &mut report);
        
        // Find slowest traces
        self.find_slowest_traces(&filtered_traces, &mut report);
        
        // Find traces with errors
        self.find_error_traces(&filtered_traces, &mut report);
        
        report
    }
    
    /// Filter traces based on current filters
    fn filter_traces(&self) -> Vec<&Trace> {
        self.traces.iter()
            .filter(|trace| {
                // Apply service filter
                if let Some(service_name) = &self.service_filter {
                    if trace.service_name != *service_name {
                        return false;
                    }
                }
                
                // Apply time range filter
                if let Some((start_time, end_time)) = self.time_range {
                    if trace.start_time < start_time || trace.start_time > end_time {
                        return false;
                    }
                }
                
                true
            })
            .collect()
    }
    
    /// Analyze spans within traces
    fn analyze_spans(&self, traces: &[&Trace], report: &mut TraceAnalysisReport) {
        let mut span_counts = HashMap::new();
        let mut span_durations = HashMap::new();
        let mut span_errors = HashMap::new();
        
        for trace in traces {
            for span in &trace.spans {
                // Count spans by name
                *span_counts.entry(span.name.clone()).or_insert(0) += 1;
                
                // Track durations by span name
                if let Some(duration) = span.duration {
                    span_durations.entry(span.name.clone())
                        .or_insert_with(Vec::new)
                        .push(duration);
                }
                
                // Track errors by span name
                if span.status == SpanStatus::Error {
                    *span_errors.entry(span.name.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Find most frequent spans
        let mut span_count_vec: Vec<(String, usize)> = span_counts.into_iter().collect();
        span_count_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.most_frequent_spans = span_count_vec.into_iter()
            .take(10)
            .collect();
        
        // Find slowest spans (by average duration)
        let mut avg_durations: Vec<(String, u128)> = span_durations.into_iter()
            .map(|(name, durations)| {
                let avg = durations.iter().sum::<u128>() / durations.len() as u128;
                (name, avg)
            })
            .collect();
        
        avg_durations.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.slowest_spans = avg_durations.into_iter()
            .take(10)
            .collect();
        
        // Find error-prone spans
        let mut error_rates: Vec<(String, f64)> = span_errors.into_iter()
            .map(|(name, error_count)| {
                let total_count = report.most_frequent_spans.iter()
                    .find(|(n, _)| n == &name)
                    .map(|(_, count)| *count)
                    .unwrap_or(1);
                
                let error_rate = error_count as f64 / total_count as f64;
                (name, error_rate)
            })
            .collect();
        
        error_rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        report.most_error_prone_spans = error_rates.into_iter()
            .take(10)
            .collect();
    }
    
    /// Identify bottlenecks in traces
    fn identify_bottlenecks(&self, traces: &[&Trace], report: &mut TraceAnalysisReport) {
        // For each trace, find the critical path (longest sequence of spans)
        let mut critical_paths = Vec::new();
        
        for trace in traces {
            if let Some(critical_path) = self.find_critical_path(trace) {
                critical_paths.push(critical_path);
            }
        }
        
        // Count bottlenecks (spans that appear frequently in critical paths)
        let mut bottleneck_counts = HashMap::new();
        
        for path in &critical_paths {
            for span_name in path {
                *bottleneck_counts.entry(span_name.clone()).or_insert(0) += 1;
            }
        }
        
        let mut bottlenecks: Vec<(String, usize)> = bottleneck_counts.into_iter().collect();
        bottlenecks.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.bottlenecks = bottlenecks.into_iter()
            .take(10)
            .collect();
    }
    
    /// Find critical path in a trace (simplified algorithm)
    fn find_critical_path(&self, trace: &Trace) -> Option<Vec<String>> {
        if trace.spans.is_empty() {
            return None;
        }
        
        // Find root span (no parent)
        let root_spans: Vec<&Span> = trace.spans.iter()
            .filter(|s| s.parent_id.is_none())
            .collect();
        
        if root_spans.is_empty() {
            return None;
        }
        
        // For each root span, find the longest path
        let mut longest_path = Vec::new();
        
        for root_span in root_spans {
            let path = self.find_longest_path_from_span(root_span, &trace.spans);
            if path.len() > longest_path.len() {
                longest_path = path;
            }
        }
        
        Some(longest_path)
    }
    
    /// Find longest path starting from a span
    fn find_longest_path_from_span(&self, start_span: &Span, all_spans: &[Span]) -> Vec<String> {
        let mut path = vec![start_span.name.clone()];
        
        // Find child spans
        let child_spans: Vec<&Span> = all_spans.iter()
            .filter(|s| s.parent_id == Some(start_span.id))
            .collect();
        
        if child_spans.is_empty() {
            return path;
        }
        
        // For each child, find the longest path
        let mut longest_child_path = Vec::new();
        
        for child_span in child_spans {
            let child_path = self.find_longest_path_from_span(child_span, all_spans);
            if child_path.len() > longest_child_path.len() {
                longest_child_path = child_path;
            }
        }
        
        path.extend(longest_child_path);
        path
    }
    
    /// Find slowest traces
    fn find_slowest_traces(&self, traces: &[&Trace], report: &mut TraceAnalysisReport) {
        let mut trace_durations: Vec<(&Trace, u128)> = traces.iter()
            .filter_map(|&trace| trace.duration.map(|d| (trace, d)))
            .collect();
        
        trace_durations.sort_by(|a, b| b.1.cmp(&a.1));
        
        report.slowest_traces = trace_durations.into_iter()
            .take(5)
            .map(|(trace, duration)| SlowTrace {
                trace_id: trace.id,
                duration_micros: duration,
                span_count: trace.spans.len(),
                service_name: trace.service_name.clone(),
            })
            .collect();
    }
    
    /// Find traces with errors
    fn find_error_traces(&self, traces: &[&Trace], report: &mut TraceAnalysisReport) {
        let error_traces: Vec<&Trace> = traces.iter()
            .filter(|&&trace| {
                trace.spans.iter().any(|s| s.status == SpanStatus::Error)
            })
            .copied()
            .collect();
        
        report.error_traces = error_traces.into_iter()
            .map(|trace| ErrorTrace {
                trace_id: trace.id,
                error_count: trace.spans.iter()
                    .filter(|s| s.status == SpanStatus::Error)
                    .count(),
                total_spans: trace.spans.len(),
                service_name: trace.service_name.clone(),
            })
            .collect();
    }
}

/// Trace analysis report
pub struct TraceAnalysisReport {
    /// Total number of traces analyzed
    pub total_traces: usize,
    
    /// Total number of spans analyzed
    pub total_spans: usize,
    
    /// Average trace duration in microseconds
    pub avg_trace_duration_micros: u128,
    
    /// Minimum trace duration in microseconds
    pub min_trace_duration_micros: u128,
    
    /// Maximum trace duration in microseconds
    pub max_trace_duration_micros: u128,
    
    /// 50th percentile (median) trace duration
    pub p50_trace_duration_micros: u128,
    
    /// 90th percentile trace duration
    pub p90_trace_duration_micros: u128,
    
    /// 95th percentile trace duration
    pub p95_trace_duration_micros: u128,
    
    /// 99th percentile trace duration
    pub p99_trace_duration_micros: u128,
    
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    
    /// Most frequent spans (name, count)
    pub most_frequent_spans: Vec<(String, usize)>,
    
    /// Slowest spans by average duration (name, avg_duration_micros)
    pub slowest_spans: Vec<(String, u128)>,
    
    /// Most error-prone spans (name, error_rate)
    pub most_error_prone_spans: Vec<(String, f64)>,
    
    /// Identified bottlenecks (span name, frequency in critical paths)
    pub bottlenecks: Vec<(String, usize)>,
    
    /// Slowest traces
    pub slowest_traces: Vec<SlowTrace>,
    
    /// Traces with errors
    pub error_traces: Vec<ErrorTrace>,
}

impl TraceAnalysisReport {
    /// Create empty report
    pub fn empty() -> Self {
        Self {
            total_traces: 0,
            total_spans: 0,
            avg_trace_duration_micros: 0,
            min_trace_duration_micros: 0,
            max_trace_duration_micros: 0,
            p50_trace_duration_micros: 0,
            p90_trace_duration_micros: 0,
            p95_trace_duration_micros: 0,
            p99_trace_duration_micros: 0,
            error_rate: 0.0,
            most_frequent_spans: Vec::new(),
            slowest_spans: Vec::new(),
            most_error_prone_spans: Vec::new(),
            bottlenecks: Vec::new(),
            slowest_traces: Vec::new(),
            error_traces: Vec::new(),
        }
    }
    
    /// Create new report
    pub fn new() -> Self {
        Self::empty()
    }
    
    /// Format report as string
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("📊 Trace Analysis Report\n"));
        output.push_str(&format!("=======================\n\n"));
        
        output.push_str(&format!("📈 Summary Statistics:\n"));
        output.push_str(&format!("  Total Traces: {}\n", self.total_traces));
        output.push_str(&format!("  Total Spans: {}\n", self.total_spans));
        output.push_str(&format!("  Error Rate: {:.2}%\n", self.error_rate * 100.0));
        
        output.push_str(&format!("\n⏱️  Duration Statistics:\n"));
        output.push_str(&format!("  Average: {} ms\n", self.avg_trace_duration_micros / 1000));
        output.push_str(&format!("  Minimum: {} ms\n", self.min_trace_duration_micros / 1000));
        output.push_str(&format!("  Maximum: {} ms\n", self.max_trace_duration_micros / 1000));
        output.push_str(&format!("  P50 (Median): {} ms\n", self.p50_trace_duration_micros / 1000));
        output.push_str(&format!("  P90: {} ms\n", self.p90_trace_duration_micros / 1000));
        output.push_str(&format!("  P95: {} ms\n", self.p95_trace_duration_micros / 1000));
        output.push_str(&format!("  P99: {} ms\n", self.p99_trace_duration_micros / 1000));
        
        if !self.most_frequent_spans.is_empty() {
            output.push_str(&format!("\n🔢 Most Frequent Spans:\n"));
            for (i, (name, count)) in self.most_frequent_spans.iter().enumerate().take(5) {
                output.push_str(&format!("  {}. {} ({} occurrences)\n", i + 1, name, count));
            }
        }
        
        if !self.slowest_spans.is_empty() {
            output.push_str(&format!("\n🐌 Slowest Spans (by average duration):\n"));
            for (i, (name, duration)) in self.slowest_spans.iter().enumerate().take(5) {
                output.push_str(&format!("  {}. {} ({} ms)\n", i + 1, name, duration / 1000));
            }
        }
        
        if !self.most_error_prone_spans.is_empty() {
            output.push_str(&format!("\n❌ Most Error-Prone Spans:\n"));
            for (i, (name, error_rate)) in self.most_error_prone_spans.iter().enumerate().take(5) {
                output.push_str(&format!("  {}. {} ({:.2}% error rate)\n", i + 1, name, error_rate * 100.0));
            }
        }
        
        if !self.bottlenecks.is_empty() {
            output.push_str(&format!("\n⚠️  Identified Bottlenecks:\n"));
            for (i, (name, frequency)) in self.bottlenecks.iter().enumerate().take(5) {
                output.push_str(&format!("  {}. {} (appears in {} critical paths)\n", i + 1, name, frequency));
            }
        }
        
        if !self.slowest_traces.is_empty() {
            output.push_str(&format!("\n🐌 Slowest Traces:\n"));
            for (i, trace) in self.slowest_traces.iter().enumerate() {
                output.push_str(&format!("  {}. Trace {} ({} ms, {} spans, {})\n", 
                    i + 1, 
                    trace.trace_id, 
                    trace.duration_micros / 1000,
                    trace.span_count,
                    trace.service_name));
            }
        }
        
        if !self.error_traces.is_empty() {
            output.push_str(&format!("\n❌ Traces with Errors:\n"));
            for (i, trace) in self.error_traces.iter().enumerate().take(5) {
                output.push_str(&format!("  {}. Trace {} ({} errors, {} total spans, {})\n", 
                    i + 1, 
                    trace.trace_id, 
                    trace.error_count,
                    trace.total_spans,
                    trace.service_name));
            }
        }
        
        output
    }
}

/// Slow trace information
#[derive(Debug, Clone)]
pub struct SlowTrace {
    pub trace_id: u128,
    pub duration_micros: u128,
    pub span_count: usize,
    pub service_name: String,
}

/// Error trace information
#[derive(Debug, Clone)]
pub struct ErrorTrace {
    pub trace_id: u128,
    pub error_count: usize,
    pub total_spans: usize,
    pub service_name: String,
}

/// Calculate percentile of sorted values
fn percentile(sorted_values: &[u128], percentile: f64) -> u128 {
    if sorted_values.is_empty() {
        return 0;
    }
    
    let index = (percentile * (sorted_values.len() - 1) as f64).round() as usize;
    sorted_values[index]
}

/// Trace visualizer for generating visual representations
pub struct TraceVisualizer;

impl TraceVisualizer {
    /// Generate ASCII timeline for a trace
    pub fn generate_ascii_timeline(trace: &Trace) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("📈 Trace {} Timeline\n", trace.id));
        output.push_str(&format!("Service: {}\n", trace.service_name));
        
        if let Some(duration) = trace.duration {
            output.push_str(&format!("Duration: {} ms\n\n", duration / 1000));
        }
        
        // Sort spans by start time
        let mut sorted_spans = trace.spans.clone();
        sorted_spans.sort_by_key(|s| s.start_time);
        
        // Find time range
        let min_time = sorted_spans.first().map(|s| s.start_time).unwrap_or(0);
        let max_time = sorted_spans.iter()
            .filter_map(|s| s.end_time)
            .max()
            .unwrap_or(min_time);
        
        let total_duration = max_time.saturating_sub(min_time);
        if total_duration == 0 {
            return output;
        }
        
        // Create timeline with 80 characters width
        const TIMELINE_WIDTH: u128 = 80;
        
        for span in &sorted_spans {
            let relative_start = span.start_time.saturating_sub(min_time);
            let start_pos = (relative_start * TIMELINE_WIDTH / total_duration).min(TIMELINE_WIDTH - 1);
            
            let duration = span.duration.unwrap_or(0);
            let width = (duration * TIMELINE_WIDTH / total_duration).max(1);
            
            let status_char = match span.status {
                SpanStatus::Ok => '✓',
                SpanStatus::Error => '✗',
                SpanStatus::Unset => '•',
            };
            
            let name_display = if span.name.len() > 20 {
                format!("{}...", &span.name[..17])
            } else {
                span.name.clone()
            };
            
            let timeline_bar = "─".repeat(width as usize);
            let padding = " ".repeat(start_pos as usize);
            
            output.push_str(&format!("{}{}{} {} ({:.1}ms)\n", 
                padding, timeline_bar, status_char, name_display, 
                duration as f64 / 1000.0));
        }
        
        output
    }
    
    /// Generate dependency graph for a trace
    pub fn generate_dependency_graph(trace: &Trace) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("🕸️  Trace {} Dependency Graph\n", trace.id));
        output.push_str(&format!("Service: {}\n\n", trace.service_name));
        
        // Build parent-child relationships
        let mut children: HashMap<Option<u64>, Vec<&Span>> = HashMap::new();
        
        for span in &trace.spans {
            children.entry(span.parent_id).or_default().push(span);
        }
        
        // Print tree starting from root (None parent)
        Self::print_span_tree(&children, None, 0, &mut output);
        
        output
    }
    
    /// Print span tree recursively
    fn print_span_tree(
        children: &HashMap<Option<u64>, Vec<&Span>>,
        parent_id: Option<u64>,
        depth: usize,
        output: &mut String,
    ) {
        if let Some(child_spans) = children.get(&parent_id) {
            for (i, span) in child_spans.iter().enumerate() {
                let prefix = if depth == 0 {
                    String::new()
                } else {
                    "  ".repeat(depth - 1) + if i == child_spans.len() - 1 {
                        "└─ "
                    } else {
                        "├─ "
                    }
                };
                
                let status_symbol = match span.status {
                    SpanStatus::Ok => "✓",
                    SpanStatus::Error => "✗",
                    SpanStatus::Unset => "•",
                };
                
                let duration_str = if let Some(duration) = span.duration {
                    format!(" ({:.1}ms)", duration as f64 / 1000.0)
                } else {
                    String::new()
                };
                
                output.push_str(&format!("{}{} {}{}\n", 
                    prefix, status_symbol, span.name, duration_str));
                
                // Recursively print children
                Self::print_span_tree(children, Some(span.id), depth + 1, output);
            }
        }
    }
}

/// Trace debugger for interactive debugging
pub struct TraceDebugger {
    /// Traces available for debugging
    traces: Vec<Trace>,
    
    /// Current trace being debugged
    current_trace_index: Option<usize>,
}

impl TraceDebugger {
    /// Create new trace debugger
    pub fn new(traces: Vec<Trace>) -> Self {
        Self {
            traces,
            current_trace_index: None,
        }
    }
    
    /// List available traces
    pub fn list_traces(&self) -> String {
        let mut output = String::new();
        
        output.push_str("Available Traces:\n");
        output.push_str("================\n");
        
        for (i, trace) in self.traces.iter().enumerate() {
            let duration_str = if let Some(duration) = trace.duration {
                format!("{} ms", duration / 1000)
            } else {
                "N/A".to_string()
            };
            
            let error_count = trace.spans.iter()
                .filter(|s| s.status == SpanStatus::Error)
                .count();
            
            let status = if error_count > 0 {
                format!("❌ ({} errors)", error_count)
            } else {
                "✅".to_string()
            };
            
            output.push_str(&format!("{}. Trace {} [{}] - {} spans, {}, {}\n", 
                i + 1, 
                trace.id, 
                status,
                trace.spans.len(),
                duration_str,
                trace.service_name));
        }
        
        output
    }
    
    /// Select trace for debugging
    pub fn select_trace(&mut self, index: usize) -> Result<(), String> {
        if index >= self.traces.len() {
            return Err(format!("Invalid trace index: {}. Available: {}", index, self.traces.len()));
        }
        
        self.current_trace_index = Some(index);
        Ok(())
    }
    
    /// Show current trace details
    pub fn show_trace_details(&self) -> Result<String, String> {
        let trace = self.current_trace()?;
        
        let mut output = String::new();
        
        output.push_str(&format!("📋 Trace {} Details\n", trace.id));
        output.push_str(&format!("==================\n\n"));
        
        output.push_str(&format!("Service: {}\n", trace.service_name));
        
        if let Some(duration) = trace.duration {
            output.push_str(&format!("Duration: {} ms\n", duration / 1000));
        }
        
        if let Some(start_time) = trace.spans.first().map(|s| s.start_time) {
            output.push_str(&format!("Start Time: {} μs\n", start_time));
        }
        
        output.push_str(&format!("Total Spans: {}\n", trace.spans.len()));
        
        let error_count = trace.spans.iter()
            .filter(|s| s.status == SpanStatus::Error)
            .count();
        
        output.push_str(&format!("Errors: {}\n\n", error_count));
        
        // Show spans
        output.push_str("Spans:\n");
        output.push_str("------\n");
        
        for (i, span) in trace.spans.iter().enumerate() {
            let status_symbol = match span.status {
                SpanStatus::Ok => "✓",
                SpanStatus::Error => "✗",
                SpanStatus::Unset => "•",
            };
            
            let duration_str = if let Some(duration) = span.duration {
                format!("{} ms", duration / 1000)
            } else {
                "N/A".to_string()
            };
            
            output.push_str(&format!("{}. {} {} ({})\n", 
                i + 1, status_symbol, span.name, duration_str));
            
            if !span.attributes.is_empty() {
                output.push_str(&format!("   Attributes: {:?}\n", span.attributes));
            }
            
            if !span.events.is_empty() {
                output.push_str(&format!("   Events: {}\n", span.events.len()));
            }
        }
        
        Ok(output)
    }
    
    /// Show span details
    pub fn show_span_details(&self, span_index: usize) -> Result<String, String> {
        let trace = self.current_trace()?;
        
        if span_index >= trace.spans.len() {
            return Err(format!("Invalid span index: {}. Available: {}", span_index, trace.spans.len()));
        }
        
        let span = &trace.spans[span_index];
        let mut output = String::new();
        
        output.push_str(&format!("🔍 Span {} Details\n", span.id));
        output.push_str(&format!("=================\n\n"));
        
        output.push_str(&format!("Name: {}\n", span.name));
        output.push_str(&format!("Trace ID: {}\n", span.trace_id));
        output.push_str(&format!("Parent ID: {}\n", span.parent_id.map(|id| id.to_string()).unwrap_or("None".to_string())));
        
        output.push_str(&format!("Status: {:?}\n", span.status));
        output.push_str(&format!("Kind: {:?}\n", span.kind));
        
        output.push_str(&format!("Start Time: {} μs\n", span.start_time));
        if let Some(end_time) = span.end_time {
            output.push_str(&format!("End Time: {} μs\n", end_time));
        }
        
        if let Some(duration) = span.duration {
            output.push_str(&format!("Duration: {} ms\n", duration / 1000));
        }
        
        output.push_str(&format!("Service: {}\n", span.service_name));
        
        if let Some(tenant_id) = span.tenant_id {
            output.push_str(&format!("Tenant ID: {}\n", tenant_id));
        }
        
        output.push_str(&format!("Correlation ID: {}\n", span.correlation_id));
        
        if !span.attributes.is_empty() {
            output.push_str(&format!("\nAttributes:\n"));
            for (key, value) in &span.attributes {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }
        
        if !span.events.is_empty() {
            output.push_str(&format!("\nEvents ({}):\n", span.events.len()));
            for (i, event) in span.events.iter().enumerate() {
                output.push_str(&format!("  {}. {} at {} μs\n", i + 1, event.name, event.timestamp));
                if !event.attributes.is_empty() {
                    output.push_str(&format!("     Attributes: {:?}\n", event.attributes));
                }
            }
        }
        
        Ok(output)
    }
    
    /// Get current trace
    fn current_trace(&self) -> Result<&Trace, String> {
        self.current_trace_index
            .and_then(|index| self.traces.get(index))
            .ok_or_else(|| "No trace selected. Use 'select_trace' first.".to_string())
    }
}