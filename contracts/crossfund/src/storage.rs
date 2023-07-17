use cosmwasm_schema::cw_serde;
use cosmwasm_std::{from_binary, to_vec, Addr, Binary, Order, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};
use cw_utils::Expiration;

#[cw_serde]
pub struct SudoPayload {
    pub message: String,
    pub port_id: String,
}

#[cw_serde]
pub struct GoodFee {
    pub recipient: Addr,
    pub percent_share: u8,
}

#[cw_serde]
pub struct ProjectFunding {
    pub sender: Addr,
    pub amount: Uint128,
    pub auto_agree: bool,
    pub native: bool,
}

impl Default for ProjectFunding {
    fn default() -> Self {
        ProjectFunding {
            sender: Addr::unchecked(""),
            amount: Uint128::zero(),
            auto_agree: false,
            native: true,
        }
    }
}

#[cw_serde]
pub struct CustodyFunds {
    pub amount: Uint128,
    pub proposal_id: u64,
    pub locked: bool,
    pub remote: Option<String>,
}

#[cw_serde]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub funding: Vec<ProjectFunding>,
}

#[cw_serde]
pub struct Application {
    pub applicants: Vec<GoodFee>,
    pub auditors: Vec<GoodFee>,
    pub deliver_by: Expiration,

    pub accepted: bool,
    pub verifications: Vec<Addr>,
}

#[cw_serde]
pub struct Transfer {
    pub recipient: String,
    pub sender: String,
    pub denom: String,
    pub amount: String,
}

pub static PROPOSAL_INDEX: Item<u64> = Item::new("proposal_index");
pub static PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub static PROPOSAL_FUNDING: Map<(u64, &str), ProjectFunding> = Map::new("project_funding");

pub static APPLICATIONS: Map<(u64, Addr), Application> = Map::new("applications");
pub static APPLICATION_FUNDING: Map<(u64, Addr, &str), Uint128> = Map::new("application_funding");

pub static CUSTODY_FUNDS: Map<(&Addr, &str), CustodyFunds> = Map::new("custody_funds");

pub static LAST_CHECKED_BLOCKS: Map<(u64, &str), u64> = Map::new("last_checked_blocks");

pub static INTERCHAIN_ACCOUNTS: Map<String, Option<(String, String)>> =
    Map::new("interchain_accounts");

pub static ADDRESS_TO_PROPOSAL: Map<String, u64> = Map::new("address_to_proposal");

pub static PROCESSED_TXS: Map<u64, bool> = Map::new("processed_tx");

pub const DEFAULT_UPDATE_PERIOD: u64 = 6u64;

pub const SUDO_PAYLOAD_REPLY_ID: u64 = 1;

pub const REPLY_ID_STORAGE: Item<Vec<u8>> = Item::new("reply_queue_id");
pub const SUDO_PAYLOAD: Map<(String, u64), Vec<u8>> = Map::new("sudo_payload");

//pub const proce

// interchain transaction responses - ack/err/timeout state to query later
pub const ACKNOWLEDGEMENT_RESULTS: Map<(String, u64), AcknowledgementResult> =
    Map::new("acknowledgement_results");

pub const ERRORS_QUEUE: Map<u32, String> = Map::new("errors_queue");

#[cw_serde]
pub enum AcknowledgementResult {
    /// Success - Got success acknowledgement in sudo with array of message item types in it
    Success(Vec<String>),
    /// Error - Got error acknowledgement in sudo with payload message in it and error details
    Error((String, String)),
    /// Timeout - Got timeout acknowledgement in sudo with payload message in it
    Timeout(String),
}

pub fn save_reply_payload(store: &mut dyn Storage, payload: SudoPayload) -> StdResult<()> {
    REPLY_ID_STORAGE.save(store, &to_vec(&payload)?)
}

pub fn read_reply_payload(store: &mut dyn Storage) -> StdResult<SudoPayload> {
    let data = REPLY_ID_STORAGE.load(store)?;
    from_binary(&Binary(data))
}

pub fn add_error_to_queue(store: &mut dyn Storage, error_msg: String) -> Option<()> {
    let result = ERRORS_QUEUE
        .keys(store, None, None, Order::Descending)
        .next()
        .and_then(|data| data.ok())
        .map(|c| c + 1)
        .or(Some(0));

    result.and_then(|idx| ERRORS_QUEUE.save(store, idx, &error_msg).ok())
}

pub fn read_errors_from_queue(store: &dyn Storage) -> StdResult<Vec<(Vec<u8>, String)>> {
    ERRORS_QUEUE
        .range_raw(store, None, None, Order::Ascending)
        .collect()
}

pub fn read_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
) -> StdResult<SudoPayload> {
    let data = SUDO_PAYLOAD.load(store, (channel_id, seq_id))?;
    from_binary(&Binary(data))
}

pub fn save_sudo_payload(
    store: &mut dyn Storage,
    channel_id: String,
    seq_id: u64,
    payload: SudoPayload,
) -> StdResult<()> {
    SUDO_PAYLOAD.save(store, (channel_id, seq_id), &to_vec(&payload)?)
}
