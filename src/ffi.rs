use crate::error::Galat;
use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FfiType {
    Void,
    Int32,
    Int64,
    Float32,
    Float64,
    Bool,
    String,
    Pointer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FfiFunctionSignature {
    pub name: String,
    pub symbol: String,
    pub library: String,
    pub param_types: Vec<FfiType>,
    pub return_type: FfiType,
    pub is_variadic: bool,
}

#[derive(Debug, Clone)]
pub struct DynamicLibrary {
    pub name: String,
    pub path: String,
    pub loaded: bool,
    pub exported_symbols: Vec<String>,
}

pub struct FfiManager {
    signatures: HashMap<String, FfiFunctionSignature>,
    libraries: HashMap<String, DynamicLibrary>,
    simulated_memory: Arc<RwLock<HashMap<usize, Vec<u8>>>>,
    next_ptr: usize,
}

impl FfiManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            signatures: HashMap::new(),
            libraries: HashMap::new(),
            simulated_memory: Arc::new(RwLock::new(HashMap::new())),
            next_ptr: 0x1000,
        };
        mgr.register_builtin_c_abi();
        mgr
    }

    fn register_builtin_c_abi(&mut self) {
        // Register standard C libc prototypes
        self.register_signature(FfiFunctionSignature {
            name: "puts".to_string(),
            symbol: "puts".to_string(),
            library: "libc".to_string(),
            param_types: vec![FfiType::String],
            return_type: FfiType::Int32,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "abs".to_string(),
            symbol: "abs".to_string(),
            library: "libc".to_string(),
            param_types: vec![FfiType::Int32],
            return_type: FfiType::Int32,
            is_variadic: false,
        });

        self.register_signature(FfiFunctionSignature {
            name: "sqrt".to_string(),
            symbol: "sqrt".to_string(),
            library: "libm".to_string(),
            param_types: vec![FfiType::Float64],
            return_type: FfiType::Float64,
            is_variadic: false,
        });
    }

    pub fn load_library(&mut self, name: &str, path: &str) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path pustaka dinamis tidak boleh kosong".to_string());
        }

        self.libraries.insert(
            name.to_string(),
            DynamicLibrary {
                name: name.to_string(),
                path: path.to_string(),
                loaded: true,
                exported_symbols: Vec::new(),
            },
        );
        Ok(())
    }

    pub fn register_signature(&mut self, sig: FfiFunctionSignature) {
        self.signatures.insert(sig.name.clone(), sig);
    }

    pub fn get_signature(&self, name: &str) -> Option<&FfiFunctionSignature> {
        self.signatures.get(name)
    }

    pub fn allocate_raw(&mut self, bytes: &[u8]) -> usize {
        let ptr = self.next_ptr;
        self.next_ptr += bytes.len() + 16;
        let mut mem = self.simulated_memory.write().unwrap();
        mem.insert(ptr, bytes.to_vec());
        ptr
    }

    pub fn read_string_ptr(&self, ptr: usize) -> Result<String, Galat> {
        let mem = self.simulated_memory.read().unwrap();
        if let Some(bytes) = mem.get(&ptr) {
            let s = String::from_utf8_lossy(bytes).trim_end_matches('\0').to_string();
            Ok(s)
        } else {
            Err(Galat::runtime_polos(format!("Segmentation fault: Pointer 0x{:X} tidak valid", ptr)))
        }
    }

    pub fn call_foreign_function(&mut self, name: &str, args: &[Value]) -> Result<Value, Galat> {
        let sig = match self.signatures.get(name) {
            Some(s) => s.clone(),
            None => {
                return Err(Galat::runtime_polos(format!(
                    "Fungsi FFI eksternal '{}' belum dideklarasikan",
                    name
                )));
            }
        };

        if sig.param_types.len() != args.len() && !sig.is_variadic {
            return Err(Galat::runtime_polos(format!(
                "FFI {}: Jumlah argumen tidak sesuai. Diharapkan {}, ditemukan {}",
                name,
                sig.param_types.len(),
                args.len()
            )));
        }

        // Handle C math/libc primitives
        match sig.symbol.as_str() {
            "abs" => {
                let val = match args.first() {
                    Some(Value::Number(n)) => n.abs(),
                    _ => 0.0,
                };
                Ok(Value::Number(val))
            }
            "sqrt" => {
                let val = match args.first() {
                    Some(Value::Number(n)) => n.sqrt(),
                    _ => 0.0,
                };
                Ok(Value::Number(val))
            }
            "puts" => {
                let text = match args.first() {
                    Some(Value::String(s)) => s.clone(),
                    Some(v) => v.to_debug_repr(),
                    None => "".to_string(),
                };
                println!("{}", text);
                Ok(Value::Number(text.len() as f64))
            }
            _ => {
                // Dynamic execution simulation for registered libraries
                match sig.return_type {
                    FfiType::Void => Ok(Value::Nil),
                    FfiType::Int32 | FfiType::Int64 | FfiType::Float32 | FfiType::Float64 => {
                        Ok(Value::Number(0.0))
                    }
                    FfiType::Bool => Ok(Value::Bool(true)),
                    FfiType::String => Ok(Value::String("ffi_result".to_string())),
                    FfiType::Pointer => Ok(Value::Number(self.next_ptr as f64)),
                }
            }
        }
    }
}

impl Default for FfiManager {
    fn default() -> Self {
        Self::new()
    }
}
