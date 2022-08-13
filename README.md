# ptex-bind

High-level cppmm-based bindings for [Ptex](https://github.com/wdas/ptex)

## Links

- [source repository](https://github.com/vfx-rs/ptex-bind)
- [ptex on crates.io](https://crates.io/crates/ptex/latest)
- [ptex documentation](https://docs.rs/crate/ptex/latest)
- [ptex-sys on crates.io](https://crates.io/crates/ptex-sys/latest)
- [ptex-sys documentation](https://docs.rs/crate/ptex-sys/latest)
- [ptex core documentation](https://ptex.us/documentation.html)


## Usage

Add this to your `Cargo.toml`:

    [dependencies]
    ptex = "0.0.6"


## Introduction

`bind` contains the [cppmm](https://github.com/vfx-rs/cppmm) bindings.

`bind.sh` uses `cppmm` and the bindings to output a `ptex-sys` directory with
the [ptex-sys](https://crates.io/crates/ptex-sys) crate.

`src` contains the high-level [ptex](https://crates.io/crates/ptex) crate.

The `ptex` crate provides a high-level safe API over the `ptex-sys` bindings.
`ptex-sys` should not be used directly.


## Generating ptex-sys

- Clone, build and install [Ptex](https://github.com/wdas/ptex) into this directory.

- Configure `$PTEX_ROOT` to point to your Ptex installation.

- Configure `$LLVM_ROOT` to point to your LLVM installation.

```bash
git clone https://github.com/wdas/ptex
make -C ptex install prefix=$PWD/ptex/dist
export PTEX_ROOT=$PWD/ptex/dist
export LLVM_ROOT=/path/to/llvm
```

- Download and install LLVM.

- Install the `astgen` and `asttoc` commands from
  [cppmm](https://github.com/vfx-rs/cppmm) and make them available in `$PATH`.

- Run `./bind.sh` after specifying the path to your Ptex and and LLVM installations via
  the `PTEX_ROOT` and `LLVM_ROOT` environment variables.

```bash
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

The resulting bindings will be created in `ptex-sys/ptex-c` and `ptex-sys`
for the C and Rust bindings, respectively.


## Development

The test suite in `tests` is used to validate `ptex` and `ptex-sys`.
Build and test `ptex-sys` and `ptex` using `cargo`.

```bash
cargo build
cargo test
```


## Versions

The generated `ptex-sys` crate version matches the version of the Ptex library.

The high-level `ptex` crate currently track the latest stable Ptex version.

Branches will be created for older versions in the future when newer
major or minor Ptex releases are available.

The tags in this repository correspond to the `ptex` crate versions.
The `ptex` crate version will be tagged and released with a non-v0
version number that matches the underlying C++ Ptex library once the
`ptex` crate is feature-complete.
