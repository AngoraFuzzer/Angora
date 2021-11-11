#!/bin/sh
set -eux

BUILD_TYPE="debug"
# BUILD_TYPE="release"
num_jobs=1
#sync_afl="--sync_afl"
sync_afl=""
LOG_TYPE=angora
MODE="pin"
MODE="llvm"
#LOG_TYPE=info

if [ ! -z ${RELEASE+x} ]; then
    BUILD_TYPE="release"
fi

if [ ! -z ${LLVM_MODE+x} ]; then
    MODE="llvm"
fi
if [ ! -z ${PIN_MODE+x} ]; then
    MODE="pin"
fi


envs="BUILD_TYPE=${BUILD_TYPE} LOG_TYPE=${LOG_TYPE}"
fuzzer="../angora_fuzzer"

input="./input"
if [ -d "$1/input" ]; then
    input="$1/input"
fi
output="./output"

if [ "$#" -ne 1 ] || ! [ -d "$1" ]; then
    echo "Usage: $0 DIRECTORY" >&2
    exit 1
fi

rm -rf $output
name=$1

echo "Compile..."

target=${name}/${name}

rm -f ${target}.fast ${target}.cmp ${target}.taint

# export ANGORA_CUSTOM_FN_CONTEXT=0

bin_dir=../bin/
gclang ${target}.c -o ${target} -lz -g
get-bc ${target}
rm ${target}
mv ${target}.bc orig.bc
opt -break-crit-edges -o ${target}.bc orig.bc

clang ${target}.bc -lz -lrt -fsanitize=address -U_FORTIFY_SOURCE -o ${target}.asan

USE_FAST=1 ${bin_dir}/angora-clang ${target}.bc -lz -lrt -o ${target}.fast
USE_FAST=1 ${bin_dir}/angora-clang ${target}.bc -lz -lrt -o ${target}.fast.ll -S -emit-llvm

USE_TRACK=1 ${bin_dir}/angora-clang ${target}.bc -lz -lrt -o ${target}.taint
USE_TRACK=1 ${bin_dir}/angora-clang ${target}.bc -lz -lrt -o ${target}.taint.ll -S -emit-llvm
echo "Compile Done.."

args_file="./${name}/args"
if [ ! -f ${args_file} ]; then
    echo "Can't find args file in ${name}!"
    exit 1
fi

args=`cat ${args_file}`

cmd="$envs $fuzzer -M 0 -A -E -i $input -o $output -j $num_jobs"
if [ $MODE = "llvm" ]; then
    cmd="$cmd -m llvm ${sync_afl} --track ${target}.taint -- ${target}.fast ${args}"
elif [ $MODE = "pin" ]; then
    cmd="$cmd -m pin -t ${target}.pin ${sync_afl} -- ${target}.fast ${args}"
fi;

echo "run: ${cmd}"
eval $cmd
