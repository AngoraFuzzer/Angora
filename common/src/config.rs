// ************ Switches **************
// length
pub const ENABLE_INPUT_LEN_EXPLORATION: bool = true;
pub const ENABLE_RANDOM_LEN: bool = false;
pub const ENABLE_MICRO_RANDOM_LEN: bool = true;

// other
pub const DISABLE_INFER_SHAPE_IF_HAS_AND_OP: bool = true;
pub const PREFER_FAST_COND: bool = true;

// ************ Resources ****************
pub const MAX_INPUT_LEN: usize = 15000;

// branch.rs
pub const MAP_SIZE_POW2: usize = 20;
pub const BRANCHES_SIZE: usize = 1 << MAP_SIZE_POW2;

// executor.rs:
pub const TMOUT_SKIP: usize = 3;
pub const TIME_LIMIT: u64 = 1;
pub const MEM_LIMIT: u64 = 200; // MB
pub const TIME_LIMIT_TRACK: u64 = 12;
pub const MEM_LIMIT_TRACK: u64 = 0;
pub const LONG_FUZZ_TIME: usize = 8;
pub const MAX_INVARIABLE_NUM: usize = 16;
pub const MAX_NUM_MINIMAL_OPTIMA_ALL: usize = 28;
// based the bit bucket: [1], [2], [3], [4, 7], [8, 15], [16, 31], [32, 127], [128, infinity]
pub const MAX_COND_ORDER: u32 = 16;

// ************ Mutation ****************
// SEARCH
pub const ENABLE_DET_MUTATION: bool = true;
pub const MAX_SEARCH_EXEC_NUM: usize = 376;
pub const MAX_EXPLOIT_EXEC_NUM: usize = 66;
pub const MAX_NUM_MINIMAL_OPTIMA_ROUND: usize = 8;
pub const MAX_RANDOM_SAMPLE_NUM: usize = 10;
pub const GD_MOMENTUM_BETA: f64 = 0.0;
pub const GD_ESCAPE_RATIO: f64 = 1.0;
pub const BONUS_EXEC_NUM: usize = 66;

// AFL
pub const MUTATE_ARITH_MAX: u32 = 30;
pub const RANDOM_LEN_NUM: usize = 30;
pub const MAX_HAVOC_FLIP_TIMES: usize = 45; // for all bytes
pub const MAX_SPLICE_TIMES: usize = 45;
