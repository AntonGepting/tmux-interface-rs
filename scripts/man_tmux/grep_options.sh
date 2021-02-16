#!/bin/bash

# .....option-name[] [value | value]
#egrep -nrw . -e '^ {5}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' --color=always | sort

SRC="manuals/txt"
DEST="options"
# global variables file
VARIABLES_FILE="versions.var"

# import varibales
source $VARIABLES_FILE


mkdir -p $DEST

# iterate over versions
for version in "${TMUX_VERSIONS[@]}"
do
    echo
    echo "Processing tmux $version ..."
    echo

    # get lines between OPTIONS and next chapter
    sed -n -e '/^OPTIONS/,/^[A-Z]/p' $SRC/tmux_$version.txt | \
    # grep options
    egrep -e '^ {5,13}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' > $DEST/options_$version.txt
    # remove leading spaces
    sed "s/^[ \t]*//" -i $DEST/options_$version.txt

    echo
    echo "Processing tmux $version ... done"
    echo
done
