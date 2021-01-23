use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::borrow::Borrow;
use std::error::Error;

use crate::avm::parser::base_tx_parser::base_tx_parser;
use crate::avm::parser::transferable_output_parser::transferable_output_parser;
use crate::avm::parser::{Context, ExportTx};
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_u32;

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn export_tx_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<ExportTx, Box<dyn Error>> {
    let base = base_tx_parser(_raw_msg, _context)?;

    // Destination chain
    let destination_chain = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "Export Parser - {} -- {} \n Destination chain : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        destination_chain
    );
    *_context.offset += 32;

    // Inputs Array Size
    let number_of_outputs = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "ExportTx Parser - {} -- {} \n Output' array size : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        number_of_outputs
    );
    *_context.offset += 4;

    // Inputs
    let mut outputs = Vec::new();
    let mut index = 0;

    while index < number_of_outputs {
        trace!(
            "ExportTx Parser - {} -- {} \n Output number {} -- bytes {:?} \n +++++++",
            _context.ipc,
            _context.tx_id,
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        outputs.push(transferable_output_parser(&_raw_msg, _context)?);
        index += 1;
    }

    Ok(ExportTx {
        base_tx: base,
        transferable_outputs: outputs,
        destination_chain: destination_chain.to_base58(),
    })
}
