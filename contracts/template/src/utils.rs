use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};
use cosmwasm_std::{BlockInfo};
use cw_utils::Expiration;
use neutron_sdk::bindings::msg::IbcFee;
use crate::{storage::{Application, GoodFee}, msg::ApplicationSubmission};

const FEE_DENOM: &str = "untrn";

pub fn valid_application(
    application: &ApplicationSubmission,
    block_info: &BlockInfo,
) -> bool {
    let sh =  shareholders_sub(application);
    sh.len() < 100 && 
    application.deliver_by != Expiration::Never {} &&
    application.deliver_by.is_expired(&block_info) &&
    application.applicants.len() > 0 &&
    application.auditors.len() > 0 &&
    sh.iter().map(|f| f.percent_share).sum::<u8>() == 1
}



pub fn shareholders_sub(
    application: &ApplicationSubmission
) -> Vec<GoodFee> {
    [&application.applicants[..], &application.auditors[..]].concat()
}

pub fn shareholders(
    application: &Application
) -> Vec<GoodFee> {
    [&application.applicants[..], &application.auditors[..]].concat()
}


pub fn min_ntrn_ibc_fee(fee: IbcFee) -> IbcFee {
    IbcFee {
        recv_fee: fee.recv_fee,
        ack_fee: fee
            .ack_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
        timeout_fee: fee
            .timeout_fee
            .into_iter()
            .filter(|a| a.denom == FEE_DENOM)
            .collect(),
    }
}


pub fn hash_data<T>(data: &T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    hasher.finish()
}