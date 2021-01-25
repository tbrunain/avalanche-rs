use tracing::{error, instrument, trace};

use std::borrow::Borrow;

use crate::avm::parser::Context;
use crate::pvm::input_parser::SECP256KTransferInput;
use crate::utils::conversion::{pop_i32, pop_i64};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub type_id: i32,
    pub stakeable_locked_output: Option<StakeableLockedOutput>,
    pub secp256k1_transfer_output: Option<SECP256KTransferOutput>,
    pub secp256k1_owner_output: Option<SECP256KTransferOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StakeableLockedInput {
    pub locktime: i64,
    pub asset_id: String,
    pub input: SECP256KTransferInput,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StakeableLockedOutput {
    pub locktime: i64,
    pub secp256k_transfer_output: Option<SECP256KTransferOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SECP256KTransferOutput {
    pub type_id: i32,
    pub amount: Option<i64>,
    pub locktime: i64,
    pub threshold: i32,
    pub addresses: Vec<Vec<u8>>,
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn output_parser(_raw_msg: &[u8], _context: &mut Context) -> Result<Output, Box<dyn Error>> {
    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());

    *_context.offset += 4;

    let mut secp256k1_transfer_output = None;
    let mut secp256k1_owner_output = None;
    let mut stakeable_locked_output = None;

    match type_id {
        7 => {
            secp256k1_transfer_output = Some(secp256k1_transfer_output_parser(_raw_msg, _context)?)
        }
        11 => {
            secp256k1_owner_output = Some(secp256k1_output_owner_output_parser(_raw_msg, _context)?)
        }
        22 => stakeable_locked_output = Some(stackeable_lockout_parser(_raw_msg, _context)?),
        _ => {
            error!("{} type_id for output is not valid or not yet supported ! \n offset : {:?}, \n raw message : {:?}", type_id, _context.offset, _raw_msg);
            panic!("NOT SUPPORTED")
        }
    }

    Ok(Output {
        type_id,
        stakeable_locked_output,
        secp256k1_transfer_output,
        secp256k1_owner_output,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn secp256k1_transfer_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<SECP256KTransferOutput, Box<dyn Error>> {
    // Amount
    let amount = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Amount : {:?}", amount);
    *_context.offset += 8;

    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Locktime : {:?}", locktime);
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Threshold : {:?}", threshold);
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of addresses : {:?}", number_of_address);
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut addresses = Vec::new();

    while index < number_of_address {
        let address = _raw_msg[*_context.offset..=(*_context.offset + 19)].to_vec();
        trace!("Addresses number {} {:?}", index, address);
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(SECP256KTransferOutput {
        type_id: 7,
        amount: Some(amount),
        locktime,
        threshold,
        addresses,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn secp256k1_output_owner_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<SECP256KTransferOutput, Box<dyn Error>> {
    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Locktime : {:?}", locktime);
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Threshold : {:?}", threshold);
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of addresses : {:?}", number_of_address);
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut addresses = Vec::new();

    while index < number_of_address {
        let address = _raw_msg[*_context.offset..=(*_context.offset + 19)].to_vec();
        trace!("Addresses number {} {:?}", index, address);
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(SECP256KTransferOutput {
        type_id: 11,
        amount: None,
        locktime,
        threshold,
        addresses,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn stackeable_lockout_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<StakeableLockedOutput, Box<dyn Error>> {
    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Locktime : {:?}", locktime);
    *_context.offset += 8;

    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Type Id : {:?}", type_id);
    *_context.offset += 4;

    let mut output = None;

    match type_id {
        7 => output = Some(secp256k1_transfer_output_parser(_raw_msg, _context)?),
        11 => output = Some(secp256k1_output_owner_output_parser(_raw_msg, _context)?),
        _ => panic!(
            "This output type is incorrect or not yet supported {}",
            type_id
        ),
    }

    Ok(StakeableLockedOutput {
        locktime,
        secp256k_transfer_output: output,
    })
}
