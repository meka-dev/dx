# Arby

## Deploy

via terrad cli

1. from the root of the repo run the optimizer

```Bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

this will output the optimized wasm blob into the `artifacts` directory

2. store the wasm blob on-chain

```Bash
terrad tx wasm store ./artifacts/arby.wasm --from $deployer \
  --keyring-backend test --chain-id bombay-12 --node https://bombay.stakesystems.io:2053 \
  --gas auto --gas-prices 0.169598uluna --gas-adjustment 1.2 --broadcast-mode block
```

the response will include the code-id which we then use to make instances from

3. instantiate with a bunch of sprinters, the sender will be set as owner for the instances,
   `66009` is the code-id and `'{}'` is the instantiate message

```Bash
terrad tx wasm instantiate 66009 '{}' \
  --from $deployer --keyring-backend test --chain-id bombay-12 --node https://bombay.stakesystems.io:2053 \
  --gas auto --gas-prices 0.169598uluna --gas-adjustment 1.2 --broadcast-mode block
```

or with an admin account which is allowed to migrate the contract to new code:

```Bash
terrad tx wasm instantiate 66009 '{}' \
  --from $deployer --keyring-backend test --chain-id bombay-12 --node https://bombay.stakesystems.io:2053 \
  --gas auto --gas-prices 0.169598uluna --gas-adjustment 1.2 --broadcast-mode block \
  --admin terra145gf3tt6k9yxf9km79vjfnu9x848c84z3faz3y
```
