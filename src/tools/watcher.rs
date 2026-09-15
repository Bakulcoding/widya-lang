use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub fn awasi_dan_jalankan(path: &PathBuf) {
    println!("👀 [Widya Live Watcher] Memantau perubahan pada: {}", path.display());
    println!("Tekan Ctrl+C untuk berhenti.");

    let mut last_modified = fs::metadata(path).and_then(|m| m.modified()).ok();

    // Jalankan pertama kali
    jalankan_proses_widya(path);

    loop {
        sleep(Duration::from_millis(500));
        if let Ok(meta) = fs::metadata(path) {
            if let Ok(modified) = meta.modified() {
                if Some(modified) != last_modified {
                    last_modified = Some(modified);
                    println!("\n🔄 [Deteksi Perubahan Berkas] Menjalankan ulang secara instan...\n");
                    jalankan_proses_widya(path);
                }
            }
        }
    }
}

fn jalankan_proses_widya(path: &PathBuf) {
    if let Ok(exe_path) = std::env::current_exe() {
        let _ = Command::new(exe_path)
            .arg("run")
            .arg(path)
            .status();
    }
}
