use crate::avm::parser::Context;
use crate::pvm::BlockData;
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i64;
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[instrument(fields(block_id = %_context.tx_id, block_type = "abort_block"))]
pub fn abort_block_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<BlockData, Box<dyn Error>> {
    let parent_block_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("Parent block id : {:?}", parent_block_id);

    *_context.offset += 32;

    let height = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Block Height : {:?}", height);

    *_context.offset += 8;

    Ok(BlockData {
        type_id: 1,
        height,
        parent_block_id,
        transactions: vec![],
        credentials: vec![],
    })
}
