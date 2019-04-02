#include "branch_pred.h"
#include "cond_stmt.h"
#include "debug.h"
#include "logger.h"
#include "pin.H"
#include "syscall_hook.h"

Logger logger;

VOID CmpHandler(THREADID tid, u32 cid, u32 ctx, u32 size, u32 op, u64 arg1,
                u64 arg2, u32 cond) {

  tag_t t1 = tagmap_getn_reg(tid, X64_ARG4_REG, size);
  tag_t t2 = tagmap_getn_reg(tid, X64_ARG5_REG, size);

  u32 ctr = logger.get_order(cid, ctx);
  // LOGD("[cmp] tid: %d, cid : %d, size: %d, ctr: %d, args (%ld, %ld) \n", tid,
  //     cid, size, ctr, arg1, arg2);
  if (BDD_HAS_LEN_LB(t1) || BDD_HAS_LEN_LB(t2)) {
    if (ctr <= MAX_ORDER) {
      u32 len_ctr = ctr + 0x10000;
      CondStmt stmt = {cid,         ctx,  len_ctr, 0, cond, 0,
                       COND_LEN_OP, size, 0,       1, arg1, arg2};
      logger.save_cond(stmt);
    }
    BDD_CLEAR_LEN_MASK(t1);
    BDD_CLEAR_LEN_MASK(t2);
  }

  if (tag_is_empty(t1) && tag_is_empty(t2)) {
    return;
    // LOGD("[cmp] cid: %d, tag is empty\n", cid);
  }

  if (ctr <= MAX_ORDER) {
    LOGD("[cmp] cid: %d, ctx: %d, size: %d, op: %d, cond: %d, arg1: %lu, arg2: "
         "%lu, t1(%u): %s, t2(%u): %s \n",
         cid, ctx, size, op, cond, arg1, arg2, t1, tag_sprint(t1).c_str(), t2,
         tag_sprint(t2).c_str());

    CondStmt stmt = {cid, ctx, ctr, 0, cond, 0, op, size, t1, t2, arg1, arg2};
    logger.save_cond(stmt);
  }
}

VOID SwHandler(THREADID tid, u32 cid, u32 ctx, u32 size, u64 cond, u32 num,
               u64 *args) {

  tag_t t = tagmap_getn_reg(tid, X64_ARG3_REG, size);
  u32 ctr = logger.get_order(cid, ctx);

  BDD_CLEAR_LEN_MASK(t);
  if (tag_is_empty(t)) {
    return;
  }

  if (ctr <= MAX_ORDER) {
    LOGD("[switch] cid: %d, ctx: %d, size: %d, cond: %lu, t: %s,\n", cid, ctx,
         size, cond, tag_sprint(t).c_str());

    CondStmt stmt = {cid,        ctx,  ctr, 0, COND_FALSE_ST, 0,
                     COND_SW_OP, size, t,   0, cond,          0};

    for (u32 i = 0; i < num; i++) {
      stmt.order = ctr + (i << 16);
      stmt.arg2 = args[i];
      if (stmt.arg1 == stmt.arg2) {
        stmt.condition = COND_DONE_ST;
      } else {
        stmt.condition = COND_FALSE_ST;
      }
      logger.save_cond(stmt);
    }
  }
}

// can be track in pin?
VOID FnHandler(THREADID tid, u32 cid, u32 ctx, u32 size, char *arg1,
               char *arg2) {

  u32 arg1_len = size;
  u32 arg2_len = size;
  if (size == 0) {
    arg1_len = strlen(arg1);
    arg2_len = strlen(arg2);
  }

  tag_t t1 = tagmap_getn((ADDRINT)arg1, arg1_len);
  tag_t t2 = tagmap_getn((ADDRINT)arg2, arg2_len);

  BDD_CLEAR_LEN_MASK(t1);
  BDD_CLEAR_LEN_MASK(t2);

  u32 ctr = logger.get_order(cid, ctx);

  if (ctr <= MAX_ORDER) {
    if (!tag_is_empty(t1)) {
      CondStmt stmt = {cid, ctx, ctr, 0, COND_FALSE_ST, 0, COND_FN_OP, arg2_len,
                       t1,  0,   0,   0};
      u32 cond_idx = logger.save_cond(stmt);
      logger.save_mb(cond_idx, arg1_len, arg2_len, arg1, arg2);
    } else if (!tag_is_empty(t2)) {
      CondStmt stmt = {cid, ctx, ctr, 0, COND_FALSE_ST, 0, COND_FN_OP, arg1_len,
                       0,   t2,  0,   0};
      u32 cond_idx = logger.save_cond(stmt);
      logger.save_mb(cond_idx, arg1_len, arg2_len, arg1, arg2);
    }
  }
}

VOID ExploitHandler(THREADID tid, u32 cid, u32 ctx, u32 size, u32 op, u64 val) {
  tag_t t = tagmap_getn_reg(tid, X64_ARG4_REG, size);
  u32 ctr = logger.get_order(cid, ctx);
  // TODO: len-based exploitation
  BDD_CLEAR_LEN_MASK(t);
  if (tag_is_empty(t)) {
    return;
  }

  if (ctr <= MAX_ORDER) {
    LOGD("[exploit] cid: %d, ctx: %d, size: %d, op: %d, val: %lu, t(%d): %s,\n",
         cid, ctx, size, op, val, t, tag_sprint(t).c_str());

    CondStmt stmt = {cid, ctx,  ctr, 0, COND_FALSE_ST, 0,
                     op,  size, t,   0, val,           0};
    logger.save_cond(stmt);
  }
}

VOID EntryPoint(VOID *v) {

  for (IMG img = APP_ImgHead(); IMG_Valid(img); img = IMG_Next(img)) {

    RTN cmp_rtn = RTN_FindByName(img, "__angora_trace_cmp_tt");

    if (RTN_Valid(cmp_rtn)) {
      RTN_Open(cmp_rtn);
      RTN_InsertCall(
          cmp_rtn, IPOINT_BEFORE, (AFUNPTR)CmpHandler, IARG_THREAD_ID,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 4, IARG_FUNCARG_ENTRYPOINT_VALUE, 5,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 6, IARG_END);
      RTN_Close(cmp_rtn);
    }

    RTN sw_rtn = RTN_FindByName(img, "__angora_trace_switch_tt");
    if (RTN_Valid(sw_rtn)) {
      RTN_Open(sw_rtn);
      RTN_InsertCall(
          sw_rtn, IPOINT_BEFORE, (AFUNPTR)SwHandler, IARG_THREAD_ID,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 4, IARG_FUNCARG_ENTRYPOINT_VALUE, 5,
          IARG_END);
      RTN_Close(sw_rtn);
    }
    RTN fn_rtn = RTN_FindByName(img, "__angora_trace_fn_tt");

    if (RTN_Valid(fn_rtn)) {
      RTN_Open(fn_rtn);
      RTN_InsertCall(
          fn_rtn, IPOINT_BEFORE, (AFUNPTR)FnHandler, IARG_THREAD_ID,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 4, IARG_END);
      RTN_Close(fn_rtn);
    }

    RTN exploit_rtn = RTN_FindByName(img, "__angora_trace_exploit_val_tt");
    if (RTN_Valid(exploit_rtn)) {
      RTN_Open(exploit_rtn);
      RTN_InsertCall(
          exploit_rtn, IPOINT_BEFORE, (AFUNPTR)ExploitHandler, IARG_THREAD_ID,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 4, IARG_END);
      RTN_Close(exploit_rtn);
    }
  }
}

VOID Fini(INT32 code, VOID *v) {
  logger.save_buffers();
  LOGD("[pin] finish \n");
}

int main(int argc, char *argv[]) {

  PIN_InitSymbols();

  if (unlikely(PIN_Init(argc, argv))) {
    LOGE("Sth error in PIN_Init. Plz use the right command line options.");
    return -1;
  }

  if (unlikely(libdft_init() != 0)) {
    LOGE("Sth error libdft_init.");
    return -1;
  }

  PIN_AddApplicationStartFunction(EntryPoint, 0);

  hook_file_syscall();

  PIN_AddFiniFunction(Fini, 0);

  PIN_StartProgram();

  return 0;
}
