pub mod ffds;
pub mod len_label;
pub mod logger;
mod tag_set;
pub mod tag_set_wrap;
pub mod track;

use crate::logger::Logger;
pub use crate::{logger::get_log_data, tag_set::TagSet};

pub type DfsanLabel = u32;
extern "C" {
    fn dfsan_read_label(addr: *const i8, size: usize) -> DfsanLabel;
    static mut __angora_cond_cmpid: u32;
    static mut __angora_context: u32;
// static __angora_tid: u32;
}

#[inline(always)]
pub fn get_context() -> u32 {
    unsafe { __angora_context }
}
