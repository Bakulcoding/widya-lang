use widya::jalankan;
use widya::value::Value;

#[test]
fn test_wave38_dma_buffer_operations() {
    let code = r#"
        var dma = BufferDMA(1024)
        dma_tulis_u8(dma, 0, 255)
        dma_tulis_u8(dma, 1, 128)
        var b0 = dma_baca_u8(dma, 0)
        var b1 = dma_baca_u8(dma, 1)

        dma_tulis_u32(dma, 16, 12345678)
        var u32_val = dma_baca_u32(dma, 16)
        var size = dma_ukuran(dma)

        kembalikan b0 == 255 dan b1 == 128 dan u32_val == 12345678 dan size == 1024
    "#;
    let res = jalankan(code).expect("DMA test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave38_atomic_operations() {
    let code = r#"
        var atom = NilaiAtomik(50)
        var read1 = atomik_baca(atom)
        var prev = atomik_tambah(atom, 25)
        var read2 = atomik_baca(atom)

        var cas_fail = atomik_banding_dan_tukar(atom, 50, 100)
        var cas_ok = atomik_banding_dan_tukar(atom, 75, 100)
        var final_val = atomik_baca(atom)

        kembalikan read1 == 50 dan prev == 50 dan read2 == 75 dan cas_fail == salah dan cas_ok == benar dan final_val == 100
    "#;
    let res = jalankan(code).expect("Atomic ops test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave38_memory_arena() {
    let code = r#"
        var arena = ArenaMemori(512)
        var p1 = arena_alokasi(arena, 32)
        var p2 = arena_alokasi(arena, 64)
        var used1 = arena_penggunaan_byte(arena)

        arena_reset(arena)
        var used2 = arena_penggunaan_byte(arena)
        var p3 = arena_alokasi(arena, 16)

        kembalikan p1 == 0 dan p2 == 32 dan used1 == 96 dan used2 == 0 dan p3 == 0
    "#;
    let res = jalankan(code).expect("Memory Arena test failed");
    assert_eq!(res, Value::Bool(true));
}

#[test]
fn test_wave38_sys_socket() {
    let code = r#"
        var sock = SoketJaringan("tcp")
        soket_ikat(sock, "127.0.0.1", 9090)
        soket_dengarkan(sock, 10)

        var client = soket_terima_koneksi(sock)
        var sent = soket_kirim_bytes(client, "PING")
        var recv = soket_baca_bytes(client, 4)

        kembalikan sent == 4 dan panjang(recv) == 4
    "#;
    let res = jalankan(code).expect("SysSocket test failed");
    assert_eq!(res, Value::Bool(true));
}
