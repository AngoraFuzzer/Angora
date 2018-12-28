use super::*;
use crate::cond_stmt::{CondState, CondStmt};
use serde_derive::Serialize;

#[derive(Clone, Default, Serialize)]
struct PendingCounter {
    pub pending: Counter,
    pub done: Counter,
}

impl PendingCounter {
    pub fn count(&mut self, done: bool) {
        if done {
            self.done.count();
        } else {
            self.pending.count();
        }
    }
}

impl fmt::Display for PendingCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}d - {}p", self.done, self.pending)
    }
}

#[derive(Default, Serialize)]
pub struct StateStats {
    normal: PendingCounter,
    normal_end: PendingCounter,
    det: PendingCounter,
    one_byte: PendingCounter,
    unsolvable: PendingCounter,
    timeout: PendingCounter,
}

impl StateStats {
    pub fn count(&mut self, cond: &CondStmt) {
        let is_done = cond.is_done();
        match cond.state {
            CondState::Offset | CondState::OffsetOpt | CondState::OffsetAll => {
                self.normal.count(is_done);
            },
            CondState::OffsetAllEnd => {
                self.normal_end.count(is_done);
            },
            CondState::OneByte => {
                self.one_byte.count(is_done);
            },
            CondState::Unsolvable => {
                self.unsolvable.count(is_done);
            },
            CondState::Deterministic => {
                self.det.count(is_done);
            },
            CondState::Timeout => {
                self.timeout.count(is_done);
            },
        }
    }
}

impl fmt::Display for StateStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            r#"           |    NORMAL: {},   NORMAL_END: {},   ONE_BYTE: {}
           |       DET: {},    TIMEOUT: {},     UNSOLVABLE: {}"#,
            self.normal, self.normal_end, self.one_byte, self.det, self.timeout, self.unsolvable,
        )
    }
}
