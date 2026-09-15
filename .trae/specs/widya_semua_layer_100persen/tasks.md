# Widya 9 Layer 100% - Implementation Plan

## Task 1: Modul L3 - Resource cgroup v2 Controller (TIDAK bergantung)
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - Buat file BARU `modul/resource_cgroup_v2.wya` (self-contained, 100+ baris)
  - Struktur: `CgroupV2Controller { nama, daftar_pid, cpu_max_ms, memory_max_bytes, io_max_rbps, statistik_enforce }`
  - Fungsi: `cg_buat(nama)`, `cgroup_tambah_proses(cg, pid, cpu_ms, mem_bytes, io_bps)`, `cgroup_terapkan_limit(cg)` (return bool), `cgroup_baca_statistik(cg)` (return dict)
  - Self-test di AKHIR file: buat "widya_db_container", tambah PID 1234, CPU=50000, Memory=536870912 (512MB), IO=104857600 (100MB/s), terapkan → pastikan statistik.enforcement_terapkan=benar, statistik.daftar_pid[0]=1234, statistik.max_mem_mb=512
- **Acceptance Criteria Addressed**: AC-3, FR-3
- **Test Requirements**:
  - `rule` TR-1.1: Exit self-test `widya jalankan modul/resource_cgroup_v2.wya` = 0.
  - `rule` TR-1.2: 3 assertions self-test TRUE (enforce_apply=T, pid=1234, max_mem=512).
  - `rule` TR-1.3: Tidak import modul lain; self-contained.
- **Notes**: TIDAK butuh Linux asli; simulasi state controller yang konsisten.

## Task 2: Modul L2 - Interrupt IDT APIC + ISR 6 Vector
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - Buat file BARU `modul/interrupt_idt_apic.wya`
  - Struktur: `IDTBuilder { daftar_isr = dict[vector_num] = handler_fun, pic_mask = array[16], apic_eoi_terakhir }`
  - Struktur `ISRHandler { vector, nama, dipanggil_flag, data_terakhir }`
  - Fungsi: `idt_buat()`, `idt_tambah_isr(idt, vector, nama_handler, handler_func)`, `idt_dispatch_interrupt(idt, vector, data_args)` (return hasil handler), `pic_set_mask(idt, irq_num, is_masked)`, `apic_simpan_eoi(idt, vector)`
  - Self-test: idt_tambah 6 vector (0x20 timer, 0x21 keyboard, 0x24 serial, 0x0E pagefault, 0x0D gpfault, 0x80 syscall) → dispatch(0x21) return "KEYBOARD_ISR_DIPANGGIL" → dispatch(0x80) return "SYSCALL_0X80_OK" → panjang isr_list=6
- **Acceptance Criteria Addressed**: AC-1, FR-1
- **Test Requirements**:
  - `rule` TR-2.1: Self-test exit 0; 6 vector terdaftar; dispatch(0x21) = KEYBOARD_OK; dispatch(0x80) = SYSCALL_OK.
  - `rule` TR-2.2: apic_simpan_eoi(0x21) update apic_eoi_terakhir = 0x21.
- **Notes**: ISR handler cukup closure sederhana yang return string.

## Task 3: Modul L6 - CUDA FFI Runtime Matmul Kernel Launch
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/cuda_ffi_runtime.wya`
  - Struktur: `CUDARuntime { versi, device_count, daftar_ptr_device }` + `CUDAKernel { nama, grid_dim, block_dim, shared_kb }`
  - Fungsi: `cuda_runtime(versi)`, `cuda_get_device_count(r)`, `cuda_malloc(r, mb) → device_ptr`, `cuda_memcpy_h2d(r, ptr, data_matriks_array)`, `cuda_kernel_launch(r, kernel_spec, grid_x, grid_y, block_x, block_y)`, `cuda_sinkronkan(r) → string "CUDA_SUCCESS"`
  - Self-test: Runtime 12.2, device_count ≥ 1, cudaMalloc 256MB → ptr tidak null, kernel_launch matmul_float32 grid=(4,4) block=(64,1), cuda_sinkronkan = "CUDA_SUCCESS" → 3 assertions TRUE.
- **Acceptance Criteria Addressed**: AC-6, FR-6
- **Test Requirements**:
  - `rule` TR-3.1: device_count ≥ 1; malloc_size_mb=256; sync_result = CUDA_SUCCESS.
  - `rule` TR-3.2: Kernel launch return tanpa error; self-test exit 0.
- **Notes**: Simulasi state CUDA API yang konsisten (tidak butuh libcuda.so).

## Task 4: Modul L6 - SGX Enclave Simulasi Ecall + Local Attestation
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/sgx_enclave_simulasi.wya`
  - Struktur: `SGXEnclave { id, kunci_enkrip_AES, state = "UNINIT", daftar_ecall, report_mac_valid, sealed_data }`
  - Struktur `SgxReport { cpu_svn, isvsvn, mrenclave, report_mac_256_bit }`
  - Fungsi: `sgx_buat_enclave(id_enclave)`, `sgx_ecall_create(enc, id_ecall, code_func_body_dict)`, `sgx_ecall_panggil(enc, id_ecall, args_dict) → hasil`, `sgx_attestasi_lokal(enc) → SgxReport`, `sgx_seal_data(enc, plain) → cipher_text`, `sgx_unseal_data(enc, cipher) → plain`
  - Self-test: Enclave "signer_widya" AES256, ecall "sign_tx_hash" body { return SHA256(args.hash+"_SIGNED_BYTE_SGX") }, ecall_panggil dengan hash=0xDEADBEEF, attestasi_lokal() report.mac_valid = benar, enclave state = "INITIALIZED_SECURE".
- **Acceptance Criteria Addressed**: AC-5, FR-5
- **Test Requirements**:
  - `rule` TR-4.1: Signature panjang ≥ 64 hex chars; attestasi.report_mac_valid = T; state = "INITIALIZED_SECURE".
  - `rule` TR-4.2: Seal("RAHASIA") → Unseal() = "RAHASIA"; self-test exit 0.
- **Notes**: AES encryption dapat menggunakan XOR sederhana dengan kunci + SHA256 untuk hash signature (self-contained pure Widya).

## Task 5: Modul L6 - FPGA Partial Reconfig + Register RW
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/fpga_bitstream_config.wya`
  - Struktur: `FPGABoard { vendor, part, bus, status_config, daftar_region_pr, register_map }`
  - Fungsi: `fpga_buat_board(vendor, part, bus)`, `fpga_program_bitstream(fpga, file_nama_bit) → bool`, `fpga_partial_reconfig(fpga, region_nama, file_partial) → dict`, `fpga_baca_register(fpga, addr_hex) → value`, `fpga_tulis_register(fpga, addr_hex, value)`
  - Default: register 0x00 = 0xDEADBEEF (FPGA ID). Self-test: Buat Xilinx XC7A35T JTAG, program "kernel_accel.bit" → status_config = "DONE"; partial region "fft_256" → daftar_region["fft_256"] = "CONFIGURED"; tulis 0x04=42 → baca 0x04 = 42.
- **Acceptance Criteria Addressed**: AC-9, FR-9
- **Test Requirements**:
  - `rule` TR-5.1: program_status = "DONE"; region_fft = "CONFIGURED"; reg_0x04 = 42.
  - `rule` TR-5.2: Baca default register 0x00 = 3735928559 (0xDEADBEEF decimal).
- **Notes**: Tidak butuh file bitstream asli; nama file cukup string state tracking.

## Task 6: Modul L2 - Hypervisor KVM/Xen VMCS/VMCB + Nested VT
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/hypervisor_kvm_xen.wya`
  - Struktur: `HypervisorControl { vendor, daftar_vm, nested_vmx_supported_flag }`
  - Struktur `VMCS_VMCB { nama_vm, vcpu, ram_mb, register_set{rax,rbx,rcx,rdx,rip,rsp}, status_running }`
  - Fungsi: `hv_buat(vendor="KVM"/"XEN")`, `vmcs_buat_vmcb(hv, nama, vcpu, ram) → vm_id`, `hv_nested_vt_check(hv) → string "VT-x_Enabled_Nested" atau "DISABLED"`, `vmcs_luncurkan(hv, vm_id) → dict{status}`, `vmcs_baca_register(hv, vm_id, reg_nama) → value`, `vmcs_tulis_register(hv, vm_id, reg_nama, val)`
  - Self-test: Buat HV KVM; VM1 (edge_router, 2vcpu, 512) + VM2 (cloud_server, 4vcpu, 2048); Nested_VT = "VT-x_Enabled_Nested"; tulis RAX VM1 = 2863311530 (0xAAAABABA hex); Launch VM1 status = "RUNNING"; baca RAX = 2863311530; total VMs = 2.
- **Acceptance Criteria Addressed**: AC-2, FR-2
- **Test Requirements**:
  - `rule` TR-6.1: Total VM = 2; NestedVT = "Enabled_Nested"; RAX VM1 = 2863311530.
  - `rule` TR-6.2: VM1 Running = "RUNNING"; self-test exit 0.
- **Notes**: VMCS/VMCB dapat diwakili struktur dict.

## Task 7: Modul L3 - Wasmtime/Wasmer Runtime Features Config
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/wasmtime_wasmer_runtime.wya`
  - Struktur: `WasmRuntime { vendor, daftar_module_inst, fitur_aktif, engine_state }`
  - Fungsi: `runtime_buat(vendor="Wasmtime"/"Wasmer"/"Wazero")`, `runtime_tambah_fitur(r, nama_fitur)` (wasi,threads,component_model,simd), `runtime_muat_module(r, file_wasm)` → module_id, `runtime_instantiate(r, mod_id) → status`, `runtime_panggil_fungsi(r, mod_id, nama_func, args_list) → hasil`
  - Self-test: Buat Wasmtime; tambah fitur wasi_snapshot_preview1 & threads (count=2); muat "app.wasm"; instantiate = "READY"; panggil main([]) exit_code = 0; state = "READY".
- **Acceptance Criteria Addressed**: AC-4, FR-4
- **Test Requirements**:
  - `rule` TR-7.1: feature_count = 2; instantiate = "READY"; exit_main = 0.
  - `rule` TR-7.2: Panggil fib(10)=55 (tambahkan function call fibonacci sebagai test tambahan) → hasil = 55.
- **Notes**: Module function body dapat dibuat closure inline (pure Widya).

## Task 8: Modul L6 - OpenCL Platform Context + NDRange VecAdd
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/opencl_wrapper_platform.wya`
  - Struktur: `OpenCLPlatform { id, nama, versi, daftar_device }` + `OpenCLContext { device_type, program_kernels, command_queue }`
  - Fungsi: `ocl_dapatkan_platform(index_platform) → OpenCLPlatform`, `ocl_buat_context(platform_id, device_type="GPU"/"CPU") → ctx_id`, `ocl_buat_program(ctx, source_kernel_string) → prog_id`, `ocl_enqueue_ndrange_kernel(ctx, kernel_nama, global_work_size_list, local_work_size_list) → dict`
  - Self-test: Platform 0 (NVIDIA OpenCL 3.0, nama mengandung "OpenCL"); context GPU; NDRange vecadd global=1024, local=64 → global_work = 1024; platform mengandung "OpenCL"; devices = ["GPU"].
- **Acceptance Criteria Addressed**: AC-7, FR-7
- **Test Requirements**:
  - `rule` TR-8.1: platform_nama mengandung "OpenCL"; ctx_device = GPU; global_work = 1024.
  - `rule` TR-8.2: NDRange enqueue return sukses TRUE; self-test exit 0.
- **Notes**: Kernel source cukup string sebagai state tracking; device list hardcoded standard.

## Task 9: Modul L6 - HSM PKCS#11 RSA2048 KeyGen + Sign Verify
- **Status**: `pending`
- **Priority**: high
- **Depends On**: None
- **Description**:
  - File BARU `modul/hsm_pkcs11_emulasi.wya`
  - Struktur: `HSMPKCS11 { slot_id, token_label, daftar_key_handle, login_status_so }`
  - Struktur `KeyPairHSM { handle, alg, label, pub_hex, priv_hex_tersimpan }`
  - Fungsi: `hsm_buat(slot, label)`, `c_login_so(hsm, so_pin_string) → bool`, `c_generate_key_pair(hsm, algoritma="RSA2048"/"ECDSA_P256", label_kunci) → handle`, `c_sign(hsm, handle_key, data_hash_sha256_hex) → signature_hex`, `c_verify(hsm, handle_key, data_hash, signature) → bool`
  - Default: Login berhasil jika SO_PIN = "12345678" (panjang 8). Self-test: HSM slot 0 label "WidyaHSM", login "12345678" = TRUE; generate RSA2048 label "WIDYA_SIGN_KEY_01" (alg = "RSA2048"); data_hash = SHA256("PAYMENT_123"); sign signature panjang ≥ 256 chars; verify = benar.
- **Acceptance Criteria Addressed**: AC-8, FR-8
- **Test Requirements**:
  - `rule` TR-9.1: login = TRUE; alg = "RSA2048"; signature ≥256; verify = TRUE.
  - `rule` TR-9.2: ECDSA_P256 generate key → sign → verify juga = TRUE (tambahan kasus edge).
- **Notes**: Signature dapat berupa hash panjang tetap yang disimpan state (PKCS11 abstraction level).

## Task 10: Append KEDUA modul existing + Demo 168 + 2 Test Rust Suite
- **Status**: `pending`
- **Priority**: high
- **Depends On**: Task 1, Task 2, Task 3, Task 4, Task 5, Task 6, Task 7, Task 8, Task 9 (semua modul BARU dulu baru append + demo + test)
- **Description**:
  - Bagian A: **Append `modul/widya_os.wya` DI AKHIR FILE**: Tambahkan 6 struktur syscall interrupt vector: SYSCALL_INT_0X20 (timer), SYSCALL_INT_0X21 (keyboard), SYSCALL_INT_0X24 (serial), SYSCALL_INT_0X0E (pagefault), SYSCALL_INT_0X0D (gpfault), SYSCALL_INT_0X80 (syscall). Tambahkan fungsi `os_interrupt_syscall(kernel, cap, vector_num, args_int_dict)` → capability check SEBELUM dispatch ke ISR handler (mirip `os_syscall()` yang existing). Append JANGAN edit baris lama.
  - Bagian B: **Append `modul/ai_federasi.wya` DI AKHIR FILE**: Tambahkan struktur `CUDAShardNode { gpu_count, model_nama, mem_gb, tensor_core_ada }` + fungsi `fedavg_cuda_stream_sharded(cluster_ai, data_all_dict)` → dispatch training matmul ke CUDA (jika CUDA runtime tidak ada → fallback CPU SIMD). Tambah 1 assertion inline append.
  - Bagian C: **Buat `contoh/168_semua_layer_100persen.wya`** end-to-end 15 step assertions: (1) init_interrupt_6vector; (2) init_hypervisor_2vm_nested; (3) init_cgroup_widya_db; (4) init_wasmtime_runtime; (5) sgx_sign_ecall; (6) cuda_matmul_256; (7) opencl_vecadd_1024; (8) hsm_rsa_2048_sign_verify; (9) fpga_program_partial; (10) widyaos_append_interrupt_test; (11) **INTEGRASI_L2_L3_L6**: interrupt timer 0x20 → cgroup limit check CPU tersedia → FedAvg CUDA dispatch → cetak "INTEGRASI_L2_L3_L6_OK"; (12) L1 baremetal GPIO toggle; (13) L4 mesh DHT lookup; (14) L5 widydb spatial_join; (15) Banner print `🎉 WIDYA 9 LAYER 100% SIAP 🎉` dan exit 0. Semua 15 step harus ada assertion `pastikan(...)`. Self-contained: copy inline semua struktur/fungsi pendukung jika perlu.
  - Bagian D: **Buat `tests/test_all9layer_core.rs`** 8 test inline Rust dengan delimiter `r##"... "##;`: test_idt_isr, test_hypervisor_2vm, test_cgroup_limit, test_wasmtime_run, test_sgx_attest, test_cuda_matmul, test_opencl_vecadd, test_hsm_rsa_sign. Setiap test: `jalankan(code).unwrap() == Value::Bool(true)`.
  - Bagian E: **Buat `tests/test_all9layer_full_stack.rs`** 14 test inline: test_idt_6vectors, test_hypervisor_nested_vt, test_cgroup_memory_512, test_wasmtime_2features, test_sgx_seal_unseal, test_cuda_sync_success, test_opencl_gpu, test_hsm_pkcs11_login, test_fpga_partial, test_widyaos_append_syscall, test_aifed_cuda_dispatch, test_168banner_exit0, test_11modules_self_test, test_aggregate_30tests. DELIMITER SELALU r##"..."## (double hash, hindari premature close).
- **Acceptance Criteria Addressed**: AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-8, AC-9, AC-10, AC-11, AC-12, AC-13 (SEMUA 13 AC)
- **Test Requirements**:
  - `rule` TR-10.1: `widya jalankan contoh/168_semua_layer_100persen.wya` exit 0, banner "WIDYA 9 LAYER 100% SIAP" ada di stdout.
  - `rule` TR-10.2: `cargo test --test test_all9layer_core --test test_all9layer_full_stack` TOTAL running ≥ 22, 0 failed.
  - `rubric` TR-10.3: Cross-layer integration L2→L3→L6. Scale 1-5. Threshold ≥4. Evidence: demo 168 step 11 assertion "INTEGRASI_L2_L3_L6_OK" = TRUE.
  - `rubric` TR-10.4: Backward compat 0 break. Scale 1-5. Threshold ≥4. Evidence: 5 modul BARU + append AKHIR 2 modul existing + 0 perubahan src/Cargo.toml.
- **Notes**: SELALU gunakan delimiter `r##"` di Rust inline test; TIDAK pernah `r#"` (akan ditutup `"#` di kode widya).
