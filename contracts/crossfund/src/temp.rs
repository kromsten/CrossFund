use cosmwasm_std::Response;
use neutron_sdk::interchain_queries::v045::new_register_transfers_query_msg;

use crate::msg::NeutronResponse;

pub fn register_transfers_query(
    connection_id: String,
    recipient: String,
    update_period: u64,
    min_height: Option<u64>,
) -> NeutronResponse {
    let msg =
        new_register_transfers_query_msg(connection_id, recipient, update_period, min_height)?;

    Ok(Response::new().add_message(msg))
}
