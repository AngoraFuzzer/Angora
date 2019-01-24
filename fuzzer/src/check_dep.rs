use memmap;
use std::{fs::File, io::prelude::*, path::Path};
use twoway;

static CHECK_CRASH_MSG: &str = r#"
If your system is configured to send core dump, there will be an
extended delay after the program crash, which might makes crash to
misinterpreted as timeouts.
You can modify /proc/sys/kernel/core_pattern to disable it by:
# echo core | sudo tee /proc/sys/kernel/core_pattern
"#;

static CORE_PATTERN_FILE: &str = "/proc/sys/kernel/core_pattern";

fn check_crash_handling() {
    let mut f = File::open(CORE_PATTERN_FILE).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    if buffer.trim() != "core" {
        panic!(CHECK_CRASH_MSG);
    }
}

fn check_target_binary(target: &str) {
    let program_path = Path::new(target);
    if !program_path.exists() || !program_path.is_file() {
        panic!("Invalid executable file! {:?}", target);
    }
}

pub fn check_asan(target: &str) -> bool {
    let file = File::open(target).expect("Unable to open file");
    let f_data = unsafe {
        memmap::MmapOptions::new()
            .map(&file)
            .expect("unable to mmap file")
    };
    let libasan = "libasan.so";
    let has_asan = twoway::find_bytes(&f_data[..], libasan.as_bytes()).is_some();
    let masn = "__msan_init";
    let has_masn = twoway::find_bytes(&f_data[..], masn.as_bytes()).is_some();
    has_asan || has_masn
}

fn check_io_dir(in_dir: &str, out_dir: &str) {
    let in_dir_p = Path::new(in_dir);
    let out_dir_p = Path::new(out_dir);

    if in_dir == "-" {
        if !out_dir_p.exists() {
            panic!("Original output directory is required to resume fuzzing.");
        }
    } else {
        if !in_dir_p.exists() || !in_dir_p.is_dir() {
            panic!("Input dir does not exist or is not a directory!");
        }
    }
}

pub fn check_dep(in_dir: &str, out_dir: &str, target: &str, target2: &str) {
    check_crash_handling();
    check_target_binary(target);
    check_target_binary(target2); // track binary
    check_io_dir(in_dir, out_dir);
}
