#!/bin/bash -e

echo "Building for target: $1"

cross build --release --target $1

echo "Build complete"