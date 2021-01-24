use std::time::SystemTime;
use uuid::Uuid;

mod base_tx_parser;
mod create_asset_tx_parser;
mod credential_parser;
mod export_tx_parser;
mod import_tx_parser;
mod initial_state_parser;
mod input_parser;
mod operation_tx_parser;
mod output_owner_parser;
mod output_parser;
mod signed_tx_parser;
mod transfer_op_parser;
mod transferable_input_parser;
mod transferable_output_parser;

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
}

/// https://docs.avax.network/build/references/avm-transaction-serialization#credentials
#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub type_id: i32,
    pub signatures: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct SignedTx {
    pub codec_id: i16,
    pub unsigned_tx_offset: usize,
    pub type_id: i32,
    pub tx_id: String,
    pub base_tx: Option<BaseTx>,
    pub create_asset_tx: Option<CreateAssetTx>,
    pub operation_tx: Option<OperationTx>,
    pub import_tx: Option<ImportTx>,
    pub export_tx: Option<ExportTx>,
    pub credentials: Vec<Credential>,
}

/// https://docs.avax.network/build/references/avm-transaction-serialization#what-base-tx-contains
#[derive(Debug)]
pub struct BaseTx {
    pub type_id: i32,
    pub network_id: u32,
    pub blockchain_id: String,
    pub transferable_outputs: Vec<TransferableOutput>,
    pub transferable_inputs: Vec<TransferableInput>,
    pub memo: Vec<u8>,
}

#[derive(Debug)]
pub struct CreateAssetTx {
    pub base_tx: BaseTx,
    pub name: String,
    pub symbol: String,
    pub denomination: i16,
    pub initial_states: Vec<InitialState>,
}

#[derive(Debug)]
pub struct OperationTx {
    pub base_tx: BaseTx,
    pub transferable_ops: Vec<TransferableOperation>,
}

#[derive(Debug)]
pub struct ImportTx {
    pub base_tx: BaseTx,
    pub source_chain: String,
    pub transferable_inputs: Vec<TransferableInput>,
}

#[derive(Debug)]
pub struct ExportTx {
    pub base_tx: BaseTx,
    pub destination_chain: String,
    pub transferable_outputs: Vec<TransferableOutput>,
}

#[derive(Debug)]
pub struct TransferableOutput {
    pub asset_id: String,
    pub output: Output,
}

#[derive(Debug)]
pub struct Output {
    pub type_id: i32,
    pub amount: Option<i64>,
    pub group_id: Option<i32>,
    pub payload: Option<Vec<u8>>,
    pub locktime: i64,
    pub threshold: i32,
    pub addresses: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct TransferableInput {
    pub tx_id: String,
    pub utxo_index: i32,
    pub asset_id: String,
    pub input: SECP256KTransferInput,
}

#[derive(Debug)]
pub struct SECP256KTransferInput {
    pub type_id: i32,
    pub amount: i64,
    pub address_indices: Vec<i32>,
}

#[derive(Debug)]
pub struct TransferableOperation {
    pub asset_id: String,
    pub utxo_ids: Vec<UtxoIds>,
    pub secp256k1_mint_op: Option<SECP256K1MintOp>,
    pub nft_mint_op: Option<NFTMintOp>,
    pub nft_transfer_op: Option<NFTTransferOp>,
}

#[derive(Debug)]
pub struct SECP256K1MintOp {
    pub type_id: i32,
    pub address_indices: Vec<i32>,
    pub secp256k1_mint_output: Output,
    pub secp256k1_transfer_output: Output,
}

#[derive(Debug)]
pub struct NFTMintOp {
    pub type_id: i32,
    pub address_indices: Vec<u32>,
    pub group_id: i32,
    pub payload: Vec<u8>,
    pub outputs: Vec<OutputOwner>,
}

#[derive(Debug)]
pub struct NFTTransferOp {
    pub type_id: i32,
    pub address_indices: Vec<u32>,
    pub group_id: i32,
    pub payload: Vec<u8>,
    pub output_owner: OutputOwner,
}

#[derive(Debug)]
pub struct OutputOwner {
    pub locktime: i64,
    pub threshold: i32,
    pub addresses: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub struct UtxoIds {
    pub tx_id: String,
    pub utxo_index: i32,
}

#[derive(Debug)]
pub struct InitialState {
    pub fx_id: i32,
    pub outputs: Vec<Output>,
}
