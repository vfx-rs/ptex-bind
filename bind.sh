#!/bin/sh
if test -z "${PTEX_ROOT}"
then
  echo 1>&2 "error: PTEX_ROOT must be set in the environment"
  exit 1
fi

set -e
set -x

PTEX_MAJOR_VERSION=2
PTEX_MINOR_VERSION=4
PTEX_PATCH_VERSION=1


rm -rf build

astgen bind -u -v 1 -o build/ast -- -I${PTEX_ROOT}/include
asttoc build/ast -o build -p ptex -L $PTEX_ROOT/lib -l Ptex \
        --fp Ptex --tll Ptex::Ptex_dynamic \
        -major ${PTEX_MAJOR_VERSION} \
        -minor ${PTEX_MINOR_VERSION} \
        -patch ${PTEX_PATCH_VERSION}
