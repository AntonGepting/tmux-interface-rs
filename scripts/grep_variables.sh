#!/bin/bash

# .....option-name[] [value | value]
#egrep -nrw . -e '^ {5}[a-zA-Z0-9-]+\[?\]? \[?[\| a-zA-Z0-9-]+\]?$' --color=always | sort

SRC="manuals/txt"
DEST="variables"
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
