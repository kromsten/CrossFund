use neutron_sdk::bindings::msg::IbcFee;
use crate::storage::{Application, GoodFee};

const FEE_DENOM: &str = "untrn";

pub fn valid_application(
    application: &Application
) -> bool {
    // TODO: implement
    let sh =  shareholders(application);
    sh.len() < 100 && sh.iter().map(|f| f.percent_share).sum::<u8>() == 1
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
