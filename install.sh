#!/usr/bin/env sh

git clone git@gitlab.com:miutamihai/master-maker-cli.git mm

cd mm || exit

cargo build -r
sudo mv ./target/release/master_maker /usr/local/bin/mm

cd ..
rm -rf mm
