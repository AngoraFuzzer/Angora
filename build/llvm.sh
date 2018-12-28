#!/bin/bash
set -eux
LLVM_DEP_URL=${LLVM_DEP_URL:-http://releases.llvm.org/4.0.0}
PREFIX=${PREFIX:-${HOME}}
TAR_NAME=clang+llvm-4.0.0-x86_64-linux-gnu-ubuntu-16.04
wget -q ${LLVM_DEP_URL}/${TAR_NAME}.tar.xz
tar -C ${PREFIX} -xf ${TAR_NAME}.tar.xz
rm ${TAR_NAME}.tar.xz
mv ${PREFIX}/${TAR_NAME} ${PREFIX}/clang+llvm

set +x
echo "Please set:"
echo "export PATH=\$PREFIX/clang+llvm/bin:\$PATH"
echo "export LD_LIBRARY_PATH=\$PREFIX/clang+llvm/lib:\$LD_LIBRARY_PATH"
