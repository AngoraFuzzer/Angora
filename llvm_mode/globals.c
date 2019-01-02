
#include "./config.h"

u32 __angora_cond_cmpid;
__thread u32 __angora_prev_loc;
__thread u32 __angora_context;
__thread u32 __angora_level;
// __thread u32 __angora_tid;

u32 __angora_get_context() { return __angora_context; }

void __angora_set_cmpid(u32 id) { __angora_cond_cmpid = id; }

void __angora_reset_globals() {
  __angora_prev_loc = 0;
  __angora_context = 0;
  __angora_level = 0;
}

void __unfold_branch_fn(u32 x) {}