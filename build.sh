#!/bin/sh

set -xe

SRC_DIR=src
TARGET_DIR=target
DISABLED_WARNINGS=dead_code

rustc -O -A $DISABLED_WARNINGS --edition 2021 $SRC_DIR/main.rs -o $TARGET_DIR/main

if [ $1 = "run" ]
then
    ./target/main
fi
