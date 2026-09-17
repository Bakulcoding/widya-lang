# 🚀 Developer Experience Quick Start Guide

## 🔥 Immediate Improvements (To Implement Now)

Based on the developer experience roadmap, here are the quick wins that can be implemented immediately:

### 1. Integrate Enhanced REPL ✅
Update the current REPL to use the enhanced version:

```diff
// src/main.rs or equivalent
- use crate::repl::start_repl;
+ use crate::repl_enhanced::start_repl;
```

### 2. Integrate Pretty Error Formatting ✅
Update error handling to use the pretty formatter:

```rust
// In lexer.rs, parser.rs, or wherever errors are shown
use crate::error_pretty::format_pretty_error;

// Replace: eprintln!("{}", error);
// With:
let pretty_error = format_pretty_error("error_type", &error.message(), line, col, source, context);
eprintln!("{}", pretty_error);
```

### 3. Add Quick CLI Commands ✅
Add these simple CLI commands:

```rust
// src/cli/commands.rs
pub fn quick_check(file_path: &Path) -> Result<(), String> {
    let source = std::fs::read_to_string(file_path)?;
    
    // Quick syntax check
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.scan_tokens()?;
    
    let mut parser = Parser::new(tokens);
    parser.parse()?;
    
    println!("✅ Syntax OK: {}", file_path.display());
    Ok(())
}
```

### 4. Create Example Programs ✅
Create 20+ example programs in `examples/`:

```bash
# Create example directory structure
mkdir -p examples/{basic,functions,control,data,advanced}

# Basic examples
touch examples/basic/{01_hello.wya,02_variables.wya,03_arithmetic.wya}
# Function examples
touch examples/functions/{01_simple.wya,02_recursive.wya,03_higher_order.wya}
# Control flow examples
touch examples/control/{01_if_else.wya,02_loops.wya,03_pattern_match.wya}
# Data structure examples
touch examples/data/{01_arrays.wya,02_maps.wya,03_structs.wya}
# Advanced examples
touch examples/advanced/{01_generics.wya,02_error_handling.wya,03_async.wya}
```

---

## 📋 Next Hour Implementation Plan

### 1. REPL Integration (15 minutes)
1. Update `src/repl.rs` to use enhanced version
2. Add new commands: `.waktu`, `.contoh`, `.lingkungan`
3. Test in REPL

### 2. Error Formatting (15 minutes)
1. Create `src/error/pretty.rs` integration
2. Update error handling in lexer and parser
3. Test with error cases

### 3. CLI Commands (15 minutes)
1. Add `widya cek` command
2. Add `widya contoh` command
3. Add colored output support

### 4. Examples (15 minutes)
1. Create 10 basic example files
2. Add comments and documentation
3. Test each example

---

## 🛠️ Code Changes Needed

### 1. Update main.rs

```rust
// src/main.rs
mod error_pretty;
mod repl_enhanced;

// In CLI parsing
match args[0] {
    "cek" => commands::quick_check(&file_path),
    "contoh" => commands::show_examples(),
    "run" => commands::run_file(&file_path),
    // ...
}
```

### 2. Enhanced Error Handling

```rust
// src/error.rs
pub fn format_with_suggestion(&self, source: &str) -> String {
    use crate::error_pretty::format_pretty_error;
    
    format_pretty_error(
        self.error_type(),
        &self.message(),
        self.line(),
        self.column(),
        source,
        self.context(),
    )
}
```

### 3. Simple Example Files

```widya
// examples/basic/01_hello.wya
/// Program Hello World
/// Kode pertama di Widya-Lang

cetak("Halo, dunia!")
```

```widya
// examples/basic/02_variables.wya
/// Demonstrasi variabel dan tipe data

var nama = "Widya"          // Teks
var umur = 25               // Angka
var tinggi = 175.5          // Angka desimal
var mahasiswa = benar       // Boolean
var daftar = [1, 2, 3, 4]   // Daftar
var peta = {                // Peta
    "nama": "Widya",
    "versi": "v0.1.0"
}

cetak("Nama:", nama)
cetak("Umur:", umur)
cetak("Tinggi:", tinggi)
cetak("Mahasiswa:", mahasiswa)
cetak("Daftar:", daftar)
cetak("Peta:", peta)
```

---

## 🚀 Ready-to-Use Code Snippets

### 1. Time Command for REPL

```rust
// Add to repl_enhanced.rs
pub fn handle_time_command(&mut config: &mut ReplConfig) {
    config.show_time = !config.show_time;
    println!("{} Mode waktu {}", 
        "⏱️".bright_cyan(),
        if config.show_time { "diaktifkan" } else { "dimatikan" }
    );
}
```

### 2. Environment Display

```rust
pub fn show_environment(interpreter: &Interpreter) {
    println!("\n{}", "📦 Lingkungan Variabel".bright_cyan().bold());
    
    let vars = interpreter.get_global_variables();
    for (name, value) in vars {
        println!("  {} {} {} {}", 
            name.bright_green(),
            "=".bright_black(),
            value.to_debug_repr().bright_yellow(),
            format!("[{}]", value.type_name()).bright_black()
        );
    }
}
```

### 3. Quick Syntax Check

```rust
pub fn quick_check(path: &Path) -> Result<(), String> {
    use colored::*;
    
    println!("🔍 Mengecek sintaks: {}", path.display());
    
    let source = std::fs::read_to_string(path)
        .map_err(|e| format!("Gagal membaca file: {}", e))?;
    
    let mut lexer = Lexer::new(&source);
    match lexer.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(_) => {
                    println!("✅ {}", "Sintaks OK".bright_green());
                    Ok(())
                }
                Err(e) => {
                    let pretty = format_pretty_error("syntax", &e.message(), 0, 0, &source, None);
                    eprintln!("{}", pretty);
                    Err("Sintaks error".to_string())
                }
            }
        }
        Err(e) => {
            let pretty = format_pretty_error("lexer", &e.message(), 0, 0, &source, None);
            eprintln!("{}", pretty);
            Err("Lexer error".to_string())
        }
    }
}
```

---

## 📊 Test Immediately

### REPL Tests
```bash
# Start REPL
cargo run --bin widya

# Test commands
> .waktu
> var x = 10
> cetak(x)
> .lingkungan
> .contoh
> .keluar
```

### CLI Tests
```bash
# Create test file
echo 'cetak("Hello")' > test.wya

# Test syntax check
widya cek test.wya

# Run it
widya run test.wya
```

### Error Tests
```bash
# Create error file
echo 'var x =' > error.wya

# Test error handling
widya cek error.wya

# Should show pretty error with suggestion
```

---

## 🎯 Success Metrics

### After This Implementation:
- [ ] REPL has `.waktu`, `.contoh`, `.lingkungan` commands
- [ ] Error messages have suggestions and examples
- [ ] `widya cek` command works for syntax checking
- [ ] 10+ example programs available
- [ ] All existing tests still pass

### To Verify:
```bash
# Run tests
cargo test --lib

# Test REPL
cargo run --bin widya

# Test CLI
cargo run -- cek examples/basic/01_hello.wya
```

---

## 🔧 What Needs to Be Fixed Now

### 1. Build Dependencies
Add these to Cargo.toml if needed:
```toml
[dependencies]
colored = "2"
rustyline = "12"
```

### 2. Module Exports
Update `src/lib.rs`:
```rust
pub mod error_pretty;
pub mod repl_enhanced;
pub mod lsp_enhanced;
```

### 3. CLI Integration
Update CLI parsing to handle new commands.

---

**Status:** Ready for immediate implementation  
**Effort:** 1-2 hours  
**Impact:** High - dramatically improves developer experience  
**Next:** Start with REPL integration
