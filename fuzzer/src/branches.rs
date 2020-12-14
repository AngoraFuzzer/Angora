use crate::executor::StatusType;
use angora_common::{config::BRANCHES_SIZE, shm::SHM};
#[cfg(feature = "unstable")]
use std::intrinsics::unlikely;
use std::{
    self,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

pub type BranchBuf = [u8; BRANCHES_SIZE];
#[cfg(target_pointer_width = "32")]
type BranchEntry = u32;
#[cfg(target_pointer_width = "64")]
type BranchEntry = u64;
#[cfg(target_pointer_width = "32")]
const ENTRY_SIZE: usize = 4;
#[cfg(target_pointer_width = "64")]
const ENTRY_SIZE: usize = 8;
type BranchBufPlus = [BranchEntry; BRANCHES_SIZE / ENTRY_SIZE];

// Map of bit bucket
// [1], [2], [3], [4, 7], [8, 15], [16, 31], [32, 127], [128, infinity]
static COUNT_LOOKUP: [u8; 256] = [
    0, 1, 2, 4, 8, 8, 8, 8, 16, 16, 16, 16, 16, 16, 16, 16, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
    32, 32, 32, 32, 32, 32, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
    64, 64, 64, 64, 64, 64, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
    128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
];

macro_rules! cast {
    ($ptr:expr) => {{
        unsafe { std::mem::transmute($ptr) }
    }};
}

pub struct GlobalBranches {
    virgin_branches: RwLock<Box<BranchBuf>>,
    tmouts_branches: RwLock<Box<BranchBuf>>,
    crashes_branches: RwLock<Box<BranchBuf>>,
    density: AtomicUsize,
}

impl GlobalBranches {
    pub fn new() -> Self {
        Self {
            virgin_branches: RwLock::new(Box::new([255u8; BRANCHES_SIZE])),
            tmouts_branches: RwLock::new(Box::new([255u8; BRANCHES_SIZE])),
            crashes_branches: RwLock::new(Box::new([255u8; BRANCHES_SIZE])),
            density: AtomicUsize::new(0),
        }
    }

    pub fn get_density(&self) -> f32 {
        let d = self.density.load(Ordering::Relaxed);
        (d * 10000 / BRANCHES_SIZE) as f32 / 100.0
    }
}

pub struct Branches {
    global: Arc<GlobalBranches>,
    trace: SHM<BranchBuf>,
}

impl Branches {
    pub fn new(global: Arc<GlobalBranches>) -> Self {
        let trace = SHM::<BranchBuf>::new();
        Self { global, trace }
    }

    pub fn clear_trace(&mut self) {
        self.trace.clear();
    }

    pub fn get_id(&self) -> i32 {
        self.trace.get_id()
    }

    fn get_path(&self) -> Vec<(usize, u8)> {
        let mut path = Vec::<(usize, u8)>::new();
        let buf_plus: &BranchBufPlus = cast!(&*self.trace);
        let buf: &BranchBuf = &*self.trace;
        for (i, &v) in buf_plus.iter().enumerate() {
            macro_rules! run_loop {
                () => {{
                    let base = i * ENTRY_SIZE;
                    for j in 0..ENTRY_SIZE {
                        let idx = base + j;
                        let new_val = buf[idx];
                        if new_val > 0 {
                            path.push((idx, COUNT_LOOKUP[new_val as usize]))
                        }
                    }
                }};
            }
            #[cfg(feature = "unstable")]
            {
                if unsafe { unlikely(v > 0) } {
                    run_loop!()
                }
            }
            #[cfg(not(feature = "unstable"))]
            {
                if v > 0 {
                    run_loop!()
                }
            }
        }
        // debug!("count branch table: {}", path.len());
        path
    }

    pub fn has_new(&mut self, status: StatusType) -> (bool, bool, usize) {
        let gb_map = match status {
            StatusType::Normal => &self.global.virgin_branches,
            StatusType::Timeout => &self.global.tmouts_branches,
            StatusType::Crash => &self.global.crashes_branches,
            _ => {
                return (false, false, 0);
            },
        };
        let path = self.get_path();
        let edge_num = path.len();

        let mut to_write = vec![];
        let mut has_new_edge = false;
        let mut num_new_edge = 0;
        {
            // read only
            let gb_map_read = gb_map.read().unwrap();
            for &br in &path {
                let gb_v = gb_map_read[br.0];

                if gb_v == 255u8 {
                    num_new_edge += 1;
                }

                if (br.1 & gb_v) > 0 {
                    to_write.push((br.0, gb_v & (!br.1)));
                }
            }
        }

        if num_new_edge > 0 {
            if status == StatusType::Normal {
                // only count virgin branches
                self.global
                    .density
                    .fetch_add(num_new_edge, Ordering::Relaxed);
            }
            has_new_edge = true;
        }

        if to_write.is_empty() {
            return (false, false, edge_num);
        }

        {
            // write
            let mut gb_map_write = gb_map.write().unwrap();
            for &br in &to_write {
                gb_map_write[br.0] = br.1;
            }
        }

        (true, has_new_edge, edge_num)
    }
}

impl std::fmt::Debug for Branches {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn branch_empty() {
        let global_branches = Arc::new(GlobalBranches::new());
        let mut br = Branches::new(global_branches);
        assert_eq!(br.has_new(StatusType::Normal), (false, false, 0));
        assert_eq!(br.has_new(StatusType::Timeout), (false, false, 0));
        assert_eq!(br.has_new(StatusType::Crash), (false, false, 0));
    }

    #[test]
    #[ignore]
    fn branch_find_new() {
        let global_branches = Arc::new(GlobalBranches::new());
        let mut br = Branches::new(global_branches);
        assert_eq!(br.has_new(StatusType::Normal), (false, false, 0));
        {
            let trace = &mut br.trace;
            trace[4] = 1;
            trace[5] = 1;
            trace[8] = 3;
        }
        let path = br.get_path();
        assert_eq!(path.len(), 3);
        assert_eq!(path[2].1, COUNT_LOOKUP[3]);
        assert_eq!(br.has_new(StatusType::Normal), (true, true, 3));
    }
}
