#!/bin/bash

echo "Cleaning bin..."
# make bin if not exist
mkdir -p bin

# kill old files inside the bin
rm bin/*

# only set e now bc the old command could fail and be okay
set -e

# rec globing with the ** thingy
shopt -s globstar

echo "Compiling oseda-cli to bin/oseda"
gcc -Isrc src/**/*.c -o bin/oseda
