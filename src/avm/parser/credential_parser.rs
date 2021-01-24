use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::Context;
use crate::utils::conversion::pop_i32;

/// https://docs.avax.network/build/references/avm-transaction-serialization#credentials
#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub type_id: i32,
    pub signatures: Vec<Vec<u8>>,
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn credential_parser<'a>(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<Credential, Box<dyn Error>> {
    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Credential -- typeID : {:?} \n +++++++",
        _context.tx_id,
        type_id
    );
    *_context.offset += 4;

    // Number of addresses
    let number_of_signature = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n Credential parser -- Number of signature : {:?}",
        _context.tx_id,
        number_of_signature
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut signatures = Vec::new();

    while index < number_of_signature {
        let signature = _raw_msg[*_context.offset..=(*_context.offset + 64)].to_vec();
        trace!(
            "{} \n Credential parser -- Signature number {} {:?}",
            _context.tx_id,
            index,
            signature
        );
        signatures.push(signature);
        *_context.offset += 65;
        index += 1;
    }

    Ok(Credential {
        type_id,
        signatures,
    })
}
