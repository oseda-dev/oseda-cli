#!/usr/bin/env bash

prereqs=("npm" "cargo")


for prereq in "${prereqs[@]}"; do

    if ! command -v ${prereq} &> /dev/null; then
        echo "You are missing ${prereq}"
        echo "Please install ${prereq} to continue"
        exit 1
    fi

done


if ! cargo install oseda-cli; then
    echo "oopsies could not install oseda cli"
fi



if ! echo "$PATH" | grep -q "$HOME/.cargo/bin"; then
    echo "~/.cargo/bin is not in your \$PATH."
    echo "You will not be able to run oseda globally without this"
    # echo "Add this to your shell profile to use this tool globally:"
    # echo '  export PATH="$HOME/.cargo/bin:$PATH"'

    # mostly ripped from https://jvns.ca/blog/2025/02/13/how-to-add-a-directory-to-your-path/
    read -p "Would you like to add ~/.cargo/bin to your PATH? [Y/n] now (bash/zsh only, fish users, on your own)?" confirm
    if [[ "$confirm" =~ ^[Yy]?$ ]]; then
        profile="$HOME/.bashrc"
        [ -n "$ZSH_VERSION" ] && profile="$HOME/.zshrc"

        echo '' >> "$profile"
        echo '# added by oseda-cli installation' >> "$profile"
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$profile"
        echo "Added to $profile. Restart your shell to use the command."
        fi


fi
