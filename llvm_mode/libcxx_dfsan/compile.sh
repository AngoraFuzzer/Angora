#!/usr/bin/env bash


NINJA_B=`which ninja 2>/dev/null`

if [ "$NINJA_B" = "" ]; then
    echo "[-] Error: can't find 'ninja' in your \$PATH. please install ninja-build" 1>&2
    echo "[-] Debian&Ubuntu: sudo apt-get install ninja-build" 1>&2
    exit 1
fi

set -euxo pipefail
wget http://releases.llvm.org/4.0.1/llvm-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/cfe-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/compiler-rt-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/libcxx-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/libcxxabi-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/libunwind-4.0.1.src.tar.xz
wget http://releases.llvm.org/4.0.1/clang-tools-extra-4.0.1.src.tar.xz

CUR_DIR=`pwd`
CLANG_SRC=${CUR_DIR}/llvm_src
rm -rf $CLANG_SRC

tar -Jxf ${CUR_DIR}/llvm-4.0.1.src.tar.xz 
mv llvm-4.0.1.src $CLANG_SRC

cd ${CLANG_SRC}/tools
tar -Jxf ${CUR_DIR}/cfe-4.0.1.src.tar.xz 
mv cfe-4.0.1.src clang
cd ${CLANG_SRC}/tools/clang/tools
tar -Jxf ${CUR_DIR}/clang-tools-extra-4.0.1.src.tar.xz 
mv clang-tools-extra-4.0.1.src extra
cd ${CLANG_SRC}/projects
tar -Jxvf ${CUR_DIR}/compiler-rt-4.0.1.src.tar.xz
mv compiler-rt-4.0.1.src compiler-rt
tar -Jxvf ${CUR_DIR}/libcxx-4.0.1.src.tar.xz
mv libcxx-4.0.1.src libcxx
tar -Jxvf ${CUR_DIR}/libcxxabi-4.0.1.src.tar.xz
mv libcxxabi-4.0.1.src libcxxabi
tar -Jxvf ${CUR_DIR}/libunwind-4.0.1.src.tar.xz
mv libunwind-4.0.1.src libunwind

rm -rf ${CUR_DIR}/*.tar.xz

cd $CUR_DIR
mkdir build && cd build/

CC=~/angora/bin/angora-clang CXX=~/angora/bin/angora-clang++ cmake -G Ninja ../llvm_src  -DLIBCXXABI_ENABLE_SHARED=NO -DLIBCXX_ENABLE_SHARED=NO -DLLVM_FORCE_USE_OLD_TOOLCHAIN=YES -DLIBCXX_CXX_ABI=libcxxabi

USE_DFSAN=1 ninja cxx cxxabi

@echo "if cxxabi.h not found, try: cp ./libcxxabi/include/*  ./libcxx/include, or -I"

cp $CUR_DIR/build/lib/libc++.a $CUR_DIR/.
cp $CUR_DIR/build/lib/libc++abi.a $CUR_DIR/libc++abidfsan.a
