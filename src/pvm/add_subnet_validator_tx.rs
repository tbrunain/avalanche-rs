use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::pvm::atomic_block_parser::Transaction;
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_i64};
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddSubnetValidatorTx {
    pub node_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub weight: i64,
    pub subnet_id: String,
    pub sig_indices: Vec<i32>,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "add_subnet_validator"))]
pub fn add_subnet_validator_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let node_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 19)]).to_base58();
    trace!("NodeId : {:?}", node_id);

    *_context.offset += 20;

    let start_time = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Subnet Validator Start Time : {:?}", start_time);

    *_context.offset += 8;

    let end_time = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Subnet Validator End Time : {:?}", end_time);

    *_context.offset += 8;

    let weight = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Weight : {:?}", weight);

    *_context.offset += 8;

    let subnet_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("SubnetId : {:?}", subnet_id);

    *_context.offset += 32;

    let subnet_auth_type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Subnet Auth TypeId : {:?}", subnet_auth_type_id);

    *_context.offset += 4;

    let number_of_sig_indices =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of sig indices : {:?}", number_of_sig_indices);

    *_context.offset += 4;

    // Outputs
    let mut sig_indices = Vec::new();
    let mut index = 0;

    while index < number_of_sig_indices {
        trace!("Sig Indice number {}", index);
        sig_indices.push(pop_i32(
            _raw_msg[*_context.offset..=(*_context.offset + 3)].borrow(),
        ));
        *_context.offset += 4;

        index += 1;
    }

    let add_subnet_validator = AddSubnetValidatorTx {
        node_id,
        start_time,
        end_time,
        weight,
        subnet_id,
        sig_indices,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: Some(add_subnet_validator),
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
