# ptex-bind

High-level cxx-based bindings for [Ptex](https://github.com/wdas/ptex)

## Introduction

The `src` directory contains the [ptex](https://crates.io/crates/ptex) crate.
The `ptex` crate provides a high-level safe API over the low-level `ptex-sys` crate.

The `ptex-sys` directory contains the [ptex-sys](https://crates.io/crates/ptex-sys/) crate.
`ptex-sys` uses [cxx](https://cxx.rs) to wrap the C++ Ptex API.

The `ptex-sys` crate should not be used directly.


## Usage

Building the `ptex-sys` crates requires you to install the C++
[Ptex](https://github.com/wdas/ptex) library in order to build the `ptex` crate.

Add ptex to your `Cargo.toml`:

    [dependencies]
    ptex = "0.2.0"

The `ptex` crate is built using `cargo build` but you must ensure that
`pkg-config` is able to find your Ptex installation.

If the C++ Ptex library is installed to a non-system directory then you must
configure  the `PKG_CONFIG_PATH` environment variable to point to the
`share/pkgconfig` directory inside the Ptex installtion.

`pkg-config` is used by `ptex-sys/build.rs` in order to locate the
`Ptex` libraries and C++ headers. If Ptex is installed to a global system
location such as `/usr/local` then `PKG_CONFIG_PATH` does not need to be
configured. `pkg-config` searches in the system locations by default.


## Development

This repository provides a
[garden.yaml](https://gitlab.com/garden-rs/garden/tree/main/garden.yaml)
[Garden](https://gitlab.com/garden-rs/garden) recipe to automate the
process of installing Ptex and building the `ptex` Rust crate.

Garden can be used to clone [Ptex](https://github.com/wdas/ptex) to
the `ptex` subdirectory and install the C++ Ptex library to `ptex/dist`.

Garden configures the `PKG_CONFIG_PATH` environment variable to contain
`$PWD/ptex/dist/share/pkgconfig` when running commands.

Run the following [Garden](https://gitlab.com/garden-rs/garden) commands
to bootstrap a development environment containing Ptex so that the the
`ptex` crate can be built.

```bash
garden grow ptex
garden build all
```

These commands perform the following steps:

- `garden grow` clones [Ptex](https://github.com/wdas/ptex) into the `ptex` directory
  using the URL configured in the `garden.yaml`.

- `garden build all` builds the C++ Ptex library, installs it to `ptex/dist`,
  configures `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` to point into `ptex/dist`,
  and builds the `ptex` crate using `cargo build`.


### Testing

The test suite in the `tests` directory is used to validate the `ptex` crate.
The Garden recipe contains `test` and `check` commands that are used to run
the test suite and run checks over the source code.

```bash
# Run tests
garden test

# Run checks and linters
garden check
```


### Workflow

A typical development workflow loop in this repository contains the following steps:

- Build the Ptex C++ library.  (`garden build ptex`)

- Buid the `ptex-sys` low-level Ptex bindings.  (`garden build ptex-sys`)

- Build the high-level `ptex` bindings.  (`garden build` or `garden build ptex`))

- Run tests across all crates.  (`garden test`)

- Run checks across all crates.  (`garden check`)

The `garden.yaml` recipe provides a command to perform all of the above steps in a single shot:

```bash
# Build, test and check everything.
garden dev
```


## Versions

The `ptex` crate currently tracks the latest stable Ptex version.

Branches will be created for older versions in the future when newer
major or minor Ptex releases are available.

The tags in this repository correspond to the `ptex` crate versions.
The `ptex` crate version will be tagged and released with a non-v0
version number that matches the underlying C++ Ptex library once the
`ptex` crate is feature-complete.


## Links

- [source repository](https://github.com/vfx-rs/ptex-bind)
- [ptex on crates.io](https://crates.io/crates/ptex/latest)
- [ptex-sys on crates.io](https://crates.io/crates/ptex-sys/latest)
- [ptex documentation](https://docs.rs/crate/ptex/latest)
- [ptex-sys documentation](https://docs.rs/crate/ptex-sys/latest)
- [ptex C++ documentation](https://ptex.us/documentation.html)
