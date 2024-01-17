#!/usr/bin/env bash

last_commit_hash=$(git rev-parse HEAD)

echo $last_commit_hash

# git clone git@gitlab.com:miutamihai/master-maker-cli.git mm
# 
# cd mm || exit
# 
# cargo build -r
# sudo mv ./target/release/mm /usr/local/bin/mm
# 
# cd ..
# rm -rf mm
