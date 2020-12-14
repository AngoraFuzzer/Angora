use angora_common::defs;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct DepotDir {
    pub inputs_dir: PathBuf,
    pub hangs_dir: PathBuf,
    pub crashes_dir: PathBuf,
    pub seeds_dir: PathBuf,
}

impl DepotDir {
    pub fn new(seeds_dir: PathBuf, out_dir: &Path) -> Self {
        let inputs_dir = out_dir.join(defs::INPUTS_DIR);
        let hangs_dir = out_dir.join(defs::HANGS_DIR);
        let crashes_dir = out_dir.join(defs::CRASHES_DIR);

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
