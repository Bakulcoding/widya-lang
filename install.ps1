# =================================================================
# Script Instalasi Otomatis Widya-Lang untuk Windows (PowerShell)
# =================================================================

$ErrorActionPreference = "Stop"

Write-Host "Menginstal Widya-Lang pada Windows..." -ForegroundColor Cyan

$installDir = "$env:USERPROFILE\.widya\bin"

if (Get-Command cargo -ErrorAction SilentlyContinue) {
    Write-Host "Mengompilasi binary Widya dengan Cargo..." -ForegroundColor Yellow
    cargo build --release
    
    if (-not (Test-Path $installDir)) {
        New-Item -ItemType Directory -Path $installDir -Force | Out-Null
    }
    
    Copy-Item -Path ".\target\release\widya.exe" -Destination "$installDir\widya.exe" -Force
    
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$installDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
        Write-Host "Menambahkan $installDir ke User PATH." -ForegroundColor Yellow
    }
    
    Write-Host "Widya-Lang berhasil dipasang ke $installDir\widya.exe!" -ForegroundColor Green
    Write-Host "Buka terminal baru dan jalankan 'widya' untuk mulai menggunakan." -ForegroundColor Cyan
} else {
    Write-Host "Cargo atau Rust tidak ditemukan. Silakan pasang Rust dari https://rustup.rs" -ForegroundColor Red
}
