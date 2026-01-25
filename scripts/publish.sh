#!/bin/bash
#
# publish CLI to crates.io
# Enables installation through `cargo install oseda-cli`
#

set -e

cargo build --release

# write the usage file
# if this updates, you may need to run the publish script again
cargo run --bin oseda-usage

cargo test

# see if valid compilation w/o warnings
cargo clippy --all-targets -- -D warnings

cargo fmt --check || echo "Format check failed, please run 'cargo fmt'"

cargo doc

cargo publish --dry-run && cargo publish || echo "Could not publish due to --dry-run failure"
