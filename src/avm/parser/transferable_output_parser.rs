use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::error::Error;

use crate::avm::parser::output_parser::{output_parser, Output};
use crate::avm::parser::Context;
use crate::utils::cb58::encode;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferableOutput {
    pub asset_id: String,
    pub output: Output,
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn transferable_output_parser<'a>(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<TransferableOutput, Box<dyn Error>> {
    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "{} \n TransferableOutput -- AssetID : {:?} \n +++++++",
        _context.tx_id,
        asset_id
    );
    *_context.offset += 32;

    let output = output_parser(_raw_msg, _context);

    Ok(TransferableOutput {
        asset_id: asset_id.to_base58(),
        output: output?,
    })
}
