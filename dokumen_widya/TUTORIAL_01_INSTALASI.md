# Tutorial 01 — Instalasi Widya-Lang

**Target**: Kamu bisa menjalankan perintah `widya` di terminal dalam 5-10 menit.
**Level**: Pemula ⭐
**Waktu**: ~10 menit
**File contoh terkait**: [install.ps1](../install.ps1), [install.sh](../install.sh), [Cargo.toml](../Cargo.toml)

---

## Prasyarat

Pastikan sudah terinstal di komputer Anda:

| Alat | Versi Minimum | Cara Cek | Keterangan |
|------|--------------|----------|-----------|
| **Git** | ≥ 2.30 | `git --version` | Untuk clone repository |
| **Rust Toolchain** | ≥ Rust 1.75 + Cargo | `rustc --version` & `cargo --version** | Install via [rustup.rs](https://rustup.rs/) |
| **C++ Build Tools** (Hanya Windows) | MSVC 2022 / MinGW-w64 | `cl.exe` atau `gcc --version` | Linker untuk produce .exe native |

> 💡 **Tips**: Di Windows, install **Visual Studio Build Tools 2022** pilih workload "Desktop development with C++" (3 GB) agar MSVC tersedia. Atau install MSYS2 untuk MinGW.

---

## Langkah 1 — Clone Repository dari GitHub

```bash
# HTTPS
git clone https://github.com/Bakulcoding/widya-lang.git
cd widya-lang
```

Atau jika pakai SSH:
```bash
git clone git@github.com:Bakulcoding/widya-lang.git
cd widya-lang
```

Setelah selesai, struktur folder utama:
```
widya-lang/
├── src/                  # Sumber interpreter + 6 compiler target (Rust)
├── modul/                # 80+ Modul industri siap pakai (.wya)
├── contoh/               # 168 Contoh kode runnable (.wya)
├── tests/                # 60+ Test Suite Rust (cargo test)
├── Cargo.toml            # Manifest proyek Rust
├── install.ps1           # Auto-installer Windows
└── install.sh            # Auto-installer Linux/macOS
```

---

## Langkah 2 — Jalankan Auto-Installer (Paling Cepat)

### 🪟 Windows (PowerShell)
```powershell
# Pastikan Execution Policy mengizinkan (sekali saja)
Set-ExecutionPolicy -Scope CurrentUser RemoteSigned

# Jalankan installer
.\install.ps1
```

### 🐧 Linux & 🍎 macOS (Bash/Zsh)
```bash
chmod +x install.sh
./install.sh
```

Installer otomatis melakukan:
1. `cargo build --release` → menghasilkan `target/release/widya(.exe)`
2. Copy biner ke folder user PATH
3. Verifikasi dengan `widya --versi`

---

## Langkah 2 Alternatif — Build Manual (Untuk Developer)

Jika mau kontrol penuh:
```bash
# 1. Build mode release (optimal performa)
cargo build --release

# 2. Biner berada di:
#    Windows: target\release\widya.exe
#    Linux:   target/release/widya
#    macOS:   target/release/widya

# 3. (Opsional) Tambahkan ke PATH secara permanen
#    Windows PowerShell:
$env:PATH += ";C:\widya-lang\target\release"
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, "User")

#    Linux/macOS:
export PATH="$HOME/widya-lang/target/release:$PATH"
echo 'export PATH="$HOME/widya-lang/target/release:$PATH"' >> ~/.bashrc
```

---

## Langkah 3 — Verifikasi Instalasi

Buka **terminal BARU** (agar PATH reload), lalu jalankan:

```bash
# 1. Cek versi
widya --versi
# Output: Widya-Lang v1.0.0 (atau terbaru)

# 2. Jalankan skrip pertama
widya jalankan contoh/01_halo_dunia.wya

# Output diharapkan:
# ====================================
# ✨ Selamat Datang di Widya-Lang! ✨
# ====================================
# Bahasa: Widya
# Versi: 1
# Tahun Rilis: 2026
# ... dst

# 3. Masuk ke REPL (Interactive Shell)
widya
# Widya> ketik: cetak("Halo dari REPL!")
# Widya> keluar
```

Jika ketiga hal di atas berjalan → **Instalasi BERHASIL!** 🎉

---

## Troubleshooting Instalasi Umum

| Pesan Galat | Penyebab | Solusi |
|-------------|----------|--------|
| `widya: command not found` | PATH belum direload | Tutup & buka terminal BARU; atau jalankan `.\target\release\widya.exe` langsung |
| `error: linker 'link.exe' not found` | MSVC Build Tools tidak ada | Install Visual Studio Build Tools 2022 pilih "Desktop C++" |
| `could not compile dependency` | Koneksi internet putus saat cargo download | Jalankan ulang `cargo build --release` (cargo cache progresif) |
| `Permission denied` di Linux | Folder target tidak writable | `chmod -R u+w target/` atau ulang build dengan user biasa |

---

## ⭐ Langkah Selanjutnya — Master Demo!

Setelah instalasi sukses, **wajib coba master demo** untuk melihat kapabilitas penuh Widya:

```powershell
# Demo FLAGSHIP: 9 Layer Universal Stack 100% SIAP (810/810 POINT)
widya jalankan contoh\168_semua_layer_100persen.wya

# Atau boot WidyaOS
widya jalankan modul\widya_os.wya

# Atau demo WidyaDB Multi-Modal
widya jalankan contoh\167_widyadb_produksi.wya
```

---

**Navigasi Tutorial**:
⬆️ Kembali ke [TUTORIAL_INDEX.md](./TUTORIAL_INDEX.md) |
➡️ Lanjut ke **[TUTORIAL_02_HALO_DUNIA.md](./TUTORIAL_02_HALO_DUNIA.md)**
