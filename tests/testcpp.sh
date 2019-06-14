#!/bin/sh
set -eux

BUILD_TYPE="debug"
# MODE="hybrid"
num_jobs=1
#sync_afl="--sync_afl"
sync_afl=""
LOG_TYPE=angora
MODE="pin"
MODE="llvm"
# LOG_TYPE=info

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

bin_dir=../bin/
USE_FAST=1 ${bin_dir}/angora-clang++ -std=c++11 ${target}.cpp -o ${target}.fast
USE_TRACK=1 ${bin_dir}/angora-clang++ -std=c++11 ${target}.cpp -o ${target}.taint
# USE_PIN=1 ${bin_dir}/angora-clang++ -std=c++11 ${target}.cpp -o ${target}.pin

echo "Compile Done.."

args_file="./${name}/args"
if [ ! -f ${args_file} ]; then
    echo "Can't find args file in ${name}!"
    exit 1
fi

args=`cat ${args_file}`

cmd="$envs $fuzzer -A -i $input -o $output -j $num_jobs"

if [ $MODE = "llvm" ]; then
    cmd="$cmd -m llvm -t ${target}.taint ${sync_afl} -- ${target}.fast ${args}"
elif [ $MODE = "pin" ]; then
    cmd="$cmd -m pin -t ${target}.pin ${sync_afl} -- ${target}.fast ${args}"
fi;


echo "run: ${cmd}"
eval $cmd
