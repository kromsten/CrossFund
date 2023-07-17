# CrossFund

Cross-chain funding platform powered by CosmWasm Neutron-SDK and IBC

## Building Contract
`
RUST_BACKTRACE=1 docker run --rm -v "$(pwd)":/code   \
 --mount type=volume,source="dev_cache_template",target=/contracts/template/target \
 --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
 cosmwasm/workspace-optimizer:0.13.0  ./contracts/template
`

Name your built wasm file `crossfund.wasm`and put it inside `contracts` directory for integration testing

## Running integration tests
Follow this guide for setting up a local envieronment
https://docs.neutron.org/tutorials/integration_tests

After finishing installatiion you can put the directory of this repository as a sibling to other 4 if you wish to run the tests with docker using
`yarn test`

Alternatively run 
`make start-cosmopark`
in `setup` folder of git@github.com:neutron-org/neutron-integration-tests.git

and then in this repository:
`NO_DOCKER=1 yarn test`
