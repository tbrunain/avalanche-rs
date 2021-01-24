pub mod abort_block_parser;
pub mod add_delegator_tx;
pub mod add_subnet_validator_tx;
pub mod add_validator_tx;
pub mod advance_time_tx_parser;
pub mod atomic_block_parser;
pub mod base_tx_parser;
pub mod block_parser;
pub mod commit_block_parser;
pub mod create_blockchain_tx;
pub mod create_subnet_tx;
pub mod export_tx_parser;
pub mod import_tx;
pub mod input_parser;
pub mod output_parser;
pub mod proposal_block_parser;
pub mod reward_validator_tx_parser;
pub mod standard_block_parser;
pub mod transferable_input_parser;
pub mod transferable_output_parser;

use std::time::SystemTime;

use crate::avm::parser::credential_parser::Credential;
use crate::pvm::add_delegator_tx::AddDelegatorTx;
use crate::pvm::add_subnet_validator_tx::AddSubnetValidatorTx;
use crate::pvm::add_validator_tx::AddValidatorTx;
use crate::pvm::advance_time_tx_parser::AdvanceTimeTx;
use crate::pvm::base_tx_parser::BaseTx;
use crate::pvm::create_blockchain_tx::CreateBlockchainTx;
use crate::pvm::create_subnet_tx::CreateSubnetTx;
use crate::pvm::export_tx_parser::ExportTx;
use crate::pvm::import_tx::ImportTx;
use crate::pvm::reward_validator_tx_parser::RewardValidatorTx;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub codec_id: i16,
    pub unsigned_tx_offset: usize,
    pub type_id: i32,
    pub block_data: BlockData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockData {
    pub type_id: i32,
    pub height: i64,
    pub parent_block_id: String,
    pub transactions: Vec<Option<Transaction>>,
    pub credentials: Vec<Credential>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub base_tx: BaseTx,
    pub tx_id: String,
    pub add_validator_tx: Option<AddValidatorTx>,
    pub import_tx: Option<ImportTx>,
    pub export_tx: Option<ExportTx>,
    pub add_subnet_validator_tx: Option<AddSubnetValidatorTx>,
    pub add_delegator_tx: Option<AddDelegatorTx>,
    pub create_blockchain_tx: Option<CreateBlockchainTx>,
    pub create_subnet_tx: Option<CreateSubnetTx>,
    pub advance_time_tx: Option<AdvanceTimeTx>,
    pub reward_validator_tx: Option<RewardValidatorTx>,
    pub credentials: Vec<Credential>,
}
