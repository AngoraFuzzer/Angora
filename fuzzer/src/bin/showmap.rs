#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate angora;
extern crate angora_common;

use angora::executor::Forksrv;
use angora_common::defs;
use angora::{branches};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Write,
    os::unix::io::RawFd,
    sync::Arc,
};

fn main() {
    let matches = App::new("angora-showmap")
        .version(crate_version!())
        .about("Displays the contents of the trace bitmap.")
        .arg(Arg::with_name("output_file")
             .short("o")
             .long("output")
             .value_name("FILE")
             .help("File to write the trace data to")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("time_limit")
             .short("T")
             .long("time_limit")
             .value_name("TIME")
             .help("Time limit for each run, default is 1(s)")
             .takes_value(true))
        .arg(Arg::with_name("memory_limit")
             .short("M")
             .long("memory_limit")
             .value_name("MEM")
             .help("Memory limit for programs, default is 200(MB), set 0 for unlimited memory")
             .takes_value(true))
        .arg(Arg::with_name("branch_only")
             .short("b")
             .long("branch_only")
             .help("Show branch coverage only, ignore hit counts"))
        .arg(Arg::with_name("pargs")
            .help("Targeted program and arguments")
            .required(true)
            .multiple(true)
            .allow_hyphen_values(true)
            .last(true)
            .index(1))
        .get_matches();

    let branch_only = matches.occurrences_of("branch_only") > 0;

    let pargs = matches.values_of_lossy("pargs").unwrap();
    let prog_bin = pargs[0].clone();
    let prog_args = pargs[1..].to_vec();

    let global_branches = Arc::new(branches::GlobalBranches::new());
    let branches = branches::Branches::new(global_branches);

    let mut envs = HashMap::new();
    envs.insert(
        defs::BRANCHES_SHM_ENV_VAR.to_string(),
        branches.get_id().to_string(),
    );

    let out_file_path = matches.value_of("output_file").unwrap();
    let mut out_file = match File::create(out_file_path) {
        Ok(file) => file,
        Err(err) => panic!("could not open {:?}: {:?}", out_file_path, err),
    };

    let mut forksrv = Forksrv::new(
        "/tmp/angora_showmap",
        &(prog_bin, prog_args),
        &envs,
        0 as RawFd,
        false,
        false,
        value_t!(matches, "time_limit", u64).unwrap_or(angora_common::config::TIME_LIMIT),
        value_t!(matches, "memory_limit", u64).unwrap_or(angora_common::config::MEM_LIMIT),
    );
    forksrv.run();
    let path = branches.get_path();

    for (idx, mut count) in path {
        count = if branch_only {
            1
        } else {
            count
        };
        writeln!(out_file, "{}:{}", idx, count).unwrap();
    };
}
