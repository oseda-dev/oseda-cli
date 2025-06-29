#!/bin/bash
#
# publish CLI to crates.io
# Enables installation through `cargo install oseda-cli`
#
cargo build --release

# cargo test # should have these :skull:

# cargo clippy --all-targets -- -D warnings
# should pass clippy first too
#

cargo fmt --check

cargo doc --open # should generate all documenation -> oops no documentation

cargo publish --dry-run

if [ $? -eq 0 ]; then
    cargo publish
else
    echo "Could not publish due to --dry-run failure"
fi
