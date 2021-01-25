use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::pvm::transferable_input_parser::{transferable_input_parser, TransferableInput};
use crate::pvm::Transaction;
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i32;
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct ImportTx {
    pub source_chain_id: String,
    pub transferable_inputs: Vec<TransferableInput>,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "import"))]
pub fn import_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let source_chain_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("Source Chain Id : {:?}", source_chain_id);
    *_context.offset += 32;

    let transfer_ins_number = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Transfer Ins Number : {:?}", transfer_ins_number);

    *_context.offset += 4;

    let mut inputs = Vec::new();
    let mut index = 0;

    while index < transfer_ins_number {
        trace!("Output number {}", index,);
        inputs.push(transferable_input_parser(&_raw_msg, _context)?);
        index += 1;
    }

    let import_tx = ImportTx {
        source_chain_id,
        transferable_inputs: inputs,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: Some(import_tx),
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
