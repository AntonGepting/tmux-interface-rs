#!/bin/bash

# .....option-name[] [value | value]
#egrep -nrw . -e '^ {5}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' --color=always | sort

SRC="manuals/txt"
DEST="options"
VERSIONS=("0.8" "0.9" \
    "1.0" "1.1" "1.2" "1.3" "1.4" "1.5" "1.6" "1.7" "1.8" "1.9" "1.9a" \
    "2.0" "2.1" "2.2" "2.3" "2.4" "2.5" "2.6" "2.7" "2.8" "2.9" "2.9a")

mkdir -p $DEST

# iterate over versions
for version in "${VERSIONS[@]}"
do
    echo
    echo "Processing tmux $version ..."
    echo

    egrep -rw $SRC/tmux_$version.txt -e '^ {13}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' > $DEST/options_$version.txt
    # remove leading spaces
    sed "s/^[ \t]*//" -i $DEST/options_$version.txt

    echo
    echo "Processing tmux $version ... done"
    echo
done
