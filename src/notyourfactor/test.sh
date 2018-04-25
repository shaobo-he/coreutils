#! /usr/bin/env bash

set -e

rustc --emit=llvm-ir -g -C opt-level=0 driver.rs
llvm-link driver.ll factor.ll > compare.bc
clang -dynamic-linker /usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-b4054fae3db32020.so compare.bc -o compare
./compare
