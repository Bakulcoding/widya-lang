# 🚀 Widya-Lang Developer Experience Roadmap

## 📊 Current Developer Experience Status

### ✅ What's Working
- **REPL:** Interactive REPL with syntax highlighting, multiline support, history
- **LSP Server:** Basic diagnostics (lexer + parser errors)
- **VS Code Extension:** Basic syntax highlighting
- **CLI Tools:** `widya run`, `widya kompilasi`, `widya wasm`, etc.
- **Error Messages:** Indonesian error messages with source context

### 🔴 What's Missing
- **IDE Features:** Auto-completion, go-to-definition, find references
- **Debugging:** No debugger support
- **Documentation:** No inline documentation, hover info
- **Code Actions:** No quick fixes, refactorings
- **Testing:** No test runner integration
- **Profiling:** No performance tools

---

## 🎯 Developer Experience Vision

**Goal:** Make Widya-Lang the most developer-friendly programming language in Indonesia with world-class tooling.

### Principles
1. **Indonesian-First:** All messages, docs, and UI in Bahasa Indonesia
2. **Helpful Errors:** Error messages should teach, not just report
3. **Fast Feedback:** Instant feedback in IDE, < 1s build times
4. **Zero Config:** Works out of the box, minimal setup required
5. **Interactive:** REPL, playground, visual debugger

---

## 📋 Developer Experience Features Roadmap

### Phase 1: Enhanced LSP (Week 1-2)
**Priority:** 🔴 Critical  
**Effort:** 5 days  
**Dependencies:** Type Checker

#### Features
- [ ] **Auto-completion**
  - Keywords (`fungsi`, `jika`, `untuk`, etc.)
  - Built-in functions (`cetak`, `panjang`, etc.)
  - User-defined functions and variables
  - Type-aware suggestions (after type checker)
  - Snippet completions

- [ ] **Go to Definition**
  - Jump to function definition
  - Jump to variable declaration
  - Jump to imported modules

- [ ] **Find References**
  - Find all usages of a function
  - Find all usages of a variable
  - Workspace-wide search

- [ ] **Hover Information**
  - Show type signatures
  - Show documentation comments
  - Show function parameters

- [ ] **Signature Help**
  - Show function signature while typing
  - Parameter documentation
  - Current parameter highlighting

#### Implementation
```rust
// src/lsp/completion.rs
pub struct CompletionProvider {
    keywords: Vec<CompletionItem>,
    builtins: Vec<CompletionItem>,
}

impl CompletionProvider {
    pub fn complete(&self, context: &CompletionContext) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        
        // Add keywords
        items.extend(self.keywords.iter().filter(|k| {
            k.label.starts_with(&context.prefix)
        }).cloned());
        
        // Add builtins
        items.extend(self.builtins.iter().filter(|b| {
            b.label.starts_with(&context.prefix)
        }).cloned());
        
        // Add scope variables (from interpreter environment)
        if let Some(scope) = context.scope {
            items.extend(scope.variables.iter().map(|(name, value)| {
                CompletionItem {
                    label: name.clone(),
                    kind: CompletionItemKind::Variable,
                    detail: Some(format!(": {}", value.type_name())),
                    ..Default::default()
                }
            }));
        }
        
        items
    }
}
```

---

### Phase 2: Debugger Integration (Week 3-4)
**Priority:** 🟠 High  
**Effort:** 7 days  
**Dependencies:** Runtime enhancements

#### Features
- [ ] **Breakpoints**
  - Set breakpoints in IDE
  - Conditional breakpoints
  - Logpoints (print without stopping)

- [ ] **Step Execution**
  - Step over
  - Step into
  - Step out
  - Continue

- [ ] **Variable Inspection**
  - View local variables
  - View call stack
  - View memory state

- [ ] **Debug Adapter Protocol (DAP)**
  - Implement DAP for VS Code integration
  - Support other IDEs (IntelliJ, etc.)

#### Implementation
```rust
// src/debugger/mod.rs
pub struct Debugger {
    breakpoints: HashMap<usize, Breakpoint>,
    call_stack: Vec<StackFrame>,
    variables: HashMap<String, Value>,
    state: DebuggerState,
}

impl Debugger {
    pub fn set_breakpoint(&mut self, line: usize, condition: Option<String>) {
        self.breakpoints.insert(line, Breakpoint {
            line,
            condition,
            hit_count: 0,
        });
    }
    
    pub fn step_over(&mut self) -> DebuggerState {
        // Execute next statement, don't enter functions
        self.state = DebuggerState::StepOver;
        self.execute_one_statement()
    }
    
    pub fn inspect_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}
```

---

### Phase 3: Documentation & Hover (Week 5-6)
**Priority:** 🟡 Medium  
**Effort:** 5 days  
**Dependencies:** WDoc enhancement

#### Features
- [ ] **Inline Documentation**
  - Parse `///` doc comments
  - Generate hover content
  - Parameter documentation

- [ ] **Documentation Viewer**
  - Side panel with docs
  - Search functionality
  - Example code

- [ ] **IntelliSense Documentation**
  - Show docs in completion
  - Show docs in signature help

#### Implementation
```rust
// src/lsp/hover.rs
pub struct HoverProvider;

impl HoverProvider {
    pub fn get_hover(&self, position: Position, source: &str) -> Option<Hover> {
        // Find symbol at position
        let symbol = self.find_symbol_at(position, source)?;
        
        // Get documentation
        let docs = self.get_documentation(&symbol)?;
        
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "```widya\n{}\n```\n\n{}",
                    symbol.signature,
                    docs.description
                ),
            }),
            range: Some(symbol.range),
        })
    }
}
```

---

### Phase 4: Code Actions & Refactoring (Week 7-8)
**Priority:** 🟡 Medium  
**Effort:** 6 days  
**Dependencies:** Parser enhancements

#### Features
- [ ] **Quick Fixes**
  - Fix syntax errors
  - Import missing modules
  - Add missing return type

- [ ] **Refactorings**
  - Rename symbol
  - Extract function
  - Extract variable
  - Inline function

- [ ] **Code Generation**
  - Generate function from usage
  - Generate struct from usage
  - Generate match arms

#### Implementation
```rust
// src/lsp/code_action.rs
pub struct CodeActionProvider;

impl CodeActionProvider {
    pub fn get_actions(&self, diagnostics: &[Diagnostic]) -> Vec<CodeAction> {
        let mut actions = Vec::new();
        
        for diagnostic in diagnostics {
            match diagnostic.code.as_deref() {
                Some("undefined-variable") => {
                    actions.push(self.create_variable_action(diagnostic));
                }
                Some("missing-import") => {
                    actions.push(self.import_module_action(diagnostic));
                }
                _ => {}
            }
        }
        
        actions
    }
    
    fn create_variable_action(&self, diagnostic: &Diagnostic) -> CodeAction {
        CodeAction {
            title: "Buat variabel baru".to_string(),
            kind: Some(CodeActionKind::QUICKFIX),
            edit: Some(self.create_workspace_edit(diagnostic)),
            ..Default::default()
        }
    }
}
```

---

### Phase 5: Testing Integration (Week 9-10)
**Priority:** 🟢 Medium  
**Effort:** 4 days  
**Dependencies:** Test framework

#### Features
- [ ] **Test Explorer**
  - List all tests in file
  - Run tests from IDE
  - Show test results

- [ ] **Test Runner**
  - Run single test
  - Run test file
  - Run all tests

- [ ] **Test Coverage**
  - Show coverage in editor
  - Highlight uncovered lines

---

### Phase 6: Profiling & Performance (Week 11-12)
**Priority:** 🟢 Low  
**Effort:** 5 days  
**Dependencies:** Runtime profiling

#### Features
- [ ] **Profiler**
  - CPU profiling
  - Memory profiling
  - Allocation tracking

- [ ] **Performance Insights**
  - Bottleneck detection
  - Optimization suggestions
  - Flame graphs

---

## 🛠️ Developer Tools Enhancement

### 1. REPL Improvements

**Current State:** Basic REPL with multiline support

**Enhancements:**
- [ ] **Syntax Highlighting in REPL**
- [ ] **Auto-completion in REPL** (Tab key)
- [ ] **Multi-line Editing** (better brace handling)
- [ ] **REPL Commands:**
  - `.beban <file>` - Load file into REPL
  - `.simpan <file>` - Save REPL session
  - `.waktu` - Show execution time
  - `.profil` - Profile execution
  - `.bantuan <topik>` - Topic-specific help
  - `.contoh <fitur>` - Show examples
  - `.dokumen <simbol>` - Show documentation

```rust
// src/repl/commands.rs
pub enum ReplCommand {
    Load(PathBuf),
    Save(PathBuf),
    Time(bool),
    Profile(bool),
    Help(Option<String>),
    Example(String),
    Doc(String),
    Clear,
    Exit,
}
```

### 2. CLI Improvements

**Current Commands:**
- `widya run <file>`
- `widya kompilasi <file> -o <output>`
- `widya wasm <file>`
- `widya llvm <file>`

**New Commands:**
- [ ] `widya baru <nama>` - Create new project
- [ ] `widya cek <file>` - Quick syntax check
- [ ] `widya format <file>` - Format code
- [ ] `widya tes <file>` - Run tests
- [ ] `widya doc <file>` - Generate docs
- [ ] `widya dep add <package>` - Add dependency
- [ ] `widya dep list` - List dependencies
- [ ] `widya build --release` - Build release
- [ ] `widya lint <file>` - Lint code

```bash
# Example usage
widya baru myproject
cd myproject
widya run src/main.wya
widya tes
widya build --release
```

### 3. Error Message Improvements

**Current Error:**
```
Error: Sintaks Parser: Expected expression
  baris 5, kolom 10
```

**Improved Error:**
```
❌ Error: Expected expression after '='

   3 | fungsi hitung(a, b) {
   4 |     hasil = 
   5 |            ^ Expected value here
   6 |     kembalikan hasil
   7 | }

💡 Suggestion: Did you mean to assign a value?
   Example: hasil = a + b

📖 Documentation: https://widya-lang.org/docs/variables
```

**Implementation:**
```rust
// src/error/pretty.rs
pub struct PrettyError {
    message: String,
    source_line: String,
    location: Location,
    suggestion: Option<String>,
    documentation: Option<String>,
}

impl PrettyError {
    pub fn format(&self) -> String {
        let mut output = format!("\n❌ Error: {}\n\n", self.message);
        
        output += &format!("   {} |\n", self.location.line - 1);
        output += &format!("   {} | {}\n", self.location.line, self.source_line);
        output += &format!("     {}{}\n", " ".repeat(self.location.column), "^".red());
        output += &format!("   {} |\n", self.location.line + 1);
        
        if let Some(suggestion) = &self.suggestion {
            output += &format!("\n💡 Suggestion: {}\n", suggestion);
        }
        
        if let Some(doc) = &self.documentation {
            output += &format!("\n📖 Documentation: {}\n", doc);
        }
        
        output
    }
}
```

---

## 📚 Documentation Experience

### 1. Inline Documentation

```widya
/// Menghitung faktorial dari sebuah angka
/// 
/// # Contoh
/// ```widya
/// hasil = faktorial(5)  // 120
/// ```
/// 
/// # Parameter
/// - `n` - Angka yang akan dihitung faktorialnya
/// 
/// # Kembalian
/// Hasil faktorial dari n
fungsi faktorial(n) {
    jika n <= 1 {
        kembalikan 1
    }
    kembalikan n * faktorial(n - 1)
}
```

### 2. Documentation Website

Generate static documentation site:
- Landing page with features
- Language reference
- Standard library docs
- Tutorials
- Examples
- Playground

### 3. Interactive Examples

Each documentation page includes:
- Editable code examples
- Live execution
- Output visualization

---

## 🎨 IDE Experience

### VS Code Extension Enhancements

**Current:**
- Basic syntax highlighting
- File icons

**Future:**
- Full LSP integration
- Debugger integration
- Test explorer
- Project templates
- Snippets
- Code lenses

```json
// package.json snippets
"contributes": {
  "snippets": [
    {
      "language": "widya",
      "path": "./snippets/widya.json"
    }
  ]
}
```

```json
// snippets/widya.json
{
  "Function": {
    "prefix": "fungsi",
    "body": [
      "fungsi ${1:nama}(${2:params}) {",
      "    ${3:// body}",
      "}"
    ],
    "description": "Buat fungsi baru"
  },
  "If Statement": {
    "prefix": "jika",
    "body": [
      "jika ${1:kondisi} {",
      "    ${2:// body}",
      "}"
    ],
    "description": "Statement kondisional"
  }
}
```

---

## 🎓 Learning Experience

### 1. Interactive Tutorials

Built-in tutorial system in REPL:
```
widya> .tutorial mulai

📚 Tutorial 1: Variabel dan Tipe Data

Mari mulai dengan membuat variabel:

widya> nama = "Widya"

Bagus! Sekarang coba tampilkan:

widya> cetak(nama)

✅ Benar! Sekarang lanjut ke langkah berikutnya...
```

### 2. Example Gallery

```bash
widya contoh daftar
widya contoh lihat fibonacci
widya contoh jalankan sorting
```

### 3. Error Learning Mode

```
widya> x = 

❌ Error: Expected expression

🤔 Belajar: Variabel harus memiliki nilai.
   
   Contoh benar:
   x = 10        # Angka
   x = "halo"    # Teks
   x = benar     # Boolean

💡 Tips: Ketik '.bantuan variabel' untuk info lebih lanjut
```

---

## 📊 Developer Experience Metrics

### Success Metrics

| Metric | Current | Target | Timeline |
|--------|---------|--------|----------|
| LSP Features | 2 | 15+ | 2 weeks |
| Debugger Features | 0 | 10+ | 4 weeks |
| Error Message Quality | Basic | Helpful | 2 weeks |
| Documentation Coverage | 30% | 80% | 6 weeks |
| Test Integration | 0% | 100% | 3 weeks |
| REPL Features | 5 | 20+ | 4 weeks |
| Build Time | 0.34s | < 0.3s | Ongoing |
| IDE Response Time | N/A | < 100ms | Ongoing |

### User Satisfaction Goals

- [ ] 90% of users can write first program in < 5 minutes
- [ ] 80% of errors have helpful suggestions
- [ ] 95% of users rate error messages as "helpful"
- [ ] 85% of users can debug without external help
- [ ] 90% of users can find documentation quickly

---

## 🚀 Implementation Timeline

### Sprint 1: LSP Foundation (Sep 18 - Oct 1)
- Week 1: Auto-completion, Hover
- Week 2: Go to Definition, Find References

### Sprint 2: Debugger (Oct 2 - Oct 15)
- Week 3: Breakpoints, Step Execution
- Week 4: Variable Inspection, DAP

### Sprint 3: Documentation (Oct 16 - Oct 29)
- Week 5: Inline Docs, Hover Docs
- Week 6: Documentation Website

### Sprint 4: Code Actions (Oct 30 - Nov 12)
- Week 7: Quick Fixes
- Week 8: Refactorings

### Sprint 5: Testing & Profiling (Nov 13 - Nov 26)
- Week 9: Test Explorer, Test Runner
- Week 10: Profiling Tools

### Sprint 6: Polish & Launch (Nov 27 - Dec 10)
- Week 11: Error Messages, REPL
- Week 12: Documentation, Examples

---

## 🎯 Quick Wins (Implement Now)

These features can be implemented quickly with high impact:

### 1. Better Error Messages (1 day)
- Add suggestions to common errors
- Add documentation links
- Improve formatting

### 2. REPL Commands (1 day)
- Add `.waktu` command
- Add `.beban` command
- Add `.contoh` command

### 3. CLI Enhancements (1 day)
- Add `widya cek` command
- Add `widya format` command
- Add colored output

### 4. VS Code Snippets (1 day)
- Create comprehensive snippets
- Add keyboard shortcuts
- Improve syntax highlighting

### 5. Example Programs (1 day)
- Add 20 example programs
- Organize by topic
- Add comments and docs

---

## 📝 Developer Experience Checklist

### Immediate (This Week)
- [ ] Add error suggestions
- [ ] Add REPL `.waktu` command
- [ ] Add REPL `.contoh` command
- [ ] Create 20 example programs
- [ ] Improve error formatting

### Short Term (Next 2 Weeks)
- [ ] Implement auto-completion
- [ ] Implement hover information
- [ ] Implement go to definition
- [ ] Add VS Code snippets
- [ ] Add `widya cek` command

### Medium Term (Next Month)
- [ ] Implement debugger
- [ ] Implement find references
- [ ] Add inline documentation
- [ ] Create documentation website
- [ ] Add code actions

### Long Term (Next 3 Months)
- [ ] Implement refactorings
- [ ] Add test explorer
- [ ] Add profiler
- [ ] Create interactive tutorials
- [ ] Launch playground

---

**Next Step:** Start with Quick Wins - better error messages and REPL commands
