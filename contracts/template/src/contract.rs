#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, SubMsg,
};

use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, ExecuteResponse};
use crate::execute::{submit_proposal, fund_proposal_native, submit_application, approve_application, register_ica, verify_application, accept_application};
use crate::query::{query_interchain_address, query_interchain_address_contract, query_acknowledgement_result, query_errors_queue};
use crate::sudo::{sudo_error, sudo_open_ack, sudo_response, sudo_timeout, prepare_sudo_payload};

use neutron_sdk::{
    bindings::{
        query::{NeutronQuery},
    },

    sudo::msg::{SudoMsg},
    NeutronResult,
};

use crate::storage::{
    save_reply_payload, SudoPayload, SUDO_PAYLOAD_REPLY_ID,
};

// Default timeout for SubmitTX is two weeks
const DEFAULT_TIMEOUT_SECONDS: u64 = 60 * 60 * 24 * 7 * 2;

const CONTRACT_NAME: &str = concat!("crates.io:neutron-sdk__", env!("CARGO_PKG_NAME"));
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ExecuteResponse {
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
) -> ExecuteResponse {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
    //ExecuteMsg::Register{connection_id,interchain_account_id,}=>execute_register_ica(deps,env,connection_id,interchain_account_id),
    //ExecuteMsg::Delegate{validator,interchain_account_id,amount,denom,timeout,}=>execute_delegate(deps,env,interchain_account_id,validator,amount,denom,timeout,),
    //ExecuteMsg::Undelegate{validator,interchain_account_id,amount,denom,timeout,}=>execute_undelegate(deps,env,interchain_account_id,validator,amount,denom,timeout,),
        ExecuteMsg::SubmitProposal { 
            title, 
            description } => submit_proposal(deps.storage, title, description),
        ExecuteMsg::SubmitApplication { 
            proposal_id, 
            configuration } => submit_application(deps.storage, info.sender, proposal_id, configuration),
        ExecuteMsg::FundProposal {
            auto_agree, 
            proposal_id 
        } => fund_proposal_native(deps.storage, info, proposal_id, auto_agree),
        ExecuteMsg::ApproveApplication { 
            proposal_id, 
            application_sender 
        } => approve_application(deps.storage, info.sender, proposal_id, application_sender),

        ExecuteMsg::RegisterICA { 
            proposal_id ,
            connection_id
        } => register_ica(
            deps.storage, 
            env,
            connection_id,
            proposal_id)
        ,
        ExecuteMsg::AcceptApplication { 
            proposal_id, 
            application_sender 
        } => accept_application(deps.storage, info.sender, proposal_id, application_sender),

        ExecuteMsg::VerifyApplication { 
            proposal_id, 
            application_sender,
            stop_at: _ 
        } => verify_application(deps.storage, info.sender, proposal_id, application_sender)
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<NeutronQuery>, env: Env, msg: QueryMsg) -> NeutronResult<Binary> {

    match msg {

        QueryMsg::InterchainAccountAddress {
            connection_id,
            proposal_id,
        } => query_interchain_address(deps, env, connection_id, proposal_id, ),

        QueryMsg::InterchainAccountAddressFromContract {
            proposal_id,
        } => query_interchain_address_contract(deps, env, proposal_id),

        QueryMsg::AcknowledgementResult {
            sequence_id,
            proposal_id,
        } => query_acknowledgement_result(deps, env, sequence_id, proposal_id),

        QueryMsg::ErrorsQueue {} => query_errors_queue(deps),
    }
}





// saves payload to process later to the storage and returns a SubmitTX Cosmos SubMsg with necessary reply id
fn msg_with_sudo_callback<C: Into<CosmosMsg<T>>, T>(
    deps: DepsMut<NeutronQuery>,
    msg: C,
    payload: SudoPayload,
) -> StdResult<SubMsg<T>> {
    save_reply_payload(deps.storage, payload)?;
    Ok(SubMsg::reply_on_success(msg, SUDO_PAYLOAD_REPLY_ID))
}



/* fn execute_delegate(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    interchain_account_id: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let (delegator, connection_id) = get_ica(deps.as_ref(), &env, &interchain_account_id)?;
    let delegate_msg = MsgDelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(Coin {
            denom,
            amount: amount.to_string(),
        }),
    };
    let mut buf = Vec::new();
    buf.reserve(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(buf),
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![any_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    // We use a submessage here because we need the process message reply to save
    // the outgoing IBC packet identifier for later.
    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: get_port_id(env.contract.address.as_str(), &interchain_account_id),
            message: "message".to_string(),
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}


fn execute_undelegate(
    mut deps: DepsMut<NeutronQuery>,
    env: Env,
    interchain_account_id: String,
    validator: String,
    amount: u128,
    denom: String,
    timeout: Option<u64>,
) -> NeutronResult<Response<NeutronMsg>> {
    // contract must pay for relaying of acknowledgements
    // See more info here: https://docs.neutron.org/neutron/feerefunder/overview
    let fee = min_ntrn_ibc_fee(query_min_ibc_fee(deps.as_ref())?.min_fee);
    let (delegator, connection_id) = get_ica(deps.as_ref(), &env, &interchain_account_id)?;
    let delegate_msg = MsgUndelegate {
        delegator_address: delegator,
        validator_address: validator,
        amount: Some(Coin {
            denom,
            amount: amount.to_string(),
        }),
    };
    let mut buf = Vec::new();
    buf.reserve(delegate_msg.encoded_len());

    if let Err(e) = delegate_msg.encode(&mut buf) {
        return Err(NeutronError::Std(StdError::generic_err(format!(
            "Encode error: {}",
            e
        ))));
    }

    let any_msg = ProtobufAny {
        type_url: "/cosmos.staking.v1beta1.MsgUndelegate".to_string(),
        value: Binary::from(buf),
    };

    let cosmos_msg = NeutronMsg::submit_tx(
        connection_id,
        interchain_account_id.clone(),
        vec![any_msg],
        "".to_string(),
        timeout.unwrap_or(DEFAULT_TIMEOUT_SECONDS),
        fee,
    );

    // We use a submessage here because we need the process message reply to save
    // the outgoing IBC packet identifier for later.
    let submsg = msg_with_sudo_callback(
        deps.branch(),
        cosmos_msg,
        SudoPayload {
            port_id: get_port_id(env.contract.address.as_str(), &interchain_account_id),
            message: "message".to_string(),
        },
    )?;

    Ok(Response::default().add_submessages(vec![submsg]))
}
 */


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo: received sudo msg: {:?}", msg).as_str());

    match msg {
        // For handling successful (non-error) acknowledgements.
        SudoMsg::Response { request, data } => sudo_response(deps, request, data),

        // For handling error acknowledgements.
        SudoMsg::Error { request, details } => sudo_error(deps, request, details),

        // For handling error timeouts.
        SudoMsg::Timeout { request } => sudo_timeout(deps, env, request),

        // For handling successful registering of ICA
        SudoMsg::OpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        } => sudo_open_ack(
            deps,
            env,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version,
        ),
        _ => Ok(Response::default()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: migrate");
    Ok(Response::default())
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

