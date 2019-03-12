

#ifndef COND_STMT_H
#define COND_STMT_H

#include <stdint.h>

typedef uint32_t u32;
typedef uint64_t u64;

#define MAX_ORDER 16

struct CondStmt {
  u32 cid;
  u32 context;
  u32 order;
  u32 belong;

  u32 condition;
  u32 level;
  u32 op;
  u32 size;

  u32 lb1;
  u32 lb2;
  u64 arg1;
  u64 arg2;
};

#endif