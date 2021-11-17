use super::*;
use crate::{branches::GlobalBranches, depot::Depot};
use colored::*;
use serde_derive::Serialize;
use std::sync::Arc;

#[derive(Default, Serialize)]
pub struct ChartStats {
    init_time: TimeIns,
    track_time: TimeDuration,
    density: Average,

    num_rounds: Counter,
    max_rounds: Counter,
    num_exec: Counter,
    speed: Average,

    avg_exec_time: Average,
    avg_edge_num: Average,

    num_inputs: Counter,
    num_hangs: Counter,
    num_crashes: Counter,

    fuzz: FuzzStats,
    search: SearchStats,
    state: StateStats,
}

impl ChartStats {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn sync_from_local(&mut self, local: &mut LocalStats) {
        self.track_time += local.track_time;
        self.num_rounds.count();

        local.avg_edge_num.sync(&mut self.avg_edge_num);
        local.avg_exec_time.sync(&mut self.avg_exec_time);

        let st = self.fuzz.get_mut(local.fuzz_type.index());
        st.time += local.start_time.into();
        // st.num_conds.count();

        st.num_exec += local.num_exec;
        self.num_exec += local.num_exec;
        // if has new
        st.num_inputs += local.num_inputs;
        self.num_inputs += local.num_inputs;
        st.num_hangs += local.num_hangs;
        self.num_hangs += local.num_hangs;
        st.num_crashes += local.num_crashes;
        self.num_crashes += local.num_crashes;

        //local.clear();
    }

    pub fn sync_from_global(&mut self, depot: &Arc<Depot>, gb: &Arc<GlobalBranches>) {
        self.get_speed();
        self.iter_pq(depot);
        self.sync_from_branches(gb);
    }

    fn iter_pq(&mut self, depot: &Arc<Depot>) {
        let q = match depot.queue.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("Lock poisoned. Results can be incorrect! Continuing...");
                poisoned.into_inner()
            },
        };
        self.search = Default::default();
        self.state = Default::default();
        self.fuzz.clear();
        let mut max_round = 0;
        for (item, _) in q.iter() {
            if item.fuzz_times > max_round {
                max_round = item.fuzz_times;
            }
            self.fuzz.count(&item);
            if item.base.is_explore() {
                self.search.count(&item);
                self.state.count(&item);
            }
        }
        self.max_rounds = max_round.into();
    }

    fn sync_from_branches(&mut self, gb: &Arc<GlobalBranches>) {
        self.density = Average::new(gb.get_density(), 0);
    }

    fn get_speed(&mut self) {
        let t: TimeDuration = self.init_time.into();
        let d: time::Duration = t.into();
        let ts = d.as_secs() as f64;
        let speed = if ts > 0.0 {
            let v: usize = self.num_exec.into();
            v as f64 / ts
        } else {
            0.0
        };
        self.speed = Average::new(speed as f32, 0);
    }

    pub fn mini_log(&self) -> String {
        format!(
            "{}, {}, {}, {}, {}",
            self.init_time.0.elapsed().as_secs(),
            self.density.0,
            self.num_inputs.0,
            self.num_hangs.0,
            self.num_crashes.0
        )
    }

    pub fn get_explore_num(&self) -> usize {
        self.fuzz
            .get(fuzz_type::FuzzType::ExploreFuzz.index())
            .num_conds
            .into()
    }
}

impl fmt::Display for ChartStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.density.0 > 10.0 {
            warn!("Density is too large (> 10%). Please increase `MAP_SIZE_POW2` in and `common/src/config.rs`. Or disable function-call context(density > 50%) by compiling with `ANGORA_CUSTOM_FN_CONTEXT=k` (k is an integer and 0 <= k <= 32) environment variable. Angora disables context if k is 0.");
        }

        if self.search.multiple_inconsist() {
            warn!("Multiple inconsistent warnings. It caused by the fast and track programs has different behaviors. If most constraints are inconsistent, ensure they are compiled with the same environment. Otherwise, please report us.");
            // panic()!
        }

        if self.fuzz.may_be_model_failure() {
            warn!("Find small number constraints, please make sure you have modeled the read functions.")
        }

        write!(
            f,
            r#"
{}
{}
    TIMING |     RUN: {},   TRACK: {}
  COVERAGE |    EDGE: {},   DENSITY: {}%
    EXECS  |   TOTAL: {},     ROUND: {},     MAX_R: {}
    SPEED  |  PERIOD: {:6}r/s    TIME: {}us, 
    FOUND  |    PATH: {},     HANGS: {},   CRASHES: {}
{}
{}
{}
{}
{}
{}

"#,
            get_bunny_logo().bold(),
            " -- OVERVIEW -- ".blue().bold(),
            self.init_time,
            self.track_time,
            self.avg_edge_num,
            self.density,
            self.num_exec,
            self.num_rounds,
            self.max_rounds,
            self.speed,
            self.avg_exec_time,
            self.num_inputs,
            self.num_hangs,
            self.num_crashes,
            " -- FUZZ -- ".blue().bold(),
            self.fuzz,
            " -- SEARCH -- ".blue().bold(),
            self.search,
            " -- STATE -- ".blue().bold(),
            self.state,
        )
    }
}
