use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Response};
use neutron_sdk::{NeutronResult, bindings::msg::NeutronMsg};

use crate::storage::{Application, ProjectFunding};


pub type ExecuteResponse = NeutronResult<Response<NeutronMsg>>;


#[cw_serde]
pub enum QueryMsg {
    /// this query goes to neutron and get stored ICA with a specific query
    InterchainAccountAddress {
        interchain_account_id: String,
        connection_id: String,
    },
    // this query returns ICA from contract store, which saved from acknowledgpub(crate) ement
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
    // Register {
    //     connection_id: String,
    //     interchain_account_id: String,
    // },
    // Delegate {
    //     interchain_account_id: String,
    //     validator: String,
    //     amount: u128,
    //     denom: String,
    //     timeout: Option<u64>,
    // },
    // Undelegate {
    //     interchain_account_id: String,
    //     validator: String,
    //     amount: u128,
    //     denom: String,
    //     timeout: Option<u64>,
    // },

    SubmitProposal {
        title: String,
        description: String
    },
    SubmitApplication {
        proposal_id: u64,
        configuration: Application
    },
    FundProposal {
        proposal_id: u64,
        auto_agree: Option<bool>
    },
    RegisterICA {
        proposal_id: u64,
        connection_id: String,
    },
    ApproveApplication {
        proposal_id: u64,
        application_sender: Addr,
    },

    AcceptApplication {
        proposal_id: u64,
        application_sender: Addr,
    },

    VerifyApplication {
        proposal_id: u64,
        application_sender: Addr,
        stop_at: Option<u64>
    },
}


#[cw_serde]
pub struct FullProposalInfo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub funding: Vec<(String, ProjectFunding)>,
    pub applications: Vec<(Addr, Application)>
}


#[cw_serde]
pub struct AllProposalResponse {
    pub proposals: Vec<FullProposalInfo>
}