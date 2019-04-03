#!/bin/bash

set -euxo pipefail

#wllvm and gllvm
pip install --upgrade pip==9.0.3
pip install wllvm
mkdir /go
go get github.com/SRI-CSL/gllvm/cmd/...

