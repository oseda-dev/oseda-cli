#!/bin/bash

# This is not meant directly executed
# just the set of commands to install in a fresh ubuntu vm for testing

#TODO: make a MOCK_DATA.sh file to hold sudo password, as well as GitHub credentials
sudo apt install curl

# Pipe yes into these installation commands
yes | sudo apt install npm

yes | sudo apt install pkg-config

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Restart the terminal after installing rust so it gets added to the path.
reset

curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL

# TODO: 
# Automate logging into a dummy GitHub account so a new oseda project can be initialized
# Initialize and run a new oseda project in the user's browser

oseda init

oseda run