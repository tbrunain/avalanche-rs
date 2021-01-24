use crate::avm::parser::credential_parser::credential_parser;
use crate::avm::parser::Context;
use crate::pvm::add_delegator_tx::add_delegator_tx_parser;
use crate::pvm::add_subnet_validator_tx::add_subnet_validator_tx_parser;
use crate::pvm::add_validator_tx::add_validator_tx_parser;
use crate::pvm::advance_time_tx_parser::advance_time_tx_parser;
use crate::pvm::create_blockchain_tx::create_blockchain_tx_parser;
use crate::pvm::create_subnet_tx::create_subnet_tx_parser;
use crate::pvm::export_tx_parser::export_tx_parser;
use crate::pvm::import_tx::import_tx_parser;
use crate::pvm::reward_validator_tx_parser::reward_validator_parser;
use crate::pvm::BlockData;
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_i64, pop_u32};
use crate::utils::misc::generate_id;
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[instrument(fields(block_id = % _context.tx_id, tx_type = "standard"))]
pub fn standard_block_parser(
    _raw_msg: &mut Vec<u8>,
    _context: &mut Context,
) -> Result<BlockData, Box<dyn Error>> {
    let parent_block_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("Parent block id : {:?}", parent_block_id);

    *_context.offset += 32;

    let height = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Height : {:?}", height);

    *_context.offset += 8;

    let number_of_tx = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of Tx : {:?}", number_of_tx);

    *_context.offset += 4;

    let mut index = 0;

    let mut transactions = Vec::new();

    while index < number_of_tx {
        trace!("Output number {}", index,);

        let tx_type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!("tx_type_id : {:?}", tx_type_id);

        // *_context.offset += 4;

        let mut transaction = None;

        // Here to get the correct Tx ID we need to perform a small manipulation onto the _raw_msg .
        // In short in the bytes we get out of the socket , we have :
        // - 2B CODEC
        // - 4B TYPE OF BLOCK
        // - 32B PARENT BLOCK ID
        // - 8B HEIGHT of block
        // - 4B TYPE OF TX
        // - XB THE TX DATA
        // BUT we need to have the codec version added in between HEIGHT anf TYPE OF TX .
        _raw_msg.insert(*_context.offset, 0);
        _raw_msg.insert(*_context.offset, 0);
        let tx_id = generate_id(&_raw_msg[*_context.offset..=(_raw_msg.len() - 1)]);

        trace!("tx_id : {:?}", tx_id);

        match tx_type_id {
            12 => transaction = Some(add_validator_tx_parser(_raw_msg, tx_id, _context)?),
            13 => transaction = Some(add_subnet_validator_tx_parser(_raw_msg, tx_id, _context)?),
            14 => transaction = Some(add_delegator_tx_parser(_raw_msg, tx_id, _context)?),
            15 => transaction = Some(create_blockchain_tx_parser(_raw_msg, tx_id, _context)?),
            16 => transaction = Some(create_subnet_tx_parser(_raw_msg, tx_id, _context)?),
            17 => transaction = Some(import_tx_parser(_raw_msg, tx_id, _context)?),
            18 => transaction = Some(export_tx_parser(_raw_msg, tx_id, _context)?),
            19 => transaction = Some(advance_time_tx_parser(_raw_msg, tx_id, _context)?),
            20 => transaction = Some(reward_validator_parser(_raw_msg, tx_id, _context)?),
            _ => panic!(
                "This tx type is incorrect or not yet supported {}",
                tx_type_id
            ),
        }

        transactions.push(transaction);

        index += 1;
    }

    // Number of credentials
    let number_of_credentials: u32 =
        pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Credential number : {:?}", number_of_credentials);
    *_context.offset += 4;

    // Credentials
    let mut index = 0;
    let mut credentials = Vec::new();
    while index < number_of_credentials {
        trace!("Credential number {}", index);
        let credential = credential_parser(&_raw_msg, _context)?;
        credentials.push(credential);
        index += 1;
    }

    Ok(BlockData {
        type_id: 3,
        height,
        parent_block_id,
        transactions,
        credentials,
    })
}
