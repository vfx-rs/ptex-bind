# ptex-bind

Rust bindings for [Ptex](https://github.com/wdas/ptex)


## Introduction

The `ptex` crate provides a high-level safe API over `cxx`-generated `ptex-sys` bindings.
`ptex-sys` should not be used directly.

`src` contains the [ptex](https://crates.io/crates/ptex) crate.

`ptex-sys` contains the low-level Ptex sys bindings.
`ptex-sys` uses [cxx](https://cxx.rs) to wrap the C++ Ptex API.


## Links

- [source repository](https://github.com/vfx-rs/ptex-bind)
- [ptex on crates.io](https://crates.io/crates/ptex/latest)
- [ptex-sys on crates.io](https://crates.io/crates/ptex-sys/latest)
- [ptex documentation](https://docs.rs/crate/ptex/latest)
- [ptex-sys documentation](https://docs.rs/crate/ptex-sys/latest)
- [ptex C++ documentation](https://ptex.us/documentation.html)


## Usage

`ptex-bind` requires you to install the C++ [Ptex](https://github.com/wdas/ptex)
library in order to build the `ptex` crate.

Add ptex to your `Cargo.toml`:

    [dependencies]
    ptex = "0.0.6"

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

`ptex-bind` provides a `garden.yaml`
[Garden file](https://gitlab.com/garden-rs/garden) to automate the steps of
installing Ptex and building the `ptex` Rust crate.

The Garden file can be used to clone [Ptex](https://github.com/wdas/ptex) to
the `ptex` subdirectory and install the C++ Ptex library to `ptex/dist`.

The Garden file configures the `PKG_CONFIG_PATH` environment variable to
contain `$PWD/ptex/dist/share/pkgconfig` when running commands.

Run the following commands using [garden](https://gitlab.com/garden-rs/garden)
to bootstrap a development environment containing Ptex so that the the
`ptex` crate can be built.

```bash
garden grow ptex
garden build all
```

These commands perform the following steps:

- `garden grow` clones [Ptex](https://github.com/wdas/ptex) into the `ptex` directory.

- `garden build all` builds the C++ Ptex library, installs it to `ptex/dist`,
  configures `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` to point into `ptex/dist`,
  and builds the `ptex` crate using `cargo build`.

The test suite in `tests` is used to validate the `ptex` crate.

```bash
# Run tests
garden test

# Run checks and linters
garden check
```

### Workflow

A development workflow loop for `ptex-bind` involves the following steps:

- Build the Ptex C++ library.

- Buid the `ptex-sys` low-level Ptex bindings.

- Build the high-level `ptex` bindings.

- Run tests across all crates.

- Run checks across all crates.

The provided `garden.yaml` Garden configuration provides a convenient command to perform
all of the above steps in a single shot:

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
