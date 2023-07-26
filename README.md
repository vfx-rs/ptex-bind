# ptex-bind

Rust bindings for [Ptex](https://github.com/wdas/ptex)

## Links

- [source repository](https://github.com/vfx-rs/ptex-bind)
- [ptex on crates.io](https://crates.io/crates/ptex/latest)
- [ptex documentation](https://docs.rs/crate/ptex/latest)
- [ptex core documentation](https://ptex.us/documentation.html)


## Usage

Add this to your `Cargo.toml`:

    [dependencies]
    ptex = "0.0.6"


## Introduction

The `ptex` crate provides a high-level safe API over `cxx`-generated `ptex::sys::ffi` bindings.
`ptex::sys::ffi` should not be used directly.

`src` contains the [ptex](https://crates.io/crates/ptex) crate.

`bind` contains the [cxx](https://cxx.rs) C++ bridge headers that are used to
generate the internal `ptex::sys::ffi` module.


## Development

The easiest way to build the `ptex` crate is to run the following
setup commands using [garden](https://gitlab.com/garden-rs/garden).

```bash
garden grow ptex
garden build all
```


These commands perform the following steps:

- `garden grow` clones [Ptex](https://github.com/wdas/ptex) into the `ptex` directory.

- `garden build all` builds the C++ Ptex library, installs it to `ptex/dist`,
  configures `PKG_CONFIG_PATH` and `LD_LIBRARY_PATH` to point into `ptex/dist`,
  and builds the `ptex` create using `cargo build`.


The test suite in `tests` is used to validate the `ptex` crate.

```bash
# Run tests
garden test

# Run checks
garden check
```

## Versions

The `ptex` crate currently tracks the latest stable Ptex version.

Branches will be created for older versions in the future when newer
major or minor Ptex releases are available.

The tags in this repository correspond to the `ptex` crate versions.
The `ptex` crate version will be tagged and released with a non-v0
version number that matches the underlying C++ Ptex library once the
`ptex` crate is feature-complete.
