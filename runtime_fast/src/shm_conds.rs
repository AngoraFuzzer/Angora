// corresponding to fuzzer/src/cond_stmt/shm_conds.rs

use super::context;
use angora_common::{cond_stmt_base::CondStmtBase, defs, shm};
use lazy_static::lazy_static;
use std::{env, ops::DerefMut, process, sync::Mutex};

#[no_mangle]
static mut __angora_cond_cmpid: u32 = 0;

#[inline(always)]
fn set_cmpid(cid: u32) {
    unsafe {
        __angora_cond_cmpid = cid;
    }
}

pub struct ShmConds {
    cond: shm::SHM<CondStmtBase>,
    rt_order: u32,
}

// shm contains pointer..
unsafe impl Send for ShmConds {}

// Drop in common/shm.rs:
// Though SHM<T> implement "drop" function, but it won't call (as we want) since ShmConds is in lazy_static!
impl ShmConds {
    pub fn get_from_env_id() -> Option<Self> {
        let id_val = env::var(defs::COND_STMT_ENV_VAR);
        match id_val {
            Ok(val) => {
                let shm_id = val.parse::<i32>().expect("Could not parse i32 value.");
                let cond = shm::SHM::<CondStmtBase>::from_id(shm_id);
                if cond.is_fail() {
                    process::exit(1);
                }
                Some(Self { cond, rt_order: 0 })
            },
            Err(_) => None,
        }
    }

    #[inline(always)]
    fn mark_reachable(&mut self, condition: u32) {
        self.cond.lb1 = condition;
    }

    pub fn check_match(&mut self, cmpid: u32, context: u32) -> bool {
        if self.cond.cmpid == cmpid && self.cond.context == context {
            self.rt_order += 1;
            if self.cond.order & 0xFFFF == self.rt_order {
                return true;
            }
        }
        false
    }

    pub fn update_cmp(&mut self, condition: u32, arg1: u64, arg2: u64) -> u32 {
        self.cond.arg1 = arg1;
        self.cond.arg2 = arg2;
        self.rt_order = 0x8000;
        self.mark_reachable(condition);
        set_cmpid(0);
        condition
    }

    pub fn update_switch(&mut self, condition: u64) -> u64 {
        self.cond.arg1 = condition;
        self.rt_order = 0x8000;
        self.mark_reachable((condition == self.cond.arg2) as u32);
        set_cmpid(0);
        condition
    }

    pub fn reset(&mut self) {
        self.rt_order = 0;
        set_cmpid(self.cond.cmpid);
    }
}

lazy_static! {
    pub static ref SHM_CONDS: Mutex<Option<ShmConds>> = Mutex::new(ShmConds::get_from_env_id());
}

#[inline(always)]
pub fn reset_shm_conds() {
    let mut conds = SHM_CONDS.lock().expect("SHM mutex poisoned.");
    match conds.deref_mut() {
        &mut Some(ref mut c) => {
            c.reset();
        },
        _ => {},
    }

    unsafe {
        context::reset_context();
    }
}
