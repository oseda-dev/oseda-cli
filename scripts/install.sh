#!/usr/bin/env bash
set -e

cargo build --release --bin oseda

INSTALL_DIR="$HOME/.local/bin"

mkdir -p "$INSTALL_DIR"

cp target/release/oseda "$INSTALL_DIR"

chmod +x "$INSTALL_DIR/oseda"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    # checks if the bin dir is in PATH
    # cause if not this install is basically pointless
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "You might want to add this line to your shell config:"
    echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo "oseda installed to $INSTALL_DIR"
