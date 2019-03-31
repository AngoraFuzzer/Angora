#!/bin/bash

CUR_PATH=$(readlink -f "$0")                                         
PIN_MODE_DIR=$(dirname $CUR_PATH)

set -eux
${PIN_ROOT}/pin -t  ${PIN_MODE_DIR}/obj-intel64/pin_track.so -- "$@"