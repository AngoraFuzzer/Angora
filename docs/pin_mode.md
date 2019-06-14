# Angora's Pin mode - Use libdft64 for taint tracking

Angora now supports using [libdft64](https://github.com/AngoraFuzzer/libdft64) for taint anlysis instead of DFSan (LLVM mode).
The feature is still experimental.

## Build requirements

- [libdft64](https://github.com/AngoraFuzzer/libdft64)

### Environment Variables
```
export LIBDFT_PATH=/path-to-libdft64
```

## Build Pin mode
```
cd pin_mode
make OBJDIR=../bin/lib/
```

## Build a target program

As [Build a target program](./build_target.md) mentioned, Angora uses two variables `USE_FAST` and `USE_TRACK` to compile two different version programs respectively. In Pin mode, Angora uses variable `USE_PIN` to compile the one with taint tracking instead of `USE_TRACK`. 

```
USE_PIN=1 CC=/path-to-angora/bin/angora-clang CXX=/path-to-angora/bin/angora-clang++ make 
```

## Run Angora in Pin mode

Command line options `-m` is used to set which mode you are using in fuzzing. We have "llvm" and "pin" modes.

```
./angora_fuzzer -m pin -i input -o output -t path-to-taint-program-pin -- program args(..)
```