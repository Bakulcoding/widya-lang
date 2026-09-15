# Tutorial 15 — FFI (Foreign Function Interface) C & Bare-Metal GPIO RISC-V

**Target**: Memanggil fungsi C dari Widya, sebaliknya memanggil Widya dari C, dan menjalankan Widya sebagai bahasa bare-metal tanpa OS di mikrokontroller RISC-V / ARM Cortex-M.
**Level**: Menengah ⭐⭐
**Waktu**: ~25 menit
**File contoh lengkap**: [16_c_ffi_dan_baremetal.wya](../contoh/16_c_ffi_dan_baremetal.wya) | [program_native.c](../program_native.c) | [contoh/100_gpio_dan_bus_i2c_spi.wya](../contoh/100_gpio_dan_bus_i2c_spi.wya)

---

## Bagian 1: FFI C (Widya Memanggil Fungsi Native C)

FFI = mekanisme memanggil **dynamic library (.dll / .so / .dylib)** berisi kode C (atau Rust/C++) yang sudah di-compile — untuk akses OS API / hardware yang tidak tercover stdlib.

### Langkah 1 — Buat pustaka C dulu
Simpan file `libmatematika.c`:
```c
// libmatematika.c  — compile jadi DLL / shared object
#include <stdint.h>
#include <math.h>

// __declspec(dllexport) untuk Windows .dll
#ifdef _WIN32
  #define EXPORT __declspec(dllexport)
#else
  #define EXPORT __attribute__((visibility("default")))
#endif

EXPORT double   euclidean_distance(double x1, double y1, double x2, double y2) {
    double dx = x2 - x1, dy = y2 - y1;
    return sqrt(dx*dx + dy*dy);
}

EXPORT int64_t  faktorial_C(int64_t n) {
    int64_t hasil = 1;
    for (int64_t i = 2; i <= n; i++) hasil *= i;
    return hasil;
}
```

Compile ke shared library:
```bash
# Windows (MSVC / MinGW)
gcc -shared -O2 libmatematika.c -o libmatematika.dll

# Linux (GCC)
gcc -shared -fPIC -O2 libmatematika.c -o libmatematika.so

# macOS (Clang)
clang -shared -fPIC -O2 libmatematika.c -o libmatematika.dylib
```

### Langkah 2 — Panggil Dari Widya Pakai Keyword `eksternal`

```widya
// ============== FFI BINDING WIDYA ==============
eksternal "libmatematika.dll" {
    fungsi euclidean_distance(x1: angka, y1: angka, x2: angka, y2: angka) -> angka;
    fungsi faktorial_C(n: angka) -> angka;
}

// ============== PAKAI SEPERTI FUNGSI BIASA ==============
misal jarak = euclidean_distance(0.0, 0.0, 3.0, 4.0);
cetak("Jarak (0,0) ke (3,4):", jarak);  // tepat 5.0 (triple pitagoras 3-4-5) ✅

misal fac_c = faktorial_C(15);
cetak("15! via C FFI:", fac_c);          // 1307674368000 ✅
```

### Langkah 3 — Validasi Kebenaran
Jika ada ketidaksesuaian calling convention / tipe, FFI akan undefined behavior. Selalu test assertion:

```widya
pastikan(euclidean_distance(0.0,0.0,3.0,4.0) == 5.0);
pastikan(faktorial_C(10) == 3628800);
cetak("✅ FFI binding test LULUS.");
```

---

## Bagian 2: Sebaliknya — Panggil Widya dari Host C

Untuk embed Widya sebagai scripting engine ke aplikasi C utama (game engine, plugin system, etc.):

### Host Program C: `program_native.c`
```c
#include <stdio.h>
#include <string.h>

// Header embedding Widya interpreter
#include "widya_embed.h"   // di-generate oleh Cargo build script
                            // dari src/lib.rs: pub extern "C" fn widya_jalankan_c(...)

int main(void) {
    const char* script_widya =
        "misal total = 0;"
        "misal i = 1;"
        "selama i <= 100 { total = total + i; i = i + 1; }"
        "cetak(\"[Widya embedded] jumlah 1..100 =\", total);"
        "kembalikan total;";

    widya_value_t hasil;
    widya_error_t err = widya_jalankan_c(script_widya, strlen(script_widya), &hasil);

    if (err.kode != 0) {
        fprintf(stderr, "Gagal: %s\n", err.pesan);
        return 1;
    }
    printf("[C host] Widya return integer = %lld\n", hasil.angka);
    // Output: [Widya embedded] jumlah 1..100 = 5050
    //         [C host] Widya return integer = 5050
    return 0;
}
```

Compile & link dengan libwidya:
```bash
# Build libwidya.a static dulu
cargo build --release --lib
# → target/release/widya.lib (Windows) atau target/release/libwidya.a (Linux/macOS)

# Link program C dengan static library:
gcc -O2 program_native.c -Iinclude target/release/widya.lib -o host_dengan_widya.exe
./host_dengan_widya.exe
```

---

## Bagian 3: Bare-Metal GPIO (Widya sebagai OS Embedded Tanpa Kernel)

Target: **STM32F103 (ARM Cortex-M3, 128KB Flash)**, **ESP32-C3 RISC-V**, atau **SiFive FE310**.

### Prinsip Dasar Tanpa OS
1. Tidak ada heap / malloc (tidak ada libc).
2. Akses memory-mapped I/O secara LANGSUNG via alamat fisik.
3. Entry point = `_start` (bukan `main`).
4. Interrupt vector table didefinisikan di section `.vector_table`.

### File: `baremetal_gpio_blink.widya`
```widya
// =====================================================
// Widya Bare-Metal: Blink LED GPIOA Pin 5 (STM32F103)
// =====================================================
#[profil("edge")]  // Constraint L1: 2KB RAM, stack ≤ 512 byte, FPU disabled

// Alamat MMIO STM32F103
tetap RCC_BASE     = 0x40021000;
tetap RCC_APB2ENR  = RCC_BASE + 0x18;

tetap GPIOA_BASE   = 0x40010800;
tetap GPIOA_CRL    = GPIOA_BASE + 0x00;
tetap GPIOA_BSRR   = GPIOA_BASE + 0x10;
tetap GPIOA_BRR    = GPIOA_BASE + 0x14;

fungsi mmio_tulis(addr: angka, value: angka) {
    // Tulis 32-bit ke memory-mapped register (volatile)
    isi_volatile_mmio(addr, value);
}
fungsi mmio_baca(addr: angka) -> angka {
    kembalikan baca_volatile_mmio(addr);
}

fungsi init_GPIOA5_output() {
    // 1. Enable clock GPIOA (bit 2 di RCC_APB2ENR)
    mmio_tulis(RCC_APB2ENR, mmio_baca(RCC_APB2ENR) ATAU (1 << 2));

    // 2. GPIOA Pin 5 = MODE=11 (50MHz output), CNF=00 (push-pull) → CRL bits [23:20] = 0011 = 0x3
    misal crl = mmio_baca(GPIOA_CRL);
    crl = crl DAN (~ (0xF << 20));   // clear
    crl = crl ATAU  (0x3 << 20);     // set 0x3
    mmio_tulis(GPIOA_CRL, crl);
}

fungsi GPIOA5_set() { mmio_tulis(GPIOA_BSRR, (1 << 5));  }  // BS5 = SET
fungsi GPIOA5_clr() { mmio_tulis(GPIOA_BRR,  (1 << 5));  }  // BR5 = RESET

// Software delay busy-loop (Cycle counter)
fungsi delay_ms(ms: angka) {
    misal perulangan_per_ms = 8000;  // disesuaikan dengan HSI 8MHz
    misal total = ms * perulangan_per_ms;
    misal c = 0;
    selama c < total { c = c + 1; }
}

// ===== ENTRY POINT BARE-METAL =====
fungsi _start() -> nihil {
    init_GPIOA5_output();
    selama benar {
        GPIOA5_set();      // LED NYALA
        delay_ms(500);
        GPIOA5_clr();      // LED MATI
        delay_ms(500);
    }
}
```

### Compile + Flash ke Board
```bash
# Step 1: Compile Widya → Rust bare-metal (no_std) → ARM Cortex-M3
widya rust baremetal_gpio_blink.wya -o gen/blink.rs --no-std

# Step 2: Build pakai cortex-m target (rustup component add llvm-tools-preview rust-src)
rustup target add thumbv7m-none-eabi
cargo build --release --target thumbv7m-none-eabi

# Step 3: Generate .bin & flash ke STM32 via stlink / openocd
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/blink blink.bin
st-flash write blink.bin 0x08000000
```

Hasil: LED di pin PA5 berkedip 1 detik periode! 💡

---

## ⚠️ FFI & Bare-Metal Risiko

1. **Undefined Behavior** jika type signature FFI salah (panjang struct tidak cocok, calling convention stdcall vs cdecl).
2. **Use-after-free** jika memory C sudah di-free tapi Widya masih memegang pointer → selalu copy nilai ke memory managed Widya segera setelah FFI call.
3. **MMIO out-of-bounds write = brick hardware**. Selalu cek datasheet register sebelum `mmio_tulis`.
4. **Interrupt Safety**: Jangan share mutable state dengan ISR tanpa critical section (disable interrupts / lock-free atomic).

---

**Navigasi Tutorial**:
⬆️ [INDEX](./TUTORIAL_INDEX.md) |
⬅️ [Tutorial 14 — Makro & Ownership](./TUTORIAL_14_MAKRO_OWNERSHIP.md) |
➡️ **[Tutorial 16 — WidyaDB Multi-Modal](./TUTORIAL_16_WIDYADB.md)** (Level 4 Enterprise)
