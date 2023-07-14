use cosmwasm_std::{Storage, Addr, MessageInfo, Uint128, Response};
use neutron_sdk::{NeutronResult};

use crate::{storage::{PROPOSALS, PROPOSAL_INDEX, Configuration, PROJECT_FUNDING, Proposal, LOCKED_FUNDS, CONFIGURATIONS}, utils::valid_config};


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


pub fn submit_config(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    configuration: Configuration
) -> NeutronResult<Response> {
    valid_config(&configuration)?;
    CONFIGURATIONS.save(store, (proposal_id, sender), &configuration)?;
    Ok(Response::default())
}


pub fn fund_proposal_native(
    store: &mut dyn Storage,
    info: MessageInfo,
    proposal_id: u64,
    auto_agree: Option<bool>
) -> NeutronResult<Response> {

    let sender = info.sender;
    
    for coin in info.funds {
        if coin.amount == Uint128::zero() {
            continue;
        }
        let mut funding = PROJECT_FUNDING.load(store, (proposal_id, coin.denom.as_str())).unwrap_or_default();
        funding.amount += coin.amount;
        funding.auto_agree = auto_agree.unwrap_or(false);
        

        PROJECT_FUNDING.save(store, (proposal_id, sender.as_ref()), &funding)?;
        LOCKED_FUNDS.save(store, (sender.clone(), coin.denom.as_str()), &coin.amount)?;
    }

    Ok(Response::default())
}

