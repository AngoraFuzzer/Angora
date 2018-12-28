#!/bin/sh
set -eux

BUILD_TYPE="debug"
# MODE="hybrid"
num_jobs=1
#sync_afl="--sync_afl"
sync_afl=""
LOG_TYPE=angora
# LOG_TYPE=info

if [ ! -z ${RELEASE+x} ]; then
    BUILD_TYPE="release"
fi

envs="RUST_BACKTRACE=1 RUST_LOG=${LOG_TYPE}"
fuzzer="../target/${BUILD_TYPE}/fuzzer"
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

echo "Compile Done.."

args_file="./${name}/args"
if [ ! -f ${args_file} ]; then
    echo "Can't find args file in ${name}!"
    exit 1
fi

args=`cat ${args_file}`

cmd="$envs $fuzzer -A -i $input -o $output -j $num_jobs"
cmd="$cmd -t ${target}.taint ${sync_afl} -- ${target}.fast ${args}"

echo "run: ${cmd}"
eval $cmd
