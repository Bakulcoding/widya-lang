# Tutorial 17 — WidyaOS: Kernel Abstraction Layer (KAL) + Scheduler Heterogen

**Target**: Membangun OS sendiri dengan modul `widya_os.wya` — capability-based syscall, KAL security gate, scheduler heterogen DAG untuk edge/server, dan tutup gap L2 (IDT ISR + Hypervisor Nested VT) + L3 (Cgroup V2 + Wasmtime).
**Level**: Enterprise ⭐⭐⭐
**Waktu**: ~60 menit
**File terkait**: [modul/widya_os.wya](../modul/widya_os.wya) | [modul/scheduler_heterogen.wya](../modul/scheduler_heterogen.wya) | [modul/interrupt_idt_apic.wya](../modul/interrupt_idt_apic.wya) | [modul/hypervisor_kvm_xen.wya](../modul/hypervisor_kvm_xen.wya) | [modul/resource_cgroup_v2.wya](../modul/resource_cgroup_v2.wya) | [modul/wasmtime_wasmer_runtime.wya](../modul/wasmtime_wasmer_runtime.wya) | [contoh/165_widyaos_boot.wya](../contoh/165_widyaos_boot.wya) | [tests/test_widyaos_full_stack.rs](../tests/test_widyaos_full_stack.rs)

---

## Arsitektur WidyaOS 7 Lapisan Internal

| Lapisan OS | Modul Pelaksana | Fungsi Utama |
|-----------|----------------|-------------|
| L0 Bootloader | widya_os.wya `os_boot_stage0/1/2` | Cek memori, GDT/IDT init, jump ke kernel_main |
| L1 Kernel + Capability KAL | widya_os.wya `kernel_*` + CapabilityKAL struct | Syscall hanya via capability token, tidak ada akses langsung hardware dari userland |
| L2 Interrupt Controller | interrupt_idt_apic.wya | 6 ISR vector: timer 0x20, keyboard 0x21, serial 0x24, Page Fault 0x0E, GP Fault 0x0D, Syscall 0x80 |
| L2 Hypervisor | hypervisor_kvm_xen.wya | VMCS/VMCB, 2VM tamu edge+server, Nested VT-x |
| L3 Resource Controller | resource_cgroup_v2.wya | cgroup v2: cpu.max, memory.max, io.max per PID |
| L3 Wasm Runtime Sandbox | wasmtime_wasmer_runtime.wya | User apps dikompilasi Wasm, sandbox linear memory, WASI + Threads |
| L4 Scheduler Heterogen DAG | scheduler_heterogen.wya | Task DAG dependency-aware, placement edge vs server berdasarkan atribut `#[profil]` |

---

## 1. Booting WidyaOS: Stage 0 → Stage 1 → Stage 2 → Userland

Jalankan boot demo:
```bash
widya jalankan modul\widya_os.wya
widya jalankan contoh\165_widyaos_boot.wya
```

Output akan menampilkan banner:
```
        __        ___    __  _____  ___  
       / /  ___  / _ \  / / /  _/ |/ / |
      / /  / _ \/ / | |/ /  / / /| |_  |
     / /__/  __/ /_/ / / /__/ / / /_/ /  
    /_____\___/\____/____/___/_/ /____/   
  Versi OS Kernel Widya v1.0.0 (Build 20260915)
  Profil Boot: HYBRID_EDGE_SERVER_64CORE_128GB
  [Boot Stage 0] OK — Checksum bootloader = 0x9E3779B9
  [Boot Stage 1] OK — GDT + Page Table 4 level mapping
  [Boot Stage 2] OK — Kernel Main init, 128GB DDR5 terdeteksi
  [Userland    ] OK — Init process PID 1, shell prompt siap
```

---

## 2. Capability KAL (Kernel Access Layer) — Syscall Aman

Ini **fitur keamanan PENTING** — semua akses ke kernel HARUS melalui object **CapabilityKAL**:

```widya
impor "widya_os.wya" sebagai os;

// 2 Capability token: level akses BEDA
misal cap_biasa = os.CapabilityKAL(
    "APLIKASI_USER_BIASA",
    1,   // level akses 1-5 (5 = superuser / root)
    [os.PerizinanSumberDaya("baca_file", 0, 1024)],  // cuma boleh baca s.d 1KB
    "APPROVED"
);

misal cap_admin = os.CapabilityKAL(
    "SISTEM_ADMIN_SUDO",
    5,
    [os.PerizinanSumberDaya("baca_file", 0, 100000000),
     os.PerizinanSumberDaya("tulis_file", 0, 100000000),
     os.PerizinanSumberDaya("modifikasi_kernel", 0, 1)],
    "APPROVED"
);
```

### Panggil Syscall dengan KAL Gate Check

```widya
// Test 1 — cap_biasa mencoba syscall modifikasi kernel (level >=3 butuh)
misal r1 = os.os_syscall(cap_biasa, "KERNEL_SET_TIME", {"time_baru": 1726350000});
cetak("cap_biasa modif kernel:", r1["status"]);   // ❌ "DENIED_CAPABILITY_INSUFFICIENT"

// Test 2 — cap_admin (level 5) bisa
misal r2 = os.os_syscall(cap_admin, "KERNEL_SET_TIME", {"time_baru": 1726350000});
cetak("cap_admin modif kernel:", r2["status"]);   // ✅ "OK_EPERM_GRANTED"
```

---

## 3. Scheduler Heterogen DAG (Placement Edge vs Server)

```widya
impor "scheduler_heterogen.wya" sebagai sch;

// Daftar node hybrid: 2 node edge (4C 8GB) + 3 node server (64C 128GB)
misal cluster = sch.cluster_buat("HPC_WIDYA_PROD");
sch.tambah_node(cluster, "edge-a", "edge",   4, 8192,  ["ai_quant_int4", "gpio"]);
sch.tambah_node(cluster, "edge-b", "edge",   4, 8192,  ["ai_quant_int8", "uart_coap"]);
sch.tambah_node(cluster, "srv-01", "server", 64, 131072,["cuda_ampere", "fpga_accel", "hsm_pkcs11"]);
sch.tambah_node(cluster, "srv-02", "server", 64, 131072,["sgx_enclave", "vectordb_1M"]);
sch.tambah_node(cluster, "srv-03", "server", 96, 262144,["gpu_h100_nvlink8"]);

// Task 1 — inferensi AI edge (cocok ditempatkan ke edge node)
misal t1 = sch.task_buat("infer_suhu_sensor", #[profil("edge")], {"flops": 0.002, "mem_mb": 12}, ["gpio", "uart_coap"]);
// Task 2 — training LLM 7B (cocok ditempatkan ke srv-03 H100)
misal t2 = sch.task_buat("train_llm_7b_epoch3", #[profil("server")], {"flops": 1e15, "mem_mb": 80*1024}, ["gpu_h100_nvlink8"]);
// Task 3 — vector search HNSW 1M (butuh vectordb_1M cap → srv-02)
misal t3 = sch.task_buat("rag_hnsw_query_1k_qps", #[profil("server")], {"flops":1e9, "mem_mb": 32*1024}, ["vectordb_1M"]);

// Build DAG: task3 depends_on task2 selesai, task2 depends_on task1 selesai
sch.dag_tambah_edge(cluster, t1, t2);
sch.dag_tambah_edge(cluster, t2, t3);

// Schedule & Jalankan
misal jadwal = sch.dag_schedule(cluster);
cetak("\n📋 Hasil Placement Scheduler:");
untuk t in jadwal["task"] {
    cetak("   Task:", t["nama"], "→ Node:", t["node_assigned"]);
}
// Expected output:
//   Task: infer_suhu_sensor → edge-a / edge-b
//   Task: train_llm_7b_epoch3 → srv-03
//   Task: rag_hnsw_query_1k_qps → srv-02
```

---

## 4. L2 Interrupt: IDT + 6 ISR Handler + APIC EOI

```widya
impor "interrupt_idt_apic.wya" sebagai intr;

misal idt = intr.idt_buat();
// 6 vector standard terdaftar otomatis: 32/33/36/14/13/128
cetak("Jumlah vector interrupt terdaftar:", intr.idt_hitung_vector(idt));  // ✅ 6

// Dispatch interrupt keyboard → handler otomatis set APIC EOI 0x21
misal dis1 = intr.isr_dispatch(idt, 33, {"scancode": 0x1E});  // vector 33 = keyboard
cetak("Dispatch keyboard → ISR:", dis1["nama_isr"]);       // "KEYBOARD_ISR_DIPANGGIL"
cetak("Last APIC EOI:", intr.apic_last_eoi(idt));          // ✅ 33

// Syscall via INT 0x80 (Linux legacy)
misal sys = intr.isr_dispatch(idt, 128, {"syscall_num": 4, "arg1": "Halo userland"});
cetak("INT 0x80 syscall write:", sys["data_terakhir"]["status"]);  // ✅ "SYSCALL_0X80_OK"
```

---

## 5. L2 Hypervisor: 2 VM + Nested VT-x

```widya
impor "hypervisor_kvm_xen.wya" sebagai hv;

misal host = hv.hv_buat("KVM");
misal vm1 = hv.vm_buat(host, "edge_512MB_v2", 2, 512);   // 2 vCPU, 512MB edge
misal vm2 = hv.vm_buat(host, "server_2GB_v4", 4, 2048); // 4 vCPU, 2048MB server HV

// Nested VT enable — bisa jalankan VM di dalam VM (VMX in VMX)
cetak("Nested VT check:", hv.hv_nested_vt_check(host));   // ✅ "VT-x_Enabled_Nested"

// Tulis register RAX vm1, launch, baca kembali
hv.vm_tulis_register(vm1, "RAX", 2863311530);  // 0xAAAAAAA0
hv.vm_launch(vm1);
cetak("VM1 state:", vm1["state"]);              // ✅ "RUNNING"
cetak("VM1 baca RAX cocok:", hv.vm_baca_register(vm1, "RAX") == 2863311530);  // ✅ benar
```

---

## 6. L3 Cgroup V2 + Wasmtime Runtime

### Cgroup V2 Resource Enforce
```widya
impor "resource_cgroup_v2.wya" sebagai cg;
misal widya_db_cg = cg.cg_buat("widya_db_produksi");
cg.cgroup_tambah_proses(widya_db_cg, 1234, 50000, 536870912, 104857600);  // pid 1234, cpu=50ms/100ms, mem=512MB, io=100MB/s
misal st = cg.cgroup_terapkan_limit(widya_db_cg);
cetak("Cgroup enforce status:", st);  // ✅ true
cetak("Max mem MB:", cg.cgroup_baca_statistik(widya_db_cg)["max_mem_mb"]); // ✅ 512
```

### Wasmtime Runner
```widya
impor "wasmtime_wasmer_runtime.wya" sebagai wr;
misal rt = wr.runtime_buat("Wasmtime");
wr.runtime_tambah_fitur(rt, "wasi_snapshot_preview1");
wr.runtime_tambah_fitur(rt, "threads");
cetak("Total fitur di-enable:", panjang(rt["fitur_aktif"]));  // ✅ 2

misal mod = wr.runtime_muat(rt, "aplikasi_produksi.wasm");
misal inst = wr.runtime_instantiate(rt, mod);
cetak("Engine state:", inst["engine_state"]);   // ✅ "READY"
cetak("Call main([]) exit code:", wr.mod_panggil(inst, "main", []));  // ✅ 0
cetak("Call fib(10) =", wr.mod_panggil(inst, "fibonacci_iter", [10])); // ✅ 55
```

---

## Verifikasi Full Stack

```bash
# Jalankan self-test 5 modul L2+L3 + append widya_os
widya jalankan modul\interrupt_idt_apic.wya
widya jalankan modul\hypervisor_kvm_xen.wya
widya jalankan modul\resource_cgroup_v2.wya
widya jalankan modul\wasmtime_wasmer_runtime.wya
widya jalankan modul\widya_os.wya

# Cargo test suite
cargo test --test test_widyaos_full_stack
```

Expected: **semua exit 0 / passed ✅**

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 16 — WidyaDB](./TUTORIAL_16_WIDYADB.md) |
➡️ **[Tutorial 18 — AI Federasi & RAG Pipeline](./TUTORIAL_18_AI_FEDERASI.md)**
