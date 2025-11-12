#!/bin/bash

# This is not meant directly executed
# just the set of commands to install in a fresh ubuntu vm for testing
source mock_data.sh

#TODO test to make sure this works
echo $ADMIN_PASSWD |sudo -S apt install curl 
yes | sudo apt install git

# Pipe yes into these installation commands
yes | sudo apt install npm

yes | sudo apt install pkg-config

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


# Puts cargo on the PATH without having to restart the current shell
. "$HOME/.cargo/env"

curl -sL https://raw.githubusercontent.com/oseda-dev/oseda-cli/refs/heads/main/scripts/curl-install.sh | $SHELL

#TODO: test this in VM before merging branch

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

oseda init

oseda run