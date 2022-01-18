# ptex-bind
cppmm bindings for Ptex

# Generating bindings
First you must have `astgen` and `asttoc` in your PATH. 

run `bind.sh`, passing the path to your Ptex and installation as an environment variable:
```bash
export PTEX_ROOT=/path/to/ptex
export LLVM_ROOT=/path/to/llvm

./bind.sh

# You may need to pass additional flags to make the compiler include paths
# available to astgen. For example:

export CLANG_VERSION=11.0.0
export MACOS_SDK=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk

./bind.sh \
    -isysroot${MACOS_SDK} \
    -isystem ${MACOS_SDK}/usr/include \
    -isystem ${LLVM_ROOT}/include/c++/v1 \
    -isystem ${LLVM_ROOT}/lib/clang/${CLANG_VERSION}/include
```

The resulting bindings will be create in `build/ptex-c` and `build/ptex-sys` for the C and Rust bindings, respectively

# Versions
There is a branch for each supported minor version of the target library. Other versions may or may not bind successfully.
