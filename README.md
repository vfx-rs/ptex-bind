# ptex-bind
cppmm bindings for Ptex 2.1.1

# Generating bindings
First you must have `astgen` and `asttoc` in your PATH. 

run `bind.sh`, passing the path to your Ptex and installation as an environment variable:
```bash
env PTEX_ROOT=/path/to/ptex
```

The resulting bindings will be create in `build/ptex-c` and `build/ptex-sys` for the C and Rust bindings, respectively

# Versions
There is a branch for each supported minor version of the target library. Other versions may or may not bind successfully.
