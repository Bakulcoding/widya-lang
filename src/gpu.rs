use crate::ast::*;
use crate::error::Galat;

pub struct GpuShaderEmitter {
    buffer: String,
}

impl GpuShaderEmitter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn emit_wgsl(&mut self, program: &Program) -> Result<String, Galat> {
        self.buffer.clear();
        self.buffer.push_str("// === WebGPU Shading Language (WGSL) Compute Shader ===\n");
        self.buffer.push_str("// Generated automatically by Widya-Lang GPU Emitter\n\n");

        self.buffer.push_str("@group(0) @binding(0) var<storage, read> bufferA: array<f32>;\n");
        self.buffer.push_str("@group(0) @binding(1) var<storage, read> bufferB: array<f32>;\n");
        self.buffer.push_str("@group(0) @binding(2) var<storage, read_write> bufferOut: array<f32>;\n\n");

        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, params, .. } = stmt {
                self.buffer.push_str(&format!("// Widya Function: {}\n", name));
                self.buffer.push_str(&format!("fn widya_fn_{}(", name));
                let p_str = params.iter().map(|p| format!("{}: f32", p)).collect::<Vec<_>>().join(", ");
                self.buffer.push_str(&p_str);
                self.buffer.push_str(") -> f32 {\n");
                self.buffer.push_str("    return ");
                if params.len() >= 2 {
                    self.buffer.push_str(&format!("{} * {};\n", params[0], params[1]));
                } else if !params.is_empty() {
                    self.buffer.push_str(&format!("{} * 2.0;\n", params[0]));
                } else {
                    self.buffer.push_str("1.0;\n");
                }
                self.buffer.push_str("}\n\n");
            }
        }

        self.buffer.push_str("@compute @workgroup_size(64)\n");
        self.buffer.push_str("fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {\n");
        self.buffer.push_str("    let idx: u32 = global_id.x;\n");
        self.buffer.push_str("    let a: f32 = bufferA[idx];\n");
        self.buffer.push_str("    let b: f32 = bufferB[idx];\n");
        self.buffer.push_str("    bufferOut[idx] = a * b;\n");
        self.buffer.push_str("}\n");

        Ok(self.buffer.clone())
    }
}
