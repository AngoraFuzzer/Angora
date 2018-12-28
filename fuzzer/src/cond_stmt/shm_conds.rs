use super::CondOutput;
use crate::cond_stmt;
use angora_common::{cond_stmt_base::CondStmtBase, defs, shm};
use std;

pub struct ShmConds {
    pub cond: shm::SHM<CondStmtBase>,
}

impl ShmConds {
    pub fn new() -> Self {
        Self {
            cond: shm::SHM::<CondStmtBase>::new(),
        }
    }

    #[inline(always)]
    pub fn get_id(&self) -> i32 {
        self.cond.get_id()
    }

    #[inline(always)]
    fn get_len(&self) -> usize {
        self.cond.level as usize
    }

    #[inline(always)]
    fn set_len(&mut self, len: usize) {
        self.cond.level = len as u32;
    }

    #[inline(always)]
    fn reset_reachable_state(&mut self) {
        self.cond.lb1 = std::u32::MAX;
    }

    #[inline(always)]
    pub fn is_cond_reachable(&self) -> bool {
        self.cond.lb1 < std::u32::MAX
    }

    pub fn set(&mut self, cond: &cond_stmt::CondStmt) -> bool {
        if self.get_len() == 0 {
            *self.cond = cond.base.clone();
            self.set_len(1);
            self.reset_reachable_state();
            true
        } else {
            self.reset_reachable_state();
            false
        }
    }

    pub fn clear(&mut self) {
        self.cond.cmpid = 0;
        self.cond.order = 0;
        self.cond.context = 0;
        self.set_len(0);
    }

    pub fn get_cond_output(&self) -> u64 {
        if !self.is_cond_reachable() {
            debug!("unreachable, output is MAX");
            return defs::UNREACHABLE;
        }
        let mut output = self.cond.get_output();
        if output == defs::UNREACHABLE {
            output -= 1;
        }
        output
    }
}
