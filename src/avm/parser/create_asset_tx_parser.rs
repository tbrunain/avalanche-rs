use tracing::{instrument, trace};

use std::borrow::Borrow;
use std::error::Error;

use crate::avm::parser::base_tx_parser::base_tx_parser;
use crate::avm::parser::initial_state_parser::initial_state_parser;
use crate::avm::parser::{Context, CreateAssetTx};
use crate::utils::conversion::{pop_u16, pop_u32, pop_u8};

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn create_asset_tx_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<CreateAssetTx, Box<dyn Error>> {
    let base = base_tx_parser(_raw_msg, _context)?;

    let name_length = pop_u16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());
    *_context.offset += 2;
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- name_length : {:?} \n =======",
        _context.ipc,
        _context.tx_id,
        name_length
    );

    let name = std::str::from_utf8(
        &_raw_msg[*_context.offset..=(*_context.offset + usize::from(name_length))],
    )?;
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- name : {:?} \n =======",
        _context.ipc,
        _context.tx_id,
        name
    );
    *_context.offset += usize::from(name_length);

    let symbol_length = pop_u16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- symbol_length : {:?} \n =======",
        _context.ipc,
        _context.tx_id,
        symbol_length
    );
    *_context.offset += 2;

    let symbol = std::str::from_utf8(
        &_raw_msg[*_context.offset..=(*_context.offset + usize::from(symbol_length))],
    )?;
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- symbol : {:?} \n =======",
        _context.ipc,
        _context.tx_id,
        symbol
    );
    *_context.offset += usize::from(symbol_length);

    let denomination_raw = pop_u8(_raw_msg[*_context.offset..=(*_context.offset)].borrow());
    let denomination = ((0) << 8) | denomination_raw as i16;
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- denomination : {:?} \n =======",
        _context.ipc,
        _context.tx_id,
        denomination
    );
    *_context.offset += 1;

    let initial_states_number =
        pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow()) as usize;
    *_context.offset += 4;
    trace!(
        "Ipc: {} -- TxID: {} \n CreateAssetTx -- Initial State number {} \n =======",
        _context.ipc,
        _context.tx_id,
        initial_states_number
    );

    let mut index = 0;
    let mut initial_states = Vec::new();
    while index < initial_states_number {
        trace!(
            "initial state number {} -- bytes {:?} \n =======",
            index,
            &_raw_msg[*_context.offset..=(*_context.offset + 31)]
        );
        initial_states.push(initial_state_parser(_raw_msg, _context)?);
        index += 1;
    }

    Ok(CreateAssetTx {
        base_tx: base,
        name: name.to_string(),
        symbol: symbol.to_string(),
        denomination,
        initial_states,
    })
}
