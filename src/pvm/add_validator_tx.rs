use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::pvm::output_parser::{secp256k1_output_owner_output_parser, SECP256KTransferOutput};
use crate::pvm::transferable_output_parser::{transferable_output_parser, TransferableOutput};
use crate::pvm::atomic_block_parser::Transaction;
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_i64};
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddValidatorTx {
    pub node_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub weight: i64,
    pub stake: Vec<TransferableOutput>,
    pub reward_owner: SECP256KTransferOutput,
    pub shares: i32,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "add_validator"))]
pub fn add_validator_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let node_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 19)]).to_base58();
    trace!("NodeId : {:?}", node_id);

    *_context.offset += 20;

    let start_time = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Validator Start Time : {:?}", start_time);

    *_context.offset += 8;

    let end_time = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Validator End Time : {:?}", end_time);

    *_context.offset += 8;

    let weight = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Weight : {:?} ", weight);

    *_context.offset += 8;

    let number_of_stacked_output =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of Stacked Output : {:?}", number_of_stacked_output);

    *_context.offset += 4;

    // Outputs
    let mut outputs = Vec::new();
    let mut index = 0;

    while index < number_of_stacked_output {
        trace!("Output number {}", index,);
        outputs.push(transferable_output_parser(&_raw_msg, _context)?);
        index += 1;
    }

    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Type Id : {:?}", type_id);

    *_context.offset += 4;

    let reward_owner = secp256k1_output_owner_output_parser(_raw_msg, _context)?;

    let shares = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Shares : {:?}", shares);

    *_context.offset += 4;

    let add_validator_tx = AddValidatorTx {
        node_id,
        start_time,
        end_time,
        weight,
        stake: outputs,
        reward_owner,
        shares,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: Some(add_validator_tx),
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
