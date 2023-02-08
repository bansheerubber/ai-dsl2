#!/bin/sh

AI_DSL_RUNTIME="../../ai-dsl-runtime"
LIBTORCH="/home/me/Projects/ai-dsl-runtime/libtorch" # location of libtorch installation (from https://pytorch.org/cppdocs/installing.html)
CUDA="/opt/cuda" # location of cuda installation

cp ../main.bc main.bc

llc -filetype=obj --relocation-model=pic main.bc
clang main.o \
$LIBTORCH/lib/libtorch.so \
$LIBTORCH/lib/libc10.so \
$LIBTORCH/lib/libkineto.a \
$CUDA/lib64/stubs/libcuda.so \
$CUDA/lib64/libnvrtc.so \
$CUDA/lib64/libnvToolsExt.so \
$CUDA/lib64/libcudart.so \
$LIBTORCH/lib/libc10_cuda.so \
$LIBTORCH/lib/libtorch_cuda.so \
$LIBTORCH/lib/libc10_cuda.so \
$LIBTORCH/lib/libc10.so \
$CUDA/lib64/libcufft.so \
$CUDA/lib64/libcurand.so \
$CUDA/lib64/libcublas.so \
/usr/lib/libcudnn.so \
$LIBTORCH/lib/libtorch_cuda_cu.so \
$LIBTORCH/lib/libtorch.so \
$AI_DSL_RUNTIME/build/libairt.so
