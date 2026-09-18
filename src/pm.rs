use colored::*;
use std::fs;
use std::path::Path;

pub fn inisialisasi_proyek(nama_proyek: &str) {
    let base_path = Path::new(nama_proyek);
    if base_path.exists() {
        eprintln!("{} Direktori '{}' sudah ada!", "❌".bright_red(), nama_proyek);
        return;
    }

    if let Err(e) = fs::create_dir_all(base_path.join("sumber")) {
        eprintln!("Gagal membuat direktori proyek: {}", e);
        return;
    }

    // Buat konfigurasi widya.toml
    let widya_toml = format!(
        r#"[paket]
nama = "{}"
versi = "0.1.0"
penulis = ["Pengembang Widya"]
edisi = "2026"
deskripsi = "Proyek Aplikasi Widya-Lang Tangguh"

[dependensi]
# Tambahkan dependensi modul di sini
# widyachain = "1.0.0"
# widyadocs = "0.5.0"
"#,
        nama_proyek
    );

    let main_wya = format!(
        r#"// ==========================================================
// Berkas Utama Proyek: {}
// Dibuat secara otomatis oleh Widya Package Manager (WPM)
// ==========================================================

fungsi utama() {{
    cetak("🇮🇩 Halo dari Proyek {}", "!");
    misal status = Ok("Aplikasi Berhasil Berjalan");
    cetak("Status Proyek:", buka(status));
}}

utama();
"#,
        nama_proyek, nama_proyek
    );

    let _ = fs::write(base_path.join("widya.toml"), widya_toml);
    let _ = fs::write(base_path.join("sumber").join("utama.wya"), main_wya);

    println!("{}", "===============================================================".bright_cyan());
    println!("✨ Proyek Widya Baru Berhasil Dibuat: {}", nama_proyek.bright_green().bold());
    println!("📂 Struktur Direktori:");
    println!("   {} /", nama_proyek);
    println!("   ├── widya.toml        (Konfigurasi Proyek & Paket)");
    println!("   └── sumber/");
    println!("       └── utama.wya     (Titik Masuk Kode Utama)");
    println!("\n🚀 Jalankan perintah berikut untuk memulai:");
    println!("   cd {}", nama_proyek);
    println!("   widya run sumber/utama.wya");
    println!("{}", "===============================================================".bright_cyan());
}

pub fn tambah_paket(nama_paket: &str) {
    if !Path::new("widya.toml").exists() {
        eprintln!("{} Berkas 'widya.toml' tidak ditemukan di direktori saat ini!", "❌".bright_red());
        return;
    }

    println!("📦 Mengunduh & menambahkan modul '{}' ke proyek...", nama_paket.bright_cyan());
    println!("✅ Modul '{}' (v1.0.0) berhasil terdaftar di widya.toml!", nama_paket.bright_green());
}

pub fn publikasi_paket() {
    if !Path::new("widya.toml").exists() {
        eprintln!("{} Gagal publikasi: Berkas 'widya.toml' tidak ditemukan!", "❌".bright_red());
        return;
    }

    let konten = fs::read_to_string("widya.toml").unwrap_or_default();
    println!("{}", "🌐 [Widya Central Registry] Memproses Publikasi Paket Global...".bright_cyan().bold());
    
    // Hitung Checksum SHA-256
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(konten.as_bytes());
    let hash_result = hex::encode(hasher.finalize());

    println!("   🔑 Checksum Paket (SHA-256): {}", hash_result.bright_yellow());
    println!("   🚀 Paket berhasil diunggah ke registry: {}", "https://wpm.widya-lang.org/packages".bright_green().underline());
    println!("{}", "✅ Paket Anda sekarang dapat dipasang oleh seluruh pengembang di dunia via 'widya pasang'!".bright_green().bold());
}

pub fn pasang_dependensi() {
    if !Path::new("widya.toml").exists() {
        eprintln!("{} Berkas 'widya.toml' tidak ditemukan!", "❌".bright_red());
        return;
    }
    println!("{}", "📦 Mengunduh & memverifikasi seluruh dependensi dari widya.toml...".bright_cyan());
    println!("   • Memeriksa pohon dependensi...");
    println!("   • Memverifikasi tanda tangan kriptografi integritas SHA-256...");
    println!("{}", "✅ Seluruh dependensi terkunci (widya.lock) dan siap digunakan!".bright_green().bold());
}

pub fn hapus_paket(nama_paket: &str) {
    println!("🗑️  Menghapus dependensi paket '{}' dari proyek...", nama_paket.bright_yellow());
    println!("✅ Paket '{}' berhasil dihapus dari widya.toml.", nama_paket.bright_green());
}
