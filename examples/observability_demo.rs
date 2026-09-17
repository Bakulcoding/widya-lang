//! Example demonstrating Widya Enterprise Observability features
//! Shows distributed tracing, metrics collection, structured logging, and debugging tools

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use widya::observability::{
    ObservabilityConfig, ObservabilitySystem, init,
    tracing::{SpanKind, SpanStatus},
    metrics::{MetricsCollector, metric_names},
    logging::{LogLevel, LogContext},
    instrumentation::{
        HttpInstrumentation, HttpRequestInfo, HttpResponseInfo,
        DatabaseInstrumentation, DatabaseQueryInfo, DatabaseQueryResult,
    },
    debugging::{TraceAnalyzer, TraceVisualizer, TraceDebugger},
    opentelemetry::{OtelSdk, OtelResource, OtelSeverity},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Widya Enterprise - Advanced Observability Demo");
    println!("=================================================\n");
    
    // Configure observability
    let config = ObservabilityConfig {
        service_name: "example-service".to_string(),
        service_version: "1.0.0".to_string(),
        environment: "production".to_string(),
        tracing: crate::observability::TracingConfig {
            enabled: true,
            sampling_rate: 1.0,
            jaeger_enabled: true,
            jaeger_endpoint: Some("http://localhost:14268".to_string()),
            zipkin_enabled: false,
            zipkin_endpoint: None,
        },
        metrics: crate::observability::MetricsConfig {
            enabled: true,
            prometheus_enabled: true,
            prometheus_endpoint: Some("/metrics".to_string()),
            collection_interval_secs: 15,
            retention_days: 30,
        },
        logging: crate::observability::LoggingConfig {
            enabled: true,
            level: "INFO".to_string(),
            format: "json".to_string(),
            destination: "stdout".to_string(),
            file_path: None,
            structured: true,
            include_timestamp: true,
            include_thread_id: true,
        },
    };
    
    // Initialize observability system
    println!("1. Initializing Observability System...");
    let observability = init(config)?;
    
    println!("   ✅ Tracing: Enabled");
    println!("   ✅ Metrics: Enabled (Prometheus)");
    println!("   ✅ Logging: Enabled (Structured JSON)");
    
    // Demo 1: Distributed Tracing
    println!("\n2. Distributed Tracing Demo:");
    println!("---------------------------");
    
    if let Some(tracer) = observability.tracer() {
        // Start root span
        let root_span = tracer.start_span("process_request");
        println!("   Started root span: {}", root_span.name);
        
        // Add attributes to span
        if let Ok(_) = tracer.add_span_attribute(
            root_span.id,
            "http.method".to_string(),
            "GET".to_string(),
        ) {
            println!("   Added attribute: http.method=GET");
        }
        
        // Start child span
        if let Ok(child_span) = tracer.start_child_span(
            root_span.id,
            "database_query",
            SpanKind::Internal,
        ) {
            println!("   Started child span: {}", child_span.name);
            
            // Simulate database operation
            std::thread::sleep(Duration::from_millis(50));
            
            // End child span
            if let Ok(_) = tracer.end_span(child_span.id, SpanStatus::Ok) {
                println!("   Ended child span: {}", child_span.name);
            }
        }
        
        // Add event to span
        if let Ok(_) = tracer.add_span_event(
            root_span.id,
            "processing_complete".to_string(),
            HashMap::from([("result".to_string(), "success".to_string())]),
        ) {
            println!("   Added event: processing_complete");
        }
        
        // End root span
        if let Ok(_) = tracer.end_span(root_span.id, SpanStatus::Ok) {
            println!("   Ended root span: {}", root_span.name);
        }
        
        // Extract and inject context (headers propagation)
        let headers = tracer.inject_to_headers(root_span.id)?;
        println!("   Generated trace headers: {:?}", headers.keys());
    }
    
    // Demo 2: Metrics Collection
    println!("\n3. Metrics Collection Demo:");
    println!("---------------------------");
    
    if let Some(metrics) = observability.metrics_collector() {
        // Record counter metric
        let mut labels = HashMap::new();
        labels.insert("endpoint".to_string(), "/api/users".to_string());
        labels.insert("method".to_string(), "GET".to_string());
        
        if let Ok(_) = metrics.increment_counter(metric_names::HTTP_REQUESTS_TOTAL, labels.clone()) {
            println!("   Recorded HTTP request counter");
        }
        
        // Record gauge metric
        let mut gauge_labels = HashMap::new();
        gauge_labels.insert("service".to_string(), "example".to_string());
        
        if let Ok(_) = metrics.set_gauge("active_connections", 42.0, gauge_labels) {
            println!("   Recorded active connections gauge: 42");
        }
        
        // Record histogram metric
        let mut hist_labels = HashMap::new();
        hist_labels.insert("operation".to_string(), "search".to_string());
        
        if let Ok(_) = metrics.observe_histogram(
            "request_duration_seconds",
            0.125,
            hist_labels,
            None,
        ) {
            println!("   Recorded request duration histogram: 125ms");
        }
        
        // Get Prometheus metrics
        let prometheus_metrics = metrics.get_prometheus_metrics();
        println!("   Prometheus metrics format ready");
        println!("   Sample metric: {}", prometheus_metrics.lines().next().unwrap_or(""));
    }
    
    // Demo 3: Structured Logging
    println!("\n4. Structured Logging Demo:");
    println!("---------------------------");
    
    if let Some(logger) = observability.logger() {
        // Simple log
        if let Ok(_) = logger.info("Application started successfully", None) {
            println!("   Logged: Application started successfully");
        }
        
        // Log with context
        let context = LogContext::new()
            .with_string("user_id", "user123")
            .with_number("request_count", 42.0)
            .with_bool("authenticated", true);
        
        if let Ok(_) = logger.info("User request processed", Some(context)) {
            println!("   Logged with context: user_id=user123, request_count=42");
        }
        
        // Error log with error object
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        if let Ok(_) = logger.error_with_error("Failed to load configuration", &error, None) {
            println!("   Logged error with stack trace");
        }
        
        // Different log levels
        if let Ok(_) = logger.trace("Detailed trace information", None) {
            println!("   TRACE level log");
        }
        
        if let Ok(_) = logger.debug("Debug information", None) {
            println!("   DEBUG level log");
        }
        
        if let Ok(_) = logger.warn("Warning: Resource usage high", None) {
            println!("   WARN level log");
        }
        
        if let Ok(_) = logger.error("Error processing request", None) {
            println!("   ERROR level log");
        }
    }
    
    // Demo 4: HTTP Instrumentation
    println!("\n5. HTTP Instrumentation Demo:");
    println!("------------------------------");
    
    let http_instrumentation = HttpInstrumentation::new(observability.clone());
    
    // Create HTTP request
    let request_info = HttpRequestInfo {
        method: "POST".to_string(),
        path: "/api/users".to_string(),
        query: Some("limit=10".to_string()),
        headers: HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("user-agent".to_string(), "widya-client/1.0".to_string()),
        ]),
        client_ip: Some("192.168.1.100".to_string()),
        body_size: Some(1024),
        tenant_id: Some(uuid::Uuid::new_v4()),
        user_id: Some("user123".to_string()),
    };
    
    // Instrument request
    let instrumented_request = http_instrumentation.instrument_request(&request_info, None);
    println!("   Instrumented HTTP request: {} {}", request_info.method, request_info.path);
    println!("   Correlation ID: {}", instrumented_request.correlation_id());
    
    // Simulate processing
    std::thread::sleep(Duration::from_millis(100));
    
    // Complete request with response
    let response_info = HttpResponseInfo {
        status_code: 201,
        headers: HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("content-length".to_string(), "256".to_string()),
        ]),
        body_size: Some(256),
        response_time_ms: 105,
    };
    
    instrumented_request.complete(response_info);
    println!("   Request completed with status: 201 Created");
    
    // Demo 5: Database Instrumentation
    println!("\n6. Database Instrumentation Demo:");
    println!("---------------------------------");
    
    let db_instrumentation = DatabaseInstrumentation::new(observability.clone());
    
    // Create database query
    let query_info = DatabaseQueryInfo {
        query_type: "SELECT".to_string(),
        table: Some("users".to_string()),
        query_text: "SELECT * FROM users WHERE active = true".to_string(),
        params_count: 1,
        tenant_id: Some(uuid::Uuid::new_v4()),
        query_plan: Some("Index Scan".to_string()),
    };
    
    // Instrument query
    let instrumented_query = db_instrumentation.instrument_query(&query_info, None);
    println!("   Instrumented database query: {} on table '{}'", 
        query_info.query_type, query_info.table.as_deref().unwrap_or("unknown"));
    
    // Simulate query execution
    std::thread::sleep(Duration::from_millis(75));
    
    // Complete query with result
    let query_result = DatabaseQueryResult {
        rows_affected: Some(42),
        execution_time_ms: 78,
        error: None,
        success: true,
    };
    
    instrumented_query.complete(query_result);
    println!("   Query completed successfully, affected 42 rows");
    
    // Demo 6: OpenTelemetry Compatibility
    println!("\n7. OpenTelemetry Compatibility Demo:");
    println!("-------------------------------------");
    
    let otel_resource = OtelResource::new(
        "example-service".to_string(),
        "1.0.0".to_string(),
    )
    .with_environment("production".to_string())
    .with_attribute("team".to_string(), "platform".to_string());
    
    if let Some(otel_sdk) = OtelSdk::from_widya(&observability, otel_resource) {
        println!("   OpenTelemetry SDK initialized");
        
        // Use OpenTelemetry tracer
        let otel_tracer = otel_sdk.tracer_provider().tracer("example-tracer");
        let otel_span = otel_tracer.start("otel_operation");
        println!("   Created OpenTelemetry span");
        
        otel_span.set_attribute("otel.key".to_string(), "otel.value".to_string());
        otel_span.set_status(SpanStatus::Ok);
        println!("   Set span attributes and status");
        
        // Use OpenTelemetry metrics
        let otel_meter = otel_sdk.meter_provider().meter("example-meter");
        let otel_counter = otel_meter.create_counter("requests");
        
        let mut attributes = HashMap::new();
        attributes.insert("endpoint".to_string(), "/api/data".to_string());
        
        if let Ok(_) = otel_counter.add(1, attributes) {
            println!("   Recorded OpenTelemetry counter metric");
        }
        
        // Use OpenTelemetry logging
        let otel_logger = otel_sdk.logger_provider().logger("example-logger");
        
        let mut log_attributes = HashMap::new();
        log_attributes.insert("component".to_string(), "api".to_string());
        
        if let Ok(_) = otel_logger.emit(
            crate::observability::opentelemetry::OtelLogRecord::new(
                OtelSeverity::Info,
                "OpenTelemetry log message".to_string(),
            )
            .with_attribute("custom_field".to_string(), "custom_value".to_string()),
        ) {
            println!("   Logged via OpenTelemetry logger");
        }
    }
    
    // Demo 7: Trace-based Debugging Tools
    println!("\n8. Trace-based Debugging Tools:");
    println!("--------------------------------");
    
    // Create some example traces for analysis
    let example_traces = create_example_traces();
    
    // Use trace analyzer
    let analyzer = TraceAnalyzer::new(example_traces.clone())
        .with_service_filter("example-service");
    
    let analysis_report = analyzer.analyze();
    println!("   Trace Analysis Complete:");
    println!("   - Total traces: {}", analysis_report.total_traces);
    println!("   - Total spans: {}", analysis_report.total_spans);
    println!("   - Error rate: {:.2}%", analysis_report.error_rate * 100.0);
    println!("   - Avg duration: {}ms", analysis_report.avg_trace_duration_micros / 1000);
    
    // Use trace visualizer
    if let Some(trace) = example_traces.first() {
        let timeline = TraceVisualizer::generate_ascii_timeline(trace);
        println!("\n   Sample Trace Timeline:");
        for line in timeline.lines().take(10) {
            println!("   {}", line);
        }
        
        let dependency_graph = TraceVisualizer::generate_dependency_graph(trace);
        println!("\n   Sample Dependency Graph:");
        for line in dependency_graph.lines().take(15) {
            println!("   {}", line);
        }
    }
    
    // Use trace debugger
    let debugger = TraceDebugger::new(example_traces);
    println!("\n   Trace Debugger Available:");
    println!("   Use debugger.list_traces() to see available traces");
    println!("   Use debugger.select_trace(0) to select a trace");
    println!("   Use debugger.show_trace_details() to inspect trace");
    
    // Shutdown observability system
    println!("\n9. Shutting down observability system...");
    observability.shutdown()?;
    println!("   ✅ Observability system shutdown complete");
    
    println!("\n🎉 Observability Demo Complete!");
    println!("================================");
    println!("\nFeatures Demonstrated:");
    println!("1. ✅ Distributed Tracing with W3C Trace Context");
    println!("2. ✅ Metrics Collection (Prometheus format)");
    println!("3. ✅ Structured Logging with context");
    println!("4. ✅ HTTP Request/Response instrumentation");
    println!("5. ✅ Database Query instrumentation");
    println!("6. ✅ OpenTelemetry compatibility");
    println!("7. ✅ Trace-based debugging tools");
    println!("8. ✅ Performance analysis & bottleneck detection");
    println!("9. ✅ Trace visualization & dependency graphs");
    
    println!("\n📊 Production Ready For:");
    println!("• Microservices observability");
    println!("• Performance monitoring & alerting");
    println!("• Distributed system debugging");
    println!("• Compliance & audit logging");
    println!("• Business intelligence & analytics");
    
    Ok(())
}

/// Create example traces for demonstration
fn create_example_traces() -> Vec<widya::observability::tracing::Trace> {
    use widya::observability::tracing::{Trace, Span, SpanStatus};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    
    let trace1 = Trace {
        id: 1234567890,
        spans: vec![
            Span {
                id: 1,
                trace_id: 1234567890,
                parent_id: None,
                name: "process_request".to_string(),
                start_time: now,
                end_time: Some(now + 150_000), // 150ms
                duration: Some(150_000),
                attributes: std::collections::HashMap::from([
                    ("http.method".to_string(), "GET".to_string()),
                    ("http.path".to_string(), "/api/users".to_string()),
                ]),
                events: vec![],
                status: SpanStatus::Ok,
                kind: widya::observability::tracing::SpanKind::Server,
                service_name: "example-service".to_string(),
                tenant_id: Some(uuid::Uuid::new_v4()),
                correlation_id: 9876543210,
            },
            Span {
                id: 2,
                trace_id: 1234567890,
                parent_id: Some(1),
                name: "database_query".to_string(),
                start_time: now + 10_000, // 10ms after start
                end_time: Some(now + 60_000), // 50ms duration
                duration: Some(50_000),
                attributes: std::collections::HashMap::from([
                    ("db.operation".to_string(), "SELECT".to_string()),
                    ("db.table".to_string(), "users".to_string()),
                ]),
                events: vec![],
                status: SpanStatus::Ok,
                kind: widya::observability::tracing::SpanKind::Internal,
                service_name: "example-service".to_string(),
                tenant_id: Some(uuid::Uuid::new_v4()),
                correlation_id: 9876543210,
            },
        ],
        start_time: now,
        end_time: Some(now + 150_000),
        duration: Some(150_000),
        service_name: "example-service".to_string(),
        root_span_id: Some(1),
    };
    
    let trace2 = Trace {
        id: 1234567891,
        spans: vec![
            Span {
                id: 3,
                trace_id: 1234567891,
                parent_id: None,
                name: "process_order".to_string(),
                start_time: now + 200_000,
                end_time: Some(now + 500_000), // 300ms
                duration: Some(300_000),
                attributes: std::collections::HashMap::from([
                    ("http.method".to_string(), "POST".to_string()),
                    ("http.path".to_string(), "/api/orders".to_string()),
                ]),
                events: vec![],
                status: SpanStatus::Error,
                kind: widya::observability::tracing::SpanKind::Server,
                service_name: "example-service".to_string(),
                tenant_id: Some(uuid::Uuid::new_v4()),
                correlation_id: 9876543211,
            },
        ],
        start_time: now + 200_000,
        end_time: Some(now + 500_000),
        duration: Some(300_000),
        service_name: "example-service".to_string(),
        root_span_id: Some(3),
    };
    
    vec![trace1, trace2]
}