# CrossFund

Cross-chain funding platform powered by CosmWasm Neutron-SDK and IBC

## Building Contract
`
RUST_BACKTRACE=1 docker run --rm -v "$(pwd)":/code   \
 --mount type=volume,source="dev_cache_template",target=/contracts/template/target \
 --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
 cosmwasm/workspace-optimizer-arm64:0.13.0  ./contracts/template
`

## Running integration tests
Follow this guide for setting up a local envieronment

https://docs.neutron.org/tutorials/integration_tests

but use this repository instead of
`github.com:neutron-org/neutron-integration-tests.git`


Name your built wasm file `crossfund.wasm`and put it inside `contracts` directory

After that:
`yarn test`
