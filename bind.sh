#!/bin/sh
set -e
set -x

if test -z "${PTEX_ROOT}"
then
  echo 1>&2 "error: PTEX_ROOT must be set in the environment"
  exit 1
fi

CMAKE_PREFIX_PATH=${PTEX_ROOT}${CMAKE_PREFIX_PATH:+:$CMAKE_PREFIX_PATH}
export CMAKE_PREFIX_PATH

PTEX_MAJOR_VERSION=2
PTEX_MINOR_VERSION=4
PTEX_PATCH_VERSION=1

PTEX_SYS_MAJOR_VERSION=0
PTEX_SYS_MINOR_VERSION=0
PTEX_SYS_PATCH_VERSION=6

num_procs=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

rm -rf build
mkdir -p build/ast

astgen bind -u -v 1 -o build/ast -- "$@" -I${PTEX_ROOT}/include

ptex_root_relpath=$(python -c "import os; print(os.path.relpath('${PTEX_ROOT}'))")

asttoc build/ast -o . -p ptex \
    --author 'Anders Langlands <anderslanglands@gmail.com>' \
    --author 'David Aguilar <davvid@gmail.com>' \
    --fp Ptex --tll Ptex::Ptex_dynamic \
    -L "${ptex_root_relpath}/lib64" \
    -L "${ptex_root_relpath}/lib" \
    -l Ptex \
    -major ${PTEX_SYS_MAJOR_VERSION} \
    -minor ${PTEX_SYS_MINOR_VERSION} \
    -patch ${PTEX_SYS_PATCH_VERSION}

cmake -B ptex-sys/build -D CMAKE_INSTALL_PREFIX="${PWD}/dist" ptex-sys/ptex-c
cmake --build ptex-sys/build --target all --parallel ${num_procs}
cmake --build ptex-sys/build --target install --parallel ${num_procs}

for deps in ptex-{rs,sys}/target/{debug,release}/deps
do
    mkdir -p "$deps"
    cp "${PTEX_ROOT}"/lib*/libPtex.* "$deps"
done

# Disable command echoing.
set +x

echo
echo '#' Run these commands following to build and test the rust bindings:
echo
echo ' ' export CMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
echo ' ' cargo build --manifest-path=ptex-sys/Cargo.toml
echo ' ' cargo build --manifest-path=ptex-rs/Cargo.toml
echo ' ' cargo test --manifest-path=ptex-rs/Cargo.toml
echo
