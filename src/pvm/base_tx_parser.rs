use crate::avm::parser::Context;
use crate::pvm::transferable_input_parser::{transferable_input_parser, TransferableInput};
use crate::pvm::transferable_output_parser::{transferable_output_parser, TransferableOutput};
use crate::utils::cb58::encode;
use crate::utils::conversion::{pop_i16, pop_i32, pop_u32};
use rust_base58::ToBase58;
use std::borrow::Borrow;
use std::error::Error;
use tracing::{instrument, trace};

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseTx {
    pub type_id: i32,
    pub network_id: u32,
    pub blockchain_id: String,
    pub transferable_outputs: Vec<TransferableOutput>,
    pub transferable_inputs: Vec<TransferableInput>,
    pub memo: Vec<u8>,
}

#[instrument(fields(block_id = % _context.tx_id, tx_type = "base"))]
pub fn base_tx_parser(_raw_msg: &[u8], _context: &mut Context) -> Result<BaseTx, Box<dyn Error>> {
    let codec_id = pop_i16(_raw_msg[*_context.offset..=(*_context.offset + 1)].borrow());
    trace!("Codec_id : {:?}", codec_id);

    *_context.offset += 2;

    let tx_type_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Tx typeId : {:?}", tx_type_id);

    *_context.offset += 4;

    let network_id = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Network Id : {:?}", network_id);

    *_context.offset += 4;

    let blockchain_id = encode(&_raw_msg[*_context.offset..=(*_context.offset + 31)]).to_base58();
    trace!("blockchain Id : {:?}", blockchain_id);

    *_context.offset += 32;

    let number_of_outputs = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("number of Outputs : {:?}", number_of_outputs);

    *_context.offset += 4;
    // Outputs
    let mut outputs = Vec::new();
    let mut index = 0;

    while index < number_of_outputs {
        trace!("Output number {}", index,);
        outputs.push(transferable_output_parser(&_raw_msg, _context)?);
        index += 1;
    }

    // Inputs Array Size
    let number_of_inputs = pop_u32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!("Inputs' array size : {:?}", number_of_inputs);
    *_context.offset += 4;

    // Inputs
    let mut inputs = Vec::new();
    let mut index = 0;

    while index < number_of_inputs {
        trace!("Input number {} ", index,);
        inputs.push(transferable_input_parser(&_raw_msg, _context)?);
        index += 1;
    }

    // Memo size
    let memo_size = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)]) as usize;
    trace!("Memo size : {:?}", memo_size);
    *_context.offset += 4;

    // Memo
    let mut memo = Vec::new();
    if memo_size == 0 {
        trace!("Memo is empty");
    } else {
        trace!(
            "Memo content : {:?}",
            &_raw_msg[*_context.offset..=(*_context.offset + memo_size)]
        );
        memo = _raw_msg[*_context.offset..=(*_context.offset + memo_size)].to_vec();
        *_context.offset += memo_size;
    }

    Ok(BaseTx {
        type_id: tx_type_id,
        network_id,
        blockchain_id,
        transferable_outputs: outputs,
        transferable_inputs: inputs,
        memo,
    })
}
