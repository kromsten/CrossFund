use cosmwasm_std::{Storage, Order, StdResult, Uint128, Addr, Deps, Env, Binary, to_binary, CustomQuery, StdError};
use neutron_sdk::{bindings::query::{NeutronQuery, QueryInterchainAccountAddressResponse}, NeutronResult, interchain_txs::helpers::get_port_id};

use crate::{storage::{PROPOSALS, PROPOSAL_FUNDING, APPLICATIONS, Proposal, Application, APPLICATION_FUNDING, ProjectFunding, CustodyFunds, CUSTODY_FUNDS, INTERCHAIN_ACCOUNTS, ACKNOWLEDGEMENT_RESULTS, read_errors_from_queue}, msg::{AllProposalResponse, FullProposalInfo}};



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



pub fn query_address_funds(
    store: &dyn Storage,
    address: &Addr,
    skip_locked: bool
) -> StdResult<Vec<(String, CustodyFunds)>> {

    Ok(CUSTODY_FUNDS
        .prefix(address.clone())
        .range(store, None, None, Order::Ascending)
        .map(|f| f.unwrap())
        .filter(|(_, custody_funds)| !(skip_locked && custody_funds.locked))
        .collect::<Vec<_>>()
    )
}


pub fn query_interchain_address(
    deps: Deps<NeutronQuery>,
    env: Env,
    connection_id: String,
    proposal_id: u64,
) -> NeutronResult<Binary> {
    let query = NeutronQuery::InterchainAccountAddress {
        owner_address: env.contract.address.to_string(),
        interchain_account_id: proposal_id.to_string(),
        connection_id,
    };
    let res: QueryInterchainAccountAddressResponse = deps.querier.query(&query.into())?;
    Ok(to_binary(&res)?)
}


pub fn query_interchain_address_contract(
    deps: Deps<NeutronQuery>,
    env: Env,
    proposal_id: u64
) -> NeutronResult<Binary> {
    Ok(to_binary(&get_ica(deps, &env, &proposal_id)?)?)
}


pub fn query_acknowledgement_result(
    deps: Deps<NeutronQuery>,
    env: Env,
    sequence_id: u64,
    proposal_id: u64,
) -> NeutronResult<Binary> {
    let port_id: String = get_port_id(env.contract.address.as_str(), &proposal_id.to_string());
    let res = ACKNOWLEDGEMENT_RESULTS.may_load(deps.storage, (port_id, sequence_id))?;
    Ok(to_binary(&res)?)
}


pub fn query_errors_queue(deps: Deps<NeutronQuery>) -> NeutronResult<Binary> {
    let res = read_errors_from_queue(deps.storage)?;
    Ok(to_binary(&res)?)
}

fn get_ica(
    deps: Deps<impl CustomQuery>,
    env: &Env,
    proposal_id: &u64
) -> Result<(String, String), StdError> {
    let key = get_port_id(
        env.contract.address.as_str(), 
        proposal_id.to_string().as_str()
    );

    INTERCHAIN_ACCOUNTS
        .load(deps.storage, key)?
        .ok_or_else(|| StdError::generic_err("Interchain account is not created yet"))
}