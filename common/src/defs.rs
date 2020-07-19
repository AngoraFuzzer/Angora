use std;
// -- envs
pub static DISABLE_CPU_BINDING_VAR: &str = "ANGORA_DISABLE_CPU_BINDING";
pub static ANGORA_BIN_DIR: &str = "ANGORA_BIN_DIR";

// executor.rs
pub static TRACK_OUTPUT_VAR: &str = "ANGORA_TRACK_OUTPUT";
pub static COND_STMT_ENV_VAR: &str = "ANGORA_COND_STMT_SHM_ID";
pub static BRANCHES_SHM_ENV_VAR: &str = "ANGORA_BRANCHES_SHM_ID";
pub static LD_LIBRARY_PATH_VAR: &str = "LD_LIBRARY_PATH";
pub static ASAN_OPTIONS_VAR: &str = "ASAN_OPTIONS";
pub static MSAN_OPTIONS_VAR: &str = "MSAN_OPTIONS";
pub static ASAN_OPTIONS_CONTENT: &str =
    "abort_on_error=1:detect_leaks=0:symbolize=0:allocator_may_return_null=1";
pub const MSAN_ERROR_CODE: i32 = 86;
pub static MSAN_OPTIONS_CONTENT: &str =
    "exit_code=86:symbolize=0:abort_on_error=1:allocator_may_return_null=1:msan_track_origins=0";

// depot.rs
pub static CRASHES_DIR: &str = "crashes";
pub static HANGS_DIR: &str = "hangs";
pub static INPUTS_DIR: &str = "queue";

// forksrv.rs
pub static ENABLE_FORKSRV: &str = "ANGORA_ENABLE_FORKSRV";
pub static FORKSRV_SOCKET_PATH_VAR: &str = "ANGORA_FORKSRV_SOCKET_PATH";

// command.rs
pub static ANGORA_DIR_NAME: &str = "angora";
pub static ANGORA_LOG_FILE: &str = "angora.log";
pub static COND_QUEUE_FILE: &str = "cond_queue.csv";
pub static CHART_STAT_FILE: &str = "chart_stat.json";

// tmpfs.rs
pub static PERSIST_TRACK_FILES: &str = "ANGORA_DISABLE_TMPFS";

pub const SLOW_SPEED: u32 = 888888;
pub const UNREACHABLE: u64 = std::u64::MAX;

// ** Cond Type
// < 0xFF: simple if
// http://llvm.org/doxygen/InstrTypes_8h_source.html
// Opcode              U L G E    Intuitive operation
pub const COND_FCMP_FALSE: u32 = 0;
///< 0 0 0 0    Always false (always folded)
pub const COND_FCMP_OEQ: u32 = 1;
///< 0 0 0 1    True if ordered and equal
pub const COND_FCMP_OGT: u32 = 2;
///< 0 0 1 0    True if ordered and greater than
pub const COND_FCMP_OGE: u32 = 3;
///< 0 0 1 1    True if ordered and greater than or equal
pub const COND_FCMP_OLT: u32 = 4;
///< 0 1 0 0    True if ordered and less than
pub const COND_FCMP_OLE: u32 = 5;
///< 0 1 0 1    True if ordered and less than or equal
pub const COND_FCMP_ONE: u32 = 6;
///< 0 1 1 0    True if ordered and operands are unequal
pub const COND_FCMP_ORD: u32 = 7;
///< 0 1 1 1    True if ordered (no nans)
pub const COND_FCMP_UNO: u32 = 8;
///< 1 0 0 0    True if unordered: isnan(X) | isnan(Y)
pub const COND_FCMP_UEQ: u32 = 9;
///< 1 0 0 1    True if unordered or equal
pub const COND_FCMP_UGT: u32 = 10;
///< 1 0 1 0    True if unordered or greater than
pub const COND_FCMP_UGE: u32 = 11;
///< 1 0 1 1    True if unordered; greater than; or equal
pub const COND_FCMP_ULT: u32 = 12;
///< 1 1 0 0    True if unordered or less than
pub const COND_FCMP_ULE: u32 = 13;
///< 1 1 0 1    True if unordered; less than; or equal
pub const COND_FCMP_UNE: u32 = 14;
///< 1 1 1 0    True if unordered or not equal
pub const COND_FCMP_TRUE: u32 = 15;
///< 1 1 1 1    Always true (always folded)

pub const COND_ICMP_EQ_OP: u32 = 32;
pub const COND_ICMP_NE_OP: u32 = 33;
pub const COND_ICMP_UGT_OP: u32 = 34;
pub const COND_ICMP_UGE_OP: u32 = 35;
pub const COND_ICMP_ULT_OP: u32 = 36;
pub const COND_ICMP_ULE_OP: u32 = 37;
pub const COND_ICMP_SGT_OP: u32 = 38;
pub const COND_ICMP_SGE_OP: u32 = 39;
pub const COND_ICMP_SLT_OP: u32 = 40;
pub const COND_ICMP_SLE_OP: u32 = 41;
pub const COND_SW_OP: u32 = 0x00FF;

pub const COND_BASIC_MASK: u32 = 0xFF;
pub const COND_SIGN_MASK: u32 = 0x100;
pub const COND_BOOL_MASK: u32 = 0x200;
// pub const COND_CALL_MASK: u32 = 0x400;
// pub const COND_CALL_REV_MASK: u32 = 0xFBFF;

pub const COND_MAX_EXPLORE_OP: u32 = 0x4000 - 1;
pub const COND_MAX_EXPLOIT_OP: u32 = 0x5000 - 1;

pub const COND_AFL_OP: u32 = 0x8001;
// sensititve offsets
pub const COND_FN_OP: u32 = 0x8002;
pub const COND_LEN_OP: u32 = 0x8003;
// pub const COND_ENTER_FN: u32 = 0x8010;
// pub const COND_LEAVE_FN: u32 = 0x8011;

// condition field
pub const COND_FALSE_ST: u32 = 0;
pub const COND_TRUE_ST: u32 = 1;
pub const COND_DONE_ST: u32 = 2;
