use crate::avm::parser::Context;
use crate::pvm::base_tx_parser::BaseTx;
use crate::pvm::Transaction;
use crate::utils::conversion::{pop_i16, pop_i64};
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdvanceTimeTx {
    pub time_proposal: i64,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "advance_time"))]
pub fn advance_time_tx_parser(
    _raw_msg: &[u8],
    _tx_id: String,
    _context: &mut Context,
) -> Result<Transaction, Box<dyn Error>> {
    let codec_id = pop_i16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());
    trace!("Codec_id : {:?}", codec_id);
    *_context.offset += 2;

    // Because when we had a look at the type_id in the previous step we did not increase the offset
    *_context.offset += 4;

    let time = pop_i64(_raw_msg[*_context.offset..=(*_context.offset + 7)].borrow());
    trace!("Time : {:?}", time);

    *_context.offset += 8;

    let mut advance_time = AdvanceTimeTx {
        time_proposal: time,
    };

    Ok(Transaction {
        base_tx: BaseTx {
            type_id: 19,
            network_id: 0,
            blockchain_id: "".to_string(),
            transferable_outputs: vec![],
            transferable_inputs: vec![],
            memo: vec![],
        },
        tx_id: _tx_id,
        add_validator_tx: None,
        import_tx: None,
        export_tx: None,
        add_subnet_validator_tx: None,
        add_delegator_tx: None,
        create_blockchain_tx: None,
        create_subnet_tx: None,
        advance_time_tx: Some(advance_time),
        reward_validator_tx: None,
        credentials: vec![],
    })
}
