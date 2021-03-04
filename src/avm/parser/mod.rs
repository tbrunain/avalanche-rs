use std::time::SystemTime;

use uuid::Uuid;

pub mod base_tx_parser;
pub mod create_asset_tx_parser;
pub mod credential_parser;
pub mod export_tx_parser;
pub mod import_tx_parser;
pub mod initial_state_parser;
pub mod input_parser;
pub mod operation_tx_parser;
pub mod output_owner_parser;
pub mod output_parser;
pub mod signed_tx_parser;
pub mod transfer_op_parser;
pub mod transferable_input_parser;
pub mod transferable_output_parser;

/// Represent a Context object used to keep track of the ... context of a transaction being parsed
#[derive(Debug)]
pub struct Context<'a> {
    /// Transaction ID of this tx
    pub tx_id: &'a str,
    /// Unique ID we generate at the beginning of the parsing for debug purposes
    pub uuid: Uuid,
    /// Current byte number we are parsing
    pub offset: &'a mut usize,
    pub parsing_started: SystemTime,
    pub network_name: String,
}
