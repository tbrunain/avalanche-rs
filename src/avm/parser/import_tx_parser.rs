use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::borrow::Borrow;
use std::error::Error;

use crate::avm::parser::base_tx_parser::base_tx_parser;
use crate::avm::parser::transferable_input_parser::transferable_input_parser;
use crate::avm::parser::{Context, ImportTx};
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_u32;

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn import_tx_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<ImportTx, Box<dyn Error>> {
    let base = base_tx_parser(_raw_msg, _context)?;

    // Source chain
    let source_chain = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "ImportTx Parser - {} -- {} \n Sourrce chain : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        source_chain
    );
    *_context.offset += 32;

    // Inputs Array Size
    let number_of_inputs = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "ImportTx Parser - {} -- {} \n Inputs' array size : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        number_of_inputs
    );
    *_context.offset += 4;

    // Inputs
    let mut inputs = Vec::new();
    let mut index = 0;

    while index < number_of_inputs {
        trace!(
            "ImportTx Parser - {} -- {} \n Input number {} -- bytes {:?} \n +++++++",
            _context.ipc,
            _context.tx_id,
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        inputs.push(transferable_input_parser(&_raw_msg, _context)?);
        index += 1;
    }

    Ok(ImportTx {
        base_tx: base,
        source_chain: source_chain.to_base58(),
        transferable_inputs: inputs,
    })
}
