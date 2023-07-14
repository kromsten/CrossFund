use cosmwasm_schema::cw_serde;

use crate::storage::Configuration;

#[cw_serde]
pub enum QueryMsg {
    /// this query goes to neutron and get stored ICA with a specific query
    InterchainAccountAddress {
        interchain_account_id: String,
        connection_id: String,
    },
    // this query returns ICA from contract store, which saved from acknowledgement
    InterchainAccountAddressFromContract {
        interchain_account_id: String,
    },
    // this query returns acknowledgement result after interchain transaction
    AcknowledgementResult {
        interchain_account_id: String,
        sequence_id: u64,
    },
    // this query returns non-critical errors list
    ErrorsQueue {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register {
        connection_id: String,
        interchain_account_id: String,
    },
    Delegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
        denom: String,
        timeout: Option<u64>,
    },
    Undelegate {
        interchain_account_id: String,
        validator: String,
        amount: u128,
        denom: String,
        timeout: Option<u64>,
    },

    SubmitProposal {
        description: String
    },
    SubmitConfiguration {
        configuration: Configuration
    },
    Fund {},
    Vote {},
    Verify {}
}
