use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::borrow::Borrow;

use crate::avm::parser::Context;
use crate::pvm::parser::input_parser::{input_parser, Input};
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i32;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferableInput {
    pub tx_id: String,
    pub utxo_index: i32,
    pub asset_id: String,
    pub input: Input,
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn transferable_input_parser<'a>(
    _raw_msg: &'a [u8],
    _context: &mut Context,
) -> Result<TransferableInput, Box<dyn Error>> {
    // Tx Id
    let tx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("TxId : {:?}", tx_id);
    *_context.offset += 32;

    // UTXO Index Id
    let utxo_index = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("UTXO Index : {:?}", utxo_index);
    *_context.offset += 4;

    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("AssetID : {:?}", asset_id);
    *_context.offset += 32;

    let input = input_parser(_raw_msg, _context)?;

    Ok(TransferableInput {
        tx_id,
        utxo_index,
        asset_id,
        input,
    })
}
