use cosmwasm_std::{Storage, Order, StdResult};

use crate::{storage::{PROPOSALS, Proposal}, msg::AllProposalResponse};



fn query_proposoal(
    store: &dyn Storage,
) -> StdResult<AllProposalResponse> {

    let proposals: Vec<(u64, Proposal)> = PROPOSALS
        .range(store, None, None, Order::Ascending)
        .map(|item| item.unwrap())
        .collect();

    Ok(AllProposalResponse { proposals })
}
