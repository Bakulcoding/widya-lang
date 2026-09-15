# Widya 9 Layer Computing Stack 100% Siap - Product Requirements Document

## Overview
- **Summary**: Menutup sisa 4% gap kesiapan Widya-Lang di 3 layer (Layer 2 OS Kernel & Hypervisors: Interrupt IDT/ISR + Hypervisor simulasi; Layer 3 Virtualisasi & Container: cgroup resource limit wrapper + Wasmtime runner; Layer 6 Komputasi Khusus: Intel SGX enclave simulasi, CUDA FFI binding, OpenCL wrapper, HSM PKCS#11, FPGA partial reconfig controller) sehingga SEMUA 9 layer stack computing (L1 baremetal sampai L9 user apps) resmi 100% siap.
- **Purpose**: Mencapai status "Widya Bisa di Semua Layer 100%" tanpa pengecualian kategoris, sehingga Widya-Lang dapat dijadikan fondasi teknologi universal lintas domain.
- **Target Users**: Developer Widya yang membangun OS heterogen terdistribusi, stack cloud enterprise, DePIN/DeFi, AI edge, IoT/SCADA industrial, aerospace/drone, dan HPC high-performance.

## Goals
- **G-1 (L2 Coverage)**: Tersedia modul `interrupt_idt_apic.wya` (Interrupt Descriptor Table + PIC/APIC + ISR handler) + modul `hypervisor_kvm_xen.wya` (Hypervisor simulasi VMX/SVM dengan VMCS/VMCB control structure + nested virtualization flag).
- **G-2 (L3 Coverage)**: Tersedia modul `resource_cgroup_v2.wya` (cgroup v2 wrapper CPU/Memory/IO limit setara Linux) + modul `wasmtime_wasmer_runtime.wya` (Wasmtime/Wasmer runner CLI abstraction + runtime features config).
- **G-3 (L6 Coverage)**: Tersedia 5 modul baru: `sgx_enclave_simulasi.wya`, `cuda_ffi_runtime.wya`, `opencl_wrapper_platform.wya`, `hsm_pkcs11_emulasi.wya`, `fpga_bitstream_config.wya`.
- **G-4 (Append Existing)**: Append 6 syscall interrupt di `modul/widya_os.wya` + append CUDA kernel fallback di `modul/ai_federasi.wya`.
- **G-5 (Demo + Test)**: `contoh/168_semua_layer_100persen.wya` end-to-end 15 step + `tests/test_all9layer_core.rs` (8 test) + `tests/test_all9layer_full_stack.rs` (14 test).
- **G-6**: Review R1 = pass, 22/22 CP tercentang, 0 actionable findings.

## Non-Goals
- TIDAK menjalankan binary baremetal boot di perangkat keras nyata (scope tetap Abstraction Layer + Simulator terverifikasi, konsisten FR-36 spec WidyaOS).
- TIDAK install CUDA Toolkit / Intel SGX SDK / Xilinx Vivado di environment CI (scope simulasi API + abstraction yang valid).
- TIDAK mengubah file interpreter Rust di `src/` (ast, parser, lexer, borrow_checker, main, lib) — kecuali tests/ dan modul .wya BARU.
- TIDAK menambahkan Cargo deps baru (hanya gunakan 9 deps SUDAH ADA: clap, colored, rustyline, rand, serde_json, sha2, hex, hmac, regex).
- TIDAK memecah modul existing (`widya_os.wya`, `ai_federasi.wya`) — hanya APPEND fungsi/struktur BARU di AKHIR file.

## Background & Context
- Analisis sebelumnya tanggal 2026-09-15 09:43 WIB menunjukkan rata-rata 96%: L1 100%, L2 95% (kurang Interrupt IDT/ISR + Hypervisor), L3 90% (kurang cgroup wrapper + Wasmtime runner abstraction), L4 100%, L5 100%, L6 75% (kurang SGX/HSM/FPGA/CUDA/OpenCL 5 kategori hardware khusus), L7 100%, L8 100%, L9 100%.
- Evidence existing 100% layer terverifikasi: `modul/widya_os.wya`, `modul/scheduler_heterogen.wya`, `contoh/165_widyaos_boot.wya` (L2 OS); `stdlib/mod.rs:24996` Docker + `stdlib:25548` Wasm Linear Memory (L3 Virt); `modul/mesh_libp2p_style.wya`, `contoh/159_solusi_postgres.wya` (L4 Net); `modul/widyadb.wya` + 16 Cargo Test (L5 DB); `gpu.rs` WGSL + `ai_federasi.wya` FedAvg (L6 Komputasi); `studio.rs` Monaco + `main.rs:34` CLI (L7/L8); `contoh/167_widyadb_produksi.wya` + 167 aplikasi demo (L9 Apps).
- Naming convention: struktur/fungsi Bahasa Indonesia, standard industri nama tetap Bahasa Inggris (SGX_ENCLAVE_CREATE, CUDA_KERNEL_LAUNCH, PKCS11_SIGN, VMCS, cgroup.controllers, IDT, PIC, APIC).

## Functional Requirements
- **FR-1 (L2 Interrupt)**: `modul/interrupt_idt_apic.wya` menyediakan: IDTBuilder + idt_tambah_isr(vector, handler) + pic_set_mask(irq) + apic_simpan_eoi() + isr_default_handler(). Demo: 6 vector (keyboard 0x21, timer 0x20, syscall 0x80, pagefault 0x0E, gp_fault 0x0D, serial 0x24).
- **FR-2 (L2 Hypervisor)**: `modul/hypervisor_kvm_xen.wya` menyediakan: HypervisorControl(vendor="KVM"/"XEN") + vmcs_buat_vmcb(nama_vm, vcpu_count, ram_mb) + vmcs_luncurkan(vm) + nested_vt_flag_check() + vmcs_baca_register(vm, reg). Demo: 2 VM tamu (Linux edge 512MB + Windows server 2GB).
- **FR-3 (L3 cgroup)**: `modul/resource_cgroup_v2.wya` menyediakan: CgroupV2Controller + cgroup_tambah_proses(cg, pid, max_cpu_ms, max_mem_mb, max_io_rbps) + cgroup_terapkan_limit() + cgroup_baca_statistik(). Demo: container "widya_db" max CPU 50ms, mem 512MB, IO 100MBps.
- **FR-4 (L3 Wasmtime)**: `modul/wasmtime_wasmer_runtime.wya` menyediakan: WasmRuntime(vendor="Wasmtime"/"Wasmer"/"Wazero") + runtime_muat_module(r, file_wasm, features=["wasi_snapshot_preview1","threads"]) + runtime_panggil_fungsi(r, nama_func, args) + runtime_instantiate(). Demo: muat app.wasm, panggil main(), return 0.
- **FR-5 (L6 SGX)**: `modul/sgx_enclave_simulasi.wya` menyediakan: SGXEnclave(enkripsi_key=AESGCM256) + sgx_ecall_create(enclave, id, code) + sgx_ocall_return(enclave, hasil) + sgx_attestasi_local() + sgx_seal_data(enclave, plain). Demo: enclave "signer_widya", ecall sign_tx_hash, attestasi SGX-Local valid.
- **FR-6 (L6 CUDA)**: `modul/cuda_ffi_runtime.wya` menyediakan: CUDARuntime(version="12.2") + cuda_malloc(d_mb) + cuda_memcpy_h2d(host_data, device_ptr) + cuda_kernel_launch(grid, block, kernel_name) + cuda_sinkronkan(). Demo: matmul 256x256 GPU, cudaGetDeviceCount=1 sukses.
- **FR-7 (L6 OpenCL)**: `modul/opencl_wrapper_platform.wya` menyediakan: OpenCLPlatform + ocl_dapatkan_platform(n_platform) + ocl_buat_context(platform_id, device_type="GPU") + ocl_enqueue_ndrange_kernel(cmd_q, global_work). Demo: platform NVIDIA OpenCL 3.0, kernel vecadd 1024 items sukses.
- **FR-8 (L6 HSM)**: `modul/hsm_pkcs11_emulasi.wya` menyediakan: HSMPKCS11(token_label="WidyaHSM", slot_id=0) + c_login(so_pin) + c_generate_key_pair(alg="RSA2048" atau "ECDSA_P256") + c_sign(handle_key, data_hash) + c_verify(signature). Demo: generate RSA2048, sign hash SHA256, verify = true.
- **FR-9 (L6 FPGA)**: `modul/fpga_bitstream_config.wya` menyediakan: FPGABoard(vendor="Xilinx"/"Intel", part="XC7A35T", bus="JTAG") + fpga_program_bitstream(fpga, file_bit) + fpga_partial_reconfig(fpga, region, partial_bit) + fpga_baca_register(fpga, 0x00) + fpga_tulis_register(fpga, addr, val). Demo: program bitstream "kernel_accel.bit", partial reconfig region "fft_256", baca 0x00 = 0xDEADBEEF.
- **FR-10 (Append widya_os.wya)**: Tambahkan 6 syscall vector interrupt handler baru di AKHIR file: SYSCALL_INT_0X80, SYSCALL_INT_KEYBOARD 0x21, SYSCALL_INT_TIMER 0x20, SYSCALL_INT_PAGEFAULT 0x0E, SYSCALL_INT_GP_FAULT 0x0D, SYSCALL_INT_SERIAL 0x24. SEMUA syscall melalui KAL capability check sebelum eksekusi.
- **FR-11 (Append ai_federasi.wya)**: Tambahkan di AKHIR file: struktur `CUDAShardNode(gpu_count=8, model="NVIDIA_H100", mem_gb=80)` + fungsi `fedavg_cuda_stream_sharded(cluster, alldata)` yang men-dispatch matmul training ke CUDA kernel (fallback ke CPU vectorized jika CUDA tidak tersedia).
- **FR-12 (Demo 168)**: `contoh/168_semua_layer_100persen.wya` 15 step assertion: init IDT 6 vector → init Hypervisor 2 VM → init cgroup 2 container → init Wasmtime 2 runtime → SGX enclave sign → CUDA matmul → OpenCL vecadd → HSM RSA sign/verify → FPGA program partial → append widya_os interrupt syscall test → append ai_fed CUDA dispatch test → assert banner "WIDYA 9 LAYER 100% SIAP".
- **FR-13 (Test Core)**: `tests/test_all9layer_core.rs` 8 test: idt_isr / hypervisor_2vm / cgroup_limit / wasmtime_run / sgx_attest / cuda_matmul / opencl_vecadd / hsm_rsa_sign.
- **FR-14 (Test Full Stack)**: `tests/test_all9layer_full_stack.rs` 14 test: idt_6vectors_tambahkan / hypervisor_nested_check / cgroup_max_memory / wasmtime_features / sgx_seal_unseal / cuda_kernel_sync / opencl_context / hsm_p11_login / fpga_partial_reconfig / widyaos_append_syscall / aifed_cuda_dispatch / demo168_exit0_banner / aggregate_11modules_self_test_pass / cargo_all_test_aggregate_30_plus.

## Non-Functional Requirements
- **NFR-1 Widya file compilable**: SEMUA 5 modul baru .wya + demo 168 exit 0 via `widya jalankan <file.wya>`.
- **NFR-2 Rust tests buildable**: `cargo test --test test_all9layer_core --test test_all9layer_full_stack` PASS 0 failed (min 22 test passed).
- **NFR-3 Backward Compatible 0 break**: Modul existing 14/114/115/92/165/166/167/protokol_postgres/widya_os/ai_federasi TIDAK DIHAPUS / DIUBAH API LAMA; hanya append fungsi BARU di akhir.
- **NFR-4 Self-Contained Setiap modul**: 7 file .wya baru (5 modul + 2 append benar-benar berdiri sendiri; jika butuh helper, copy inline).
- **NFR-5 Convention**: Tidak ada komentar inline kecuali banner header (struktur/fungsi: ID, variabel lokal ID, standard EN).
- **NFR-6 0 Cargo deps**: Hanya deps yang SUDAH ADA di Cargo.toml; tidak boleh menambah crate baru.

## Constraints
- **Technical**:
  - File writes: HANYA di `modul/` (5 baru + 2 append), `contoh/168_*.wya`, `tests/` (2 test Rust BARU). DILARANG mengedit file di `src/` existing KECUALI jika tidak ada alternatif.
  - Pattern test Rust: SELALU `r##" ... "##;` (double raw hash) untuk menghindari premature close inline Widya `m["#"]=35` di pgwire/byte array.
- **Business**: 10 task pipeline diselesaikan 3 gelombang paralel subagent. Tidak ada scope creep.
- **Dependencies**: Existing patterns: `modul/widya_os.wya` KAL + `modul/ai_federasi.wya` ShardLLM + `stdlib:25000` Container + `stdlib:25478` Wasm + `stdlib:222` ZK + `stdlib:253` FHE.

## Assumptions
- **A-1**: Wasmtime tidak diinstall di environment. Maka WasmtimeRunner cukup abstraksi CLI (format command `wasmtime run --wasm-features=threads app.wasm` + mock return code 0).
- **A-2**: SGX/CUDA/OpenCL/HSM/FPGA TIDAK punya hardware di CI. Maka modul implementasi simulasi API dengan return value valid yang konsisten dengan standard industri (bukan mock random).
- **A-3**: cgroup v2 membutuhkan Linux root; tapi modul mengimplementasikan independent controller interface dengan struktur state terverifikasi (limit_count, enforcement_flags).
- **A-4**: Test inline Rust SELALU self-contained, tidak import modul lain; semua struktur dan fungsi helper copy inline ke block kode widya di `r##"..."##;`.
- **A-5**: Append widya_os.wya / ai_federasi.wya DILAKUKAN DI AKHIR FILE BARIS PALING BAWAH, tidak mengedit baris lama (0 breaking change).

## Open Questions
- **[Terjawab Default = YA]**: Apakah simulasi hardware khusus dapat diterima untuk 100% kategori (bukan real hardware binding)? Default: YA — sesuai pola spec WidyaOS/WidyaDB sebelumnya (Abstraction Layer + Simulator terverifikasi = lulus AC).
- **[Terjawab Default = YA]**: Apakah cgroup/wasmtime cukup wrapper abstraction tanpa sistem Linux? Default YA (setara L2 simulasi MMU/Capability yang sudah lulus 100% sebelumnya).

## Acceptance Criteria

### AC-1: IDT Interrupt Module 6 Vector ISR Terdaftar dan Dispatch Benar
- **Type**: `rule`
- **Given**: `modul/interrupt_idt_apic.wya` dimuat
- **When**: Jalankan self-test: tambah 6 ISR (timer 0x20, keyboard 0x21, serial 0x24, pagefault 0x0E, gpfault 0x0D, syscall 0x80), dispatch interrupt 0x21 lalu dispatch 0x80
- **Then**: Panjang daftar ISR = 6, dispatch keyboard ISR return "KEYBOARD_ISR_DIPANGGIL", dispatch syscall return "SYSCALL_0X80_OK"
- **Pass Condition**: 3 assertions TRUE, exit code 0
- **Evidence**: CLI `widya jalankan modul/interrupt_idt_apic.wya` output; test_core #1 pass

### AC-2: Hypervisor Module 2 VM Tamu + Nested VT Flag Valid
- **Type**: `rule`
- **Given**: `modul/hypervisor_kvm_xen.wya` dimuat
- **When**: Buat Hypervisor vendor "KVM", buat VM tamu W1 (edge 512MB 2vcpu) + VM W2 (server 2GB 4vcpu). Cek nested_vt_flag(). Launch W1 → baca register RAX = 0x1BADB002
- **Then**: Total VM = 2, nested_vt = "VT-x_Enabled_Nested", RAX W1 = 0x1BADB002
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test modul exit 0; test_core #2 pass

### AC-3: cgroup v2 Controller Menerapkan 3 Limit (CPU/Mem/IO) ke 1 Container
- **Type**: `rule`
- **Given**: `modul/resource_cgroup_v2.wya` dimuat
- **When**: Buat cgroup "widya_db_container", set CPU.max=50000, memory.max=536870912 (512MB), io.max=104857600 (100MB/s), tambah PID=1234, terapkan_limit() → baca_statistik()
- **Then**: statistik.enforcement_terapkan = benar, max_mem = 512, daftar_pid mengandung 1234
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test modul exit 0; test_core #3 pass

### AC-4: Wasmtime/Wasmer Runtime Abstract Muat app.wasm dan Panggil Fungsi main() return 0
- **Type**: `rule`
- **Given**: `modul/wasmtime_wasmer_runtime.wya` dimuat
- **When**: Buat WasmRuntime vendor "Wasmtime", fitur diaktifkan: wasi_snapshot_preview1 + threads, muat "app.wasm", instantiate, panggil main([])
- **Then**: exit_code = 0, feature_count = 2, instantiate_status = "READY"
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test exit 0; test_core #4 pass

### AC-5: SGX Enclave Simulasi Ecall Sign + Attestasi Lokal Valid
- **Type**: `rule`
- **Given**: `modul/sgx_enclave_simulasi.wya` dimuat
- **When**: Buat SGXEnclave "signer_widya" dengan AES256 key, ecall "sign_tx_hash" dengan hash = 0xDEADBEEF, jalankan sgx_attestasi_local()
- **Then**: signature panjang >= 64 hex chars, attestasi.report_mac_valid = benar, enclave_state = "INITIALIZED_SECURE"
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test exit 0; test_core #5 pass

### AC-6: CUDA FFI Runtime Malloc + Kernel Launch + Sinkronkan Matmul 256x256 OK
- **Type**: `rule`
- **Given**: `modul/cuda_ffi_runtime.wya` dimuat
- **When**: Buat CUDARuntime 12.2, deviceCount = 1, cudaMalloc 256MB, memcpy H2D matrix 256x256, kernel_launch grid=(4,4) block=(64,1) "matmul_float32", cudaSinkronkan return "CUDA_SUCCESS"
- **Then**: device_count >= 1, sync_result = "CUDA_SUCCESS", malloc_size_mb = 256
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test exit 0; test_core #6 pass

### AC-7: OpenCL Platform Get + Context GPU + NDRange Kernel VecAdd 1024 Sukses
- **Type**: `rule`
- **Given**: `modul/opencl_wrapper_platform.wya` dimuat
- **When**: Dapatkan platform index 0 (NVIDIA OpenCL 3.0), buat context device_type GPU, cmd_queue, enqueue_ndrange_kernel global_work_size = 1024
- **Then**: platform_nama mengandung "OpenCL", context_devices = ["GPU"], global_work = 1024
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test exit 0; test_core #7 pass

### AC-8: HSM PKCS#11 Generate RSA2048 + Sign Hash + Verify = True
- **Type**: `rule`
- **Given**: `modul/hsm_pkcs11_emulasi.wya` dimuat
- **When**: HSM login slot 0 SO_PIN = "12345678", generate_key_pair RSA2048 label "WIDYA_SIGN_KEY_01", sign hash = SHA256("PAYMENT_123"), verify signature
- **Then**: key_alg = "RSA2048", signature length >= 256, verify_hasil = benar
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test exit 0; test_core #8 pass

### AC-9: FPGA Config Program Full Bitstream + Partial Reconfig Region Sukses
- **Type**: `rule`
- **Given**: Modul `fpga_bitstream_config.wya` dimuat
- **When**: Board Xilinx XC7A35T via JTAG, program bitstream "kernel_accel.bit", partial reconfig region "fft_256" dengan file "fft_partial.bit", baca register 0x00 = 0xDEADBEEF, tulis register 0x04 = 42, baca ulang = 42
- **Then**: program_status = "DONE", partial_region = "CONFIGURED", reg_0x04 = 42
- **Pass Condition**: 3 assertions TRUE
- **Evidence**: Self-test modul exit 0; test_fs_9_fpga_partial_reconfig pass

### AC-10: Demo 168 Semua Layer Exit 0 dan Banner "WIDYA 9 LAYER 100% SIAP" Muncul
- **Type**: `rule`
- **Given**: `contoh/168_semua_layer_100persen.wya` dan semua dependency modul tersedia di folder modul/
- **When**: `widya jalankan contoh/168_semua_layer_100persen.wya`
- **Then**: Exit code = 0, stdout mengandung banner string "WIDYA 9 LAYER 100% SIAP", 15 step assertions count = 15 sukses
- **Pass Condition**: 3 observable TRUE
- **Evidence**: CLI output; test_fs_12_demo168 pass

### AC-11: Rust Cargo Aggregate Test (2 Suite) Total ≥ 22 Test Passed 0 Failed
- **Type**: `rule`
- **Given**: Dua file test dibuat: tests/test_all9layer_core.rs (8) + tests/test_all9layer_full_stack.rs (14)
- **When**: Menjalankan `cargo test --test test_all9layer_core --test test_all9layer_full_stack`
- **Then**: TOTAL running >= 22; passed >= 22; failed = 0
- **Pass Condition**: failed = 0 AND total passed >=22
- **Evidence**: Cargo test output aggregate

### AC-12: Integrasi L2→L3→L6 Cross Layer (Syscall IDT → cgroup Container → CUDA Dispatch Fallback)
- **Type**: `rubric`
- **Dimension**: Cross-layer integration seamlessness dari L2 (interrupt) → L3 (container limits) → L6 (CUDA training dispatch) melalui widya_os append + ai_federasi append
- **Scale**: 1-5
- **Anchors**: 1 = 0 interop, masing-masing standalone; 3 = ada interop 2 dari 3 layer tapi tidak end-to-end; 5 = workflow E2E: interrupt timer 0x20 trigger syscall, syscall memeriksa cgroup limit container training, jika CPU tersedia → dispatch ai_federasi FedAvg CUDAShardNode, banner "INTEGRASI_L2_L3_L6_OK" dicetak
- **Pass Threshold**: >= 4
- **Evidence**: Demo 168 step 11 (L2→L3→L6 interop) lulus; test_fs_10_widyaos_append_syscall + test_fs_11_aifed_cuda_dispatch keduanya pass

### AC-13: Convention & Backward Compatibility 0 Breaking Change
- **Type**: `rubric`
- **Dimension**: Kualitas kode dan tidak adanya breaking change ke semua file existing 165/166/167/widya_os/ai_federasi
- **Scale**: 1-5
- **Anchors**: 1 = file existing DIUBAH / DIHAPUS API; 3 = append file tapi nama API ambigu; 5 = 5 modul BARU dibuat di file sendiri (tidak sentuh modul existing), 2 append DI AKHIR FILE (tidak edit baris lama API), 0 Cargo deps baru, 0 perubahan src/, convention variabel ID + standard EN untuk industri, self-contained setiap modul.
- **Pass Threshold**: >= 4
- **Evidence**: diff review antara before/after (hanya 5 file baru + 2 append + 1 demo + 2 test baru); tidak ada penghapusan; tidak ada penambahan deps Cargo.toml; 7 self-test modul exit 0 (100%)
