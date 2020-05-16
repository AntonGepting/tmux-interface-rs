#!/bin/bash

# Dep:
# apt-get install libncurses-dev libevent-dev

# local sources dir, to download git repo in
SRC="tmux"
# generated mans dir
DEST="../manuals"
# remote sources url
GIT_URL="https://github.com/tmux/tmux.git"
VERSIONS=("0.8" "0.9" \
    "1.0" "1.1" "1.2" "1.3" "1.4" "1.5" "1.6" "1.7" "1.8" "1.9" "1.9a" \
    "2.0" "2.1" "2.2" "2.3" "2.4" "2.5" "2.6" "2.7" "2.8" "2.9" "2.9a" \
    "3.0" "3.0a" "3.1" "3.1a" "3.1b" \
    "master" )

# download sources
git clone $GIT_URL $SRC
cd $SRC

# iterate over versions
for version in "${VERSIONS[@]}"
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
