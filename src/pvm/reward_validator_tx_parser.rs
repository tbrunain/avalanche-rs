use crate::avm::parser::Context;
use crate::pvm::atomic_block_parser::Transaction;
use crate::pvm::base_tx_parser::BaseTx;
use crate::utils::cb58::encode;
use rust_base58::ToBase58;

use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct RewardValidatorTx {
    pub tx_id: String,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "reward_validator"))]
pub fn reward_validator_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    // For the codec id
    *_context.offset += 2;
    // For the type_id of the tx
    *_context.offset += 4;

    let tx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();

    trace!("Tx Id : {:?}", tx_id);

    *_context.offset += 32;

    let reward_validator = RewardValidatorTx { tx_id };

    Ok(Transaction {
        base_tx: BaseTx {
            type_id: 20,
            network_id: 1,
            blockchain_id: "".to_string(),
            transferable_outputs: vec![],
            transferable_inputs: vec![],
            memo: vec![],
        },
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: Some(reward_validator),
        credentials: vec![],
    })
}
