use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Response};
use cw_utils::Expiration;
use neutron_sdk::{bindings::msg::NeutronMsg, NeutronResult};

use crate::storage::{AcknowledgementResult, Application, CustodyFunds, GoodFee, ProjectFunding};

pub type NeutronResponse = NeutronResult<Response<NeutronMsg>>;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AllProposalResponse)]
    AllProposals {},

    #[returns(FullProposalInfo)]
    Proposal { proposal_id: u64 },

    #[returns(Vec<(String, CustodyFunds)>)]
    AddressFunds {
        address: Addr,
        skip_locked: Option<bool>,
    },

    #[returns((String, String))]
    InterchainAccountAddress {
        connection_id: String,
        proposal_id: u64,
    },

    #[returns((String, String))]
    InterchainAccountAddressFromContract { proposal_id: u64 },

    #[returns(Option<AcknowledgementResult>)]
    AcknowledgementResult { sequence_id: u64, proposal_id: u64 },

    #[returns(Vec<(Vec<u8>, String)>)]
    ErrorsQueue {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SubmitProposal {
        title: String,
        description: String,
    },
    SubmitApplication {
        proposal_id: u64,
        application: ApplicationSubmission,
    },
    FundProposal {
        proposal_id: u64,
        auto_agree: Option<bool>,
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
        stop_at: Option<u64>,
    },

    TempRegister {
        connection_id: String,
        recipient: String,
        update_period: u64,
        min_height: Option<u64>,
    },
}

#[cw_serde]
pub struct ApplicationSubmission {
    pub applicants: Vec<GoodFee>,
    pub auditors: Vec<GoodFee>,
    pub deliver_by: Expiration,
}

#[cw_serde]
pub struct FullProposalInfo {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub funding: Vec<(String, ProjectFunding)>,
    pub applications: Vec<(Addr, Application)>,
}

#[cw_serde]
pub struct AllProposalResponse {
    pub proposals: Vec<FullProposalInfo>,
}
