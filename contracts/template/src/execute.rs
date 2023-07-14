use cosmwasm_std::{Storage, StdResult, Addr, MessageInfo};

use crate::{storage::{PROPOSALS, PROPOSAL_INDEX, Proposal, CONFIGURATIONS, Configuration, FUNDINGS, Fund}, utils::valid_config};


pub fn submit_proposal(
    store: &mut dyn Storage,
    description: String
) -> StdResult<()> {
    let index = PROPOSAL_INDEX.load(store).unwrap_or(0);
    PROPOSALS.save(store, index.clone(), &Proposal { description, funding: Vec::new() });
    PROPOSAL_INDEX.save(store, &(index+1))?;
    Ok(())
}


pub fn submit_config(
    store: &mut dyn Storage,
    sender: Addr,
    proposal_id: u64,
    configuration: Configuration
) -> StdResult<()> {
    valid_config(&configuration)?;
    CONFIGURATIONS.save(store, (proposal_id, sender), &configuration)
}


pub fn fund(
    store: &mut dyn Storage,
    info: MessageInfo,
    proposal_id: u64,
    auto_agree: bool
) -> StdResult<()> {
    
    for coin in info.funds {

        let funds = FUNDINGS.load(store, (proposal_id, coin.denom.as_str())).unwrap_or(0)

        FUNDINGS.save(store, (proposal_id, info.sender.to_string().as_str()), );
        CONFIGURATIONS.save(store, (proposal_id, sender), &configuration)
    }

    Ok(())
}
