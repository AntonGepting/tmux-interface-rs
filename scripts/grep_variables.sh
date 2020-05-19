#!/bin/bash

# .....option-name[] [value | value]
#egrep -nrw . -e '^ {5}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' --color=always | sort

SRC="manuals/txt"
DEST="variables"
VERSIONS=("0.8" "0.9" \
    "1.0" "1.1" "1.2" "1.3" "1.4" "1.5" "1.6" "1.7" "1.8" "1.9" "1.9a" \
    "2.0" "2.1" "2.2" "2.3" "2.4" "2.5" "2.6" "2.7" "2.8" "2.9" "2.9a" \
    "3.0" "3.0a" "3.1" "3.1a" "3.1b" \
    "master")

mkdir -p $DEST

# iterate over versions
for version in "${VERSIONS[@]}"
do
    echo
    echo "Processing tmux $version ..."
    echo

    # get lines between FORMAT and next chapter
    sed -n -e '/The following variables are available, where appropriate/,/^[A-Z]/p' $SRC/tmux_$version.txt | \
    # grep variables
    egrep -e '^ {5,11}[a-zA-Z0-9-]+.+' > $DEST/variables_$version.txt
    # remove leading spaces
    sed "s/^[ \t]*//" -i $DEST/variables_$version.txt

    echo
    echo "Processing tmux $version ... done"
    echo
done
