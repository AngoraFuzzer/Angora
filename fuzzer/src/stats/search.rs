use super::*;
use crate::cond_stmt::CondStmt;
use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
struct ConstraintStats {
    pub num_cmp: Counter,
    pub num_bool: Counter,
    pub num_switch: Counter,
}

#[derive(Clone, Default, Serialize)]
struct ConstraintPairStats {
    done: ConstraintStats,
    all: ConstraintStats,
}

#[derive(Clone, Default, Serialize)]
pub struct SearchStats {
    sch: ConstraintPairStats,
    undesirable_sch: ConstraintPairStats,
    linear_sch: ConstraintPairStats,
    onebyte_sch: ConstraintPairStats,
    inconsistent_sch: ConstraintPairStats,
}

impl ConstraintStats {
    pub fn find(&mut self, cond: &CondStmt) {
        if cond.base.is_switch() {
            self.num_switch.count();
        } else if cond.is_bool() {
            self.num_bool.count();
        } else {
            self.num_cmp.count();
        }
    }
}

impl fmt::Display for ConstraintStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CMP: {}, BOOL: {}, SW: {}",
            self.num_cmp, self.num_bool, self.num_switch
        )
    }
}

impl ConstraintPairStats {
    fn find(&mut self, cond: &CondStmt) {
        if cond.is_done() {
            self.done.find(cond);
        }
        self.all.find(cond);
    }
}

impl fmt::Display for ConstraintPairStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CMP: {} / {}, BOOL: {} / {}, SW: {} / {}",
            self.done.num_cmp,
            self.all.num_cmp,
            self.done.num_bool,
            self.all.num_bool,
            self.done.num_switch,
            self.all.num_switch
        )
    }
}

impl SearchStats {
    pub fn count(&mut self, cond: &CondStmt) {
        self.sch.find(cond);
        if !cond.is_desirable {
            self.undesirable_sch.find(cond);
        }
        if !cond.is_consistent {
            self.inconsistent_sch.find(cond);
        }
        if cond.linear {
            self.linear_sch.find(cond);
        }
        if cond.state.is_one_byte() {
            self.onebyte_sch.find(cond);
        }
    }

    pub fn multiple_inconsist(&self) -> bool {
        self.inconsistent_sch.all.num_cmp.0 * 2 > self.sch.all.num_cmp.0
    }
}

impl fmt::Display for SearchStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"    SEARCH | {}
   UNDESIR | {}
   ONEBYTE | {}
  INCONSIS | {}"#,
            self.sch, self.undesirable_sch, self.onebyte_sch, self.inconsistent_sch,
        )
    }
}
