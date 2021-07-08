#! /usr/bin/env bash

# exit when any command fails
set -e

# keep track of the last executed command
trap 'last_command=$current_command; current_command=$BASH_COMMAND' DEBUG
# echo an error message before exiting
trap 'echo "\"${last_command}\" exited with code $?."' ERR

rm -rf build

astgen bind -u -v 1 -o build/ast -- -I${PTEX_ROOT}/include
asttoc build/ast -o build -p ptex -L $PTEX_ROOT/lib -l Ptex \
        --fp Ptex --tll Ptex::Ptex_dynamic \
        -major 2 -minor 4 -patch 0
