use rust_base58::ToBase58;
use tracing::{error, instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::{Context, Output};
use crate::utils::conversion::{pop_i32, pop_i64, pop_u32};

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn output_parser(_raw_msg: &[u8], _context: &mut Context) -> Result<Output, Box<dyn Error>> {
    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());

    *_context.offset += 4;

    let output;

    match type_id {
        6 => output = secp256k1_mint_output_parser(_raw_msg, _context)?,
        7 => output = secp256k1_transfer_output_parser(_raw_msg, _context)?,
        10 => output = nft_mint_output_parser(_raw_msg, _context)?,
        11 => output = nft_transfer_output_parser(_raw_msg, _context)?,
        _ => {
            error!(
                "{} \n This type id {} for this output is not expected \n Dump of the tx bytes : {:?} \n +++++++",

                _context.tx_id,
                type_id,
                _raw_msg
            );
            panic!("Unsupported")
        }
    }

    Ok(output)
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn secp256k1_mint_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<Output, Box<dyn Error>> {
    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput -- Locktime : {:?}",
        _context.tx_id,
        locktime
    );
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput -- Threshold : {:?}",
        _context.tx_id,
        threshold
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput -- Number of addresses : {:?}",
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
            "{} \n Output -- SECP256K1MintOutput -- Addresses number {} {:?}",
            _context.tx_id,
            index,
            address
        );
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(Output {
        type_id: 6,
        amount: None,
        group_id: None,
        payload: None,
        locktime,
        threshold,
        addresses,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn secp256k1_transfer_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<Output, Box<dyn Error>> {
    // Amount
    let amount = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Output -- SECP256K1TransferOutput -- Parser -- Amount : {:?}",
        _context.tx_id,
        amount
    );
    *_context.offset += 8;

    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Output -- SECP256K1TransferOutput -- Parser -- Locktime : {:?}",
        _context.tx_id,
        locktime
    );
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1TransferOutput -- Parser -- Threshold : {:?}",
        _context.tx_id,
        threshold
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1TransferOutput -- Parser -- Number of addresses : {:?}",
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
            "{} \n Output -- SECP256K1TransferOutput -- Parser Addresses number {} {:?}",
            _context.tx_id,
            index,
            address
        );
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(Output {
        type_id: 7,
        amount: Some(amount),
        group_id: None,
        payload: None,
        locktime,
        threshold,
        addresses,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn nft_mint_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<Output, Box<dyn Error>> {
    // Group Id
    let group_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput -- Group Id : {:?}",
        _context.tx_id,
        group_id
    );
    *_context.offset += 4;

    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput Parser -- Locktime : {:?}",
        _context.tx_id,
        locktime
    );
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput Parser -- Threshold : {:?}",
        _context.tx_id,
        threshold
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- SECP256K1MintOutput Parser -- Number of addresses : {:?}",
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
            "{} \n Output -- SECP256K1MintOutput Parser -- Addresses number {} {:?}",
            _context.tx_id,
            index,
            address
        );
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(Output {
        type_id: 10,
        amount: None,
        group_id: Some(group_id),
        payload: None,
        locktime,
        threshold,
        addresses,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = %_context.tx_id))]
pub fn nft_transfer_output_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<Output, Box<dyn Error>> {
    // Group Id
    let group_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- NftTransferOutput Parser -- Group Id : {:?}",
        _context.tx_id,
        group_id
    );
    *_context.offset += 4;

    // Payload
    let payload_size = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]) as usize;
    trace!(
        "{} \n Output -- NftTransferOutput Parser -- Payload size: {:?}",
        _context.tx_id,
        payload_size
    );
    *_context.offset += 4;

    // Payload
    let mut payload = Vec::new();
    if payload_size == 0 {
        trace!(
            "{} \n Output -- NftTransferOutput Parser -- payload_size is empty ",
            _context.tx_id
        );
    } else {
        trace!(
            "{} \n Output -- NftTransferOutput Parser -- payload content : {:?}",
            _context.tx_id,
            &_raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_base58()
        );
        payload = _raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_vec();
        *_context.offset += payload_size;
    }

    // Locktime
    let locktime = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!(
        "{} \n Output -- NftTransferOutput Parser -- Locktime : {:?}",
        _context.tx_id,
        locktime
    );
    *_context.offset += 8;

    // Threshold
    let threshold = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- NftTransferOutput Parser -- Threshold : {:?}",
        _context.tx_id,
        threshold
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_address = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Output -- NftTransferOutput Parser -- Number of addresses : {:?}",
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
            "{} \n Output -- NftTransferOutput Parser -- Addresses number {} {:?}",
            _context.tx_id,
            index,
            address
        );
        addresses.push(address);
        *_context.offset += 20;
        index += 1;
    }

    Ok(Output {
        type_id: 11,
        amount: None,
        group_id: Some(group_id),
        payload: Some(payload),
        locktime,
        threshold,
        addresses,
    })
}
