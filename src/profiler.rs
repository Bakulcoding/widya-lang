use crate::value::Value;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct AllocationRecord {
    pub object_id: usize,
    pub type_name: String,
    pub size_bytes: usize,
    pub allocated_at: Instant,
    pub references: Vec<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryProfileSnapshot {
    pub total_allocated_bytes: usize,
    pub active_objects_count: usize,
    pub peak_memory_bytes: usize,
    pub allocations_by_type: HashMap<String, usize>,
    pub detected_cycles_count: usize,
}

pub struct MemoryProfiler {
    allocations: HashMap<usize, AllocationRecord>,
    total_allocated: usize,
    peak_memory: usize,
    next_object_id: usize,
}

impl MemoryProfiler {
    pub fn new() -> Self {
        Self {
            allocations: HashMap::new(),
            total_allocated: 0,
            peak_memory: 0,
            next_object_id: 1,
        }
    }

    pub fn track_allocation(&mut self, val: &Value, size_estimate: usize) -> usize {
        let id = self.next_object_id;
        self.next_object_id += 1;

        let record = AllocationRecord {
            object_id: id,
            type_name: val.type_name().to_string(),
            size_bytes: size_estimate,
            allocated_at: Instant::now(),
            references: Vec::new(),
        };

        self.total_allocated += size_estimate;
        if self.total_allocated > self.peak_memory {
            self.peak_memory = self.total_allocated;
        }

        self.allocations.insert(id, record);
        id
    }

    pub fn add_reference(&mut self, from_id: usize, to_id: usize) {
        if let Some(record) = self.allocations.get_mut(&from_id) {
            record.references.push(to_id);
        }
    }

    pub fn track_deallocation(&mut self, object_id: usize) {
        if let Some(rec) = self.allocations.remove(&object_id) {
            self.total_allocated = self.total_allocated.saturating_sub(rec.size_bytes);
        }
    }

    /// Detect circular reference cycles (Tarjan's strongly connected components)
    pub fn detect_reference_cycles(&self) -> Vec<Vec<usize>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut recursion_stack = Vec::new();

        for &id in self.allocations.keys() {
            if !visited.contains(&id) {
                self.dfs_cycle(id, &mut visited, &mut recursion_stack, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycle(
        &self,
        node: usize,
        visited: &mut HashSet<usize>,
        stack: &mut Vec<usize>,
        cycles: &mut Vec<Vec<usize>>,
    ) {
        visited.insert(node);
        stack.push(node);

        if let Some(record) = self.allocations.get(&node) {
            for &neighbor in &record.references {
                if !visited.contains(&neighbor) {
                    self.dfs_cycle(neighbor, visited, stack, cycles);
                } else if let Some(pos) = stack.iter().position(|&x| x == neighbor) {
                    let cycle = stack[pos..].to_vec();
                    cycles.push(cycle);
                }
            }
        }

        stack.pop();
    }

    pub fn generate_snapshot(&self) -> MemoryProfileSnapshot {
        let mut by_type = HashMap::new();
        for rec in self.allocations.values() {
            *by_type.entry(rec.type_name.clone()).or_insert(0) += rec.size_bytes;
        }

        let cycles = self.detect_reference_cycles();

        MemoryProfileSnapshot {
            total_allocated_bytes: self.total_allocated,
            active_objects_count: self.allocations.len(),
            peak_memory_bytes: self.peak_memory,
            allocations_by_type: by_type,
            detected_cycles_count: cycles.len(),
        }
    }

    pub fn format_report(&self) -> String {
        let snap = self.generate_snapshot();
        let mut out = String::new();
        out.push_str("\n🧠 Widya-Lang Memory & Allocation Profiler Report\n");
        out.push_str(&"═".repeat(50));
        out.push('\n');
        out.push_str(&format!("  Memori Aktif           : {:.2} KB\n", snap.total_allocated_bytes as f64 / 1024.0));
        out.push_str(&format!("  Penggunaan Puncak (Peak): {:.2} KB\n", snap.peak_memory_bytes as f64 / 1024.0));
        out.push_str(&format!("  Jumlah Objek Aktif     : {}\n", snap.active_objects_count));
        out.push_str(&format!("  Deteksi Siklus (Cycles): {}\n\n", snap.detected_cycles_count));

        out.push_str("Distribusi Penggunaan Tipe Data:\n");
        for (ty, bytes) in &snap.allocations_by_type {
            out.push_str(&format!("  • {:<15} : {} bytes\n", ty, bytes));
        }

        if snap.detected_cycles_count > 0 {
            out.push_str("\n⚠️ Peringatan: Ditemukan siklus referensi yang berpotensi memicu kebocoran memori!\n");
        } else {
            out.push_str("\n✅ Bebas kebocoran memori siklik (Zero cyclic leaks detected).\n");
        }

        out
    }
}

impl Default for MemoryProfiler {
    fn default() -> Self {
        Self::new()
    }
}
