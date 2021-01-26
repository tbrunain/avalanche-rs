use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::Context;
use crate::pvm::parser::output_parser::StakeableLockedInput;
use crate::utils::conversion::{pop_i32, pop_i64};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub type_id: i32,
    pub stakeable_locked_input: Option<StakeableLockedInput>,
    pub secp256k_transfer_input: Option<SECP256KTransferInput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SECP256KTransferInput {
    pub type_id: i32,
    pub amount: i64,
    pub address_indices: Vec<i32>,
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn input_parser<'a>(
    _raw_msg: &'a [u8],
    _context: &mut Context,
) -> Result<Input, Box<dyn Error>> {
    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Type Id -- {}", type_id);
    *_context.offset += 4;

    // It must be 5
    let mut secp256k_transfer_input = None;
    let mut stakeable_locked_input = None;
    match type_id {
        5 => secp256k_transfer_input = Some(secp256k1_transfer_input_parser(_raw_msg, _context)?),
        21 => stakeable_locked_input = Some(stackeable_lockin_parser(_raw_msg, _context)?),
        _ => {
            panic!(
                "This input type is incorrect or not yet supported {}",
                type_id
            )
        }
    }

    Ok(Input {
        type_id,
        stakeable_locked_input,
        secp256k_transfer_input,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn secp256k1_transfer_input_parser<'a>(
    _raw_msg: &'a [u8],
    _context: &mut Context,
) -> Result<SECP256KTransferInput, Box<dyn Error>> {
    // Amount
    let amount = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Amount : {:?}", amount);
    *_context.offset += 8;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of addresses : {:?}", number_of_address);
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut address_indices = Vec::new();

    while index < number_of_address {
        let address_indice = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!("Addresses number {} {:?}", index, address_indice);
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

pub fn stackeable_lockin_parser<'a>(
    _raw_msg: &'a [u8],
    _context: &mut Context,
) -> Result<StakeableLockedInput, Box<dyn Error>> {
    // Amount
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Stack Locktime : {:?}", locktime);
    *_context.offset += 8;

    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Type Id : {:?}", type_id);
    *_context.offset += 4;

    let input;

    match type_id {
        5 => input = secp256k1_transfer_input_parser(_raw_msg, _context)?,
        _ => panic!(
            "This input type is incorrect or not yet supported {}",
            type_id
        ),
    }

    Ok(StakeableLockedInput {
        locktime,
        asset_id: "".to_string(),
        input,
    })
}
