#!/usr/bin/env sh

set -e

terrad tx feegrant grant $OWNER_ADDRESS $SPRINTER \
  --allowed-messages="/terra.wasm.v1beta1.MsgExecuteContract" \
  --from=$OWNER \
  --keyring-backend=$KEYRING_BACKEND \
  --chain-id="columbus-5" \
  --node="http://ter3:26657" \
  --gas auto \
  --gas-prices="0.38uusd" \
  --gas-adjustment="1.4" \
  --broadcast-mode block
