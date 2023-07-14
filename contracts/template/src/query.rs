use cosmwasm_std::{Storage, Order, StdResult};

use crate::{storage::{PROPOSALS, PROPOSAL_FUNDING, APPLICATIONS, Proposal}, msg::{AllProposalResponse, FullProposalInfo}};



pub fn query_all_proposals(
    store: &dyn Storage,
) -> StdResult<AllProposalResponse> {

    let full_proposal_infos : Vec<FullProposalInfo> = PROPOSALS
            .range(store, None, None, Order::Ascending)
            .map(|f| {
                let (id, proposal) = f.unwrap();
                get_proposal_full_info(store, id, &proposal).unwrap()
            })
            .collect();

    Ok(AllProposalResponse { proposals: full_proposal_infos })
}


pub fn query_proposal(
    store: &dyn Storage,
    proposal_id: u64
) -> StdResult<FullProposalInfo> {
    Ok(get_proposal_full_info(store, proposal_id, &PROPOSALS.load(store, proposal_id)?)?)
}





fn get_proposal_full_info(
    store: &dyn Storage,
    id: u64,
    proposal: &Proposal
) -> StdResult<FullProposalInfo> {

    let funding = PROPOSAL_FUNDING
        .prefix(id)
        .range(store, None, None, Order::Ascending)
        .map(|f| {
            let (key, value) = f.unwrap();
            (key, value.amount)
        })
        .collect::<Vec<_>>();

    let applications = APPLICATIONS
        .prefix(id)
        .range(store, None, None, Order::Ascending)
        .map(|f| f.unwrap())
        .collect::<Vec<_>>();

    Ok(FullProposalInfo {
        id,
        title: proposal.title.clone(),
        description: proposal.description.clone(),
        funding,
        applications
    })
}

