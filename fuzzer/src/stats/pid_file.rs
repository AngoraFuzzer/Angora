/// Make AFL can sync files Angora generated.
/// - create a file to stored fuzzer's pid
/// - remove the file after fuzzer stop

use std::{
    fs,
    io::prelude::*,
    path::{PathBuf},
};
use libc;

pub struct FuzzerPidFile {
    file_path: PathBuf,
}

impl FuzzerPidFile {
    pub fn new(angora_out_dir: &PathBuf) -> Self {
        let file_path = angora_out_dir.join("fuzzer_stats");
        let pid = unsafe { libc::getpid() as usize };
        let mut buffer = match fs::File::create(&file_path) {
           Ok(a) => a,
           Err(e) => {
               error!("Could not create stats file: {:?}", e);
               panic!();
           }
        };
        write!(buffer, "fuzzer_pid : {}", pid).expect("Could not write to stats file");
        Self {
            file_path
        }
    }
}

impl Drop for FuzzerPidFile {
    fn drop(&mut self) {
        match fs::remove_file(&self.file_path) {
           Ok(_) => (),
           Err(e) => warn!("Could not remove fuzzer stats file: {:?}", e),
        };
    }
}
