use widya::ast::*;
use widya::error::Galat;
use widya::lexer::Lexer;
use widya::parser::Parser;

fn parse_program(input: &str) -> Result<Program, Galat> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.scan_tokens()?;
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[test]
fn test_parse_simple_variable_declaration() {
    let code = "misal x = 42;";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::VarDecl { name, initializer, is_const, .. } => {
            assert_eq!(name, "x");
            assert!(!*is_const);
            assert!(initializer.is_some());
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_variable_with_type_annotation() {
    let code = "misal x: Angka = 100;";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::VarDecl { name, .. } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected VarDecl"),
    }
}

#[test]
fn test_parse_const_with_type_annotation() {
    let code = "tetap PI: Desimal = 3.14159;";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::VarDecl { name, is_const, .. } => {
            assert_eq!(name, "PI");
            assert!(*is_const);
        }
        _ => panic!("Expected const VarDecl"),
    }
}

#[test]
fn test_parse_generic_function_simple() {
    let code = "fungsi identitas<T>(nilai: T) -> T { kembalikan nilai; }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, .. } => {
            assert_eq!(name, "identitas");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "nilai");
        }
        _ => panic!("Expected FunctionDecl"),
    }
}

#[test]
fn test_parse_generic_function_with_bounds() {
    let code = "fungsi cetak_data<T: Tunjukkan + Sama>(item: T) { }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, .. } => {
            assert_eq!(name, "cetak_data");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "item");
        }
        _ => panic!("Expected FunctionDecl with bounds"),
    }
}

#[test]
fn test_parse_generic_function_with_where_clause() {
    let code = "fungsi bandingkan<T, U>(a: T, b: U) -> Boolean dimana T: Sama, U: Urutan { kembalikan benar; }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, params, .. } => {
            assert_eq!(name, "bandingkan");
            assert_eq!(params.len(), 2);
        }
        _ => panic!("Expected FunctionDecl with where clause"),
    }
}

#[test]
fn test_parse_generic_struct() {
    let code = "struktur Pasangan<A, B> { pertama: A, kedua: B }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::StructDecl { name, fields, .. } => {
            assert_eq!(name, "Pasangan");
            assert_eq!(fields.len(), 2);
            assert_eq!(fields[0], "pertama");
            assert_eq!(fields[1], "kedua");
        }
        _ => panic!("Expected StructDecl"),
    }
}

#[test]
fn test_parse_generic_enum() {
    let code = "pilihan Opsi<T> { Ada(T), Kosong }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::EnumDecl { name, variants, .. } => {
            assert_eq!(name, "Opsi");
            assert_eq!(variants.len(), 2);
            assert_eq!(variants[0].name, "Ada");
            assert_eq!(variants[0].fields_count, 1);
            assert_eq!(variants[1].name, "Kosong");
            assert_eq!(variants[1].fields_count, 0);
        }
        _ => panic!("Expected EnumDecl"),
    }
}

#[test]
fn test_parse_match_statement_wildcard() {
    let code = "cocokkan nilai { 1 => \"satu\", _ => \"lainnya\" }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Expression(Expr::Match { arms, .. }) => {
            assert_eq!(arms.len(), 2);
            assert!(matches!(arms[1].pattern, MatchPattern::Wildcard(_)));
        }
        _ => panic!("Expected Match Statement"),
    }
}

#[test]
fn test_parse_match_with_identifier() {
    let code = "cocokkan angka { x => \"nilai\", _ => \"lain\" }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Expression(Expr::Match { arms, .. }) => {
            assert_eq!(arms.len(), 2);
            assert!(matches!(arms[0].pattern, MatchPattern::Identifier(ref name, _) if name == "x"));
        }
        _ => panic!("Expected Match with Identifier"),
    }
}

#[test]
fn test_parse_match_enum_variant() {
    let code = "cocokkan hasil { Sukses(pesan) => pesan, Gagal(kode) => kode }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::Expression(Expr::Match { arms, .. }) => {
            assert_eq!(arms.len(), 2);
            match &arms[0].pattern {
                MatchPattern::EnumVariant { variant_name, bindings, .. } => {
                    assert_eq!(variant_name, "Sukses");
                    assert_eq!(bindings, &vec!["pesan".to_string()]);
                }
                _ => panic!("Expected EnumVariant pattern"),
            }
        }
        _ => panic!("Expected Match Statement"),
    }
}

#[test]
fn test_parse_derived_traits() {
    let code = "#[turunkan(Tunjukkan, Sama, Salin)]\nstruktur Titik { x: Angka, y: Angka }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::StructDecl { name, attributes, .. } => {
            assert_eq!(name, "Titik");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].name, "turunkan");
            assert_eq!(attributes[0].arguments, vec!["Tunjukkan", "Sama", "Salin"]);
        }
        _ => panic!("Expected StructDecl with derived traits"),
    }
}

#[test]
fn test_parse_test_attribute_function() {
    let code = "#[uji]\nfungsi uji_tambah() { pastikan_sama(1 + 1, 2); }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::FunctionDecl { name, attributes, .. } => {
            assert_eq!(name, "uji_tambah");
            assert_eq!(attributes.len(), 1);
            assert_eq!(attributes[0].name, "uji");
        }
        _ => panic!("Expected FunctionDecl with test attribute"),
    }
}

#[test]
fn test_parse_while_loop() {
    let code = "selama i < 10 { i = i + 1; }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(&program.statements[0], Stmt::While { .. }));
}

#[test]
fn test_parse_for_in_loop() {
    let code = "untuk item di daftar { tulis(item); }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(&program.statements[0], Stmt::ForIn { .. }));
}

#[test]
fn test_parse_if_else_chain() {
    let code = "jika x > 0 { tulis(\"positif\"); } jika_tidak jika x < 0 { tulis(\"negatif\"); } jika_tidak { tulis(\"nol\"); }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(&program.statements[0], Stmt::If { .. }));
}

#[test]
fn test_parse_try_catch() {
    let code = "coba { resiko(); } tangkap galat { tulis(galat); }";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(&program.statements[0], Stmt::TryCatch { .. }));
}

#[test]
fn test_parse_function_expression() {
    let code = "misal f = fungsi(a, b) { kembalikan a + b; };";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::VarDecl { initializer: Some(Expr::FunctionExpr { params, .. }), .. } => {
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
        }
        _ => panic!("Expected function expression"),
    }
}

#[test]
fn test_parse_complex_binary_precedence() {
    let code = "misal hasil = 2 + 3 * 4 > 10 && benar;";
    let program = parse_program(code).expect("Parsing failed");
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Stmt::VarDecl { initializer: Some(Expr::Binary { op, .. }), .. } => {
            assert_eq!(op, &BinaryOp::And);
        }
        _ => panic!("Expected Binary expression with highest logical and"),
    }
}
