#!/bin/bash
# Build the Axiom Coin project

echo "Building all crates..."
cargo build

echo "Running tests..."
cargo test

echo "Build complete!"
