use tracing::{instrument, trace};

use std::borrow::Borrow;
use std::error::Error;

use crate::avm::parser::base_tx_parser::base_tx_parser;
use crate::avm::parser::create_asset_tx_parser::create_asset_tx_parser;
use crate::avm::parser::credential_parser::credential_parser;
use crate::avm::parser::export_tx_parser::export_tx_parser;
use crate::avm::parser::import_tx_parser::import_tx_parser;
use crate::avm::parser::operation_tx_parser::operation_tx_parser;
use crate::avm::parser::{Context, SignedTx};
use crate::utils::conversion::{pop_i16, pop_i32, pop_u32};
use crate::utils::misc::generate_id;

/// Will parse a Vector of bytes (u8) and return a `SignedTx`
#[instrument(skip(_raw_msg), fields(ipc = % _context.ipc, tx_id = % _context.tx_id))]
pub fn signed_tx_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<SignedTx, Box<dyn Error>> {
    let codec_id = pop_i16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());
    trace!(
        "SignedTx Parser - Ipc: {} -- TxID: {} \n Codec_id : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        codec_id
    );
    *_context.offset += 2;

    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "SignedTx Parser - Ipc: {} -- TxID: {} \n Type_id : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        type_id
    );

    let mut base = None;
    let mut create_asset = None;
    let mut operation = None;
    let mut import = None;
    let mut export = None;

    if type_id == 0 {
        base = Some(base_tx_parser(_raw_msg, _context)?);
    } else if type_id == 1 {
        create_asset = Some(create_asset_tx_parser(_raw_msg, _context)?);
    } else if type_id == 2 {
        operation = Some(operation_tx_parser(_raw_msg, _context)?);
    } else if type_id == 3 {
        import = Some(import_tx_parser(_raw_msg, _context)?);
    } else if type_id == 4 {
        export = Some(export_tx_parser(_raw_msg, _context)?);
    }

    let unsigned_tx_offset = _context.offset.clone();

    // Number of credentials
    let number_of_credentials: u32 =
        pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "SignedTx Parser - {} -- {} \n Credential number : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        number_of_credentials
    );
    *_context.offset += 4;

    // Credentials
    let mut index = 0;
    let mut credentials = Vec::new();
    while index < number_of_credentials {
        trace!(
            "SignedTx Parser - {} -- {} \n Credential number {} \n +++++++",
            _context.ipc,
            _context.tx_id,
            index
        );
        let credential = credential_parser(&_raw_msg, _context)?;
        credentials.push(credential);
        index += 1;
    }

    Ok(SignedTx {
        codec_id,
        unsigned_tx_offset,
        tx_id: generate_id(_raw_msg),
        base_tx: base,
        create_asset_tx: create_asset,
        operation_tx: operation,
        import_tx: import,
        credentials,
        type_id,
        export_tx: export,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use tracing::field::debug;
    use tracing::{debug, instrument};

    #[test]
    fn decode_base_tx_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180,
            93, 98, 174, 151, 209, 176, 80, 198, 75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75, 0,
            0, 0, 2, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0,
            0, 2, 83, 252, 161, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 42, 35, 215,
            240, 118, 8, 138, 33, 197, 9, 97, 96, 186, 60, 10, 37, 192, 219, 164, 53, 33, 230, 115,
            23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116, 185, 214,
            5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0, 0, 48, 228, 249, 180, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 122, 5, 190, 208, 244, 252, 49, 242,
            116, 207, 9, 107, 113, 183, 5, 74, 160, 146, 100, 127, 0, 0, 0, 1, 33, 230, 115, 23,
            203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116, 185, 214, 5,
            223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 3, 90, 33, 230, 115, 23, 203, 196, 190,
            42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116, 185, 214, 5, 223, 37, 145,
            178, 48, 39, 168, 125, 255, 0, 0, 0, 5, 0, 0, 0, 51, 57, 5, 152, 0, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0, 0, 1, 236, 195, 9, 6, 243, 194, 58,
            178, 100, 232, 7, 152, 184, 28, 21, 9, 232, 80, 182, 118, 74, 73, 56, 134, 99, 6, 217,
            234, 236, 78, 85, 177, 70, 3, 28, 54, 223, 91, 120, 234, 15, 63, 152, 106, 10, 134, 52,
            47, 230, 197, 38, 251, 250, 187, 79, 107, 25, 248, 44, 31, 199, 221, 139, 118, 0,
        ]);
        let tx = signed_tx_parser(
            &raw_bytes,
            &mut Context {
                ipc: "ipc-socket",
                tx_id: "a_tx",
                uuid: Default::default(),
                offset: &mut 0,
                parsing_started: SystemTime::now(),
                thread_number: 0,
            },
        )
        .unwrap();
        assert_eq!(
            tx.tx_id, "nVDmTRdjb9T83HPsZxd4SMZ1oEGymJ5sUrWnLB4X5MSFBowkP",
            "Checking if tx_id is correctly set"
        );
        assert_eq!(
            tx.type_id, 0,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.unsigned_tx_offset, 302,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.import_tx.is_some(),
            false,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.export_tx.is_some(),
            false,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.codec_id, 0,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.create_asset_tx.is_some(),
            false,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.base_tx.is_some(),
            true,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.operation_tx.is_some(),
            false,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().type_id,
            9,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                236, 195, 9, 6, 243, 194, 58, 178, 100, 232, 7, 152, 184, 28, 21, 9, 232, 80, 182,
                118, 74, 73, 56, 134, 99, 6, 217, 234, 236, 78, 85, 177, 70, 3, 28, 54, 223, 91,
                120, 234, 15, 63, 152, 106, 10, 134, 52, 47, 230, 197, 38, 251, 250, 187, 79, 107,
                25, 248, 44, 31, 199, 221, 139, 118, 0
            ]),
            "Checking if the type of the tx is correctly set"
        );
    }

    #[test]
    fn decode_create_asset_tx_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180,
            93, 98, 174, 151, 209, 176, 80, 198, 75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75, 0,
            0, 0, 1, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0,
            0, 0, 58, 242, 241, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62,
            230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0,
            0, 1, 39, 218, 48, 1, 162, 88, 227, 247, 53, 155, 110, 210, 22, 67, 196, 63, 129, 38,
            106, 67, 107, 151, 169, 207, 146, 88, 17, 217, 224, 36, 52, 186, 0, 0, 0, 0, 33, 230,
            115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116, 185,
            214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 5, 0, 0, 0, 0, 59, 139, 135,
            192, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 15, 84, 101, 115, 116, 32, 67,
            111, 108, 108, 101, 99, 116, 105, 111, 110, 0, 4, 84, 69, 83, 84, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 0, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
            0, 1, 108, 65, 62, 230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221,
            205, 36, 52, 0, 0, 0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1,
            108, 65, 62, 230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205,
            36, 52, 0, 0, 0, 10, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108,
            65, 62, 230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36,
            52, 0, 0, 0, 10, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65,
            62, 230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52,
            0, 0, 0, 10, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62,
            230, 173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0,
            0, 10, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62, 230,
            173, 204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0, 0,
            10, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62, 230, 173,
            204, 171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0, 0, 10, 0,
            0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62, 230, 173, 204,
            171, 226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0, 0, 10, 0, 0,
            0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62, 230, 173, 204, 171,
            226, 53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0, 0, 10, 0, 0, 0, 9,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 108, 65, 62, 230, 173, 204, 171, 226,
            53, 225, 106, 142, 187, 250, 102, 207, 221, 205, 36, 52, 0, 0, 0, 1, 0, 0, 0, 9, 0, 0,
            0, 1, 186, 174, 141, 174, 79, 193, 55, 231, 65, 189, 14, 118, 165, 15, 219, 111, 177,
            164, 213, 157, 180, 45, 141, 77, 231, 13, 119, 153, 37, 87, 89, 151, 81, 179, 207, 221,
            87, 79, 91, 86, 182, 242, 163, 233, 154, 169, 112, 178, 233, 127, 194, 188, 189, 41,
            99, 125, 12, 60, 135, 61, 19, 97, 184, 105, 0,
        ]);
        let tx = signed_tx_parser(
            &raw_bytes,
            &mut Context {
                ipc: "ipc-socket",
                tx_id: "a_tx",
                uuid: Default::default(),
                offset: &mut 0,
                parsing_started: SystemTime::now(),
                thread_number: 0,
            },
        )
        .unwrap();
        assert_eq!(
            tx.tx_id, "CuD1v2ejEDC5UtLjPtZAL4rR8XGATe2kLTPbmQ4kRL2aiBuDP",
            "Checking if tx_id is correctly set"
        );
        assert_eq!(
            tx.type_id, 1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().type_id,
            9,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                186, 174, 141, 174, 79, 193, 55, 231, 65, 189, 14, 118, 165, 15, 219, 111, 177,
                164, 213, 157, 180, 45, 141, 77, 231, 13, 119, 153, 37, 87, 89, 151, 81, 179, 207,
                221, 87, 79, 91, 86, 182, 242, 163, 233, 154, 169, 112, 178, 233, 127, 194, 188,
                189, 41, 99, 125, 12, 60, 135, 61, 19, 97, 184, 105, 0
            ]),
            "Checking if the type of the tx is correctly set"
        );
    }

    #[test]
    fn decode_export_tx_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 4, 0, 0, 0, 1, 237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180,
            93, 98, 174, 151, 209, 176, 80, 198, 75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75, 0,
            0, 0, 1, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0,
            16, 94, 243, 124, 155, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 216, 221,
            183, 147, 130, 159, 121, 12, 97, 129, 127, 248, 94, 155, 223, 27, 47, 157, 203, 86, 0,
            0, 0, 1, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 16, 98, 33,
            230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116,
            185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 5, 0, 0, 18, 48, 156,
            229, 64, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 33,
            230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116,
            185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0, 1, 209, 169,
            89, 98, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 81, 214, 4, 242, 195, 60,
            44, 34, 99, 124, 173, 30, 200, 192, 83, 183, 123, 18, 193, 193, 0, 0, 0, 1, 0, 0, 0, 9,
            0, 0, 0, 1, 135, 155, 79, 198, 34, 53, 164, 172, 93, 74, 46, 211, 192, 236, 158, 95,
            202, 227, 128, 102, 111, 56, 119, 241, 59, 203, 70, 233, 234, 139, 115, 221, 62, 128,
            97, 55, 64, 138, 197, 45, 23, 223, 70, 23, 169, 254, 20, 192, 182, 138, 84, 3, 187, 58,
            212, 86, 31, 95, 121, 186, 108, 41, 99, 210, 1,
        ]);
        let tx = signed_tx_parser(
            &raw_bytes,
            &mut Context {
                ipc: "ipc-socket",
                tx_id: "a_tx",
                uuid: Default::default(),
                offset: &mut 0,
                parsing_started: SystemTime::now(),
                thread_number: 0,
            },
        )
        .unwrap();
        assert_eq!(
            tx.tx_id, "2vk4HgVnKRuJio5C4xUMmk8D6c74dkM9GtBhLRSp1mXrjnJfg",
            "Checking if tx_id is correctly set"
        );
        assert_eq!(
            tx.type_id, 4,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().type_id,
            9,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                135, 155, 79, 198, 34, 53, 164, 172, 93, 74, 46, 211, 192, 236, 158, 95, 202, 227,
                128, 102, 111, 56, 119, 241, 59, 203, 70, 233, 234, 139, 115, 221, 62, 128, 97, 55,
                64, 138, 197, 45, 23, 223, 70, 23, 169, 254, 20, 192, 182, 138, 84, 3, 187, 58,
                212, 86, 31, 95, 121, 186, 108, 41, 99, 210, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
    }

    #[test]
    fn decode_import_tx_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 3, 0, 0, 0, 1, 237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180,
            93, 98, 174, 151, 209, 176, 80, 198, 75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75, 0,
            0, 0, 1, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0,
            0, 232, 212, 134, 139, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 236, 20,
            128, 16, 162, 96, 31, 134, 195, 71, 138, 15, 99, 116, 81, 16, 240, 55, 90, 151, 0, 0,
            0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 80, 85, 249, 33, 176, 12, 145, 13,
            244, 98, 172, 208, 33, 113, 186, 189, 224, 98, 72, 177, 250, 108, 102, 43, 1, 66, 228,
            231, 33, 29, 47, 219, 0, 0, 0, 0, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103,
            122, 214, 70, 39, 120, 168, 245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168,
            125, 255, 0, 0, 0, 5, 0, 0, 0, 232, 212, 149, 205, 192, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 1, 0, 0, 0, 9, 0, 0, 0, 1, 167, 246, 119, 241, 12, 27, 126, 153, 16, 8, 9, 197, 221,
            110, 128, 39, 237, 190, 239, 51, 158, 129, 145, 76, 236, 123, 213, 210, 12, 73, 168, 1,
            40, 152, 94, 196, 19, 16, 200, 45, 207, 154, 178, 34, 40, 208, 194, 5, 177, 101, 116,
            96, 129, 239, 114, 133, 235, 2, 112, 127, 251, 100, 91, 74, 1,
        ]);
        let tx = signed_tx_parser(
            &raw_bytes,
            &mut Context {
                ipc: "ipc-socket",
                tx_id: "a_tx",
                uuid: Default::default(),
                offset: &mut 0,
                parsing_started: SystemTime::now(),
                thread_number: 0,
            },
        )
        .unwrap();
        assert_eq!(
            tx.tx_id, "2CSTafohdkkiNFD2bNCJGHiBecppZdNNnBf7DCVFuGDxMUwPvX",
            "Checking if tx_id is correctly set"
        );
        assert_eq!(
            tx.type_id, 3,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().type_id,
            9,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                167, 246, 119, 241, 12, 27, 126, 153, 16, 8, 9, 197, 221, 110, 128, 39, 237, 190,
                239, 51, 158, 129, 145, 76, 236, 123, 213, 210, 12, 73, 168, 1, 40, 152, 94, 196,
                19, 16, 200, 45, 207, 154, 178, 34, 40, 208, 194, 5, 177, 101, 116, 96, 129, 239,
                114, 133, 235, 2, 112, 127, 251, 100, 91, 74, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
    }

    #[test]
    fn decode_opreation_tx_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 237, 95, 56, 52, 30, 67, 110, 93, 70, 226, 187, 0, 180,
            93, 98, 174, 151, 209, 176, 80, 198, 75, 198, 52, 174, 16, 98, 103, 57, 227, 92, 75, 0,
            0, 0, 1, 33, 230, 115, 23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168,
            245, 34, 116, 185, 214, 5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 7, 0, 0,
            0, 0, 0, 122, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 134, 217, 188,
            200, 54, 96, 130, 89, 34, 184, 247, 49, 120, 221, 180, 15, 159, 69, 60, 29, 0, 0, 0, 1,
            30, 183, 69, 93, 118, 115, 72, 236, 207, 13, 245, 195, 143, 31, 112, 134, 195, 166, 31,
            29, 172, 29, 154, 216, 108, 89, 10, 128, 112, 149, 75, 144, 0, 0, 0, 0, 33, 230, 115,
            23, 203, 196, 190, 42, 235, 0, 103, 122, 214, 70, 39, 120, 168, 245, 34, 116, 185, 214,
            5, 223, 37, 145, 178, 48, 39, 168, 125, 255, 0, 0, 0, 5, 0, 0, 0, 0, 0, 137, 84, 64, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 27, 5, 75, 180, 25, 112, 130, 102, 53,
            245, 188, 119, 139, 141, 79, 186, 20, 186, 169, 233, 151, 226, 25, 226, 126, 191, 90,
            0, 221, 31, 238, 45, 0, 0, 0, 1, 3, 220, 201, 102, 45, 250, 246, 183, 118, 6, 123, 247,
            100, 112, 101, 41, 102, 157, 246, 134, 172, 111, 237, 68, 229, 5, 95, 225, 90, 217, 43,
            211, 0, 0, 0, 1, 0, 0, 0, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 68, 27, 104,
            116, 116, 112, 115, 58, 47, 47, 99, 114, 121, 115, 116, 97, 108, 45, 99, 100, 110, 49,
            46, 99, 114, 121, 115, 116, 97, 108, 99, 111, 109, 109, 101, 114, 99, 101, 46, 99, 111,
            109, 47, 112, 104, 111, 116, 111, 115, 47, 54, 51, 56, 50, 52, 55, 52, 47, 49, 49, 51,
            52, 53, 51, 54, 46, 106, 112, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 238,
            51, 243, 135, 209, 51, 194, 137, 3, 11, 62, 27, 143, 9, 90, 255, 109, 168, 88, 98, 27,
            5, 75, 180, 25, 112, 130, 102, 53, 245, 188, 119, 139, 141, 79, 186, 20, 186, 169, 233,
            151, 226, 25, 226, 126, 191, 90, 0, 221, 31, 238, 45, 0, 0, 0, 1, 22, 32, 156, 185,
            169, 79, 74, 246, 72, 196, 157, 134, 198, 191, 216, 24, 123, 154, 167, 100, 126, 229,
            61, 106, 4, 217, 89, 127, 193, 227, 93, 241, 0, 0, 0, 1, 0, 0, 0, 13, 0, 0, 0, 1, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 27, 104, 116, 116, 112, 115, 58, 47, 47, 53, 50, 102,
            52, 101, 50, 57, 97, 56, 51, 50, 49, 51, 52, 52, 101, 51, 48, 97, 101, 45, 48, 102, 53,
            53, 99, 57, 49, 50, 57, 57, 55, 50, 97, 99, 56, 53, 100, 54, 98, 49, 102, 52, 101, 55,
            48, 51, 52, 54, 56, 101, 54, 98, 46, 115, 115, 108, 46, 99, 102, 50, 46, 114, 97, 99,
            107, 99, 100, 110, 46, 99, 111, 109, 47, 112, 114, 111, 100, 117, 99, 116, 115, 47,
            112, 105, 99, 116, 117, 114, 101, 115, 47, 49, 54, 49, 50, 50, 50, 57, 46, 106, 112,
            103, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 238, 51, 243, 135, 209, 51, 194,
            137, 3, 11, 62, 27, 143, 9, 90, 255, 109, 168, 88, 98, 27, 5, 75, 180, 25, 112, 130,
            102, 53, 245, 188, 119, 139, 141, 79, 186, 20, 186, 169, 233, 151, 226, 25, 226, 126,
            191, 90, 0, 221, 31, 238, 45, 0, 0, 0, 1, 133, 53, 42, 147, 80, 172, 113, 216, 98, 119,
            35, 134, 148, 147, 130, 111, 196, 113, 174, 142, 86, 107, 238, 131, 102, 22, 238, 81,
            167, 115, 231, 162, 0, 0, 0, 1, 0, 0, 0, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 112, 27, 104, 116, 116, 112, 115, 58, 47, 47, 53, 50, 102, 52, 101, 50, 57, 97, 56,
            51, 50, 49, 51, 52, 52, 101, 51, 48, 97, 101, 45, 48, 102, 53, 53, 99, 57, 49, 50, 57,
            57, 55, 50, 97, 99, 56, 53, 100, 54, 98, 49, 102, 52, 101, 55, 48, 51, 52, 54, 56, 101,
            54, 98, 46, 115, 115, 108, 46, 99, 102, 50, 46, 114, 97, 99, 107, 99, 100, 110, 46, 99,
            111, 109, 47, 112, 114, 111, 100, 117, 99, 116, 115, 47, 112, 105, 99, 116, 117, 114,
            101, 115, 47, 49, 54, 49, 50, 50, 50, 57, 46, 106, 112, 103, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 238, 51, 243, 135, 209, 51, 194, 137, 3, 11, 62, 27, 143, 9, 90,
            255, 109, 168, 88, 98, 0, 0, 0, 4, 0, 0, 0, 9, 0, 0, 0, 1, 184, 110, 61, 200, 129, 77,
            111, 55, 115, 229, 20, 247, 95, 234, 177, 186, 214, 141, 225, 219, 128, 60, 153, 68,
            218, 241, 132, 17, 248, 147, 132, 234, 98, 173, 135, 242, 130, 191, 84, 74, 114, 254,
            119, 13, 7, 157, 216, 238, 18, 52, 246, 37, 220, 233, 59, 57, 216, 242, 134, 71, 143,
            1, 178, 182, 1, 0, 0, 0, 14, 0, 0, 0, 1, 33, 164, 64, 221, 206, 72, 239, 254, 102, 60,
            20, 15, 240, 3, 53, 3, 249, 236, 176, 244, 55, 211, 252, 153, 16, 48, 217, 203, 6, 239,
            249, 165, 4, 243, 233, 114, 216, 247, 84, 164, 7, 156, 226, 20, 2, 109, 34, 150, 247,
            104, 13, 42, 154, 23, 61, 169, 239, 26, 216, 36, 101, 67, 102, 97, 1, 0, 0, 0, 14, 0,
            0, 0, 1, 33, 164, 64, 221, 206, 72, 239, 254, 102, 60, 20, 15, 240, 3, 53, 3, 249, 236,
            176, 244, 55, 211, 252, 153, 16, 48, 217, 203, 6, 239, 249, 165, 4, 243, 233, 114, 216,
            247, 84, 164, 7, 156, 226, 20, 2, 109, 34, 150, 247, 104, 13, 42, 154, 23, 61, 169,
            239, 26, 216, 36, 101, 67, 102, 97, 1, 0, 0, 0, 14, 0, 0, 0, 1, 33, 164, 64, 221, 206,
            72, 239, 254, 102, 60, 20, 15, 240, 3, 53, 3, 249, 236, 176, 244, 55, 211, 252, 153,
            16, 48, 217, 203, 6, 239, 249, 165, 4, 243, 233, 114, 216, 247, 84, 164, 7, 156, 226,
            20, 2, 109, 34, 150, 247, 104, 13, 42, 154, 23, 61, 169, 239, 26, 216, 36, 101, 67,
            102, 97, 1,
        ]);
        let tx = signed_tx_parser(
            &raw_bytes,
            &mut Context {
                ipc: "ipc-socket",
                tx_id: "a_tx",
                uuid: Default::default(),
                offset: &mut 0,
                parsing_started: SystemTime::now(),
                thread_number: 0,
            },
        )
        .unwrap();
        assert_eq!(
            tx.tx_id, "ZrJ5WMH22wGrSZ1H6MFpAb9BfsPzJcACFL4y5y82AsTNTUsEK",
            "Checking if tx_id is correctly set"
        );
        assert_eq!(
            tx.type_id, 2,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.len(),
            4,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().type_id,
            9,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(0).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                184, 110, 61, 200, 129, 77, 111, 55, 115, 229, 20, 247, 95, 234, 177, 186, 214,
                141, 225, 219, 128, 60, 153, 68, 218, 241, 132, 17, 248, 147, 132, 234, 98, 173,
                135, 242, 130, 191, 84, 74, 114, 254, 119, 13, 7, 157, 216, 238, 18, 52, 246, 37,
                220, 233, 59, 57, 216, 242, 134, 71, 143, 1, 178, 182, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(1).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(1).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                33, 164, 64, 221, 206, 72, 239, 254, 102, 60, 20, 15, 240, 3, 53, 3, 249, 236, 176,
                244, 55, 211, 252, 153, 16, 48, 217, 203, 6, 239, 249, 165, 4, 243, 233, 114, 216,
                247, 84, 164, 7, 156, 226, 20, 2, 109, 34, 150, 247, 104, 13, 42, 154, 23, 61, 169,
                239, 26, 216, 36, 101, 67, 102, 97, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(2).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(2).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                33, 164, 64, 221, 206, 72, 239, 254, 102, 60, 20, 15, 240, 3, 53, 3, 249, 236, 176,
                244, 55, 211, 252, 153, 16, 48, 217, 203, 6, 239, 249, 165, 4, 243, 233, 114, 216,
                247, 84, 164, 7, 156, 226, 20, 2, 109, 34, 150, 247, 104, 13, 42, 154, 23, 61, 169,
                239, 26, 216, 36, 101, 67, 102, 97, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(3).unwrap().signatures.len(),
            1,
            "Checking if the type of the tx is correctly set"
        );
        assert_eq!(
            tx.credentials.get(3).unwrap().signatures.get(0).unwrap(),
            &Vec::<u8>::from([
                33, 164, 64, 221, 206, 72, 239, 254, 102, 60, 20, 15, 240, 3, 53, 3, 249, 236, 176,
                244, 55, 211, 252, 153, 16, 48, 217, 203, 6, 239, 249, 165, 4, 243, 233, 114, 216,
                247, 84, 164, 7, 156, 226, 20, 2, 109, 34, 150, 247, 104, 13, 42, 154, 23, 61, 169,
                239, 26, 216, 36, 101, 67, 102, 97, 1
            ]),
            "Checking if the type of the tx is correctly set"
        );
    }
}
