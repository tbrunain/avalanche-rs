use tracing::{error, instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::{Context, SECP256KTransferInput};
use crate::utils::conversion::{pop_i32, pop_i64};

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn input_parser<'a>(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<SECP256KTransferInput, Box<dyn Error>> {
    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Input parser -- typeID : {:?} \n +++++++",
        _context.tx_id,
        type_id
    );
    *_context.offset += 4;

    // It must be 5

    let input;

    match type_id {
        5 => input = secp256k1_transfer_input_parser(_raw_msg, _context)?,
        _ => {
            error!(
                "{} \n This type id {} for this input is not expected \n Dump of the tx bytes : {:?} \n +++++++",

                _context.tx_id,
                type_id,
                _raw_msg
            );
            panic!("Unsupported")
        }
    }

    Ok(input)
}

pub fn secp256k1_transfer_input_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<SECP256KTransferInput, Box<dyn Error>> {
    // Amount
    let amount = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Input -- SECP256K1TransferInput-- Amount : {:?}",
        _context.tx_id,
        amount
    );
    *_context.offset += 8;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Input -- SECP256K1TransferInput -- Number of addresses : {:?}",
        _context.tx_id,
        number_of_address
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut address_indices = Vec::new();

    while index < number_of_address {
        let address_indice = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!(
            "{} \n Input -- SECP256K1TransferInput Addresses number {} {:?}",
            _context.tx_id,
            index,
            address_indice
        );
        address_indices.push(address_indice);
        *_context.offset += 4;
        index += 1;
    }

    Ok(SECP256KTransferInput {
        type_id: 5,
        amount,
        address_indices,
    })
}
