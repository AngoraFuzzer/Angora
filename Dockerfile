FROM ubuntu:16.04

RUN mkdir -p angora
COPY . angora
WORKDIR angora

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    GOPATH=/go \
    PATH=/clang+llvm/bin:/usr/local/cargo/bin:/angora/bin/:/go/bin:$PATH \
    LD_LIBRARY_PATH=/clang+llvm/lib:$LD_LIBRARY_PATH 

#ENV RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
#ENV RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"

RUN ./build/docker_build.sh

VOLUME ["/data"]
WORKDIR /data
