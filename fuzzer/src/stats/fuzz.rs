use super::*;
use crate::cond_stmt::CondStmt;
use serde_derive::Serialize;

#[derive(Clone, Copy, Default, Serialize)]
pub struct StrategyStats {
    pub time: TimeDuration,
    pub num_conds: Counter,
    pub num_exec: Counter,
    pub num_inputs: Counter,
    pub num_hangs: Counter,
    pub num_crashes: Counter,
}

impl fmt::Display for StrategyStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "CONDS: {}, EXEC: {}, TIME: {}, FOUND: {} - {} - {}",
            self.num_conds,
            self.num_exec,
            self.time,
            self.num_inputs,
            self.num_hangs,
            self.num_crashes,
        )
    }
}

#[derive(Clone, Default, Serialize)]
pub struct FuzzStats([StrategyStats; fuzz_type::FUZZ_TYPE_NUM]);

impl FuzzStats {
    #[inline]
    pub fn get_mut(&mut self, i: usize) -> &mut StrategyStats {
        assert!(i < fuzz_type::FUZZ_TYPE_NUM);
        &mut self.0[i]
    }

    pub fn clear(&mut self) {
        for s in self.0.iter_mut() {
            s.num_conds = Default::default();
        }
    }

    pub fn count(&mut self, cond: &CondStmt) {
        self.0[cond.get_fuzz_type().index()].num_conds.count();
    }
}

impl fmt::Display for FuzzStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let contents = self
            .0
            .iter()
            .enumerate()
            .map(|(i, s)| {
                format!(
                    "  {:>8} | {}",
                    fuzz_type::get_fuzz_type_name(i).to_uppercase(),
                    s
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", contents)
    }
}
