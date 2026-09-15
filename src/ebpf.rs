use crate::ast::*;
use crate::error::Galat;

pub struct EbpfEmitter {
    buffer: String,
}

impl EbpfEmitter {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn emit_ebpf_c(&mut self, program: &Program) -> Result<String, Galat> {
        self.buffer.clear();
        self.buffer.push_str("// === Linux Kernel eBPF (Extended Berkeley Packet Filter) Program ===\n");
        self.buffer.push_str("// Generated automatically by Widya-Lang eBPF Emitter\n\n");
        self.buffer.push_str("#include <linux/bpf.h>\n");
        self.buffer.push_str("#include <bpf/bpf_helpers.h>\n\n");

        self.buffer.push_str("struct {\n");
        self.buffer.push_str("    __uint(type, BPF_MAP_TYPE_HASH);\n");
        self.buffer.push_str("    __uint(max_entries, 1024);\n");
        self.buffer.push_str("    __type(key, __u32);\n");
        self.buffer.push_str("    __type(value, __u64);\n");
        self.buffer.push_str("} packet_counter_map SEC(\".maps\");\n\n");

        for stmt in &program.statements {
            if let Stmt::FunctionDecl { name, .. } = stmt {
                self.buffer.push_str(&format!("// Widya eBPF Handler: {}\n", name));
                self.buffer.push_str(&format!("SEC(\"kprobe/{}\")\n", name));
                self.buffer.push_str(&format!("int widya_kprobe_{}(struct pt_regs *ctx) {{\n", name));
                self.buffer.push_str("    __u32 key = 0;\n");
                self.buffer.push_str("    __u64 *val = bpf_map_lookup_elem(&packet_counter_map, &key);\n");
                self.buffer.push_str("    if (val) {\n");
                self.buffer.push_str("        __sync_fetch_and_add(val, 1);\n");
                self.buffer.push_str("    }\n");
                self.buffer.push_str("    return 0;\n");
                self.buffer.push_str("}\n\n");
            }
        }

        self.buffer.push_str("char _license[] SEC(\"license\") = \"GPL\";\n");
        Ok(self.buffer.clone())
    }
}
