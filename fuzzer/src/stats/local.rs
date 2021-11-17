use super::*;
use crate::{cond_stmt::CondStmt, executor::StatusType, fuzz_type::FuzzType};

#[derive(Default)]
pub struct LocalStats {
    pub fuzz_type: FuzzType,

    pub num_exec: Counter,
    pub num_inputs: Counter,
    pub num_hangs: Counter,
    pub num_crashes: Counter,

    pub track_time: TimeDuration,
    pub start_time: TimeIns,

    pub avg_exec_time: SyncAverage,
    pub avg_edge_num: SyncAverage,
}

impl LocalStats {
    pub fn register(&mut self, cond: &CondStmt) {
        self.fuzz_type = cond.get_fuzz_type();
        self.clear();
    }

    pub fn clear(&mut self) {
        self.num_exec = Default::default();
        self.num_inputs = Default::default();
        self.num_hangs = Default::default();
        self.num_crashes = Default::default();

        self.start_time = Default::default();
        self.track_time = Default::default();
    }

    pub fn find_new(&mut self, status: &StatusType) {
        match status {
            StatusType::Normal => {
                self.num_inputs.count();
            },
            StatusType::Timeout => {
                self.num_hangs.count();
            },
            StatusType::Crash => {
                self.num_crashes.count();
            },
            _ => {},
        }
    }
}
