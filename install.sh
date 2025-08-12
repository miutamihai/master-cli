#!/usr/bin/env bash

set -e

echo "Building..."
zig build --release=safe
echo "Building... DONE"

echo "Replacing old bin..."
rm /usr/local/bin/mm && cp ./zig-out/bin/mm /usr/local/bin
echo "Replacing old bin... DONE"
