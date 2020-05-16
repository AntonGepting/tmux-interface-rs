#!/bin/bash

SRC="manuals/txt"
DEST="options"
VERSIONS=("3.0" "3.0a" "3.1" "3.1a" "3.1b" "master")

mkdir -p $DEST

# iterate over versions
for version in "${VERSIONS[@]}"
do
    echo
    echo "Processing tmux $version ..."
    echo

    egrep -rw $SRC/tmux_$version.txt -e '^ {5}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' > $DEST/options_$version.txt
    # remove leading spaces
    sed "s/^[ \t]*//" -i $DEST/options_$version.txt

    echo
    echo "Processing tmux $version ... done"
    echo
done
