use tracing::{instrument, trace};

use std::borrow::Borrow;

use std::error::Error;

use crate::avm::parser::{Context, InitialState};
use crate::avm::parser::output_parser::output_parser;
use crate::utils::conversion::{pop_u32, pop_i32};

#[instrument(skip(_raw_msg), fields(ipc = % _context.ipc, tx_id = % _context.tx_id))]
pub fn initial_state_parser(
    _raw_msg: &Vec<u8>,
    _context: &mut Context,
) -> Result<InitialState, Box<dyn Error>> {
    // Type Id
    let fx_id = pop_i32(_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "\n {} -- {} \n Output -- fx_id : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        fx_id
    );
    *_context.offset += 4;

    // Outputs Array Size
    let number_of_outputs = pop_u32(&_raw_msg[*_context.offset..=(*_context.offset + 3)].borrow());
    trace!(
        "InitialState Parser - {} -- {} \n Number of outputs : {:?} \n +++++++",
        _context.ipc,
        _context.tx_id,
        number_of_outputs
    );
    *_context.offset += 4;

    // Outputs
    let mut outputs = Vec::new();
    let mut index = 0;

    while index < number_of_outputs {
        trace!(
            "InitialState Parser - {} -- {} \n Initial state - output number {} -- offset {} -- size {} \n +++++++",
            _context.ipc,
            _context.tx_id,
            index,
            _context.offset,
            _raw_msg.len()
        );
        let output = output_parser(_raw_msg, _context)?;

        outputs.push(output);
        index += 1;
    }

    Ok(InitialState { fx_id, outputs })
}
