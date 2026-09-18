# 📚 Buku Panduan Komprehensif Bahasa Pemrograman Widya-Lang
**Versi: 0.1.0-Enterprise (Industrial & Production Grade)**  
*Bahasa Pemrograman Modern Berbasis Bahasa Indonesia untuk Sistem, Web, Mobile, AI, dan Skala Enterprise.*

---

## 📑 Daftar Isi
1. [Pengantar & Keunggulan Widya-Lang](#1-pengantar--keunggulan-widya-lang)
2. [Instalasi & Antarmuka CLI](#2-instalasi--antarmuka-cli)
3. [Sintaks Dasar & Tipe Data](#3-sintaks-dasar--tipe-data)
4. [Sistem Tipe Statis & Generics](#4-sistem-tipe-statis--generics)
5. [Struktur Data, Sifat (Traits), & Pemrograman Berorientasi Objek](#5-struktur-data-sifat-traits--pemrograman-berorientasi-objek)
6. [Pencocokan Pola (Pattern Matching)](#6-pencocokan-pola-pattern-matching)
7. [Pengembangan Web & REST API](#7-pengembangan-web--rest-api)
8. [Pengembangan Aplikasi Mobile (Android & iOS)](#8-pengembangan-aplikasi-mobile-android--ios)
9. [C/Native Foreign Function Interface (FFI)](#9-cnative-foreign-function-interface-ffi)
10. [Audit Keamanan & Pencegahan Kerentanan](#10-audit-keamanan--pencegahan-kerentanan)
11. [Manajemen Memori & Profiler](#11-manajemen-memori--profiler)
12. [Widya Package Manager (WPM) & Lockfile](#12-widya-package-manager-wpm--lockfile)
13. [Perkakas Developer (LSP, DAP, REPL, Studio IDE)](#13-perkakas-developer-lsp-dap-repl-studio-ide)
14. [Multi-Target Compilation (Native, WASM, GPU, eBPF)](#14-multi-target-compilation-native-wasm-gpu-ebpf)
15. [Sub-Sistem Operasi & Kernel Primitives](#15-sub-sistem-operasi--kernel-primitives-srcos)
16. [Distributed Database Engine](#16-distributed-database-engine-srcdb)

---

## 1. Pengantar & Keunggulan Widya-Lang

Widya-Lang dirancang dengan kata kunci ekspresif berbahasa Indonesia dengan performa tinggi berstandar industri:
- 🚀 **Performa Dekat Mesin**: Menggunakan backend LLVM, C native emitter, dan WebAssembly.
- 🛡️ **Sistem Tipe Kuat**: Hindley-Milner Type Inference dengan dukungan Generic & Type-classes.
- 📱 **Cross-Platform**: Satu basis kode untuk Server, Desktop, Web (WASM/HTML5), dan Mobile (Android & iOS).
- 🔒 **Keamanan Terpadu**: Detektor rahasia, audit SQLi, dan proteksi XSS bawaan.

---

## 2. Instalasi & Antarmuka CLI

Setelah mengompilasi biner `widya`, perintah utama yang tersedia:

```bash
# Eksekusi berkas
widya jalankan skrip.wya
widya skrip.wya

# Buka sesi interaktif REPL
widya repl

# Kompilasi native executable (.exe / .out)
widya kompilasi program.wya --output app.exe

# Audit keamanan kode
widya security scan .
widya security secrets .

# Profiling memori
widya profile program.wya

# Membuka IDE Mandiri Widya Studio
widya studio --port 8080

# Menjalankan server LSP & DAP
widya lsp
widya dap
```

---

## 3. Sintaks Dasar & Tipe Data

### Variabel & Konstanta
```widya
// Variabel yang dapat diubah (mutable)
misal nama = "Widya";
misal umur = 25;
nama = "Widya-Lang";

// Konstanta yang tidak dapat diubah (immutable)
tetap PI = 3.14159;
tetap MAKSIMAL_KONEKSI = 1000;
```

### Struktur Kontrol & Perulangan
```widya
// Percabangan Jika-Kalau-Lainnya
jika umur >= 18 {
    cetak("Dewasa");
} kalau umur >= 13 {
    cetak("Remaja");
} lainnya {
    cetak("Anak-anak");
}

// Perulangan untuk (for in)
misal angka = [1, 2, 3, 4, 5];
untuk n dalam angka {
    cetak("Nilai: " + ke_teks(n));
}

// Perulangan selama (while)
misal hitung = 3;
selama hitung > 0 {
    cetak(hitung);
    hitung = hitung - 1;
}
```

---

## 4. Sistem Tipe Statis & Generics

Widya mendukung inferensi tipe otomatis maupun anotasi eksplisit serta generics dengan type constraints:

```widya
// Fungsi Generik
fungsi tukar<T>(a: T, b: T) -> [T] {
    kembalikan [b, a];
}

// Generik dengan Batasan Sifat (Trait Bounds)
fungsi cetak_identitas<T: Tampil>(item: T) {
    cetak(item.ke_teks());
}
```

---

## 5. Struktur Data, Sifat (Traits), & Pemrograman Berorientasi Objek

```widya
// Definisi Sifat (Interface)
sifat DapatDihitung {
    fungsi hitung_luas() -> Angka;
}

// Definisi Struktur
struktur Persegi {
    panjang: Angka,
    lebar: Angka
}

// Implementasi Sifat untuk Struktur
terapkan DapatDihitung untuk Persegi {
    fungsi hitung_luas() -> Angka {
        kembalikan diri.panjang * diri.lebar;
    }
}

// Instansiasi
misal p = Persegi { panjang: 10, lebar: 5 };
cetak("Luas: " + ke_teks(p.hitung_luas()));
```

---

## 6. Pencocokan Pola (Pattern Matching)

Pencocokan pola yang kuat dan aman:

```widya
misal status_http = 200;

misal pesan = cocokkan status_http {
    200 => "Berhasil OK",
    404 => "Halaman Tidak Ditemukan",
    500 => "Kesalahan Server Internal",
    _ => "Kode Status Lain"
};

cetak(pesan);
```

---

## 7. Pengembangan Web & REST API

Widya memiliki server web HTTP/1.1, HTTP/2, dan WebSocket terintegrasi:

```widya
// Layanan Microservice REST API
misal server = ServerHttp::buat(8080);

server.tangani_get("/api/pengguna", fungsi(req) {
    kembalikan Tanggapan::json({
        "status": 200,
        "data": ["Budi", "Siti", "Widya"]
    });
});

server.tangani_post("/api/pesan", fungsi(req) {
    misal data = req.baca_json();
    kembalikan Tanggapan::json({
        "status": "Diterima",
        "pesan": data["pesan"]
    });
});

server.mulai();
```

---

## 8. Pengembangan Aplikasi Mobile (Android & iOS)

Satu kode UI deklaratif untuk Android & iOS menggunakan WidyaUI:

```widya
// contoh/aplikasi_mobile.wya
fungsi utama() {
    kembalikan Aplikasi({
        "judul": "Toko Online Widya",
        "badan": Halaman({
            "badan": Kolom([
                TeksWidget("Selamat Datang di Mobile App!"),
                Tombol("Beli Sekarang", fungsi() {
                    cetak("Pesanan diproses!");
                })
            ])
        })
    });
}
```

Kompilasi ke mobile:
```bash
# Buat proyek mobile baru
widya mobile buat TokoApp

# Bangun paket APK (Android)
widya mobile bangun src/main.wya --target android

# Bangun paket IPA (iOS)
widya mobile bangun src/main.wya --target ios
```

---

## 9. C/Native Foreign Function Interface (FFI)

Memanggil library C ABI dan shared library OS (`.dll`, `.so`, `.dylib`):

```widya
// Memanggil fungsi matematika libc C
misal hasil_akar = Ffi::panggil("sqrt", [144.0]);
cetak("Akar 144: " + ke_teks(hasil_akar)); // 12.0

// Memanggil C puts
Ffi::panggil("puts", ["Halo dari Native C ABI!"]);
```

---

## 10. Audit Keamanan & Pencegahan Kerentanan

Widya dilengkapi scanner statis untuk keamanan enterprise:

```bash
# Pindai seluruh direktori proyek
widya security scan .

# Pindai kebocoran kunci API & token rahasia
widya security secrets .

# Pasang otomatis proteksi ke Git pre-commit
widya security install-hook
```

---

## 11. Manajemen Memori & Profiler

Untuk aplikasi backend berskala besar, jalankan memory profiler untuk melacak alokasi heap dan mendeteksi siklus referensi (*circular memory leaks*):

```bash
widya profile server.wya
```

Contoh Laporan:
```
🧠 Widya-Lang Memory & Allocation Profiler Report
══════════════════════════════════════════════════
  Memori Aktif           : 42.50 KB
  Penggunaan Puncak (Peak): 128.00 KB
  Jumlah Objek Aktif     : 140
  Deteksi Siklus (Cycles): 0

Distribusi Penggunaan Tipe Data:
  • Teks            : 18400 bytes
  • Daftar          : 12500 bytes
  • Peta            : 11600 bytes

✅ Bebas kebocoran memori siklik (Zero cyclic leaks detected).
```

---

## 12. Widya Package Manager (WPM) & Lockfile

Manajemen dependensi dengan verifikasi checksum SHA256:

```bash
# Inisialisasi proyek baru
widya inisialisasi TokoOnline

# Menambah dependensi
widya tambah widya-http

# Membuat/memperbarui widya.lock
widya wpm lock .

# Memvalidasi integritas paket
widya wpm verify .
```

---

## 13. Perkakas Developer (LSP, DAP, REPL, Studio IDE)

- **Language Server Protocol (LSP)**: Menyediakan auto-completion, hover docs, go-to-definition, dan diagnostics untuk VS Code.
- **Debug Adapter Protocol (DAP)**: Mendukung breakpoint dan visual stepping debugger.
- **Interactive REPL**: Dilengkapi `.waktu` (stopwatch eksekusi), `.tipe` (tipe ekspresi), dan `.lingkungan` (variabel aktif).
- **Widya Studio**: IDE mandiri lokal berbasis GUI/Browser dengan editor, terminal terintegrasi, dan live canvas.

---

## 14. Multi-Target Compilation

Widya dapat dikompilasi ke berbagai target komputasi khusus:

```bash
# WebAssembly untuk browser dan edge runtime
widya wasm modul.wya --output modul.wasm

# GPU Compute Shaders (WGSL / WebGPU)
widya gpu komputasi.wya --output shader.wgsl

# eBPF Linux Kernel Tracing & Probing
widya ebpf monitor.wya --output probe.c

# LLVM Intermediate Representation (IR)
widya llvm core.wya --output core.ll
```

---

## 15. Sub-Sistem Operasi & Kernel Primitives (`src/os`)

Widya memiliki modul kernel POSIX & microkernel primitives bawaan untuk sistem embedded, bare-metal, dan OS runtime:

- **Virtual Memory & Paging (`src/os/memory.rs`)**: Paging 4KB, isolasi Virtual Address Space, dan alokator halaman memori.
- **Kernel Threading & Scheduler (`src/os/threading.rs`)**: Preemptive Multi-Level Priority Scheduler dan context switching.
- **Interrupts & Traps (`src/os/interrupts.rs`)**: Global Interrupt Controller (GIC) dan Hardware Exception Handler.
- **Virtual Filesystem (`src/os/vfs/`)**: Inode-based VFS dengan mount points (`/`, `/dev`, `/proc`).
- **Syscall Dispatch Table (`src/os/syscall/`)**: POSIX interface untuk `read`, `write`, `open`, `close`, `fork`, `exec`.

---

## 16. Distributed Database Engine (`src/db`)

Widya menyediakan mesin basis data terdistribusi ACID berkinerja tinggi:

- **B+Tree Storage Engine (`src/db/btree.rs`)**: Indexing disk page berstruktur B+Tree dengan node balancing.
- **Write-Ahead Logging / WAL (`src/db/wal.rs`)**: Log Sequence Number (LSN) dan crash recovery instan.
- **Raft & Paxos Distributed Consensus (`src/db/consensus.rs`)**: Leader Election otomatis, Term replication, dan quorum guarantees.
- **Consistent Hashing Sharding (`src/db/sharding.rs`)**: Virtual nodes (vnodes) horizontal scale-out.
- **Vector Clock Replication (`src/db/replication.rs`)**: Multi-master replication dengan resolusi konflik otomatis.
- **Cost-Based Query Optimizer (`src/db/query/`)**: Evaluasi pohon eksekusi dan kalkulasi biaya kueri relasional.

---

### 🇮🇩 Widya-Lang: Kedaulatan Teknologi & Standar Rekayasa Perangkat Lunak Modern.
