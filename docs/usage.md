# Angora usage
```
# /path-to-angora/angora_fuzzer --help
angora-fuzzer 1.3.0
fuzz some program

USAGE:
    fuzzer [FLAGS] [OPTIONS] --input <DIR> --output <DIR> [--] <pargs>...

FLAGS:
    -A, --disable_afl_mutation    Disable the fuzzer to mutate inputs using AFL's mutation strategies
    -E, --disable_exploitation    Disable the fuzzer to mutate sensitive bytes to exploit bugs
    -h, --help                    Prints help information
    -S, --sync_afl                Sync the seeds with AFL. Output directory should be in AFL's directory structure.
    -V, --version                 Prints version information

OPTIONS:
    -i, --input <DIR>                     Sets the directory of input seeds, use "-" to restart with existing output directory
    -M, --memory_limit <MEM>              Memory limit for programs, default is 200(MB)
    -m, --mode <Mode>                     Which binary instrumentation framework are you using? [possible values: llvm, pin]
    -o, --output <DIR>                    Sets the directory of outputs
    -r, --search_method <SearchMethod>    Which search method to run the program in? [possible values: gd, random, mb]
    -j, --jobs <JOB>                      Sets the number of thread jobs, default is 1
    -T, --time_limit <TIME>               time limit for programs, default is 1(s), the tracking timeout is 12 * TIME
    -t, --track <PROM>                    Sets the target (USE_TRACK or USE_PIN) for tracking, including taints, cmps. 

ARGS:
    <pargs>...    Targeted program (USE_FAST) and arguments. Any "@@" will be substituted with the input filename from Angora.
```

