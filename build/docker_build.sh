#!/bin/bash

set -euxo pipefail

# sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list
apt-get update
apt-get install -y git build-essential wget zlib1g-dev golang-go python-pip python-dev build-essential 

PREFIX=/ ./build/llvm.sh

# from https://github.com/rust-lang-nursery/docker-rust-nightly/blob/master/nightly/Dockerfile
url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"
wget "$url"
chmod +x rustup-init
./rustup-init -y --no-modify-path --default-toolchain stable
# ./rustup-init -y --no-modify-path --default-toolchain nightly
rm rustup-init
chmod -R a+w $RUSTUP_HOME $CARGO_HOME
rustup --version
cargo --version
rustc --version

./build/build.sh

#wllvm and gllvm
pip install --upgrade pip==9.0.3
pip install wllvm
mkdir /go
go get github.com/SRI-CSL/gllvm/cmd/...

# install pin mode
git submodule update --init --recursive
cd pin_mode/libdft64
PREFIX=/ ./install_pin.sh
make
cp env.init /opt/
chmod +x /opt/env.init
cd ..
make