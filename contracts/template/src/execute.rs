use cosmwasm_std::{Storage, Addr, MessageInfo, Uint128, Response, Order};
use neutron_sdk::{NeutronResult, NeutronError};

use crate::{storage::{PROPOSALS, PROPOSAL_INDEX, Application, PROJECT_FUNDING, Proposal, LOCKED_FUNDS, APPLICATIONS, LockedFunds, TOTAL_PROJECT_FUNDING, APPLICATION_FUNDING}, utils::valid_application};


pub fn submit_proposal(
    store: &mut dyn Storage,
    title: String,
    description: String
) -> NeutronResult<Response> {
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
) -> NeutronResult<Response> {
    valid_application(&application)?;
    APPLICATIONS.save(store, (proposal_id, sender), &application)?;
    Ok(Response::default())
}


pub fn fund_proposal_native(
    store: &mut dyn Storage,
    info: MessageInfo,
    proposal_id: u64,
    auto_agree: Option<bool>
) -> NeutronResult<Response> {

    let sender = info.sender;
    let total = TOTAL_PROJECT_FUNDING.load(store, (proposal_id, sender.as_ref())).unwrap_or_default();

    for coin in info.funds {
        if coin.amount == Uint128::zero() {
            continue;
        }
        let mut funding = PROJECT_FUNDING.load(store, (proposal_id, coin.denom.as_str())).unwrap_or_default();
        funding.amount += coin.amount;
        funding.auto_agree = auto_agree.unwrap_or(false);
        

        PROJECT_FUNDING.save(store, (proposal_id, sender.as_ref()), &funding)?;
        TOTAL_PROJECT_FUNDING.save(store, (proposal_id, sender.as_ref()), &(total + funding.amount))?;

        LOCKED_FUNDS.save(store, (sender.clone(), coin.denom.as_str()), &LockedFunds {
            amount: coin.amount,
            proposal_id,
            locked: false
        })?;
    }

    Ok(Response::default())
}



pub fn vote_for_application(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    application_sender: Addr
) -> NeutronResult<Response> {

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


