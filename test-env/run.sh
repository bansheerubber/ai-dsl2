#!/bin/sh

export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd):/home/me/Projects/ai-dsl-runtime/libtorch/lib/:/home/me/Projects/ai-dsl-runtime/build
./a.out
