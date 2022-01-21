#!/bin/bash

if [ -z "$KEEP_NAMES" ]; then
  export RUSTFLAGS='-C link-arg=-s'
else
  export RUSTFLAGS=''
fi

cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/near_contract_sample1.wasm ./res/
#wasm-opt -Oz --output ./res/status_message_collections.wasm ./res/status_message_collections.wasm
