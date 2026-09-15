#!/usr/bin/env bash
# =================================================================
# Script Instalasi Otomatis Widya-Lang untuk Linux & macOS
# =================================================================

set -e

echo "🇮🇩 Menginstal Widya-Lang..."

# Deteksi OS dan Arsitektur
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)     PLATFORM="linux" ;;
    Darwin*)    PLATFORM="macos" ;;
    *)          echo "❌ OS tidak didukung secara otomatis: $OS"; exit 1 ;;
esac

echo "Deteksi Sistem: $PLATFORM ($ARCH)"

# Cek apakah Cargo/Rust terpasang
if command -v cargo >/dev/null 2>&1; then
    echo "⚙️  Mengompilasi binary Widya dengan Cargo..."
    cargo build --release
    sudo cp target/release/widya /usr/local/bin/widya
    sudo chmod +x /usr/local/bin/widya
    echo "🎉 Widya-Lang berhasil dipasang ke /usr/local/bin/widya!"
    echo "Ketik 'widya' untuk mulai menggunakan REPL."
else
    echo "⚠️  Cargo tidak ditemukan. Silakan pasang Rust terlebih dahulu:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
