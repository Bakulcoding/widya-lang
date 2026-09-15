use std::process::Command;

#[test]
fn test_advanced_wire_protocols_and_package_resolver() {
    let output = Command::new("cargo")
        .args(&["run", "--", "contoh/159_solusi_postgres_streaming_graphql_dan_paket.wya"])
        .output()
        .expect("Gagal menjalankan contoh 159");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT:\n{}", stdout);
    println!("STDERR:\n{}", stderr);

    assert!(output.status.success(), "Eksekusi contoh 159 harus berhasil");
    assert!(stdout.contains("PostgreSQL Wire Protocol Pipeline:"));
    assert!(stdout.contains("Partitioned Event Streaming Commit Log:"));
    assert!(stdout.contains("Protobuf Binary Encoding & GraphQL Resolver:"));
    assert!(stdout.contains("Semantic Versioning Package Resolver & Lockfile:"));
    assert!(stdout.contains("SEMUA MASALAH TINGKAT LANJUT TERATASI DENGAN SUKSES"));
}
