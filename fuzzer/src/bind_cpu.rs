use std::{env, fs::File, io::prelude::*, mem, path::Path};

#[cfg(target_os = "linux")]
fn get_info_from_status(p: &Path) -> Option<usize> {
    let mut f = if let Ok(f) = File::open(p) {
        f
    } else {
        return None;
    };

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    let has_vm = buffer.contains("VmSize:");
    if !has_vm {
        // kernel tasks.
        return None;
    }

    if let Some(off) = buffer.find("Cpus_allowed_list:\t") {
        let off = off + 19;
        let mut sub = &buffer[off..];
        if let Some(end_off) = sub.find("\n") {
            sub = &sub[..end_off];
        }
        if sub.contains("-") || sub.contains(",") {
            return None;
        }
        if let Ok(cpuid) = sub.parse::<usize>() {
            return Some(cpuid);
        }
        None
    } else {
        None
    }
}

#[cfg(not(target_os = "linux"))]
pub fn find_free_cpus(_ask_num: usize) -> Vec<usize> {
    vec![0; 1]
}

#[cfg(target_os = "linux")]
pub fn find_free_cpus(ask_num: usize) -> Vec<usize> {
    let mut free_cpus = vec![];
    if env::var(angora_common::defs::DISABLE_CPU_BINDING_VAR).is_ok() {
        return free_cpus;
    }

    let proc_dir = Path::new("/proc");
    let max_num = num_cpus::get();
    let mut cpu_used = vec![false; max_num];
    info!("Found {} cores.", max_num);
    let entries = proc_dir.read_dir().unwrap();
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string();
            if let Ok(name) = file_name {
                // pid
                if let Ok(_) = name.parse::<u32>() {
                    let path = entry.path();
                    if let Some(cpuid) = get_info_from_status(&path.join("status")) {
                        if cpuid < max_num {
                            cpu_used[cpuid] = true;
                        }
                    }
                }
            }
        }
    }

    for i in 0..max_num {
        if cpu_used[i] == false {
            free_cpus.push(i);
        }
    }
    info!("Free Cpus: {:?}", free_cpus);

    if free_cpus.len() > ask_num {
        free_cpus.truncate(ask_num);
    }

    free_cpus
}

#[cfg(not(target_os = "linux"))]
pub fn bind_thread_to_cpu_core(_cid: usize) {
    warn!("Do not implement thread binding!");
}

#[cfg(target_os = "linux")]
pub fn bind_thread_to_cpu_core(cid: usize) {
    unsafe {
        let mut c: libc::cpu_set_t = mem::zeroed();
        libc::CPU_ZERO(&mut c);
        libc::CPU_SET(cid, &mut c);
        if libc::sched_setaffinity(0, mem::size_of_val(&c), &c as *const libc::cpu_set_t) != 0 {
            panic!("sched_setaffinity failed");
        }
    }
}
