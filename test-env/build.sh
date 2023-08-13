#!/bin/sh

AI_DSL_RUNTIME="../../ai-dsl-runtime"
LIBTORCH="/home/me/Projects/ai-dsl-runtime/libtorch" # location of libtorch installation (from https://pytorch.org/cppdocs/installing.html)
CUDA="/opt/cuda" # location of cuda installation

pushd .
cd ../
cargo run
popd

cp ../main.bc main.bc

llc -filetype=obj --relocation-model=pic main.bc
clang main.o \
$LIBTORCH/lib/libtorch.so \
$LIBTORCH/lib/libtorch_cuda.so \
$LIBTORCH/lib/libtorch_cpu.so \
$LIBTORCH/lib/libc10.so \
$AI_DSL_RUNTIME/build/libairt.so
