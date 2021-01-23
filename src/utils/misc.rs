// ToDo Find a better name for this , was out of idea


use ring::digest;
use rust_base58::ToBase58;

/// We want to generate the TxId (readable format) for this Tx .
/// In order to do so we need to :
/// 1) Generate a sha256 hash out of the bytes of the tx .
/// 2) Generate a sha256 out of the hash calculated in step 1
/// 3) Take the last 4 bytes of the hash from step 2 and push hash from step 1 et those 4 bytes
/// in an array .
/// Then return a base_58 string
pub fn generate_id(_raw_msg: &Vec<u8>) -> String {
    let mut tx_id = Vec::new();
    let tx_hash = digest::digest(&digest::SHA256, &_raw_msg);
    tx_id.extend(tx_hash.as_ref().iter().cloned());

    let hash_of_tx_hash = digest::digest(&digest::SHA256, tx_hash.as_ref());
    let (_, right) = hash_of_tx_hash.as_ref().split_at(28);
    tx_id.extend(right.iter().cloned());

    tx_id.to_base58()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{instrument, debug};
    use tracing::field::debug;

    #[test]
    fn encode() {
        //
        let raw_bytes = Vec::from([0, 0, 0, 0, 0, 0, 7, 134, 25, 112, 255, 118, 103, 173, 145, 89, 229, 195, 241, 253, 222, 78, 157, 221, 152, 47, 127, 71, 103, 59, 49, 181, 194, 209, 153, 92, 189, 125, 0, 0, 0, 0, 0, 1, 130, 234, 0, 0, 0, 19, 0, 0, 0, 0, 95, 111, 231, 200, 0, 0, 0, 0]);
        assert_eq!(generate_id(&raw_bytes), "ucQvpG8mfvSZJjvctiwpN9r81Aihh1nuibKT8Vh1wesEySg23", "Bouh");
    }
}
