use crate::ast::{Stmt, StructMethod};
use crate::environment::Environment;
use crate::error::Galat;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type BuiltinFn = fn(&[Value], &crate::error::Span) -> Result<Value, Galat>;

#[derive(Clone)]
pub struct WidyaFunction {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

#[derive(Clone)]
pub struct BuiltinFunction {
    pub name: String,
    pub arity: Option<usize>, // None for variadic
    pub func: BuiltinFn,
}

#[derive(Clone)]
pub struct WidyaStruct {
    pub name: String,
    pub fields: Vec<String>,
    pub methods: HashMap<String, StructMethod>,
    pub closure: Rc<RefCell<Environment>>,
}

#[derive(Clone)]
pub struct WidyaInstance {
    pub struct_def: Rc<WidyaStruct>,
    pub fields: HashMap<String, Value>,
}

#[derive(Clone)]
pub struct WidyaEnumVariant {
    pub enum_name: String,
    pub variant_name: String,
    pub fields: Vec<Value>,
}

#[derive(Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<HashMap<String, Value>>>),
    Function(Rc<WidyaFunction>),
    Builtin(Rc<BuiltinFunction>),
    StructDef(Rc<WidyaStruct>),
    Instance(Rc<RefCell<WidyaInstance>>),
    EnumVariant(Rc<WidyaEnumVariant>),
    Range(i64, i64),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.borrow().is_empty(),
            Value::Map(m) => !m.borrow().is_empty(),
            Value::Function(_)
            | Value::Builtin(_)
            | Value::StructDef(_)
            | Value::Instance(_)
            | Value::EnumVariant(_)
            | Value::Range(_, _) => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nihil",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "angka",
            Value::String(_) => "teks",
            Value::Array(_) => "daftar",
            Value::Map(_) => "kamus",
            Value::Function(_) => "fungsi",
            Value::Builtin(_) => "fungsi_bawaan",
            Value::StructDef(_) => "definisi_struktur",
            Value::Instance(_) => "objek_struktur",
            Value::EnumVariant(_) => "varian_enum",
            Value::Range(_, _) => "rentang",
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Nil => "nihil".to_string(),
            Value::Bool(b) => {
                if *b {
                    "benar"
                } else {
                    "salah"
                }
                .to_string()
            }
            Value::Number(n) => {
                if n.fract() == 0.0
                    && !n.is_infinite()
                    && !n.is_nan()
                    && *n >= (i64::MIN as f64)
                    && *n <= (i64::MAX as f64)
                {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::String(s) => s.clone(),
            Value::Array(a) => {
                let items = a.borrow();
                let reprs: Vec<String> = items.iter().map(|item| item.to_debug_repr()).collect();
                format!("[{}]", reprs.join(", "))
            }
            Value::Map(m) => {
                let items = m.borrow();
                let mut pairs: Vec<String> = items
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.to_debug_repr()))
                    .collect();
                pairs.sort();
                format!("{{{}}}", pairs.join(", "))
            }
            Value::Function(f) => {
                if let Some(ref name) = f.name {
                    format!("<fungsi {}>", name)
                } else {
                    "<fungsi anonim>".to_string()
                }
            }
            Value::Builtin(b) => format!("<fungsi bawaan {}>", b.name),
            Value::StructDef(s) => format!("<struktur {}>", s.name),
            Value::Instance(inst) => {
                let instance = inst.borrow();
                let mut fields_str: Vec<String> = instance
                    .fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_debug_repr()))
                    .collect();
                fields_str.sort();
                format!("{} {{{}}}", instance.struct_def.name, fields_str.join(", "))
            }
            Value::EnumVariant(ev) => {
                if ev.fields.is_empty() {
                    format!("{}::{}", ev.enum_name, ev.variant_name)
                } else {
                    let field_strs: Vec<String> = ev.fields.iter().map(|f| f.to_debug_repr()).collect();
                    format!("{}::{}({})", ev.enum_name, ev.variant_name, field_strs.join(", "))
                }
            }
            Value::Range(start, end) => format!("{}..{}", start, end),
        }
    }

    pub fn to_debug_repr(&self) -> String {
        match self {
            Value::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            _ => self.to_string_repr(),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => *a.borrow() == *b.borrow(),
            (Value::Map(a), Value::Map(b)) => *a.borrow() == *b.borrow(),
            (Value::EnumVariant(a), Value::EnumVariant(b)) => {
                a.enum_name == b.enum_name && a.variant_name == b.variant_name && a.fields == b.fields
            }
            (Value::Range(a1, a2), Value::Range(b1, b2)) => a1 == b1 && a2 == b2,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}
