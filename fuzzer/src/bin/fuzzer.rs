#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate angora;
extern crate angora_common;
use angora::fuzz_main;

fn main() {
    let matches = App::new("angora-fuzzer")
        .version(crate_version!())
        .about("Angora is a mutation-based fuzzer. The main goal of Angora is to increase branch coverage by solving path constraints without symbolic execution.")
        .arg(Arg::with_name("mode")
             .short("m")
             .long("mode")
             .value_name("Mode")
             .help("Which binary instrumentation framework are you using?")
             .possible_values(&["llvm", "pin"]))
        .arg(Arg::with_name("input_dir")
             .short("i")
             .long("input")
             .value_name("DIR")
             .help("Sets the directory of input seeds, use \"-\" to restart with existing output directory")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("output_dir")
             .short("o")
             .long("output")
             .value_name("DIR")
             .help("Sets the directory of outputs")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("track_target")
             .short("t")
             .long("track")
             .value_name("PROM")
             .help("Sets the target (USE_TRACK or USE_PIN) for tracking, including taints, cmps.  Only set in LLVM mode.")
             .takes_value(true))
        .arg(Arg::with_name("pargs")
            .help("Targeted program (USE_FAST) and arguments. Any \"@@\" will be substituted with the input filename from Angora.")
            .required(true)
            .multiple(true)
            .allow_hyphen_values(true)
            .last(true)
            .index(1))
        .arg(Arg::with_name("memory_limit")
             .short("M")
             .long("memory_limit")
             .value_name("MEM")
             .help("Memory limit for programs, default is 200(MB), set 0 for unlimit memory")
             .takes_value(true))
        .arg(Arg::with_name("time_limit")
             .short("T")
             .long("time_limit")
             .value_name("TIME")
             .help("time limit for programs, default is 1(s), the tracking timeout is 12 * TIME")
             .takes_value(true))
          .arg(Arg::with_name("bind")
          .short("b")
          .long("bind").value_name("BIND").help("\
               Bind Angora to cores starting from the id specified. \
               We assume all cores after the specified core are free. \
               If the cores specified are not enough, we won't bind at all.")
          .takes_value(true))
        .arg(Arg::with_name("thread_jobs")
             .short("j")
             .long("jobs")
             .value_name("JOB")
             .help("Sets the number of thread jobs, default is 1")
             .takes_value(true))
       .arg(Arg::with_name("search_method")
             .short("r")
             .long("search_method")
             .value_name("SearchMethod")
             .help("Which search method to run the program in?")
             .possible_values(&["gd", "random", "mb"]))
        .arg(Arg::with_name("sync_afl")
             .short("S")
             .long("sync_afl")
             .help("Sync the seeds with AFL. Output directory should be in AFL's directory structure."))
        .arg(Arg::with_name("disable_afl_mutation")
             .short("A")
             .long("disable_afl_mutation")
             .help("Disable the fuzzer to mutate inputs using AFL's mutation strategies"))
        .arg(Arg::with_name("disable_exploitation")
             .short("E")
             .long("disable_exploitation")
             .help("Disable the fuzzer to mutate sensitive bytes to exploit bugs"))
       .get_matches();

    fuzz_main(
        matches.value_of("mode").unwrap_or("llvm"),
        matches.value_of("input_dir").unwrap(),
        matches.value_of("output_dir").unwrap(),
        matches.value_of("track_target").unwrap_or("-"),
        matches.values_of_lossy("pargs").unwrap(),
        value_t!(matches, "bind", usize).ok(),
        value_t!(matches, "thread_jobs", usize).unwrap_or(1),
        value_t!(matches, "memory_limit", u64).unwrap_or(angora_common::config::MEM_LIMIT),
        value_t!(matches, "time_limit", u64).unwrap_or(angora_common::config::TIME_LIMIT),
        matches.value_of("search_method").unwrap_or("gd"),
        matches.occurrences_of("sync_afl") > 0,
        matches.occurrences_of("disable_afl_mutation") == 0,
        matches.occurrences_of("disable_exploitation") == 0,
    );
}
