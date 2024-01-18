#!/usr/bin/env bash

git clone git@gitlab.com:miutamihai/master-maker-cli.git mm

cd mm || exit

cargo build -r
sudo mv ./target/release/mm /usr/local/bin/mm

cd ..
rm -rf mm
