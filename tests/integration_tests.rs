use widya::error::{Galat, Span};
use widya::interpreter::Interpreter;
use widya::lexer::Lexer;
use widya::parser::Parser;
use widya::value::Value;

fn execute_source(source: &str) -> (Interpreter, Result<Value, Galat>) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexing failed");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parsing failed");
    let mut interpreter = Interpreter::new();
    let res = interpreter.interpret(&program);
    (interpreter, res)
}

#[test]
fn test_integration_arithmetic_pipeline() {
    let code = r#"
        misal a = 10;
        misal b = 25;
        misal c = (a * 2) + b - 5;
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("c", &dummy_span).unwrap(),
        Value::Number(40.0)
    );
}

#[test]
fn test_integration_function_definition_and_call() {
    let code = r#"
        fungsi hitung_faktorial(n) {
            jika n <= 1 {
                kembalikan 1;
            }
            kembalikan n * hitung_faktorial(n - 1);
        }
        misal hasil = hitung_faktorial(5);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("hasil", &dummy_span).unwrap(),
        Value::Number(120.0)
    );
}

#[test]
fn test_integration_assertions_stdlib() {
    let code = r#"
        misal x = 100;
        pastikan_sama(x, 100);
        pastikan_benar(x > 50);
        pastikan_salah(x < 50);
        pastikan_beda(x, 200);
    "#;
    let (_, res) = execute_source(code);
    assert!(res.is_ok());
}

#[test]
fn test_integration_control_flow_loops() {
    let code = r#"
        misal total = 0;
        misal i = 1;
        selama i <= 10 {
            total = total + i;
            i = i + 1;
        }
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("total", &dummy_span).unwrap(),
        Value::Number(55.0)
    );
}

#[test]
fn test_integration_struct_instantiation_and_methods() {
    let code = r#"
        struktur Titik {
            x,
            y,
            fungsi hitung_jumlah() {
                kembalikan ini.x + ini.y;
            }
        }
        misal p = Titik(10, 20);
        misal total = p.hitung_jumlah();
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("total", &dummy_span).unwrap(),
        Value::Number(30.0)
    );
}

#[test]
fn test_integration_higher_order_functions() {
    let code = r#"
        fungsi jalankan(f, nilai) {
            kembalikan f(nilai);
        }
        misal kuadrat = fungsi(x) {
            kembalikan x * x;
        };
        misal hasil = jalankan(kuadrat, 6);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("hasil", &dummy_span).unwrap(),
        Value::Number(36.0)
    );
}

#[test]
fn test_integration_array_operations_and_iterations() {
    let code = r#"
        misal angka_angka = [1, 2, 3, 4, 5];
        misal total = 0;
        untuk n dalam angka_angka {
            total = total + n;
        }
        pastikan_sama(total, 15);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("total", &dummy_span).unwrap(),
        Value::Number(15.0)
    );
}

#[test]
fn test_integration_string_concatenation_and_methods() {
    let code = r#"
        misal salam = "Halo, " + "Dunia!";
        pastikan_sama(salam, "Halo, Dunia!");
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("salam", &dummy_span).unwrap(),
        Value::String("Halo, Dunia!".to_string())
    );
}

#[test]
fn test_integration_try_catch_error_handling() {
    let code = r#"
        misal tertangkap = salah;
        coba {
            lempar "Kesalahan fatal";
        } tangkap (err) {
            tertangkap = benar;
        }
        pastikan_benar(tertangkap);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("tertangkap", &dummy_span).unwrap(),
        Value::Bool(true)
    );
}

#[test]
fn test_integration_map_key_value_access() {
    let code = r#"
        misal pengguna = {
            "nama": "Widya",
            "umur": 17
        };
        misal nama = pengguna["nama"];
        misal umur = pengguna["umur"];
        pastikan_sama(nama, "Widya");
        pastikan_sama(umur, 17);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("nama", &dummy_span).unwrap(),
        Value::String("Widya".to_string())
    );
    assert_eq!(
        interp.environment.borrow().get("umur", &dummy_span).unwrap(),
        Value::Number(17.0)
    );
}

#[test]
fn test_integration_nested_function_closures() {
    let code = r#"
        fungsi buat_penambah(tambahan) {
            kembalikan fungsi(x) {
                kembalikan x + tambahan;
            };
        }
        misal tambah_sepuluh = buat_penambah(10);
        misal hasil = tambah_sepuluh(25);
        pastikan_sama(hasil, 35);
    "#;
    let (interp, res) = execute_source(code);
    assert!(res.is_ok());
    let dummy_span = Span::new(0, 0);
    assert_eq!(
        interp.environment.borrow().get("hasil", &dummy_span).unwrap(),
        Value::Number(35.0)
    );
}

