#!/bin/sh

set -xe

SRC="src"
OUT="target"

rustc -O --edition 2021 $SRC/main.rs -o $OUT/main

if [ $1 = "run" ]
then
    ./target/main
fi
