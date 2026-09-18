use widya::stdlib_ffi::sqlite::{SqliteConnection, SqliteStatus};
use widya::stdlib_ffi::crypto::Crypto;
use widya::wasm_playground::WasmPlayground;

#[test]
fn test_sqlite_ffi_wrapper_operations() {
    let mut db = SqliteConnection::open("memory.db").expect("Harus berhasil membuka sqlite");
    assert!(db.is_open);

    let create_status = db.execute("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);");
    assert_eq!(create_status, SqliteStatus::Ok);

    let insert_status = db.execute("INSERT INTO users VALUES (1, 'Alice');");
    assert_eq!(insert_status, SqliteStatus::Ok);

    let rows = db.query("SELECT * FROM users;").expect("Query harus berhasil");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].columns.get("id").unwrap(), "1");

    db.close();
    assert!(!db.is_open);
}

#[test]
fn test_crypto_ffi_wrapper_operations() {
    let raw = b"Widya-Lang Enterprise Crypto Test";
    let hash = Crypto::sha256_hex(raw);
    assert_eq!(hash.len(), 64); // SHA-256 adalah 64 karakter hex

    let encoded = Crypto::base64_encode(raw);
    let decoded = Crypto::base64_decode(&encoded).expect("Base64 decode harus berhasil");
    assert_eq!(decoded, raw);

    let hmac = Crypto::hmac_sha256(b"secret_key", raw);
    assert_eq!(hmac.len(), 64);

    let token = Crypto::generate_token("widya_usr");
    assert_eq!(token.len(), 64);
}

#[test]
fn test_wasm_playground_pipeline() {
    let valid_code = "misal x = 10; misal y = 20; kembalikan x + y;";
    let res = WasmPlayground::evaluate(valid_code);
    assert!(res.success, "Evaluation failed with errors: {:?}", res.errors);
    assert!(res.ast_json.is_some());
}
