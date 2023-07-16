use cosmos_sdk_proto::cosmos::{tx::v1beta1::{TxRaw, TxBody}, bank::v1beta1::MsgSend};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{DepsMut, Env, StdResult, Response, StdError, Binary, Reply, Storage, Uint128, Addr, Event};
use neutron_sdk::{sudo::msg::RequestPacket, interchain_txs::helpers::{get_proposal_id, get_port_id}, bindings::{msg::MsgSubmitTxResponse, query::{NeutronQuery, QueryRegisteredQueryResponse}, types::Height}, NeutronResult, interchain_queries::{get_registered_query, types::{TransactionFilterItem, TransactionFilterOp, TransactionFilterValue}, v045::{types::{RECIPIENT_FIELD, COSMOS_SDK_TRANSFER_MSG_URL}, new_register_transfers_query_msg}}, NeutronError};
use prost::Message;

const MAX_ALLOWED_MESSAGES: usize = 20;
const MAX_ALLOWED_TRANSFER: u128 = u128::MAX;


use crate::{storage::{
    INTERCHAIN_ACCOUNTS, 
    add_error_to_queue, 
    read_sudo_payload, 
    ACKNOWLEDGEMENT_RESULTS, 
    AcknowledgementResult, 
    save_sudo_payload, 
    read_reply_payload, Transfer, PROCESSED_TXS, PROPOSAL_FUNDING, CUSTODY_FUNDS, CustodyFunds, ADDRESS_TO_PROPOSAL, DEFAULT_UPDATE_PERIOD
}, utils::hash_data, msg::ExecuteResponse};


#[cw_serde]
struct OpenAckVersion {
    version: String,
    controller_connection_id: String,
    host_connection_id: String,
    address: String,
    encoding: String,
    tx_type: String,
}

// handler
pub fn sudo_open_ack(
    deps: DepsMut,
    env: Env,
    port_id: String,
    _channel_id: String,
    _counterparty_channel_id: String,
    counterparty_version: String,
) -> ExecuteResponse {
    // The version variable contains a JSON value with multiple fields,
    // including the generated account address.
    let parsed_version: Result<OpenAckVersion, _> =
        serde_json_wasm::from_str(counterparty_version.as_str());

    // Update the storage record associated with the interchain account.
    if let Ok(parsed_version) = parsed_version {
        INTERCHAIN_ACCOUNTS.save(
            deps.storage,
            port_id.clone(),
            &Some((
                parsed_version.address.clone(),
                parsed_version.controller_connection_id.clone(),
            )),
        )?;
        ADDRESS_TO_PROPOSAL.save(
            deps.storage, 
            parsed_version.address.clone(), 
            &get_proposal_id(port_id.as_str())
        )?;
        
        return Ok(
            Response::default()
            .add_event(Event::new("register_transfers_query").add_attribute("keyy", "vAlYe"))
            .add_message(
                new_register_transfers_query_msg(
                    parsed_version.controller_connection_id, 
                    parsed_version.address, 
                    DEFAULT_UPDATE_PERIOD, 
                    Some(env.block.height)
                )?
            )
        );
    }
    Err( NeutronError::Std(StdError::generic_err("Can't parse counterparty_version")))
}



pub fn sudo_timeout(deps: DepsMut, _env: Env, request: RequestPacket) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: sudo timeout request: {:?}", request).as_str());

    let seq_id = request
        .sequence
        .ok_or_else(|| StdError::generic_err("sequence not found"))?;

    let channel_id = request
        .source_channel
        .ok_or_else(|| StdError::generic_err("channel_id not found"))?;

    // update but also check that we don't update same seq_id twice
    // NOTE: NO ERROR IS RETURNED HERE. THE CHANNEL LIVES ON.
    // In this particular example, this is a matter of developer's choice. Not being able to read
    // the payload here means that there was a problem with the contract while submitting an
    // interchain transaction. You can decide that this is not worth killing the channel,
    // write an error log and / or save the acknowledgement to an errors queue for later manual
    // processing. The decision is based purely on your application logic.
    // Please be careful because it may lead to an unexpected state changes because state might
    // has been changed before this call and will not be reverted because of supressed error.
    let payload = read_sudo_payload(deps.storage, channel_id, seq_id).ok();
    if let Some(payload) = payload {
        // update but also check that we don't update same seq_id twice
        ACKNOWLEDGEMENT_RESULTS.update(
            deps.storage,
            (payload.port_id, seq_id),
            |maybe_ack| -> StdResult<AcknowledgementResult> {
                match maybe_ack {
                    Some(_ack) => Err(StdError::generic_err("trying to update same seq_id")),
                    None => Ok(AcknowledgementResult::Timeout(payload.message)),
                }
            },
        )?;
    } else {
        let error_msg = "WASMDEBUG: Error: Unable to read sudo payload";
        deps.api.debug(error_msg);
        add_error_to_queue(deps.storage, error_msg.to_string());
    }

    Ok(Response::default())
}



pub fn prepare_sudo_payload(mut deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let payload = read_reply_payload(deps.storage)?;
    let resp: MsgSubmitTxResponse = serde_json_wasm::from_slice(
        msg.result
            .into_result()
            .map_err(StdError::generic_err)?
            .data
            .ok_or_else(|| StdError::generic_err("no result"))?
            .as_slice(),
    )
    .map_err(|e| StdError::generic_err(format!("failed to parse response: {:?}", e)))?;
    deps.api
        .debug(format!("WASMDEBUG: reply msg: {:?}", resp).as_str());
    let seq_id = resp.sequence_id;
    let channel_id = resp.channel;
    save_sudo_payload(deps.branch().storage, channel_id, seq_id, payload)?;
    Ok(Response::new())
}




pub fn sudo_tx_query_result(
    deps: DepsMut<NeutronQuery>,
    env: Env,
    query_id: u64,
    _height: Height,
    data: Binary,
) -> NeutronResult<Response> {
    // Decode the transaction data
    let tx: TxRaw = TxRaw::decode(data.as_slice())?;
    let body: TxBody = TxBody::decode(tx.body_bytes.as_slice())?;

    let digest = hash_data(&tx.body_bytes);

    if PROCESSED_TXS.has(deps.storage, digest) {
        return Ok(Response::default());
    }


    let auto_agree = body.memo == "auto_agree";

    // Get the registered query by ID and retrieve the raw query string
    let registered_query: QueryRegisteredQueryResponse =
        get_registered_query(deps.as_ref(), query_id)?;
    let transactions_filter = registered_query.registered_query.transactions_filter;

    // registered_query.registered_query
    // registered_query.registered_query.

    #[allow(clippy::match_single_binding)]
    // Depending of the query type, check the transaction data to see whether is satisfies
    // the original query. If you don't write specific checks for a transaction query type,
    // all submitted results will be treated as valid.
    //
    // TODO: come up with solution to determine transactions filter type
    match registered_query.registered_query.query_type {
        _ => {
            // For transfer queries, query data looks like `[{"field:"transfer.recipient", "op":"eq", "value":"some_address"}]`
            let query_data: Vec<TransactionFilterItem> =
                serde_json_wasm::from_str(transactions_filter.as_str()).unwrap();

            let recipient = query_data
                .iter()
                .find(|x| x.field == RECIPIENT_FIELD && x.op == TransactionFilterOp::Eq)
                .map(|x| match &x.value {
                    TransactionFilterValue::String(v) => v.as_str(),
                    _ => "",
                })
                .unwrap_or("");

            let deposits = recipient_deposits_from_tx_body(body, recipient)?;
            // If we didn't find a Send message with the correct recipient, return an error, and
            // this query result will be rejected by Neutron: no data will be saved to state.
            if deposits.is_empty() {
                return Err(NeutronError::Std(StdError::generic_err(
                    "failed to find a matching transaction message",
                )));
            }

            check_deposits_size(&deposits)?;
    
            for deposit in &deposits {

                let load = ADDRESS_TO_PROPOSAL.may_load(deps.storage, deposit.recipient.clone())?;
                
                if load.is_none() {
                    // TODO: Refund
                    continue;
                }

                fund_proposal_remote(
                    deps.storage,
                    env.clone(),
                    deposit.sender.as_str(),
                    &deposit.denom,
                    Uint128::from(deposit.amount.parse::<u128>().unwrap()),
                    load.unwrap(),
                    auto_agree
                )?;
            }

            Ok(Response::default())
        }

    }
}



fn fund_proposal_remote(
    store: &mut dyn Storage,
    env: Env,
    sender: &str,
    denom: &str,
    amount: Uint128,
    proposal_id: u64,
    auto_agree: bool
) -> StdResult<()> {

    if amount == Uint128::zero() {
        return Err(StdError::generic_err("zero amount"));
    }

    let mut funding = PROPOSAL_FUNDING.load(store, (proposal_id, denom)).unwrap_or_default();
    funding.amount += amount;
    funding.auto_agree = auto_agree;
    funding.sender = Addr::unchecked(sender);

    PROPOSAL_FUNDING.save(store, (proposal_id.clone(), sender.as_ref()), &funding)?;

    let port = get_port_id(
        env.contract.address.as_str(), 
        &proposal_id.clone().to_string()
    );

    CUSTODY_FUNDS.save(store, (&Addr::unchecked(sender), denom), &CustodyFunds {
        amount,
        proposal_id,
        locked: false,
        remote: Some(port)
    })?;

    Ok(())
}


fn recipient_deposits_from_tx_body(
    tx_body: TxBody,
    recipient: &str,
) -> NeutronResult<Vec<Transfer>> {
    let mut deposits: Vec<Transfer> = vec![];
    
    for msg in tx_body.messages.iter().take(MAX_ALLOWED_MESSAGES) {
        // Skip all messages in this transaction that are not Send messages.
        if msg.type_url != *COSMOS_SDK_TRANSFER_MSG_URL.to_string() {
            continue;
        }

        let transfer_msg: MsgSend = MsgSend::decode(msg.value.as_slice())?;
        if transfer_msg.to_address == recipient {
            for coin in transfer_msg.amount {

                deposits.push(Transfer {
                    sender: transfer_msg.from_address.clone(),
                    amount: coin.amount.clone(),
                    denom: coin.denom,
                    recipient: recipient.to_string(),
                });
            }
        }
    }
    Ok(deposits)
}


fn check_deposits_size(deposits: &Vec<Transfer>) -> StdResult<()> {
    for deposit in deposits {
        match deposit.amount.parse::<u128>() {
            Ok(amount) => {
                if amount > MAX_ALLOWED_TRANSFER {
                    return Err(StdError::generic_err(format!(
                        "maximum allowed transfer is {}",
                        MAX_ALLOWED_TRANSFER
                    )));
                };
            }
            Err(error) => {
                return Err(StdError::generic_err(format!(
                    "failed to cast transfer amount to u64: {}",
                    error
                )));
            }
        };
    }
    Ok(())
}