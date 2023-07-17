#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult};

use cw2::set_contract_version;

use crate::execute::{
    accept_application, approve_application, fund_proposal_native, register_ica,
    submit_application, submit_proposal, verify_application,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, NeutronResponse, QueryMsg};
use crate::query::{
    query_acknowledgement_result, query_address_funds, query_all_proposals, query_errors_queue,
    query_interchain_address, query_interchain_address_contract, query_proposal,
};
use crate::sudo::{
    prepare_sudo_payload, sudo_error, sudo_open_ack, sudo_timeout, sudo_tx_query_result,
};
use crate::temp::register_transfers_query;

use neutron_sdk::{bindings::query::NeutronQuery, sudo::msg::SudoMsg, NeutronResult};

use crate::storage::SUDO_PAYLOAD_REPLY_ID;

// Default timeout for SubmitTX is two weeks
//const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> NeutronResponse {
    deps.api.debug("WASMDEBUG: instantiate");
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> NeutronResponse {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::SubmitProposal { title, description } => {
            submit_proposal(deps.storage, title, description)
        }
        ExecuteMsg::SubmitApplication {
            proposal_id,
            application,
        } => submit_application(deps.storage, env, info.sender, proposal_id, application),
        ExecuteMsg::FundProposal {
            auto_agree,
            proposal_id,
        } => fund_proposal_native(deps.storage, info, proposal_id, auto_agree),
        ExecuteMsg::ApproveApplication {
            proposal_id,
            application_sender,
        } => approve_application(deps.storage, &info.sender, proposal_id, application_sender),

        ExecuteMsg::RegisterICA {
            proposal_id,
            connection_id,
        } => register_ica(deps.storage, env, connection_id, proposal_id),
        ExecuteMsg::AcceptApplication {
            proposal_id,
            application_sender,
        } => accept_application(deps.storage, info.sender, proposal_id, application_sender),

        ExecuteMsg::VerifyApplication {
            proposal_id,
            application_sender,
            stop_at: _,
        } => verify_application(deps.storage, info.sender, proposal_id, application_sender),

        ExecuteMsg::TempRegister {
            connection_id,
            recipient,
            update_period,
            min_height,
        } => register_transfers_query(connection_id, recipient, update_period, min_height),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {
    match msg {
        QueryMsg::AllProposals {} => query_all_proposals(deps.storage),

        QueryMsg::Proposal { proposal_id } => query_proposal(deps.storage, proposal_id),

        QueryMsg::AddressFunds {
            address,
            skip_locked,
        } => query_address_funds(deps.storage, &address, skip_locked.unwrap_or(false)),

        QueryMsg::InterchainAccountAddress {
            connection_id,
            proposal_id,
        } => query_interchain_address(deps, env, connection_id, proposal_id),

        QueryMsg::InterchainAccountAddressFromContract { proposal_id } => {
            query_interchain_address_contract(deps, env, proposal_id)
        }

        QueryMsg::AcknowledgementResult {
            sequence_id,
            proposal_id,
        } => query_acknowledgement_result(deps, env, sequence_id, proposal_id),

        QueryMsg::ErrorsQueue {} => query_errors_queue(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut<NeutronQuery>, env: Env, msg: SudoMsg) -> NeutronResponse {
    deps.api
        .debug(format!("WASMDEBUG: sudo: received sudo msg: {:?}", msg).as_str());

    match msg {
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            deps.storage,
            env,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        ),

        SudoMsg::TxQueryResult {
            query_id,
            height,
            data,
        } => sudo_tx_query_result(deps, env, query_id, height, data),

        SudoMsg::Error { request, details } => sudo_error(deps, request, details),

        SudoMsg::Timeout { request } => sudo_timeout(deps.storage, request),

        _ => Ok(Response::default()),
    }
}

#[entry_point]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", msg).as_str());

    match msg.id {
        SUDO_PAYLOAD_REPLY_ID => prepare_sudo_payload(deps, env, msg),
        _ => Err(StdError::generic_err(format!(
            "unsupported reply message id {}",
            msg.id
        ))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
}
