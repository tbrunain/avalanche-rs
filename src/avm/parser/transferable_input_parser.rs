use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::input_parser::{input_parser, SECP256KTransferInput};
use crate::avm::parser::Context;
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i32;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferableInput {
    pub tx_id: String,
    pub utxo_index: i32,
    pub asset_id: String,
    pub input: SECP256KTransferInput,
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn transferable_input_parser<'a>(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<TransferableInput, Box<dyn Error>> {
    // Tx Id
    let tx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "{} \n TransferableInput -- TxId : {:?} \n +++++++",
        _context.tx_id,
        tx_id
    );
    *_context.offset += 32;

    // UTXO Index Id
    let utxo_index = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferableInput -- utxo_index : {:?} \n +++++++",
        _context.tx_id,
        utxo_index
    );
    *_context.offset += 4;

    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "{} \n TransferableInput -- AssetID : {:?} \n +++++++",
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
