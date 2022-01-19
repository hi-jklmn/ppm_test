#!/bin/sh

set -xe

SRC_DIR=src
TARGET_DIR=target
DISABLED_LINTS="dead_code unused_variables"

LINTER_SETTINGS=$(for lint in $DISABLED_LINTS; do printf ' -A %s' $lint; done)

time rustc -O $LINTER_SETTINGS --edition 2021 $SRC_DIR/main.rs -o $TARGET_DIR/main

if [ $1 = "run" ]
then
    ./target/main
    mkdir -p 'output/history'
    magick "output/test_image.ppm" "output/history/$(date +%y_%m_%d_%H_%M_%S).png"
fi
