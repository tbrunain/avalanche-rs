use std::borrow::Borrow;
use std::error::Error;

use rust_base58::ToBase58;
use tracing::{instrument, trace};

use crate::avm::parser::transferable_input_parser::transferable_input_parser;
use crate::avm::parser::transferable_output_parser::transferable_output_parser;
use crate::avm::parser::{BaseTx, Context};
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_u32};

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn base_tx_parser(_raw_msg: &[u8], _context: &mut Context) -> Result<BaseTx, Box<dyn Error>> {
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "BaseTx Parser-- TxID: {} \n Type_id : {:?} \n +++++++",
        _context.tx_id,
        type_id
    );
    *_context.offset += 4;

    // Network Id
    let network_id = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "BaseTx Parser-- TxID: {} \n BaseTx -- network_id : {:?} \n +++++++",
        _context.tx_id,
        network_id
    );
    *_context.offset += 4;

    // Blockchain Id
    let blockchain_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "BaseTx Parser -- {} \n Blockchain_id : {:?} \n +++++++",
        _context.tx_id,
        blockchain_id
    );
    *_context.offset += 32;

    // Outputs Array Size
    let number_of_outputs = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "BaseTx Parser -- {} \n Number of outputs : {:?} \n +++++++",
        _context.tx_id,
        number_of_outputs
    );
    *_context.offset += 4;

    // Outputs
    let mut outputs = Vec::new();
    let mut index = 0;

    while index < number_of_outputs {
        trace!(
            "BaseTx Parser -- {} \n Output number {} -- bytes {:?} \n +++++++",
            _context.tx_id,
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        outputs.push(transferable_output_parser(&Vec::from(_raw_msg), _context)?);
        index += 1;
    }

    // Inputs Array Size
    let number_of_inputs = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "BaseTx Parser -- {} \n Inputs' array size : {:?} \n +++++++",
        _context.tx_id,
        number_of_inputs
    );
    *_context.offset += 4;

    // Inputs
    let mut inputs = Vec::new();
    let mut index = 0;

    while index < number_of_inputs {
        trace!(
            "BaseTx Parser -- {} \n Input number {} -- bytes {:?} \n +++++++",
            _context.tx_id,
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        inputs.push(transferable_input_parser(&Vec::from(_raw_msg), _context)?);
        index += 1;
    }

    // Memo size
    let memo_size = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]) as usize;
    trace!(
        "BaseTx Parser -- {} \n Memo size : {:?} \n +++++++",
        _context.tx_id,
        memo_size
    );
    *_context.offset += 4;

    // Memo
    let mut memo = Vec::new();
    if memo_size == 0 {
        trace!(
            "BaseTx Parser -- {} \n Memo is empty  \n +++++++",
            _context.tx_id
        );
    } else {
        trace!(
            "BaseTx Parser -- {} \n Memo content : {:?} \n +++++++",
            _context.tx_id,
            &_raw_msg[*_context.offset..=(*_context.offset + memo_size)]
        );
        memo = _raw_msg[*_context.offset..=(*_context.offset + memo_size)].to_vec();
        *_context.offset += memo_size;
    }

    Ok(BaseTx {
        type_id,
        network_id,
        blockchain_id: blockchain_id.to_base58(),
        transferable_outputs: outputs,
        transferable_inputs: inputs,
        memo,
    })
}
