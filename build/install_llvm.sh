#!/bin/bash
set -eux

LINUX_VER=${LINUX_VER:-ubuntu-16.04}
LLVM_VER=${LLVM_VER:-11.1.0}
PREFIX=${PREFIX:-${HOME}}

LLVM_DEP_URL=https://github.com/llvm/llvm-project/releases
TAR_NAME=clang+llvm-${LLVM_VER}-x86_64-linux-gnu-${LINUX_VER}

wget -q ${LLVM_DEP_URL}/download/llvmorg-${LLVM_VER}/${TAR_NAME}.tar.xz
tar -C ${PREFIX} -xf ${TAR_NAME}.tar.xz
rm ${TAR_NAME}.tar.xz
mv ${PREFIX}/${TAR_NAME} ${PREFIX}/clang+llvm

set +x
echo "Please set:"
echo "export PATH=\$PREFIX/clang+llvm/bin:\$PATH"
echo "export LD_LIBRARY_PATH=\$PREFIX/clang+llvm/lib:\$LD_LIBRARY_PATH"
