use chrono::prelude::Local;
use std::{
    fs,
    path::{Path, PathBuf},
};

static CRASHES_DIR: &str = "crashes";
static HANGS_DIR: &str = "hangs";
static INPUTS_DIR: &str = "queue";

#[derive(Debug)]
pub struct DepotDir {
    pub inputs_dir: PathBuf,
    pub hangs_dir: PathBuf,
    pub crashes_dir: PathBuf,
    pub seeds_dir: PathBuf,
}

impl DepotDir {
    pub fn new(in_dir: &str, out_dir: &Path) -> Self {
        let restart = in_dir == "-";

        let inputs_dir = out_dir.join(INPUTS_DIR);
        let hangs_dir = out_dir.join(HANGS_DIR);
        let crashes_dir = out_dir.join(CRASHES_DIR);

        let seeds_dir = if restart {
            let orig_out_dir = out_dir.with_extension(Local::now().to_rfc3339());
            fs::rename(&out_dir, orig_out_dir.clone()).unwrap();
            fs::create_dir(&out_dir).unwrap();
            PathBuf::from(orig_out_dir).join(INPUTS_DIR)
        } else {
            PathBuf::from(in_dir)
        };

        fs::create_dir(&crashes_dir).unwrap();
        fs::create_dir(&hangs_dir).unwrap();
        fs::create_dir(&inputs_dir).unwrap();

        Self {
            inputs_dir,
            hangs_dir,
            crashes_dir,
            seeds_dir,
        }
    }
}
