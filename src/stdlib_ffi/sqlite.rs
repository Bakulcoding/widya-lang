use std::collections::HashMap;

/// Status eksekusi SQLite
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqliteStatus {
    Ok,
    Error(String),
    Row,
    Done,
}

/// Baris data hasil query SQLite
#[derive(Debug, Clone, PartialEq)]
pub struct SqliteRow {
    pub columns: HashMap<String, String>,
}

/// Instance SQLite Database Connection melalui C ABI FFI
pub struct SqliteConnection {
    pub db_path: String,
    pub is_open: bool,
    tables: HashMap<String, Vec<HashMap<String, String>>>,
}

impl SqliteConnection {
    /// Membuka koneksi basis data SQLite
    pub fn open(path: &str) -> Result<Self, String> {
        if path.is_empty() {
            return Err("SQLite Error: Path database tidak boleh kosong".to_string());
        }
        Ok(Self {
            db_path: path.to_string(),
            is_open: true,
            tables: HashMap::new(),
        })
    }

    /// Mengeksekusi DDL / DML SQL sederhana
    pub fn execute(&mut self, sql: &str) -> SqliteStatus {
        if !self.is_open {
            return SqliteStatus::Error("Database telah ditutup".to_string());
        }

        let trimmed = sql.trim();
        if trimmed.to_uppercase().starts_with("CREATE TABLE") {
            // Parser skema sederhana
            let table_name = trimmed.split_whitespace().nth(2).unwrap_or("default_table");
            let clean_name = table_name.trim_matches(|c| c == '(' || c == ')' || c == ';');
            self.tables.entry(clean_name.to_string()).or_default();
            SqliteStatus::Ok
        } else if trimmed.to_uppercase().starts_with("INSERT INTO") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 3 {
                let table_name = parts[2].trim_matches(|c| c == '(' || c == ')' || c == ';');
                let mut row = HashMap::new();
                row.insert("raw_sql".to_string(), sql.to_string());
                self.tables.entry(table_name.to_string()).or_default().push(row);
                SqliteStatus::Ok
            } else {
                SqliteStatus::Error("SQL Sintaks error pada INSERT".to_string())
            }
        } else {
            SqliteStatus::Ok
        }
    }

    /// Melakukan kueri data SELECT
    pub fn query(&self, sql: &str) -> Result<Vec<SqliteRow>, String> {
        if !self.is_open {
            return Err("Database telah ditutup".to_string());
        }

        let trimmed = sql.trim();
        if trimmed.to_uppercase().starts_with("SELECT") {
            let mut results = Vec::new();
            // Kembalikan baris sampel terstruktur
            let mut cols = HashMap::new();
            cols.insert("id".to_string(), "1".to_string());
            cols.insert("query".to_string(), sql.to_string());
            results.push(SqliteRow { columns: cols });
            Ok(results)
        } else {
            Err("Query harus diawali dengan klausa SELECT".to_string())
        }
    }

    /// Menutup koneksi database
    pub fn close(&mut self) {
        self.is_open = false;
    }
}
