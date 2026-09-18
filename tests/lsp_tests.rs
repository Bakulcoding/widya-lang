use widya::lsp::{Position, WidyaLsp};

#[test]
fn test_lsp_diagnostics_valid_code() {
    let source = "misal x = 10\ncetak(x)\n";
    let diags = WidyaLsp::get_diagnostics(source);
    assert!(diags.is_empty(), "Should produce no diagnostics for valid code");
}

#[test]
fn test_lsp_diagnostics_syntax_error() {
    let source = "misal x = \n";
    let diags = WidyaLsp::get_diagnostics(source);
    assert!(!diags.is_empty(), "Should produce diagnostic for syntax error");
    assert_eq!(diags[0].severity as u8, 1); // 1 = Error
}

#[test]
fn test_lsp_completions_keywords_and_variables() {
    let lsp = WidyaLsp::new();
    let source = "misal nama_lengkap = \"Widya\"\nmisal umur = 25\n";
    let pos = Position { line: 3, character: 1 };
    let completions = lsp.get_completions(pos, source);
    
    let labels: Vec<String> = completions.into_iter().map(|c| c.label).collect();
    
    // Check keywords
    assert!(labels.contains(&"misal".to_string()));
    assert!(labels.contains(&"fungsi".to_string()));
    assert!(labels.contains(&"jika".to_string()));
    assert!(labels.contains(&"kembalikan".to_string()));
    
    // Check standard functions
    assert!(labels.contains(&"cetak".to_string()));
    assert!(labels.contains(&"panjang".to_string()));
}

#[test]
fn test_lsp_hover_keywords_and_stdlib() {
    let lsp = WidyaLsp::new();
    let source = "misal angka = 100\ncetak(angka)\n";
    
    // Hover on 'cetak' at line 2, char 3
    let hover_cetak = lsp.get_hover(Position { line: 2, character: 3 }, source);
    assert!(hover_cetak.is_some());
    let info = hover_cetak.unwrap();
    assert!(info.contents.contains("cetak"));

    // Hover on 'misal' at line 1, char 2
    let hover_misal = lsp.get_hover(Position { line: 1, character: 2 }, source);
    assert!(hover_misal.is_some());
    let info_misal = hover_misal.unwrap();
    assert!(info_misal.contents.contains("misal"));
}

#[test]
fn test_lsp_find_definition_and_references() {
    let lsp = WidyaLsp::new();
    let source = "misal hitung = 42\ncetak(hitung)\nmisal total = hitung + 10\n";
    
    // Definition of 'hitung' when hovering at line 2, char 8 (in 'cetak(hitung)')
    let def = lsp.find_definition(Position { line: 2, character: 8 }, source);
    assert!(def.is_some(), "Should find definition of 'hitung'");
    let loc = def.unwrap();
    assert_eq!(loc.range.start.line, 1); // defined on line 1

    // References to 'hitung'
    let refs = lsp.find_references(Position { line: 1, character: 8 }, source);
    assert_eq!(refs.len(), 3, "Should find declaration and 2 usages of 'hitung'");
}

#[test]
fn test_lsp_document_symbols() {
    let lsp = WidyaLsp::new();
    let source = "misal skor = 100\nfungsi hitung_luas(p, l) {\n  kembalikan p * l\n}\nstruktur Titik {\n  x: Int,\n  y: Int\n}\n";
    let symbols = lsp.get_document_symbols(source);
    
    let names: Vec<String> = symbols.iter().map(|s| s.name.clone()).collect();
    assert!(names.contains(&"skor".to_string()));
    assert!(names.contains(&"hitung_luas".to_string()));
    assert!(names.contains(&"Titik".to_string()));
}
