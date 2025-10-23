#!/bin/bash

# This is not meant directly executed
# just the set of commands to install in a fresh ubuntu vm for testing

# if a sudo password is needed, how can this process be automated?
# can it only be automated with the entire process to be automated?
sudo apt install curl

yes | sudo apt install npm

yes | sudo apt install pkg-config

# TODO: use here-strings or here-documents to provide input to this command
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL

# initialize a project
# basically do everything except deploy a final project

# open default presentation in browser

oseda init

oseda run

# open local host in browser, look up command to do that 

