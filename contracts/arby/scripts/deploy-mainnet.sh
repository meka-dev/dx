#!/usr/bin/env sh

set -e

terrad tx wasm store ./artifacts/arby.wasm \
  --from=$DEPLOYER \
  --keyring-backend=$KEYRING_BACKEND \
  --chain-id=columbus-5 \
  --node=http://ter3:26657 \
  --gas=auto \
  --gas-prices="0.38uusd" \
  --gas-adjustment=1.4 \
  --broadcast-mode=block
