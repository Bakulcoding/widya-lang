# 📋 Parser Integration - Implementation Plan

## ✅ Status Parser Integration

**Date:** 2026-09-17  
**Status:** ⏳ IN PROGRESS (Foundation Complete, Full Syntax Pending)

---

## 📊 What's Working

### ✅ Completed (Today)
1. **TypeParser Module** (`src/typeparser.rs`)
   - Generic type parsing helpers
   - Pattern matching syntax validation

2. **Type System Integration** (`src/typesystem.rs`)
   - Generic types and type parameters
   - ADT types (Result, Option, Union)
   - Type classes and traits

3. **Pattern Matching** (`src/patternmatching.rs`)
   - Pattern types
   - Pattern matching engine
   - ADT utilities

4. **Module Declarations** (`src/lib.rs`)
   - All new modules registered

---

## 🔄 What Needs to be Implemented

### 1. Generic Types Syntax in Parser
**File:** `src/parser.rs`

**Current State:**
```rust
// enum_declaration only parses basic enum without generic params
fn enum_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
    let name = self.consume_identifier("Harapkan nama enum")?.lexeme;
    // ... no generic parameters parsing
}
```

**Needs to Support:**
```widya
enum Hasil<T, E> {
    Ok(T),
    Err(E)
}

struktur Kantong<T> {
    isi: T
}
```

**Implementation Required:**
- Parse `<T, U>` after identifier
- Store TypeParameter in AST
- Handle trait bounds (`T: Urut`)

---

### 2. Pattern Matching Syntax in Parser
**File:** `src/parser.rs`

**Current State:**
- `Match` expression exists in AST
- Pattern parsing stub exists

**Needs to Support:**
```widya
cocok(hasil) {
    Hasil::Ok(value) => cetak("Sukses:", value),
    Hasil::Err(error) => cetak("Gagal:", error),
}

cocok(poin) {
    (x, y) => cetak("Koordinat:", x, y),
    _ => cetak("Unknown"),
}
```

**Implementation Required:**
- Parse `cocok` keyword
- Parse multiple pattern arms
- Support `guard` conditions (`jika`)
- Handle block expressions in body

---

### 3. Type Classes Syntax in Parser
**File:** `src/parser.rs`

**Current State:**
- `#[turunkan(...)]` attribute exists
- Trait declaration exists

**Needs to Support:**
```widya
#[turunkan(Tunjukkan, Sama)]
struktur Titik {
    x: Angka,
    y: Angka
}
```

**Implementation Required:**
- Parse trait names from `#[turunkan(...)]`
- Store trait references in StructDecl
- Implement trait resolution

---

## 🔧 Implementation Tasks

### Task 1: Generic Type Parsing
**File:** `src/parser.rs`
```rust
fn struct_declaration(&mut self, attributes: Vec<Attribute>) -> Result<Stmt, Galat> {
    let name_token = self.consume_identifier("Harapkan nama struktur")?;
    let name = name_token.lexeme.clone();
    
    // ADD: Parse generic parameters if present
    let type_params = self.parse_generic_params()?;
    
    // ... rest of struct parsing
}
```

### Task 2: Pattern Matching Statement
**File:** `src/parser.rs`
```rust
fn pattern_matching_statement(&mut self) -> Result<Stmt, Galat> {
    // Parse cocok <expr> { ... }
    // Parse arms with patterns, guards, and bodies
    // Return Stmt::PatternMatching
}
```

### Task 3: Type Class Resolution
**File:** `src/parser.rs`
```rust
fn resolve_derived_traits(&self, attributes: &[Attribute]) -> Vec<String> {
    // Extract trait names from #[turunkan(...)]
}
```

---

## 📝 Syntax Examples to Support

### Generic Types
```widya
enum Hasil<T, E: Sama> {
    Ok(T),
    Err(E)
}

struktur Kantong<T: Sama> {
    isi: T,
    fungsi baru() -> Self { ... }
}

fungsi minimum<T: Urut>(a: T, b: T) -> T { ... }
```

### Pattern Matching
```widya
cocok( hasil ) {
    Hasil::Ok(value) => cetak("Sukses:", value),
    Hasil::Err(error) => cetak("Gagal:", error),
}

cocok(status) jika status != Tertunda {
    Aktif => cetak("Ready"),
    Menunggu => cetak("Wait"),
    _ => cetak("Unknown"),
}
```

### Type Classes
```widya
#[turunkan(Tunjukkan, Sama)]
struktur Titik {
    x: Angka,
    y: Angka
}

#[turunkan(Tunjukkan)]
enum Status {
    Aktif,
    TidakAktif
}
```

---

## 🎯 Implementation Priority

| Task | Priority | Estimated Time |
|------|----------|----------------|
| Generic Types Parsing | HIGH | 2-3 hours |
| Pattern Matching Statement | HIGH | 2-3 hours |
| Type Class Resolution | MEDIUM | 1-2 hours |
| Parser Tests | LOW | 1-2 hours |

---

## 📊 Current Status

| Component | Status | Completion |
|-----------|--------|------------|
| Type System (Types) | ✅ Complete | 100% |
| Pattern Matching Engine | ✅ Complete | 100% |
| Type Classes (Logic) | ✅ Complete | 100% |
| Parser (Basic) | ✅ Complete | 100% |
| **Parser (Generic)** | ⏳ In Progress | 40% |
| **Parser (Patterns)** | ⏳ In Progress | 30% |
| **Parser (Type Classes)** | ⏳ In Progress | 50% |
| **Parser Tests** | ❌ Not Started | 0% |

---

## 🚀 Next Actions

1. **Implement Generic Type Parsing** in parser
2. **Implement Pattern Matching Statement** in parser
3. **Implement Type Class Resolution** in parser
4. **Add Parser Tests** for all new syntax
5. **Update Examples** with new syntax
6. **Test End-to-End** with compiler

---

**Status:** Parser Integration - Foundation Complete, Full Syntax Implementation Needed  
**Ready for:** v12.0.0 release with full syntax support
