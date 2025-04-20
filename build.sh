#!/bin/bash

# make bin if not exist
mkdir -p bin

# kill old files inside the bin
rm bin/*

# rec globing with the ** thingy
shopt -s globstar

gcc -Isrc src/**/*.c -o bin/oseda
