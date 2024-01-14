#!/usr/bin/env sh

cargo build -r
sudo mv ./target/release/mm /usr/local/bin/mm
