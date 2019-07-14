# Environment variables for compiling

- `USE_FAST=1`: use fast mode to compile the program. It includes branch counting, getting the feedback of the fuzzing constraint (the output of its function).
- `USE_TRACK=1`: use taint tracking and collect all constraints.
- `USE_DFSAN=1`: use taint tracking.
- `ANGORA_CUSTOM_FN_CONTEXT=k` : Use only the last k ( 0 <= k <= 32) function call location as the context, e.g. `ANGORA_CUSTOM_FN_CONTEXT=8`. Angora disables context if k is 0.
- `ANGORA_GEN_ID_RANDOM=1` : Generate ids for predicates randomly instead of the hash of their locations.
- `ANGORA_OUTPUT_COND_LOC=1` : (Debug option) Output the location of each predicate during compiling.
- `ANGORA_TAINT_CUSTOM_RULE=/path/to/object` : object contains those proxy function (how to propagate taints), e.g. `ANGORA_TAINT_CUSTOM_RULE=~/angora/bin/lib/zlib-func.o` . You should add it as custom type in the file passed by `ANGORA_TAINT_RULE_LIST` first.
- `ANGORA_TAINT_RULE_LIST=/path/to/list` : DataFlowSanitizer’s [ABI list](https://clang.llvm.org/docs/DataFlowSanitizer.html), e.g. `ANGORA_TAINT_RULE_LIST=~/angora/bin/rules/zlib_abilist.txt`.
- `ANGORA_INST_RATIO`: 

# Environment variables for running

- `RUST_LOG=trace`: enable tracing output
- `RUST_LOG=debug`: enable debugging output
- `ANGORA_DISABLE_CPU_BINDING=1`: Disable cpu binding.

