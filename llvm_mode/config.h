#ifndef _HAVE_CONFIG_H
#define _HAVE_CONFIG_H

#define MAP_SIZE_POW2 20
#define MAP_SIZE (1 << MAP_SIZE_POW2)
#define MAX_FUNCALL_LEVEL 25
#define ENABLE_UNFOLD_BRANCH 1

#define VERSION "0.00"

// Without taint tracking
#define CLANG_FAST_TYPE 0
// With LLVM's taint tracking and save constraints
#define CLANG_TRACK_TYPE 1
// just do data tracking
#define CLANG_DFSAN_TYPE 2

// #define DEBUG_INFO
// #define ALLOC_PRELOAD

#ifdef DEBUG_INFO
// #define DEBUG_PRINTF printf
#define DEBUG_PRINTF(...) do{ printf( __VA_ARGS__ ); } while( 0 )
#else
#define DEBUG_PRINTF(...) do{ } while ( 0 )
#endif


#define SHM_ENV_VAR "ANGORA_BRANCHES_SHM_ID"
#define ENABLE_FORKSRV "ANGORA_ENABLE_FORKSRV"
#define DISABLE_CTX_VAR "ANGORA_DISABLE_CONTEXT"
#define DIRECT_FN_CTX "ANGORA_DIRECT_FN_CONTEXT"
#define GEN_ID_RANDOM_VAR "ANGORA_GEN_ID_RANDOM"
#define OUTPUT_COND_LOC_VAR "ANGORA_OUTPUT_COND_LOC"
#define TAINT_CUSTOM_RULE_VAR "ANGORA_TAINT_CUSTOM_RULE"
#define TAINT_RULE_LIST_VAR "ANGORA_TAINT_RULE_LIST"
#define FUZZING_INPUT_FILE "cur_input"

#define PERSIST_ENV_VAR "ANGORA_PERSISTENT"
#define DEFER_ENV_VAR "ANGORA_DEFER_FORKSRV"
#define PERSIST_SIG "##SIG_ANGORA_PERSISTENT##"
#define DEFER_SIG "##SIG_ANGORA_DEFER_FORKSRV##"

#define COND_SW_TYPE 0x00FF

#define COND_SIGN_MASK 0x100
#define COND_BOOL_MASK 0x200
#define COND_CALL_MASK 0x400
// #define COND_BR_MASK 0x800

#define COND_EQ_OP 32

#define COND_EXPLOIT_MASK 0x4000

#define COND_FN_TYPE 0x8002
#define COND_LEN_TYPE 0x8003

#ifndef MIN
#define MIN(_a, _b) ((_a) > (_b) ? (_b) : (_a))
#define MAX(_a, _b) ((_a) > (_b) ? (_a) : (_b))
#endif /* !MIN */

#ifndef RRR
#define RRR(x) (random() % (x))
#endif

#include <stdint.h>
#include <stdlib.h>

typedef uint32_t dfsan_label;

typedef uint8_t u8;
typedef uint16_t u16;
typedef uint32_t u32;
// typedef uint64_t u64;
#ifdef __x86_64__
typedef unsigned long long u64;
#else
typedef uint64_t u64;
#endif
typedef int8_t s8;
typedef int16_t s16;
typedef int32_t s32;
typedef int64_t s64;

#endif /* ! _HAVE_CONFIG_H */
