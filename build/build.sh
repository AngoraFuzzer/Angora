#!/bin/bash

set -euxo pipefail

if ! [ -x "$(command -v llvm-config)"  ]; then
    ./build/install_llvm.sh
    export PATH=${HOME}/clang+llvm/bin:$PATH
    export LD_LIBRARY_PATH=${HOME}/clang+llvm/lib${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}
fi

cargo build
cargo build --release

PREFIX=bin/
mkdir -p ${PREFIX}
cp target/release/*.a ${PREFIX}
cp target/release/fuzzer ${PREFIX}


cd llvm_mode && make
