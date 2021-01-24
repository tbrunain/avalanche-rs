use rust_base58::ToBase58;
use tracing::{instrument, trace};

use std::error::Error;

use crate::avm::parser::output_parser::output_parser;
use crate::avm::parser::{Context, TransferableOutput};
use crate::utils::cb58::encode;

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn transferable_output_parser<'a>(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<TransferableOutput, Box<dyn Error>> {
    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "\n {} -- {} \n TransferableOutput -- AssetID : {:?} \n +++++++",
        _context.ipc,
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
