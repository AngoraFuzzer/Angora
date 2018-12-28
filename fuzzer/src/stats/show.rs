use super::ChartStats;
use crate::{branches::GlobalBranches, depot::Depot};
use angora_common::defs;
use std::{
    fs,
    io::Write,
    sync::{Arc, RwLock},
};

pub fn show_stats(
    log_f: &mut fs::File,
    depot: &Arc<Depot>,
    gb: &Arc<GlobalBranches>,
    stats: &Arc<RwLock<ChartStats>>,
) {
    stats
        .write()
        .expect("Could not write stats.")
        .sync_from_global(depot, gb);

    let dir = depot
        .dirs
        .inputs_dir
        .parent()
        .expect("Could not get parent directory.");
    let mut log_s = fs::File::create(dir.join(defs::CHART_STAT_FILE))
        .expect("Could not create chart stat file.");
    {
        let s = stats.read().expect("Could not read from stats.");
        println!("{}", *s);
        writeln!(log_f, "{}", s.mini_log()).expect("Could not write minilog.");
        write!(
            log_s,
            "{}",
            serde_json::to_string(&*s).expect("Could not serialize!")
        )
        .expect("Unable to write!");
    }
}
