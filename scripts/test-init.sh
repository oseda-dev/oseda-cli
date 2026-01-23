#!/bin/bash

rm -rf test/*
mkdir -p test

set -e
./scripts/build.sh

cp target/release/oseda test

cd test


pwd
./oseda init 
