use crate::shm_conds::*;
use std::ops::DerefMut;

#[no_mangle]
pub extern "C" fn __angora_trace_cmp(condition: u32, cmpid: u32, arg1: u64, arg2: u64) -> u32 {
    //println!("[CMP] id: {}, ctx: {}", cmpid, get_context());
    let mut conds = SHM_CONDS.lock().expect("SHM mutex poisoned.");
    match conds.deref_mut() {
        &mut Some(ref mut c) => {
            if c.check_match(cmpid) {
                return c.update_cmp(condition, arg1, arg2);
            }
        }
        _ => {}
    }
    condition
}

#[no_mangle]
pub extern "C" fn __angora_trace_switch(cmpid: u32, condition: u64) -> u64 {
    let mut conds = SHM_CONDS.lock().expect("SHM mutex poisoned.");
    match conds.deref_mut() {
        &mut Some(ref mut c) => {
            if c.check_match(cmpid) {
                return c.update_switch(condition);
            }
        }
        _ => {}
    }
    condition
}
