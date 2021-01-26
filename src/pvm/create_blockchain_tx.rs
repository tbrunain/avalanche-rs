use crate::avm::parser::Context;
use crate::pvm::atomic_block_parser::Transaction;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i32, pop_u16};
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateBlockchainTx {
    pub subnet_id: String,
    pub chain_name: String,
    pub vm_id: String,
    pub fx_ids: Vec<String>,
    pub genesis_data: Vec<u8>,
    pub sig_indices: Vec<i32>,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "create_blockchain"))]
pub fn create_blockchain_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let subnet_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("Subnet Id : {:?}", subnet_id);

    *_context.offset += 32;

    let chain_name_size = pop_u16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());

    trace!("Chain name size : {:?}", chain_name_size);

    *_context.offset += 2;

    // Memo
    let mut chain_name = String::new();
    if chain_name_size == 0 {
        trace!("Chain name is empty ");
    } else {
        trace!("Chain name content : {:?}", chain_name);
        chain_name =
            encode(&_raw_msg[*_context.offset..=(*_context.offset + usize::from(chain_name_size))])
                .to_base58();
        *_context.offset += usize::from(chain_name_size);
    }

    let vm_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("VM Id : {:?}", vm_id);

    *_context.offset += 32;

    let number_of_fx_ids = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("number of Fx ids : {:?}", number_of_fx_ids);

    *_context.offset += 4;

    // Credentials
    let mut index = 0;
    let mut fx_ids = Vec::new();
    while index < number_of_fx_ids {
        trace!("Credential number {}", index);
        let fx_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
        trace!("FX Id : {:?}", vm_id);

        *_context.offset += 32;

        fx_ids.push(fx_id);

        index += 1;
    }

    let genesis_data_size =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow()) as usize;
    trace!("Genesis Data size : {:?}", genesis_data_size);

    // Memo
    let mut genesis = Vec::new();
    if genesis_data_size == 0 {
        trace!("Genesis Data is empty ");
    } else {
        trace!("Genesis Data content : {:?}", genesis);
        genesis = _raw_msg[*_context.offset..=(*_context.offset + genesis_data_size)].to_vec();
        *_context.offset += genesis_data_size;
    }

    *_context.offset += 4;

    let subnet_auth_type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Subnet Auth Type Id : {:?}", subnet_auth_type_id);

    *_context.offset += 4;

    let number_of_sig_indices =
        pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Number of sig indices : {:?}", number_of_sig_indices);

    *_context.offset += 4;

    // Outputs
    let mut sig_indices = Vec::new();
    let mut index = 0;

    while index < number_of_sig_indices {
        trace!("Sig Indice number {}", index);
        sig_indices.push(pop_i32(
            _raw_msg[*_context.offset..=(*_context.offset + 3)].borrow(),
        ));
        *_context.offset += 4;

        index += 1;
    }

    let create_blockchain = CreateBlockchainTx {
        subnet_id,
        chain_name,
        vm_id,
        fx_ids,
        genesis_data: genesis,
        sig_indices,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: Some(create_blockchain),
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
