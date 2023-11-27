#!/bin/bash

if [ -z "$1" ]; then
    echo "Year name not provided."
    exit 1
fi

# create a library for the year
cargo new --lib --name "$1" "$1"
# add the new lib to the executable
cargo add --path "$1"

# enter the new lib
cd "$1"

# local/custom utilities
cargo add --path "../pattern_parse"
cargo add --path "../common"

# used for nice error handling
cargo add "thiserror@1.0.37"
# generally nice
cargo add "bitflags@2.4.0"
cargo add "ahash@0.8.3"