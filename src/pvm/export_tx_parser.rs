use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::base_tx_parser;
use crate::pvm::transferable_output_parser::{transferable_output_parser, TransferableOutput};
use crate::pvm::Transaction;
use crate::utils::cb58::encode;
use crate::utils::conversion::pop_i32;
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportTx {
    pub destination_chain: String,
    pub transferable_outputs: Vec<TransferableOutput>,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "export"))]
pub fn export_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let base_tx = base_tx_parser(_raw_msg, _context)?;

    let destination_chain =
        encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("destination_chain : {:?}", destination_chain);

    *_context.offset += 32;

    let transfer_out_number = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("transfer_out_number : {:?}", transfer_out_number);

    *_context.offset += 4;

    let mut outputs = Vec::new();
    let mut index = 0;

    while index < transfer_out_number {
        trace!("Output number {}", index,);
        outputs.push(transferable_output_parser(&_raw_msg, _context)?);
        index += 1;
    }

    let export_tx = ExportTx {
        destination_chain,
        transferable_outputs: outputs,
    };

    Ok(Transaction {
        base_tx,
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: Some(export_tx),
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: None,
        reward_validator_tx: None,
        credentials: vec![],
    })
}
