#!/bin/bash
curl https://raw.githubusercontent.com/osmosis-labs/assetlists/main/osmosis-1/osmosis-1.assetlist.json \
	| jq '[ .assets[] | {decimals: .denom_units[] | select(.exponent != 0) | .exponent, denom: .base, name: .name}]'
