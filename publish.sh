#!/bin/bash

set -eo xtrace

cargo clean

cargo +nightly clippy --all-targets --all-features -- -D warnings
cargo +nightly fmt --all
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-stable cargo +stable test --workspace --all-features
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-nightly cargo +nightly test --workspace --all-features
find . -name 'target' | xargs rm -rf
SNAPSHOT_DIR=rust-1.49.0 cargo +1.49 test --workspace --all-features

nvim Cargo.toml
cargo build

nvim CHANGELOG.md
nvim src/lib.rs

cargo readme > README.md

cargo publish --dry-run --allow-dirty

git add .
git commit
git push origin

set +o xtrace

echo "Next step: Wait for CI\n"
echo "Next step: \`git tag vX.Y.Z; git push --tags\`\n"
echo "Next step: Create release in Github\n"
echo "Next step: \`cargo publish\`"
