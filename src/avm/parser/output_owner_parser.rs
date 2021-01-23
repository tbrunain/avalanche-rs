
use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::{Context, OutputOwner};
use crate::utils::conversion::{pop_i64, pop_i32};

#[instrument(skip(_raw_msg), fields(ipc = %_context.ipc, tx_id = %_context.tx_id))]
pub fn output_owner_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<OutputOwner, Box<dyn Error>> {
    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "\n {} -- {} \n Output Owner -- Locktime : {:?}",
        _context.ipc,
        _context.tx_id,
        locktime
    );
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "\n {} -- {} \n Output Owner -- Threshold : {:?}",
        _context.ipc,
        _context.tx_id,
        threshold
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "\n {} -- {} \n Output Owner -- Number of addresses : {:?}",
        _context.ipc,
        _context.tx_id,
        number_of_address
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut addresses = Vec::new();

    while index < number_of_address {
        let address = _raw_msg[*_context.offset..=(*_context.offset + 19)].to_vec();
        trace!(
            "\n {} -- {} \n Output Owner -- Addresses number {} {:?}",
            _context.ipc,
            _context.tx_id,
            index,
            address
        );
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(OutputOwner {
        locktime,
        threshold,
        addresses,
    })
}
