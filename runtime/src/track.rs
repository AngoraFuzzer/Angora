use super::*;
use crate::tag_set_wrap;
use angora_common::{cond_stmt_base::*, defs};
use lazy_static::lazy_static;
use libc;
use std::{slice, sync::Mutex};

// use shm_conds;
lazy_static! {
    static ref LC: Mutex<Option<Logger>> = Mutex::new(Some(Logger::new()));
}

fn infer_eq_sign(op: u32, lb1: u32, lb2: u32) -> u32 {
    if op == defs::COND_ICMP_EQ_OP
        && ((lb1 > 0 && tag_set_wrap::tag_set_get_sign(lb1 as usize))
            || (lb2 > 0 && tag_set_wrap::tag_set_get_sign(lb2 as usize)))
    {
        return op | defs::COND_SIGN_MASK;
    }
    op
}

#[no_mangle]
pub extern "C" fn __dfsw___angora_trace_cmp_tt(
    condition: u32,
    cmpid: u32,
    size: u32,
    op: u32,
    arg1: u64,
    arg2: u64,
    _l0: DfsanLabel,
    _l1: DfsanLabel,
    _l2: DfsanLabel,
    _l3: DfsanLabel,
    l4: DfsanLabel,
    l5: DfsanLabel,
) {
    //println!("[CMP] id: {}, ctx: {}", cmpid, get_context());
    // ret_label: *mut DfsanLabel
    let lb1 = l4;
    let lb2 = l5;
    if lb1 == 0 && lb2 == 0 {
        return;
    }

    let op = infer_eq_sign(op, lb1, lb2);

    log_cmp(cmpid, condition, op, size, lb1, lb2, arg1, arg2, None);
}

#[no_mangle]
pub extern "C" fn __dfsw___angora_trace_switch_tt(
    cmpid: u32,
    size: u32,
    condition: u64,
    num: u32,
    args: *mut u64,
    _l0: DfsanLabel,
    _l1: DfsanLabel,
    l2: DfsanLabel,
    _l3: DfsanLabel,
    _l4: DfsanLabel,
) {
    let lb = l2;
    if lb == 0 {
        return;
    }

    let cond = CondStmtMb {
        base: CondStmtBase {
            cmpid,
            context: get_context(),
            order: 0,
            belong: 0,
            condition: defs::COND_FALSE_ST,
            level: 0,
            op: defs::COND_SW_OP,
            size,
            lb1: lb,
            lb2: 0,
            arg1: condition,
            arg2: 0,
        },
        magic_bytes: None,
    };

    let sw_args = unsafe { slice::from_raw_parts(args, num as usize) };

    let mut lcl = LC.lock().expect("Could not lock LC.");
    if let Some(ref mut lc) = *lcl {
        for (i, arg) in sw_args.iter().enumerate() {
            let mut cond_i = cond.clone();
            cond_i.base.order += (i << 16) as u32;
            cond_i.base.arg2 = *arg;
            if *arg == condition {
                cond_i.base.condition = defs::COND_DONE_ST;
            }
            lc.save(cond_i);
        }
    }
}

#[no_mangle]
pub extern "C" fn __dfsw___angora_trace_fn_tt(
    cmpid: u32,
    size: u32,
    parg1: *mut i8,
    parg2: *mut i8,
    _l0: DfsanLabel,
    _l2: DfsanLabel,
    _l3: DfsanLabel,
    _l4: DfsanLabel,
) {
    let (arglen1, arglen2) = if size == 0 {
        unsafe { (libc::strlen(parg1) as usize, libc::strlen(parg2) as usize) }
    } else {
        (size as usize, size as usize)
    };

    let lb1 = unsafe { dfsan_read_label(parg1, arglen1) };
    let lb2 = unsafe { dfsan_read_label(parg2, arglen2) };

    if lb1 == 0 && lb2 == 0 {
        return;
    }

    let arg1 = unsafe { slice::from_raw_parts(parg1 as *mut u8, arglen1) }.to_vec();
    let arg2 = unsafe { slice::from_raw_parts(parg2 as *mut u8, arglen2) }.to_vec();

    if lb1 > 0 {
        log_cmp(
            cmpid,
            defs::COND_FALSE_ST,
            defs::COND_FN_OP,
            arglen2 as u32,
            lb1,
            0,
            0,
            0,
            Some((arg1, arg2)),
        );
    } else if lb2 > 0 {
        log_cmp(
            cmpid,
            defs::COND_FALSE_ST,
            defs::COND_FN_OP,
            arglen1 as u32,
            0,
            lb2,
            0,
            0,
            Some((arg1, arg2)),
        );
    }
}

#[no_mangle]
pub extern "C" fn __dfsw___angora_trace_exploit_val_tt(
    cmpid: u32,
    size: u32,
    op: u32,
    val: u64,
    _l0: DfsanLabel,
    _l1: DfsanLabel,
    _l2: DfsanLabel,
    l3: DfsanLabel,
) {
    let lb: DfsanLabel = l3;
    if len_label::is_len_label(lb) || lb == 0 {
        return;
    }

    log_cmp(cmpid, defs::COND_FALSE_ST, op, size, lb, 0, val, 0, None);
}

#[inline]
fn log_cmp(
    cmpid: u32,
    condition: u32,
    op: u32,
    size: u32,
    lb1: u32,
    lb2: u32,
    arg1: u64,
    arg2: u64,
    magic_bytes: Option<(Vec<u8>, Vec<u8>)>,
) {
    let cond = CondStmtMb {
        base: CondStmtBase {
            cmpid,
            context: get_context(),
            order: 0,
            belong: 0,
            condition,
            level: 0,
            op,
            size,
            lb1,
            lb2,
            arg1,
            arg2,
        },
        magic_bytes,
    };

    let mut lcl = LC.lock().expect("Could not lock LC.");
    if let Some(ref mut lc) = *lcl {
        lc.save(cond);
    }
}

#[no_mangle]
pub extern "C" fn __angora_track_fini_rs() {
    let mut lcl = LC.lock().expect("Could not lock LC.");
    *lcl = None;
}
