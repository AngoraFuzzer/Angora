FROM ubuntu:16.04

RUN mkdir -p angora
COPY . angora
WORKDIR angora

ENV PATH=/clang+llvm/bin:/usr/local/cargo/bin:/angora/bin/:$PATH \
    LD_LIBRARY_PATH=/clang+llvm/lib:$LD_LIBRARY_PATH \
    RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo

#ENV RUSTUP_DIST_SERVER="https://mirrors.ustc.edu.cn/rust-static"
#ENV RUSTUP_UPDATE_ROOT="https://mirrors.ustc.edu.cn/rust-static/rustup"

RUN ./build/docker_build.sh

VOLUME ["/data"]
WORKDIR /data
