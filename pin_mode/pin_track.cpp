#include "branch_pred.h"
#include "debug.h"
#include "libdft_api.h"
#include "pin.H"
#include "syscall_hook.h"

typedef uint32_t u32;
typedef uint64_t u64;

// KNOB<string> KnobOutputFile(KNOB_MODE_WRITEONCE, "pintool", "o", "",
//                             "specify file name for output");

VOID CmpHandler(THREADID tid, u32 cid, u32 ctx, u32 size, u32 op, u64 arg1,
                u64 arg2, u32 cond) {

  tag_t t1 = tagmap_getn_reg(tid, X64_ARG4_REG, 8);
  tag_t t2 = tagmap_getn_reg(tid, X64_ARG5_REG, 8);

  if (tag_is_empty(t1) && tag_is_empty(t2)) {
    LOGD("[cmp] cid: %d, tag is empty\n", cid);
  }

  LOGD("[cmp] cid: %d, ctx: %d, size: %d, op: %d, cond: %d, arg1: %lu, arg2: "
       "%lu, t1: %s, t2: %s \n",
       cid, ctx, size, op, cond, arg1, arg2, tag_sprint(t1).c_str(),
       tag_sprint(t2).c_str());
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
    /*
    RTN sw_rtn = RTN_FindByName(img, "__angora_trace_switch_tt");
    RTN fn_rtn = RTN_FindByName(img, "__angora_trace_fn_tt");
    RTN so_rtn = RTN_FindByName(img, "__angora_trace_exploit_val_tt");

    if (RTN_Valid(fn_rtn)) {
      RTN_Open(fn_rtn);
      RTN_InsertCall(
          fn_rtn, IPOINT_BEFORE, (AFUNPTR)FnHandler,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 4, IARG_END);
      RTN_Close(fn_rtn);
    }

    if (RTN_Valid(sw_rtn)) {
      RTN_Open(sw_rtn);
      RTN_InsertCall(
          sw_rtn, IPOINT_BEFORE, (AFUNPTR)SwHandler,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_REFERENCE,
          3, IARG_FUNCARG_ENTRYPOINT_VALUE, 5, IARG_FUNCARG_ENTRYPOINT_VALUE, 6,
          IARG_END);
      RTN_Close(sw_rtn);
    }

    if (RTN_Valid(so_rtn)) {
      RTN_Open(so_rtn);
      RTN_InsertCall(
          so_rtn, IPOINT_BEFORE, (AFUNPTR)SoHandler,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 0, IARG_FUNCARG_ENTRYPOINT_VALUE, 1,
          IARG_FUNCARG_ENTRYPOINT_VALUE, 2, IARG_FUNCARG_ENTRYPOINT_VALUE, 3,
          IARG_FUNCARG_ENTRYPOINT_REFERENCE, 4, IARG_END);
      RTN_Close(so_rtn);
    }
    */
  }
}

/*
VOID Fini(INT32 code, VOID *v) {
  const string fileName = KnobOutputFile.Value();
  if (!fileName.empty()) {
    delete out;
  }
}
*/
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

  /*
    const string fileName = KnobOutputFile.Value();

    if (!fileName.empty()) {
      out = new std::ofstream(fileName.c_str());
    }
  */
  PIN_AddApplicationStartFunction(EntryPoint, 0);

  hook_file_syscall();

  PIN_StartProgram();

  return 0;
}
