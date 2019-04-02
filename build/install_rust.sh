#!/bin/bash

set -euxo pipefail

# from https://github.com/rust-lang-nursery/docker-rust-nightly/blob/master/nightly/Dockerfile

url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"
wget "$url"
chmod +x rustup-init
# RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static" RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup" 
./rustup-init -y --no-modify-path --default-toolchain stable
# ./rustup-init -y --no-modify-path --default-toolchain nightly

rm rustup-init
chmod -R a+w $RUSTUP_HOME $CARGO_HOME
rustup --version
cargo --version
rustc --version
