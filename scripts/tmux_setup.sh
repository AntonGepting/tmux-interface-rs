#!/bin/bash

# create sources dir, with parents
mkdir -p ~/source
# create build dir, with parents
mkdir -p ~/build/tmux-$TMUX_VERSION

# check if sources not exist? download it
if [[ ! -d ~/source/tmux-$TMUX_VERSION ]]; then
    git clone https://github.com/tmux/tmux.git ~/source/tmux-$TMUX_VERSION;
fi

cd ~/source/tmux-$TMUX_VERSION
git checkout $TMUX_VERSION
export PATH="$HOME/build/tmux-$TMUX_VERSION/bin:$PATH"
echo $PATH

if [[ $TMUX_VERSION == "master" || ! $(tmux -V | grep $TMUX_VERSION) ]]; then
    sh autogen.sh;
fi

if [[ $TMUX_VERSION == "master" || ! $(tmux -V | grep $TMUX_VERSION) ]]; then
    ./configure --prefix=$HOME/build/tmux-$TMUX_VERSION && make && make install;
fi

cd $TRAVIS_BUILD_DIR

tmux -V
