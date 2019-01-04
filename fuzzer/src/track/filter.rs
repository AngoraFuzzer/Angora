/*
  filter undesirable conds
*/

use crate::cond_stmt::CondStmt;
use angora_common::{cond_stmt_base::CondStmtBase, config, defs};
use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

struct CondArgs {
    pub cmpid: u32,
    pub lb1: u32,
    pub lb2: u32,
    pub arg1: u64,
    pub arg2: u64,
}

impl PartialEq for CondArgs {
    fn eq(&self, other: &CondArgs) -> bool {
        self.cmpid == other.cmpid
            && self.lb1 == other.lb1
            && self.lb2 == other.lb2
            && self.arg1 == other.arg1
            && self.arg2 == other.arg2
    }
}

impl Eq for CondArgs {}

impl CondArgs {
    fn from(condbase: &CondStmtBase) -> Self {
        Self {
            cmpid: condbase.cmpid,
            lb1: condbase.lb1,
            lb2: condbase.lb2,
            arg1: condbase.arg1,
            arg2: condbase.arg2,
        }
    }
}

impl Hash for CondArgs {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cmpid.hash(state);
        self.lb1.hash(state);
        self.lb2.hash(state);
        self.arg1.hash(state);
        self.arg2.hash(state);
    }
}

#[inline(always)]
fn has_no_taint(cond: &CondStmt) -> bool {
    cond.base.op != defs::COND_LEN_OP && cond.offsets.is_empty()
}

#[inline(always)]
fn exceed_max_order(cond: &CondStmt) -> bool {
    (cond.base.order & 0xFFFF) > config::MAX_COND_ORDER
}

#[inline(always)]
fn size_not_match(cond: &CondStmt) -> bool {
    (cond.base.is_explore() || cond.base.is_exploitable())
        && (cond.base.size != 1
            && cond.base.size != 2
            && cond.base.size != 4
            && cond.base.size != 8)
}

fn filter_eof(cond: &CondStmt) -> bool {
    cond.base.op & 0xFF == defs::COND_ICMP_EQ_OP
        && cond.base.arg2 == 18446744073709551615
        && cond.offsets.len() == 1
        && cond.base.arg1 < 256
        && cond.base.size == 4
}

pub fn filter_cond_list(cond_list: &mut Vec<CondStmt>) {
    // mark conds we don;t use in future to be undesirable
    // those undesirable ones won't be added to depot in `depot.rs`
    let mut exploitable_labels = HashSet::new();
    let mut unique_conds = HashSet::new();
    let mut dedup_exploit = 0;
    let mut dedup_explore = 0;

    for cond in cond_list {
        if has_no_taint(cond) || exceed_max_order(cond) || size_not_match(cond) || filter_eof(cond)
        {
            cond.is_desirable = false;
        } else if cond.base.is_exploitable() {
            // We try to (maximize or random mutate) the values in the exploitable offsets.
            // So we can simply de-dup them by their taint labels.
            if exploitable_labels.contains(&cond.base.lb1) {
                cond.is_desirable = false;
                dedup_exploit += 1;
            } else {
                exploitable_labels.insert(cond.base.lb1);
            }
        } else if cond.base.is_explore() || cond.base.op == defs::COND_LEN_OP {
            // de-dup explore conds. including len ops.
            // If conds have different context || order but the same cmpid && args && labels --> they are the same
            let condargs = CondArgs::from(&cond.base);
            if unique_conds.contains(&condargs) {
                cond.is_desirable = false;
                dedup_explore += 1;
            } else {
                unique_conds.insert(condargs);
            }
        }
    }

    debug!(
        "de-dup exploit: {}, explore: {}",
        dedup_exploit, dedup_explore
    );
}
