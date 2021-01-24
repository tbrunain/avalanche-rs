use std::borrow::Borrow;
use std::error::Error;

use rust_base58::ToBase58;
use tracing::{instrument, trace};

use crate::avm::parser::output_owner_parser::{output_owner_parser, OutputOwner};
use crate::avm::parser::output_parser::{
    secp256k1_mint_output_parser, secp256k1_transfer_output_parser, Output,
};
use crate::avm::parser::Context;
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_u32};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferableOperation {
    pub asset_id: String,
    pub utxo_ids: Vec<UtxoIds>,
    pub secp256k1_mint_op: Option<SECP256K1MintOp>,
    pub nft_mint_op: Option<NFTMintOp>,
    pub nft_transfer_op: Option<NFTTransferOp>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SECP256K1MintOp {
    pub type_id: i32,
    pub address_indices: Vec<i32>,
    pub secp256k1_mint_output: Output,
    pub secp256k1_transfer_output: Output,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NFTMintOp {
    pub type_id: i32,
    pub address_indices: Vec<u32>,
    pub group_id: i32,
    pub payload: Vec<u8>,
    pub outputs: Vec<OutputOwner>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NFTTransferOp {
    pub type_id: i32,
    pub address_indices: Vec<u32>,
    pub group_id: i32,
    pub payload: Vec<u8>,
    pub output_owner: OutputOwner,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UtxoIds {
    pub tx_id: String,
    pub utxo_index: i32,
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn transfer_op_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<TransferableOperation, Box<dyn Error>> {
    // Asset Id
    let asset_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
    trace!(
        "{} \n TransferOp -- AssetID : {:?} \n +++++++",
        _context.tx_id,
        asset_id
    );
    *_context.offset += 32;

    // Ops Array Size
    let number_of_utxo_ids = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "TransferOp Parser -- {} \n Number of utxo_ids : {:?} \n +++++++",
        _context.tx_id,
        number_of_utxo_ids
    );
    *_context.offset += 4;

    // Outputs
    let mut utxo_ids = Vec::new();
    let mut index = 0;

    while index < number_of_utxo_ids {
        trace!(
            "TransferOp Parser -- {} \n UTXO_ID number {}\n {} \n {}     +++++++",
            _context.tx_id,
            index,
            _context.offset,
            _raw_msg.len()
        );

        let tx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)].to_vec());
        *_context.offset += 32;
        let utxo_index = pop_i32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]);
        *_context.offset += 4;

        utxo_ids.push(UtxoIds {
            tx_id: tx_id.to_base58(),
            utxo_index,
        });

        index += 1;
    }

    // Type Id
    let type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- typeID : {:?} \n +++++++",
        _context.tx_id,
        type_id
    );
    *_context.offset += 4;

    let mut secp256k1_mint_op = None;
    let mut nft_mint_op = None;
    let mut nft_transfer_op = None;

    if type_id == 8 {
        secp256k1_mint_op = Some(secp256k1_mint_operation_parser(_raw_msg, _context)?);
    } else if type_id == 12 {
        nft_mint_op = Some(nft_mint_operation_parser(_raw_msg, _context)?);
    } else if type_id == 13 {
        nft_transfer_op = Some(nft_transfer_operation_parser(_raw_msg, _context)?);
    }

    Ok(TransferableOperation {
        asset_id: asset_id.to_base58(),
        utxo_ids,
        secp256k1_mint_op,
        nft_mint_op,
        nft_transfer_op,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn secp256k1_mint_operation_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<SECP256K1MintOp, Box<dyn Error>> {
    // Address indices number
    let number_of_address_indice =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- SECP256K1MintOp -- Threshold : {:?}",
        _context.tx_id,
        number_of_address_indice
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut address_indices = Vec::new();

    while index < number_of_address_indice {
        let address_indice = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!(
            "{} \n TransferOp -- SECP256K1MintOp Addresses number {} {:?}",
            _context.tx_id,
            index,
            address_indice
        );
        address_indices.push(address_indice);
        *_context.offset += 4;
        index += 1;
    }

    let secp256k1_mint_output = secp256k1_mint_output_parser(_raw_msg, _context)?;
    let secp256k1_transfer_output = secp256k1_transfer_output_parser(_raw_msg, _context)?;

    Ok(SECP256K1MintOp {
        type_id: 8,
        address_indices,
        secp256k1_mint_output,
        secp256k1_transfer_output,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn nft_mint_operation_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<NFTMintOp, Box<dyn Error>> {
    // Address indices number
    let number_of_address_indice =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- NftMintOp -- Numnber of address indices : {:?}",
        _context.tx_id,
        number_of_address_indice
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut address_indices = Vec::new();

    while index < number_of_address_indice {
        let address_indice = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!(
            "{} \n TransferOp -- NftMintOp Addresses indice number {} {:?}",
            _context.tx_id,
            index,
            address_indice
        );
        address_indices.push(address_indice);
        *_context.offset += 4;
        index += 1;
    }

    // Group ID
    let group_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- NftMintOp -- Group id : {:?}",
        _context.tx_id,
        group_id
    );
    *_context.offset += 4;

    // Payload
    let payload_size = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]) as usize;
    trace!(
        "{} \n TransferOp --  NftMintOp Parser -- Payload size: {:?}",
        _context.tx_id,
        payload_size
    );
    *_context.offset += 4;

    // Payload
    let mut payload = Vec::new();
    if payload_size == 0 {
        trace!(
            "{} \n TransferOp -- NftMintOp Parser -- payload_size is empty ",
            _context.tx_id
        );
    } else {
        trace!(
            "{} \n TransferOp -- NftMintOp Parser -- payload content : {:?}",
            _context.tx_id,
            &_raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_base58()
        );
        payload = _raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_vec();
        *_context.offset += payload_size;
    }

    // Output numbers
    let number_of_output_owner =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- NftMintOp -- Number of output owner : {:?}",
        _context.tx_id,
        number_of_output_owner
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut output_owners = Vec::new();

    while index < number_of_output_owner {
        trace!(
            "{} \n TransferOp -- NftMintOp Outputt ownerr number {} ",
            _context.tx_id,
            index
        );
        output_owners.push(output_owner_parser(_raw_msg, _context)?);
        index += 1;
    }

    Ok(NFTMintOp {
        type_id: 12,
        address_indices,
        group_id,
        payload,
        outputs: output_owners,
    })
}

#[instrument(skip(_raw_msg), fields(tx_id = % _context.tx_id))]
pub fn nft_transfer_operation_parser(
    _raw_msg: &[u8],
    _context: &mut Context,
) -> Result<NFTTransferOp, Box<dyn Error>> {
    // Address indices number
    let number_of_address_indice =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- NftTransferOp -- Number of addess indices : {:?}",
        _context.tx_id,
        number_of_address_indice
    );
    *_context.offset += 4;

    // Addresses
    let mut index = 0;
    let mut address_indices = Vec::new();

    while index < number_of_address_indice {
        let address_indice = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
        trace!(
            "{} \n TransferOp -- NftTransferOp Addresses number {} {:?}",
            _context.tx_id,
            index,
            address_indice
        );
        address_indices.push(address_indice);
        *_context.offset += 4;
        index += 1;
    }

    // Group ID
    let group_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "{} \n TransferOp -- NftTransferOp -- Group id : {:?}",
        _context.tx_id,
        group_id
    );
    *_context.offset += 4;

    // Payload
    let payload_size = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]) as usize;
    trace!(
        "{} \n TransferOp -- NftTransferOp Parser -- Payload size: {:?}",
        _context.tx_id,
        payload_size
    );
    *_context.offset += 4;

    // Payload
    let mut payload = Vec::new();
    if payload_size == 0 {
        trace!(
            "{} \n TransferOp -- NftTransferOp Parser -- payload_size is empty ",
            _context.tx_id
        );
    } else {
        trace!(
            "{} \n TransferOp -- NftTransferOp Parser -- payload content : {:?}",
            _context.tx_id,
            &_raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_base58()
        );
        payload = _raw_msg[*_context.offset..=(*_context.offset + payload_size)].to_vec();
        *_context.offset += payload_size;
    }

    let output_owner = output_owner_parser(_raw_msg, _context)?;

    Ok(NFTTransferOp {
        type_id: 13,
        address_indices,
        group_id,
        payload,
        output_owner,
    })
}
