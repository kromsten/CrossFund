use cosmwasm_std::{Storage, Order, StdResult, Uint128, Addr};

use crate::{storage::{PROPOSALS, PROPOSAL_FUNDING, APPLICATIONS, Proposal, Application, APPLICATION_FUNDING, ProjectFunding}, msg::{AllProposalResponse, FullProposalInfo}};



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

    let funding = query_proposal_funds(store, id, None)?;
    let applications = query_proposal_applications(store, id)?;

    Ok(FullProposalInfo {
        id,
        title: proposal.title.clone(),
        description: proposal.description.clone(),
        funding,
        applications
    })
}



pub fn query_proposal_funds(
    store: &dyn Storage,
    proposal_id: u64,
    auto_agree_only: Option<bool>
) -> StdResult<Vec<(String, ProjectFunding)>> {
    Ok(PROPOSAL_FUNDING
        .prefix(proposal_id)
        .range(store, None, None, Order::Ascending)
        .filter(|f| {
            let (_, funding) = f.as_ref().unwrap();
            if let Some(auto_agree_only) = auto_agree_only {
                funding.auto_agree == auto_agree_only
            } else {
                true
            }
        })
        .map(|f| f.unwrap())
        .collect::<Vec<_>>()
    )
}

pub fn query_proposal_funds_token(
    store: &dyn Storage,
    proposal_id: u64,
    token: &str
) -> StdResult<Uint128> {
    Ok(PROPOSAL_FUNDING.load(store, (proposal_id, token))?.amount)
}


fn query_proposal_applications(
    store: &dyn Storage,
    proposal_id: u64
) -> StdResult<Vec<(Addr, Application)>> {
    Ok(APPLICATIONS
        .prefix(proposal_id)
        .range(store, None, None, Order::Ascending)
        .map(|f| f.unwrap())
        .collect::<Vec<_>>()
    )
}


pub fn query_application_funds(
    store: &dyn Storage,
    proposal_id: u64,
    application_sender: Addr,
) -> StdResult<Vec<(String, Uint128)>> {

    Ok(APPLICATION_FUNDING
        .prefix((proposal_id, application_sender))
        .range(store, None, None, Order::Ascending)
        .map(|f| f.unwrap())
        .collect::<Vec<_>>())
}


pub fn query_application_funds_token(
    store: &dyn Storage,
    proposal_id: u64,
    application_sender: Addr,
    token: String
) -> StdResult<Uint128> {

    Ok(APPLICATION_FUNDING.load(store, (proposal_id, application_sender, token.as_str()))?)
}

