# =================================================================
# Script Instalasi Otomatis Widya-Lang untuk Windows (PowerShell)
# =================================================================

Write-Host "🇮🇩 Menginstal Widya-Lang pada Windows..." -ForegroundColor Cyan

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "⚙️  Mengompilasi binary Widya dengan Cargo..." -ForegroundColor Yellow
    cargo build --release
    
    $installDir = "$HOME\.widya\bin"
    if (!(Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }
    
    Copy-Item "target\release\widya.exe" "$installDir\widya.exe" -Force
    
    # Tambahkan ke User PATH jika belum ada
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
        Write-Host "📌 Menambahkan $installDir ke PATH lingkungan." -ForegroundColor Yellow
    }
    
    Write-Host "🎉 Widya-Lang berhasil dipasang ke $installDir\widya.exe!" -ForegroundColor Green
    Write-Host "Buka terminal baru dan ketik 'widya' untuk mulai menggunakan REPL." -ForegroundColor Cyan
} else {
    Write-Host "❌ Cargo/Rust tidak ditemukan. Silakan pasang Rust dari https://rustup.rs" -ForegroundColor Red
}
