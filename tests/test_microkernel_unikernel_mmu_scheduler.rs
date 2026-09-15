use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave55_microkernel_capability_and_ipc() {
    let code = r#"
        var cspace = MikrokernelCapSpace();
        cap_buat_endpoint(cspace, "ep_proc1", "KIRIM_TERIMA");
        cap_kirim_pesan_ipc(cspace, "ep_proc1", {"data": 1234, "cmd": "PING"});

        var msg = cap_terima_pesan_ipc(cspace, "ep_proc1");
        var cek_ipc = msg["data"] == 1234.0 dan msg["cmd"] == "PING";

        cap_cabut(cspace, "ep_proc1");

        kembalikan cek_ipc;
    "#;
    let res = jalankan(code).expect("Capability IPC test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave55_tcb_scheduler() {
    let code = r#"
        var sched = PenjadwalMikrokernel(10.0);
        var t1 = tcb_buat("TaskA", 1, 20.0, "SIAP");
        var t2 = tcb_buat("TaskB", 1, 10.0, "SIAP");
        penjadwal_tambah_tcb(sched, t1);
        penjadwal_tambah_tcb(sched, t2);

        var first = penjadwal_ambil_tugas_aktif(sched);
        var run1 = penjadwal_langkah_siklus(sched);
        var second = penjadwal_ambil_tugas_aktif(sched);
        var run2 = penjadwal_langkah_siklus(sched);

        var cek = first == "TaskA" dan second == "TaskB";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("TCB Scheduler test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave55_unikernel_and_mmu() {
    let code = r#"
        var uni = CitraUnikernel("MiniKernel", "x86_64");
        unikernel_sisip_modul(uni, "DriverVGA", 8.0, "0x00100000");
        var hdr = unikernel_bangun_header_boot(uni, "0x00100000");
        var valid_mb = unikernel_verifikasi_multiboot(uni);

        var mmu = TabelHalamanMMU(4.0);
        mmu_petakan_halaman(mmu, "0xC0000000", "0x00100000", "RWX");
        var trans = mmu_terjemahkan_alamat(mmu, "0xC0000000");

        var cek = valid_mb dan trans["alamat_fisik"] == "0x00100000" dan hdr["os_name"] == "MiniKernel";
        kembalikan cek;
    "#;
    let res = jalankan(code).expect("Unikernel & MMU test failed");
    assert_eq!(res, Value::Bool(true));
}
