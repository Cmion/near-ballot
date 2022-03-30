#!/bin/bash

clear
TARGET="${CARGO_TARGET_DIR:-target}"
set -e
cd "`dirname $0`"
cargo build --target wasm32-unknown-unknown --release

cp $TARGET/wasm32-unknown-unknown/release/ballot.wasm ./res

#near dev-deploy --wasmFile target/wasm32-unknown-unknown/release/ballot.wasm --helperUrl https://near-contract-helper.onrender.com

near dev-deploy --wasmFile res/ballot.wasm --helperUrl https://near-contract-helper.onrender.com

source neardev/dev-account.env