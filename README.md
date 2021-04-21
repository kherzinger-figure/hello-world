# Hello World Smart Contact

## Build

Make sure $PIO_HOME is set

Compile and install

```sh
make && make install
```

## Example Usage

### Create an account

This account will own the smart contract.

```sh
provenanced keys add hello --hd-path "44'/1'/0'/0/0" -i
```

### Give the account some hash

```sh
provenanced tx bank send \
    $(provenanced keys show -a main_validator) \
    $(provenanced keys show -a hello) \
    100000000nhash \
    --from main_validator \
    --chain-id localnet \
    --gas auto \
    --fees 2000nhash \
    --broadcast-mode block \
    --yes
```

### Create a namespace to bind the smart contract to

```sh
provenanced tx name bind \
    "sc" \
    $(provenanced keys show -a main_validator) \
    "pb" \
    --restrict=false \
    --from main_validator \
    --chain-id localnet \
    --gas auto \
    --fees 5000nhash \
    --broadcast-mode block \
    --yes
```

Store the WASM contract on the blockchain:

```sh
provenanced tx wasm store ${PIO_HOME}/hello_world.wasm \
    --source "https://github.com/provenance-io/hello-world" \
    --builder "cosmwasm/rust-optimizer:0.10.7" \
    --instantiate-only-address $(provenanced keys show -a hello) \
    --from hello \
    --chain-id localnet \
    --gas auto \
    --fees 40000nhash \
    --broadcast-mode block \
    --yes
```

Ensure that the contract was stored:

```sh
provenanced query wasm list-code -o json | jq
```

Instantiate the WASM contract:

```sh
provenanced tx wasm instantiate 1 \
    '{"bind_name":"hello_world.sc","contract_name":"hello_world"}' \
    --admin $(provenanced keys show -a hello) \
    --label hello-world \
    --from hello \
    --chain-id localnet \
    --gas-adjustment 1.4 \
    --fees 7000nhash \
    --broadcast-mode block \
    --yes
```

```sh
provenanced query wasm list-contract-by-code 1
```

========================================================================

provenanced query wasm contract-state smart pb19xn7cuk6tmxsn3gps73xvew3kqj6kfxszptc6g '{"get_ask":{"id":"ask_id"}}'