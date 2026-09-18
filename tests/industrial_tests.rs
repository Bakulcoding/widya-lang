use widya::dap::{DapEngine, DebuggerState, StackFrameInfo};
use widya::ffi::{FfiFunctionSignature, FfiManager, FfiType};
use widya::profiler::MemoryProfiler;
use widya::value::Value;

#[test]
fn test_ffi_registration_and_calls() {
    let mut ffi = FfiManager::new();
    
    // Call builtin C abs
    let res_abs = ffi.call_foreign_function("abs", &[Value::Number(-42.0)]).unwrap();
    assert_eq!(res_abs, Value::Number(42.0));

    // Call builtin C sqrt
    let res_sqrt = ffi.call_foreign_function("sqrt", &[Value::Number(16.0)]).unwrap();
    assert_eq!(res_sqrt, Value::Number(4.0));

    // Register custom C signature
    ffi.register_signature(FfiFunctionSignature {
        name: "hitung_pajak".to_string(),
        symbol: "calculate_tax".to_string(),
        library: "libtax.so".to_string(),
        param_types: vec![FfiType::Float64, FfiType::Float64],
        return_type: FfiType::Float64,
        is_variadic: false,
    });

    assert!(ffi.get_signature("hitung_pajak").is_some());
}

#[test]
fn test_memory_profiler_and_cycle_detection() {
    let mut profiler = MemoryProfiler::new();

    let id1 = profiler.track_allocation(&Value::String("Widya".to_string()), 32);
    let id2 = profiler.track_allocation(&Value::Number(100.0), 16);
    let id3 = profiler.track_allocation(&Value::Bool(true), 8);

    // Create cyclic reference id1 -> id2 -> id3 -> id1
    profiler.add_reference(id1, id2);
    profiler.add_reference(id2, id3);
    profiler.add_reference(id3, id1);

    let cycles = profiler.detect_reference_cycles();
    assert!(!cycles.is_empty(), "Should detect circular reference cycle");

    let snapshot = profiler.generate_snapshot();
    assert_eq!(snapshot.active_objects_count, 3);
    assert_eq!(snapshot.detected_cycles_count, cycles.len());

    let report = profiler.format_report();
    assert!(report.contains("Memory & Allocation Profiler Report"));
}

#[test]
fn test_dap_engine_breakpoints_and_execution() {
    let mut dap = DapEngine::new();
    assert_eq!(dap.state, DebuggerState::Initialized);

    let bps = dap.set_breakpoints("app.wya", vec![10, 25, 40]);
    assert_eq!(bps.len(), 3);

    // Hit breakpoint at line 10
    let hit = dap.check_breakpoint("app.wya", 10);
    assert!(hit);
    assert_eq!(dap.state, DebuggerState::Paused);
    assert_eq!(dap.current_line, 10);

    // Step over
    dap.step_over();
    assert_eq!(dap.current_line, 11);

    // Push call stack frame
    dap.push_frame(StackFrameInfo {
        id: 1,
        name: "hitung_total".to_string(),
        file: "app.wya".to_string(),
        line: 11,
        column: 4,
    });

    dap.set_variable("total_belanja", "50000", "Angka");
    let eval_res = dap.evaluate_expr("total_belanja").unwrap();
    assert!(eval_res.contains("50000"));

    let caps = dap.get_dap_capabilities();
    assert!(caps["supportsConfigurationDoneRequest"].as_bool().unwrap());
}
