#!/bin/bash

set -euxo pipefail

# install pin mode
git init && git submodule update --init --recursive
cd pin_mode/libdft64
PREFIX=/ ./install_pin.sh
make
cp env.init /opt/
cd ..
make OBJDIR=../bin/lib/