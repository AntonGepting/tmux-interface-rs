#!/bin/sh

# get subcommand
subcommand=$1

# process subcommand
case $subcommand in
    "-V")
        # return wrong version
        echo "tmux a2.9a\n"
        ;;
esac
