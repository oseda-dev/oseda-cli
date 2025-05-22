#!/bin/bash

rm -rf test/*
mkdir -p test



./scripts/build.sh

cp target/release/oseda test



cd test


pwd
./oseda init
