#!/bin/bash

# Dep:
# apt-get install libncurses-dev libevent-dev

# local sources dir, to download git repo in
SRC="tmux"
# generated mans dir
DEST="../manuals"
# remote sources url
GIT_URL="https://github.com/tmux/tmux.git"
# global variables file
VARIABLES_FILE="versions.var"

# import variables
source $VARIABLES_FILE

# download sources
git clone $GIT_URL $SRC
cd $SRC

# iterate over versions
for version in "${TMUX_VERSIONS[@]}"
do
    echo
    echo "Processing tmux $version ..."
    echo

    # generate man
    git checkout $version
    ./configure
    make

    # copy man
    mkdir -p $DEST/man
    cp ./tmux.1 $DEST/man/tmux_$version.1
    # convert man to txt
    mkdir -p $DEST/txt
    man ./tmux.1 > $DEST/txt/tmux_$version.txt

    echo
    echo "Processing tmux $version ... done"
    echo
done

ls -la $DEST/txt
