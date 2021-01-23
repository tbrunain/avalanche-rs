use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::input_parser::input_parser;
use crate::avm::parser::{Context, TransferableInput};
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i32;

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn transferable_input_parser<'a>(
    _raw_msg: &'a Vec<u8>,
    _context: &mut Context,
) -> Result<TransferableInput, Box<dyn Error>> {
    // Tx Id
    let tx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "\n {} -- {} \n TransferableInput -- TxId : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        tx_id
    );
    *_context.offset += 32;

    // UTXO Index Id
    let utxo_index = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "\n {} -- {} \n TransferableInput -- utxo_index : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        utxo_index
    );
    *_context.offset += 4;

    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "\n {} -- {} \n TransferableInput -- AssetID : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        asset_id
    );
    *_context.offset += 32;

    let input = input_parser(_raw_msg, _context)?;

    Ok(TransferableInput {
        tx_id: tx_id.to_base58(),
        utxo_index,
        asset_id: asset_id.to_base58(),
        input,
    })
}
