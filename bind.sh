#!/bin/sh
set -e

if test -z "${PTEX_ROOT}"
then
  echo 1>&2 "error: PTEX_ROOT must be set in the environment"
  exit 1
fi

CMAKE_PREFIX_PATH=${PTEX_ROOT}
export CMAKE_PREFIX_PATH

PTEX_MAJOR_VERSION=2
PTEX_MINOR_VERSION=4
PTEX_PATCH_VERSION=1

num_procs=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

rm -rf build

astgen bind -u -v 1 -o build/ast -- "$@" -I${PTEX_ROOT}/include

asttoc build/ast -o build -p ptex -L $PTEX_ROOT/lib -l Ptex \
        --fp Ptex --tll Ptex::Ptex_dynamic \
        -major ${PTEX_MAJOR_VERSION} \
        -minor ${PTEX_MINOR_VERSION} \
        -patch ${PTEX_PATCH_VERSION}

mkdir -p ptex-sys/build
(
  cd ./ptex-sys/build
  cmake -D CMAKE_INSTALL_PREFIX=../dist ../ptex-c
  cmake --build . --target all --parallel ${num_procs}
)

mkdir -p ptex-sys/target/debug/deps
mkdir -p ptex-sys/target/release/deps

cp "${PTEX_ROOT}/lib"/libPtex.* ptex-sys/target/debug/deps
cp "${PTEX_ROOT}/lib"/libPtex.* ptex-sys/target/release/deps

cp test.rs ptex-sys/src/

echo
echo '#' Run these commands following to build and test the rust bindings:
echo
echo ' ' export CMAKE_PREFIX_PATH=${CMAKE_PREFIX_PATH}
echo ' ' cargo build --manifest-path=ptex-sys/Cargo.toml
echo ' '
echo ' ' cp test.rs ptex-sys/src/
echo ' ' cargo test --manifest-path=ptex-sys/Cargo.toml
