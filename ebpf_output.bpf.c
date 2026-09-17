// === Linux Kernel eBPF (Extended Berkeley Packet Filter) Program ===
// Generated automatically by Widya-Lang eBPF Emitter

#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, __u32);
    __type(value, __u64);
} packet_counter_map SEC(".maps");

char _license[] SEC("license") = "GPL";
