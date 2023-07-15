#![warn(clippy::unwrap_used, clippy::expect_used)]

extern crate core;

pub mod contract;
pub mod msg;
pub mod execute;
pub mod utils;
pub mod query;
pub mod sudo;
pub mod interchain_queries;
pub mod storage;

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod testing;
