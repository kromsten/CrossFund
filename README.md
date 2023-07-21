# CrossFund

Cross-chain funding platform powered by CosmWasm Neutron-SDK and IBC


### Messages:

**Proposal Submission:**


```rs
SubmitProposal {
  title: String,
  description: String,
}
```

Simple operation that serves to propose a project to indicate interest towards a certain project missing in the ecosystem. Can be initiated by anyone including parties interested in funding a project or and development teams as a requirement for the following steps. 

Currently anyone can create a proposal on a chain where the contract is deployed. Cross-chain creations should be possible using IBC-hooks. 

Possible changing towards following Cosmos *MsgSubmitProposal* (add *metadata*, *description* -> *summary*)



**Proposal Funding:**

```rs
FundProposal {
  proposal_id: u64,
  auto_approve: Option<bool>,
}
```

Send funds towards any existing proposal. The funds are held in custody of the contract and redeemable at any point except for when they are locked for a specific prorpsal application and accepted.


*auto_approve* field will be explained later



**Submitting an application:**

```rs
SubmitApplication {
  proposal_id: u64,
  description: String,
  application: ApplicationSubmission {
    applicants: Vec<{ address, share }>,
    auditors: Vec<{ address, share }>,
    deliver_by: Expiration,
}
```

Create an application for a proposal. Each application nust submit a list of applicant addresses and their shares from proposal funding and list of independent auditors with their shares for work verification. It's up to community to decide whether the applicant and auditers are quialifed for a task. 

**Submitting an application:**

```rs
ApproveApplication {
  proposal_id: u64,
  application_sender: Addr
}
```

Targets all deposited funds of sender towards a specific application. If *auto_approve* is set when funding they will be auto-approved for a an application that has more than the half of all the funds deposited for a proposol (per token denom currently)


**Accepting work on proposal:**

```rs
AcceptApplication {
  proposal_id: u64,
  application_sender: Addr
}
```

A message send by application senders meant when they are ready to work on a proposal. As soon as submitted all the funds approved for an application are becoming locked until auditors submit confirmation message or until application expiration date. Funds send towards a proposal with auto_approve field will be also locked at this point if more than half of the funds were approved for an application. 

**Application Verification**
```rs
VerifyApplication {
  proposal_id: u64,
  application_sender: Addr
}
```

A message send by each application auditor when they are notified about delieverables and verify them. As soon as each auditors sunmit the message the funds are being released to all of the application shareholders.


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
