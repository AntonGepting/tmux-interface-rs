#!/bin/bash

# compilation root
SRC_ROOT=".."
# global variables file
VARIABLES_FILE="versions.var"

# import variables
source $VARIABLES_FILE


cd $SRC_ROOT

# iterate over versions
for crate_feature in "${CRATE_FEATURES[@]}"
do
    echo
    echo "Processing with feature $crate_feature ..."
    echo

    cargo build --features $crate_feature --no-default-features --verbose

    echo
    echo "Processing with feature $crate_feature ... done"
    echo
done
