# Building a Target Program

Angora currently only supports compile time instrumentation. Two instrumented 
copies of the target program are required, specifically one with taint tracking 
support and one with branch and constraint information counting. 

The process of building each copy is mostly identical. The resulting program is
dependent on using the environment variables `USE_FAST` and `USE_TRACK`. The 
configuration result can be used across both compile processes, thus one would
only need to store one compiled version elsewhere before compiling the other 
version.

We have also added support for wllvm.

### Makefile

*If the root directory of the project contains `configure`, use the method for autoconf.*

The following commands assume that the Makefile uses the `CC` and `CXX` 
environment variables. Manual changes should be made if otherwise.

```
CC=/path-to-angora/bin/angora-clang CXX=/path-to-angora/bin/angora-clang++ make
USE_TRACK=1 CC=/path-to-angora/bin/angora-clang CXX=/path-to-angora/bin/angora-clang++ make 
```

### autoconf

*If configuration fails, check if the LD_LIBRARY_PATH environment variable contains
the path for `libruntime` and `libruntime_fast`.*

*Some projects also requires the `LD` environment variable to be set to `angora-clang` for compilation.*

*Dynamic linking should be disabled due to DFSan implementation reasons.*

```
# Many autoconf scripts allow the use of --disable-shared switch to disable 
# dynamic linking. 
CC=/path-to-angora/bin/angora-clang CXX=/path-to-angora/bin/angora-clang++ \
   ./configure
make # default: USE_FAST=1
USE_TRACK=1 make
```

### cmake
```
cmake -DCMAKE_C_COMPILER=/path-to-angora/bin/angora-clang     \
      -DCMAKE_CXX_COMPILER=/path-to-angora/bin/angora-clang++ \
      -DBUILD_SHARED_LIBS=OFF ../src
make # default: USE_FAST=1
USE_TRACK=1 make
```

### wllvm (or gllvm)

- [wllvm](https://github.com/travitch/whole-program-llvm)
- [gllvm](https://github.com/SRI-CSL/gllvm)

```
sudo pip install wllvm
export LLVM_COMPILER=clang
CC=wllvm CFLAGS=-O0 ./configure --disable-shared
make
extract-bc xx
# You should add the shared libraries here, e.g. -lz
/path-to-angora/bin/angora-clang xx.bc -o xx.fast
USE_TRACK=1 /path-to-angora/bin/angora-clang xx.bc -o xx.taint
```

The running result of Angora that use wllvm/gllvm may be different from compiling directly because of compiler optimization.

## Build external libraries
If the external libraries are your targets (e.g. you are interested and want solve constraints in them),
we can use `USE_TRACK=1 make` to compile them.
Otherwise, we can use following rules to propagate taints while our program enters their code.
Here, we use zlib as example.

###  Model an external library
View it as a blackbox, and define models for them. See rules in https://clang.llvm.org/docs/DataFlowSanitizer.html.
- ignore this library

```
./angora/tools/gen_library_abilist.sh /usr/lib/x86_64-linux-gnu/libz.so  discard > zlib_abilist.txt
export ANGORA_TAINT_RULE_LIST=/path-to/zlib_abilist.txt
```

- use functional rules 

```
./angora/tools/gen_library_abilist.sh /usr/lib/x86_64-linux-gnu/libz.so  functional > zlib_abilist.txt
export ANGORA_TAINT_RULE_LIST=/path-to/zlib_abilist.txt
```

- use custom rules
Example: how to custom `crc32` function in `zlib` library. (see `llvm_mode/external_lib` directory)

```
# rename only certain functions to be custom after: 
./angora/tools/gen_library_abilist.sh /usr/lib/x86_64-linux-gnu/libz.so  discard > zlib_abilist.txt

# or set all as custom (not recommended)
./angora/tools/gen_library_abilist.sh /usr/lib/x86_64-linux-gnu/libz.so  custom > zlib_abilist.txt

export ANGORA_TAINT_RULE_LIST=/path-to/zlib_abilist.txt
# write your custom function, e.g. llvm_mode/external_lib/zlib-func.c and llvm_mode/external_lib/zlib_abilist.txt 
# compile it and 
export ANGORA_TAINT_CUSTOM_RULE=/path-to/zlib-func.o
```


### Build External Libraries with DFSan Support
Use `USE_DFSAN=1 make` to build them.

## Build C++ program and C++ standard library
- C++ program: CXX=/path-to-angora/bin/angora-clang++ or -DCMAKE_CXX_COMPILER=...
- C++ standard library: we have built one under ubuntu 16.04 64bits in llvm_mode/libcxx. You can built it by yourself with the following commands and move the libraries to llvm_mode/libcxx directory and bin/lib. (run libcxx_dfsan/compile.sh)

```
# http://lists.llvm.org/pipermail/cfe-dev/2015-January/040876.html
# install cmake ninja and download LLVM&CLANG source code
CC=~/angora/bin/angora-clang CXX=~/angora/bin/angora-clang++ cmake -G Ninja ../llvm  -DLIBCXXABI_ENABLE_SHARED=NO -DLIBCXX_ENABLE_SHARED=NO -DLLVM_FORCE_USE_OLD_TOOLCHAIN=YES -DLIBCXX_CXX_ABI=libcxxabi
USE_DFSAN=1 ninja cxx cxxabi
# move them to llvm_mode/libcxx and bin/lib
```

## Add taints in input functions
Angora models most input functions in `llvm_mode/external_lib/io-func.c`. But it doesn't support some input functions like `scanf` or other input function in external libraries. You can add taints by yourself by the approach described in *Model an external library*. For example, program `who` use `getutxent` to read input, and we add taints in `__dfsw_getutxent` in `io-func.c` file.