#include <stdint.h>

typedef uint32_t u32;
#ifdef __x86_64__
typedef unsigned long long u64;
#else
typedef uint64_t u64;
#endif

void __angora_trace_cmp_tt(u32 cid, u32 ctx, u32 size, u32 op, u64 arg1,
                           u64 arg2, u32 cond) {}

void __angora_trace_switch_tt(u32 cid, u32 ctx, u32 size, u64 cond, u32 num,
                              u64 *args) {}

void __angora_trace_fn_tt(u32 cid, u32 ctx, u32 size, char *arg1, char *arg2) {}

void __angora_trace_exploit_val_tt(u32 cid, u32 ctx, u32 size, u32 op,
                                   u64 val) {}
