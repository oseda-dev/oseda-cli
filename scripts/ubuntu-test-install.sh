#!/bin/bash

# This is not meant directly executed
# just the set of commands to install in a fresh ubuntu vm for testing

sudo apt install curl

sudo apt install npm

sudo apt install pkg-config

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL
