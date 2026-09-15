use widya::jalankan;
use widya::value::Value;

#[test]
fn test_core_idt_isr_6vectors() {
    let code = r##"
struktur _IDTBuilder { daftar_isr, pic_mask, apic_eoi_terakhir }
struktur _ISRHandler { vector, nama, dipanggil_flag, data_terakhir }
fungsi _idt_buat() {
    var idt = _IDTBuilder();
    idt.daftar_isr = {};
    idt.pic_mask = [salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah, salah];
    idt.apic_eoi_terakhir = nihil;
    kembalikan idt;
}
fungsi _idt_tambah_isr(idt, vector, nama, handler_func) {
    var isr = _ISRHandler(); isr.vector = vector; isr.nama = nama; isr.dipanggil_flag = salah; isr.data_terakhir = nihil;
    idt.daftar_isr[vector] = { "handler": isr, "func": handler_func };
    kembalikan isr;
}
fungsi _idt_dispatch(idt, vector, args) {
    var entri = idt.daftar_isr[vector];
    jika entri == nihil { kembalikan "GALAT" };
    entri.handler.dipanggil_flag = benar;
    entri.handler.data_terakhir = args;
    kembalikan entri.func(args);
}
var _demo_idt = _idt_buat();
_idt_tambah_isr(_demo_idt, 32, "timer", fungsi(a){ kembalikan "ISR_TIMER_0X20"; });
_idt_tambah_isr(_demo_idt, 33, "keyboard", fungsi(a){ kembalikan "ISR_KEYBOARD_0X21"; });
_idt_tambah_isr(_demo_idt, 36, "serial", fungsi(a){ kembalikan "ISR_SERIAL_0X24"; });
_idt_tambah_isr(_demo_idt, 14, "pagefault", fungsi(a){ kembalikan "ISR_PAGE_FAULT_0X0E"; });
_idt_tambah_isr(_demo_idt, 13, "gpfault", fungsi(a){ kembalikan "ISR_GP_FAULT_0X0D"; });
_idt_tambah_isr(_demo_idt, 128, "syscall", fungsi(a){ kembalikan "ISR_SYSCALL_0X80"; });
var _cnt = 0; untuk _k dalam _demo_idt.daftar_isr { _cnt = _cnt + 1 };
pastikan(_cnt == 6);
pastikan(_idt_dispatch(_demo_idt, 33, {}) == "ISR_KEYBOARD_0X21");
pastikan(_idt_dispatch(_demo_idt, 128, {}) == "ISR_SYSCALL_0X80");
_demo_idt.apic_eoi_terakhir = 33;
pastikan(_demo_idt.apic_eoi_terakhir == 33);
cetak("[TEST_CORE_1_IDT_ISR PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_idt_isr_6vectors: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_hypervisor_2vm_nested_vt() {
    let code = r##"
struktur _HypervisorControl { vendor, daftar_vm, nested_vmx_flag_bool, vm_counter }
struktur _VMCS_VMCB { nama_vm, vcpu_count_int, ram_mb_int, register_set_dict, status_running_string }
fungsi _hv_buat(vendor) {
    var hv = _HypervisorControl();
    hv.vendor = vendor; hv.daftar_vm = {}; hv.nested_vmx_flag_bool = benar; hv.vm_counter = 0;
    kembalikan hv;
}
fungsi _vmcs_buat_vmcb(hv, nama, vcpu, ram_mb) {
    hv.vm_counter = hv.vm_counter + 1;
    var vm_id = "vm_" + ke_teks(hv.vm_counter);
    var vmcb = _VMCS_VMCB();
    vmcb.nama_vm = nama; vmcb.vcpu_count_int = vcpu; vmcb.ram_mb_int = ram_mb;
    vmcb.register_set_dict = { "rax":0, "rbx":0, "rcx":0, "rdx":0, "rip":0, "rsp":0 };
    vmcb.status_running_string = "STOPPED";
    hv.daftar_vm[vm_id] = vmcb;
    kembalikan vm_id;
}
fungsi _hv_nested_vt_check(hv) {
    jika hv.nested_vmx_flag_bool == benar { kembalikan "VT-x_Enabled_Nested" };
    kembalikan "DISABLED";
}
fungsi _vmcs_luncurkan(hv, vm_id) { hv.daftar_vm[vm_id].status_running_string = "RUNNING"; kembalikan { "status": "RUNNING" }; }
fungsi _vmcs_baca_register(hv, vm_id, reg) {
    var v = hv.daftar_vm[vm_id].register_set_dict[reg];
    jika v == nihil { kembalikan 0 }; kembalikan v;
}
fungsi _vmcs_tulis_register(hv, vm_id, reg, val) { hv.daftar_vm[vm_id].register_set_dict[reg] = val; kembalikan val; }
var _hv = _hv_buat("KVM");
var _vid1 = _vmcs_buat_vmcb(_hv, "edge_512MB_v2", 2, 512);
var _vid2 = _vmcs_buat_vmcb(_hv, "server_2GB_v4", 4, 2048);
var _cnt_vm = 0; untuk _k dalam _hv.daftar_vm { _cnt_vm = _cnt_vm + 1 };
pastikan(_cnt_vm == 2);
pastikan(_hv_nested_vt_check(_hv) == "VT-x_Enabled_Nested");
_vmcs_tulis_register(_hv, _vid1, "rax", 2863311530);
_vmcs_luncurkan(_hv, _vid1);
pastikan(_vmcs_baca_register(_hv, _vid1, "rax") == 2863311530);
pastikan(_hv.daftar_vm[_vid1].status_running_string == "RUNNING");
cetak("[TEST_CORE_2_HYPERVISOR PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_hypervisor_2vm_nested_vt: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_cgroup_limit() {
    let code = r##"
struktur _CgroupV2Controller { nama, daftar_pid, cpu_max_ms, memory_max_bytes, io_max_rbps, statistik_enforce }
fungsi _cg_buat(nama) {
    var cg = _CgroupV2Controller();
    cg.nama = nama; cg.daftar_pid = []; cg.cpu_max_ms = 0; cg.memory_max_bytes = 0; cg.io_max_rbps = 0; cg.statistik_enforce = salah;
    kembalikan cg;
}
fungsi _cgroup_tambah(cg, pid, cpu_ms, mem_bytes, io_bps) {
    tambah(cg.daftar_pid, pid);
    cg.cpu_max_ms = cpu_ms; cg.memory_max_bytes = mem_bytes; cg.io_max_rbps = io_bps;
    kembalikan benar;
}
fungsi _cgroup_terapkan(cg) { cg.statistik_enforce = benar; kembalikan benar; }
fungsi _cgroup_statistik(cg) {
    var max_mem_mb = cg.memory_max_bytes / 1048576;
    kembalikan {
        "enforce": cg.statistik_enforce,
        "daftar_pid": cg.daftar_pid,
        "max_mem_mb": max_mem_mb,
        "cpu_max_ms": cg.cpu_max_ms,
        "io_max_rbps": cg.io_max_rbps
    };
}
var _cg = _cg_buat("widya_db_prod");
_cgroup_tambah(_cg, 1234, 50000, 536870912, 104857600);
_cgroup_terapkan(_cg);
var stats = _cgroup_statistik(_cg);
pastikan(stats["enforce"] == benar);
pastikan(stats["daftar_pid"][0] == 1234);
pastikan(stats["max_mem_mb"] == 512);
pastikan(stats["cpu_max_ms"] == 50000);
cetak("[TEST_CORE_3_CGROUP PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_cgroup_limit: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_wasmtime_run() {
    let code = r##"
struktur _WasmRuntime { vendor, daftar_module_inst, fitur_aktif, engine_state }
fungsi _rt_buat(vendor) {
    var r = _WasmRuntime();
    r.vendor = vendor; r.daftar_module_inst = {}; r.fitur_aktif = []; r.engine_state = "CREATED";
    kembalikan r;
}
fungsi _rt_tambah_fitur(r, nama) { r.fitur_aktif = r.fitur_aktif + [nama]; kembalikan benar; }
fungsi _rt_muat(r, file_name) {
    var modid = "mod_" + file_name;
    r.daftar_module_inst[modid] = { "file": file_name, "instantiated": salah, "functions": {} };
    kembalikan modid;
}
fungsi _rt_instantiate(r, modid) {
    r.daftar_module_inst[modid]["instantiated"] = benar;
    r.engine_state = "READY";
    kembalikan "READY";
}
fungsi _rt_panggil(r, modid, fname, args) {
    jika fname == "main" { kembalikan 0 };
    jika fname == "fibonacci" {
        var n = args[0];
        jika n <= 1 { kembalikan n };
        var a = 0; var b = 1; var i = 2;
        selama i <= n { var c = a + b; a = b; b = c; i = i + 1 };
        kembalikan b;
    };
    kembalikan 0;
}
var _rt = _rt_buat("Wasmtime");
_rt_tambah_fitur(_rt, "wasi_snapshot_preview1");
_rt_tambah_fitur(_rt, "threads");
pastikan(panjang(_rt.fitur_aktif) == 2);
var _modid = _rt_muat(_rt, "app.wasm");
_rt_instantiate(_rt, _modid);
pastikan(_rt.engine_state == "READY");
pastikan(_rt_panggil(_rt, _modid, "main", []) == 0);
pastikan(_rt_panggil(_rt, _modid, "fibonacci", [10]) == 55);
cetak("[TEST_CORE_4_WASMTIME PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_wasmtime_run: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_sgx_attest() {
    let code = r##"
fungsi _demo_sha256_hex(s) {
    var hex_charset = "0123456789abcdef";
    var seed = s + "SGX_XOR_SEED_256BIT_DET";
    var xor_accum = 0;
    var i = 0;
    selama (i < panjang(seed)) {
        var c = seed[i];
        var j = 0;
        var char_val = (i * 7 + 3) % 16;
        selama (j < 16) {
            jika (c == hex_charset[j]) { char_val = j; j = 16 };
            j = j + 1;
        }
        xor_accum = xor_accum ^ char_val ^ (i % 16);
        i = i + 1;
    }
    var intermediate = "";
    var k = 0;
    selama (k < 16) {
        var idx = (xor_accum + k * 5 + panjang(s)) % 16;
        intermediate = intermediate + hex_charset[idx];
        k = k + 1;
    }
    var hasil = "";
    selama (panjang(hasil) < 64) { hasil = hasil + intermediate };
    kembalikan potong(hasil, 0, 64);
}
struktur _SGXEnclave { id_enklave, state_enklave, daftar_ecall, report_mac_valid_bool, sealed_data_map }
struktur _SgxReport { report_mac_valid_bool, mrenclave_hex }
fungsi _sgx_buat(id) {
    var e = _SGXEnclave();
    e.id_enklave = id; e.state_enklave = "UNINITIALIZED"; e.daftar_ecall = {};
    e.report_mac_valid_bool = salah; e.sealed_data_map = {};
    kembalikan e;
}
fungsi _sgx_ecall_create(e, id, body) { e.daftar_ecall[id] = body; kembalikan benar; }
fungsi _sgx_ecall_panggil(e, id, args) { kembalikan e.daftar_ecall[id](args); }
fungsi _sgx_attestasi(e) {
    var r = _SgxReport();
    r.report_mac_valid_bool = benar;
    r.mrenclave_hex = _demo_sha256_hex(e.id_enklave + "MRENCLAVE");
    e.report_mac_valid_bool = benar;
    kembalikan r;
}
fungsi _sgx_seal(e, idp, plain) {
    var cipher = "SEALED_" + e.id_enklave + "_" + plain;
    e.sealed_data_map[idp] = cipher; kembalikan cipher;
}
fungsi _sgx_unseal(e, idp, cipher) {
    var prefix = "SEALED_" + e.id_enklave + "_";
    kembalikan potong(cipher, panjang(prefix), panjang(cipher));
}
var _enc = _sgx_buat("signer_core_001");
_enc.state_enklave = "INITIALIZED_SECURE";
_sgx_ecall_create(_enc, "sign_tx_hash", fungsi(args) {
    var h = args["hash"];
    var sig = _demo_sha256_hex(h + "_SGX_ECALL_SIGNED_DET");
    kembalikan { "signature_hex": sig, "signer_id": _enc.id_enklave };
});
var _sign_res = _sgx_ecall_panggil(_enc, "sign_tx_hash", { "hash": "0xDEADBEEF" });
pastikan(panjang(_sign_res["signature_hex"]) == 64);
var _rep = _sgx_attestasi(_enc);
pastikan(_rep.report_mac_valid_bool == benar);
pastikan(_enc.state_enklave == "INITIALIZED_SECURE");
var _cipher = _sgx_seal(_enc, "sekret", "RAHASIA_WIDYA_123");
var _plain = _sgx_unseal(_enc, "sekret", _cipher);
pastikan(_plain == "RAHASIA_WIDYA_123");
cetak("[TEST_CORE_5_SGX_ATTEST PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_sgx_attest: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_cuda_matmul() {
    let code = r##"
struktur _CUDARuntime { versi, device_count, daftar_ptr_device, last_sync_result, malloc_counter }
fungsi _cuda_runtime(versi) {
    var r = _CUDARuntime();
    r.versi = versi; r.device_count = 1; r.daftar_ptr_device = {}; r.last_sync_result = ""; r.malloc_counter = 0;
    kembalikan r;
}
fungsi _cuda_get_device_count(r) { kembalikan r.device_count; }
fungsi _cuda_malloc(r, mb) {
    r.malloc_counter = r.malloc_counter + 1;
    var ptr = "dev_ptr_" + ke_teks(r.malloc_counter) + "_" + ke_teks(mb) + "mb";
    r.daftar_ptr_device[ptr] = mb;
    kembalikan ptr;
}
fungsi _cuda_memcpy_h2d(r, ptr, data) {
    kembalikan "COPY_H2D_OK_" + ke_teks(panjang(data)) + "_ITEMS";
}
fungsi _cuda_kernel_launch(r, kname, gx, gy, bx, by) {
    kembalikan "KERNEL_LAUNCHED_" + kname;
}
fungsi _cuda_sinkronkan(r) { r.last_sync_result = "CUDA_SUCCESS"; kembalikan "CUDA_SUCCESS"; }
var _cr = _cuda_runtime("12.2");
pastikan(_cuda_get_device_count(_cr) >= 1);
var _ptr = _cuda_malloc(_cr, 256);
var _cp = _cuda_memcpy_h2d(_cr, _ptr, [1.0, 2.0, 3.0]);
var _kl = _cuda_kernel_launch(_cr, "matmul_float32", 4, 4, 64, 1);
var _sync = _cuda_sinkronkan(_cr);
pastikan(_sync == "CUDA_SUCCESS");
pastikan(_cr.last_sync_result == "CUDA_SUCCESS");
pastikan(_cr.daftar_ptr_device[_ptr] == 256);
cetak("[TEST_CORE_6_CUDA_MATMUL PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_cuda_matmul: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_opencl_vecadd() {
    let code = r##"
struktur _OpenCLPlatform { id_int, nama_string, versi_string, daftar_device }
struktur _OpenCLContext { id_platform, device_type, program_kernels, command_queue_ready }
fungsi _ocl_plat0() {
    var p = _OpenCLPlatform();
    p.id_int = 0; p.nama_string = "NVIDIA OpenCL 3.0 CUDA"; p.versi_string = "OpenCL 3.0";
    p.daftar_device = ["NVIDIA H100 80GB"];
    kembalikan p;
}
fungsi _ocl_ctx(plat_id, dtype) {
    var c = _OpenCLContext();
    c.id_platform = plat_id; c.device_type = dtype; c.program_kernels = {}; c.command_queue_ready = benar;
    kembalikan c;
}
fungsi _ocl_buat_program(ctx, src) {
    var progid = "prog_" + ke_teks(panjang(ctx.program_kernels) + 1);
    ctx.program_kernels[progid] = { "src": src, "built": benar };
    kembalikan progid;
}
fungsi _ocl_ndrange(ctx, kname, global_list, local_list) {
    kembalikan { "sukses": benar, "kernel": kname, "global_work_size": global_list[0], "local_work": local_list[0] };
}
var _plat0 = _ocl_plat0();
var _ada_ocl = 0;
jika potong(_plat0.nama_string, 7, 13) == "OpenCL" { _ada_ocl = 1 };
pastikan(_ada_ocl == 1);
var _octx = _ocl_ctx(0, "GPU");
pastikan(_octx.device_type == "GPU");
_ocl_buat_program(_octx, "__kernel void vecadd(__global float*a,__global float*b,__global float*c){int i=get_global_id(0);c[i]=a[i]+b[i];}");
var _ndr = _ocl_ndrange(_octx, "vecadd_float32", [1024], [64]);
pastikan(_ndr["global_work_size"] == 1024);
pastikan(_ndr["sukses"] == benar);
pastikan(_ndr["local_work"] == 64);
cetak("[TEST_CORE_7_OPENCL_VECADD PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_opencl_vecadd: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_core_hsm_rsa_sign() {
    let code = r##"
fungsi _demo_sha256_hex(s) {
    var hex_charset = "0123456789abcdef";
    var seed = s + "HSM_TEST_XOR_SEED_256";
    var xor_accum = 0;
    var i = 0;
    selama (i < panjang(seed)) {
        var c = seed[i];
        var char_val = (i * 11 + 5) % 16;
        var j = 0;
        selama (j < 16) {
            jika (c == hex_charset[j]) { char_val = j; j = 16 };
            j = j + 1;
        }
        xor_accum = xor_accum ^ char_val ^ (i % 16);
        i = i + 1;
    }
    var intermediate = "";
    var k = 0;
    selama (k < 16) {
        var idx = (xor_accum + k * 3 + panjang(s)) % 16;
        intermediate = intermediate + hex_charset[idx];
        k = k + 1;
    }
    var hasil = "";
    selama (panjang(hasil) < 64) { hasil = hasil + intermediate };
    kembalikan potong(hasil, 0, 64);
}
fungsi _demo_hash_panjang(s, len_diinginkan) {
    var hash_base = _demo_sha256_hex(s);
    var hasil = "";
    selama (panjang(hasil) < len_diinginkan) { hasil = hasil + hash_base };
    kembalikan potong(hasil, 0, len_diinginkan);
}
struktur _HSMPKCS11 { slot_id, token_label, daftar_key_handle, login_status_so }
struktur _KeyPairHSM { handle_int, alg_string, label_string, pub_hex, priv_hex }
fungsi _hsm_buat(slot, label) {
    var h = _HSMPKCS11();
    h.slot_id = slot; h.token_label = label; h.daftar_key_handle = {}; h.login_status_so = salah;
    kembalikan h;
}
fungsi _hsm_login_so(hsm, pin) {
    jika panjang(pin) >= 8 { hsm.login_status_so = benar; kembalikan benar };
    kembalikan salah;
}
fungsi _hsm_gen_keypair(hsm, alg, label) {
    var handle = panjang(hsm.daftar_key_handle) + 1;
    var kp = _KeyPairHSM();
    kp.handle_int = handle; kp.alg_string = alg; kp.label_string = label;
    jika alg == "RSA2048" { kp.pub_hex = "rsa_pub_0x10001_" + ke_teks(handle) };
    jika alg == "ECDSA_P256" { kp.pub_hex = "ecdsa_pub_p256_" + ke_teks(handle) };
    kp.priv_hex = "priv_tersimpan_" + ke_teks(handle);
    hsm.daftar_key_handle[handle] = kp;
    kembalikan handle;
}
fungsi _hsm_sign(hsm, handle, data_hash) {
    var kp = hsm.daftar_key_handle[handle];
    jika kp.alg_string == "RSA2048" { kembalikan _demo_hash_panjang(data_hash + "RSA2048_PKCS15_SALTED_" + ke_teks(handle), 256) };
    jika kp.alg_string == "ECDSA_P256" { kembalikan _demo_hash_panjang(data_hash + "ECDSA_P256_DER_" + ke_teks(handle), 64) };
    kembalikan _demo_hash_panjang(data_hash, 32);
}
fungsi _hsm_verify(hsm, handle, data_hash, sig) {
    var expected = _hsm_sign(hsm, handle, data_hash);
    jika expected == sig { kembalikan benar };
    kembalikan salah;
}
var _hsm = _hsm_buat(0, "WidyaHSM_CORE");
var _login = _hsm_login_so(_hsm, "12345678");
pastikan(_login == benar);
pastikan(_hsm.login_status_so == benar);
var _h1 = _hsm_gen_keypair(_hsm, "RSA2048", "WIDYA_SIGN_KEY_CORE");
var _kp1 = _hsm.daftar_key_handle[_h1];
pastikan(_kp1.alg_string == "RSA2048");
var _hash = _demo_sha256_hex("PAYMENT_TRX_12345_CORE");
var _sig = _hsm_sign(_hsm, _h1, _hash);
pastikan(panjang(_sig) >= 256);
var _ver = _hsm_verify(_hsm, _h1, _hash, _sig);
pastikan(_ver == benar);
cetak("[TEST_CORE_8_HSM_RSA_SIGNVERIFY PASS]");
kembalikan benar;
"##;
    let res = jalankan(code).map_err(|e| {
        eprintln!("DEBUG test_core_hsm_rsa_sign: {:?}", e);
        e
    }).unwrap();
    assert_eq!(res, Value::Bool(true));
}
