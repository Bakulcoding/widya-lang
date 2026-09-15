// === WebGPU Shading Language (WGSL) Compute Shader ===
// Generated automatically by Widya-Lang GPU Emitter

@group(0) @binding(0) var<storage, read> bufferA: array<f32>;
@group(0) @binding(1) var<storage, read> bufferB: array<f32>;
@group(0) @binding(2) var<storage, read_write> bufferOut: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx: u32 = global_id.x;
    let a: f32 = bufferA[idx];
    let b: f32 = bufferB[idx];
    bufferOut[idx] = a * b;
}
