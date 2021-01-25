use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::pvm::output_parser::{secp256k1_output_owner_output_parser, SECP256KTransferOutput};
use crate::pvm::Transaction;
use crate::utils::conversion::pop_i32;

use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSubnetTx {
    pub reward_owner: SECP256KTransferOutput,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "create_subnet"))]
pub fn create_subnet_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Type Id : {:?}", type_id);

    *_context.offset += 4;

    let rewards_owner = secp256k1_output_owner_output_parser(_raw_msg, _context)?;

    let create_subnet = CreateSubnetTx {
        reward_owner: rewards_owner,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: Some(create_subnet),
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
