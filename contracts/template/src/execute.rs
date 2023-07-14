use cosmwasm_std::{Storage, Addr, MessageInfo, Uint128, Response, Order, Env, StdResult};
use neutron_sdk::{NeutronError, bindings::msg::NeutronMsg, interchain_txs::helpers::get_port_id};

use crate::{storage::{PROPOSALS, PROPOSAL_INDEX, Application, PROPOSAL_FUNDING, Proposal, LOCKED_FUNDS, APPLICATIONS, LockedFunds, TOTAL_PROPOSAL_FUNDING, APPLICATION_FUNDING, INTERCHAIN_ACCOUNTS}, utils::valid_application, msg::ExecuteResponse};


pub fn submit_proposal(
    store: &mut dyn Storage,
    title: String,
    description: String
) -> ExecuteResponse {
    let index = PROPOSAL_INDEX.load(store).unwrap_or(0);
    PROPOSALS.save(store, index.clone(), &Proposal { title, description, funding: Vec::new() })?;
    PROPOSAL_INDEX.save(store, &(index+1))?;
    Ok(Response::default())
}


pub fn submit_application(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application: Application
) -> ExecuteResponse {
    valid_application(&application)?;
    APPLICATIONS.save(store, (proposal_id, sender), &application)?;
    Ok(Response::default())
}


pub fn fund_proposal_native(
    store: &mut dyn Storage,
    info: MessageInfo,
    proposal_id: u64,
    auto_agree: Option<bool>
) -> ExecuteResponse {

    let sender = info.sender;
    let total = TOTAL_PROPOSAL_FUNDING.load(store, (proposal_id, sender.as_ref())).unwrap_or_default();

    for coin in info.funds {
        if coin.amount == Uint128::zero() {
            continue;
        }
        let mut funding = PROPOSAL_FUNDING.load(store, (proposal_id, coin.denom.as_str())).unwrap_or_default();
        funding.amount += coin.amount;
        funding.auto_agree = auto_agree.unwrap_or(false);
        

        PROPOSAL_FUNDING.save(store, (proposal_id, sender.as_ref()), &funding)?;
        TOTAL_PROPOSAL_FUNDING.save(store, (proposal_id, sender.as_ref()), &(total + funding.amount))?;

        LOCKED_FUNDS.save(store, (sender.clone(), coin.denom.as_str()), &LockedFunds {
            amount: coin.amount,
            proposal_id,
            locked: false
        })?;
    }

    Ok(Response::default())
}



pub fn approve_application(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application_sender: Addr
) -> ExecuteResponse {

    let user_funds = LOCKED_FUNDS
        .prefix(sender.clone())
        .range(store, None, None, Order::Ascending)
        .map(|f| f.unwrap())
        .filter(|f| f.1.proposal_id == proposal_id && !f.1.locked)
        .collect::<Vec<_>>()
    ;

    if user_funds.is_empty() {
        return Err(NeutronError::CantVote{});
    }

    // let application = APPLICATIONS.load(store, (proposal_id, application_sender.clone()))?;

    for (key, value) in user_funds {
        let existing =  APPLICATION_FUNDING.load(store, 
            (proposal_id, application_sender.clone(), key.as_str()))
            .unwrap_or_default();

        APPLICATION_FUNDING.save(store, (proposal_id, application_sender.clone(), key.as_str()), &(existing + value.amount))?;
        LOCKED_FUNDS.save(store, (sender.clone(), key.as_str()), &LockedFunds {
            amount: value.amount,
            proposal_id,
            locked: true
        })?;
    }

    Ok(Response::default())
}



pub fn register_ica(
    store: &mut dyn Storage,
    env: Env,
    connection_id: String,
    proposal_id: u64,
) -> ExecuteResponse {

    let register =
        NeutronMsg::register_interchain_account(connection_id, proposal_id.clone().to_string());
    
    let key = get_port_id(env.contract.address.as_str(), &proposal_id.to_string());
    // we are saving empty data here because we handle response of registering ICA in sudo_open_ack method
    
    INTERCHAIN_ACCOUNTS.save(store, key, &None)?;
    
    Ok(Response::new()
        .add_message(register)
    )
}



pub fn accept(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application_sender: Addr,
) -> ExecuteResponse {

    let mut application = APPLICATIONS.load(store, (proposal_id, application_sender.clone()))?;

    if sender != application_sender &&  application.applicants.iter().all(|a| a.recipient != &sender) {
        return Err(NeutronError::NonAuthorized{});
    }

    application.accepted = true;


    APPLICATIONS.save(store, (proposal_id, application_sender.clone()), &application)?;


    Ok(Response::default())
}



pub fn verify_application(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application_sender: Addr,
    stop_at: Option<u64>
) -> ExecuteResponse {

    let mut application = APPLICATIONS.load(store, (proposal_id, application_sender.clone()))?;

    if application.auditors.iter().all(|a| a.recipient != &sender) {
        return Err(NeutronError::NonAuthorized{});
    }

    if application.verifications.iter().any(|v| v == &sender) {
        return Err(NeutronError::AlreadyVerified{});
    }

    application.verifications.push(sender.clone());
    
    APPLICATIONS.save(store, (proposal_id, application_sender.clone()), &application)?;

    if (application.auditors.len() == application.verifications.len()) {
        // TODO: release funds
    }

    Ok(Response::default())
}


fn reward_applicants(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application_sender: Addr,
    stop_at: Option<u64>
) -> StdResult<()> {
    // TODO
    Ok(())
}





