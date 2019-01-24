extern crate angora;

use angora::executor::Forksrv;
use std::{collections::HashMap, env, os::unix::io::RawFd, time::SystemTime};

static FUZZER_ID_VAR: &str = "ANGORA_FUZZER_ID";
const TIME_LIMIT: u64 = 5;
const MEM_LIMIT: u64 = 2000;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Wrong command!");
        return;
    }

    let prom_bin = args[1].clone();
    let prom_args = vec![args[2].clone()];

    // TODO bind cpu

    let mut envs = HashMap::new();
    let thread_id = 0;
    envs.insert(FUZZER_ID_VAR.to_string(), thread_id.to_string());
    // envs.insert(BRANCHES_SHM_ENV_VAR.to_string(), branches.get_id().to_string());
    // envs.insert(COND_STMT_ENV_VAR.to_string(), cond_stmt.get_id().to_string());
    let mut fs = Forksrv::new(
        "/tmp/angora_speeed_test",
        &(prom_bin, prom_args),
        &envs,
        0 as RawFd,
        false,
        false,
        TIME_LIMIT,
        MEM_LIMIT,
    );

    let init_t = SystemTime::now();
    let n = 10000;
    for _ in 0..n {
        fs.run();
    }

    let running_time = init_t.elapsed().unwrap().as_secs();
    println!("t: {}, n {} ", running_time, n);
}
