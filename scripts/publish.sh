#!/bin/bash
#
# publish CLI to crates.io
# Enables installation through `cargo install oseda-cli`
#
cargo build --release

cargo test

# see if valid compilation w/o warnings
cargo clippy --all-targets -- -D warnings

cargo fmt --check

cargo doc

cargo publish --dry-run && cargo publish || echo "Could not publish due to --dry-run failure"
