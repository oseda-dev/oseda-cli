#!/bin/bash

# This is not meant directly executed
# just the set of commands to install in a fresh ubuntu vm for testing
# must be included for this script to work
source mock_data.sh

echo $ADMIN_PASSWD | sudo -S apt update
sudo apt install curl

# Pipe yes into these installation commands
yes | sudo apt install git

yes | sudo apt install npm

yes | sudo apt install pkg-config

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


# Puts cargo on the PATH without having to restart the current shell
. "$HOME/.cargo/env"

#curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL
cargo install oseda-cli

# make sure .ssh directory exists
mkdir -p "${HOME}/.ssh"
# add public and private key
echo $PUBLIC_KEY > "${HOME}/.ssh/id_ed25519_oseda_testing.pub"
echo $PRIVATE_KEY > "${HOME}/.ssh/id_ed25519_oseda_testing"
# change permissions for keys
chmod 600 "${HOME}/.ssh/id_ed25519_oseda_testing.pub"
chmod 600 "${HOME}/.ssh/id_ed25519_oseda_testing"

# assuming git username is same as github username
git config --global user.name $USERNAME
git config --global user.email $EMAIL

# create new oseda project, once flags are implemented this will change so it's more automated.
# currently have to stop here to manually put in options for new project
oseda init
# must name the project test for this to work (for now)
cd test

#oseda run
oseda check

# if oseda check fails, then it prints the exit code
# otherwise ends the script with a message saying it succeeded
if [$? -ne 0]; then
    echo "Command failed. Exit code was $?"
else 
    echo "Script succeeded"
fi