use neutron_sdk::{
    interchain_queries::{
        v045::{
            types::{RECIPIENT_FIELD, HEIGHT_FIELD},
        }, 
        types::{TransactionFilterItem, TransactionFilterOp, TransactionFilterValue, QueryPayload}
    }, 
    bindings::msg::NeutronMsg,
    NeutronResult, 
};



pub fn new_register_transfers_query_msg(
    proposal_id: u64,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResult<NeutronMsg> {

    let mut query_data = vec![TransactionFilterItem {
        field: RECIPIENT_FIELD.to_string(),
        op: TransactionFilterOp::Eq,
        value: TransactionFilterValue::String(recipient),
    }];

    if let Some(min_height) = min_height {
        query_data.push(TransactionFilterItem {
            field: HEIGHT_FIELD.to_string(),
            op: TransactionFilterOp::Gte,
            value: TransactionFilterValue::Int(min_height),
        })
    }

    NeutronMsg::register_interchain_query(
        QueryPayload::TX(query_data),
        proposal_id.to_string(),
        update_period,
    )
}



