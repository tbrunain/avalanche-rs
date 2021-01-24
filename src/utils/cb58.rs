use ring::digest;

/// Take a Vector of u8 (bytes) as input and encode it as a base58 string .
pub fn encode(_raw_msg: &[u8]) -> Vec<u8> {
    let mut generated = _raw_msg.to_owned();
    let hash_of_tx_hash = digest::digest(&digest::SHA256, generated.as_ref());
    let (_, right) = hash_of_tx_hash.as_ref().split_at(28);
    generated.extend(right.iter().cloned());
    generated
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn encode_01() {
        let raw_bytes: Vec<u8> = Vec::from([
            167, 65, 3, 199, 84, 61, 61, 2, 114, 17, 36, 64, 162, 99, 87, 106, 138, 230, 215, 176,
            29, 127, 131, 40, 90, 48, 63, 64, 38, 194, 227, 48,
        ]);
        assert_eq!(
            encode(&raw_bytes),
            Vec::from([
                167, 65, 3, 199, 84, 61, 61, 2, 114, 17, 36, 64, 162, 99, 87, 106, 138, 230, 215,
                176, 29, 127, 131, 40, 90, 48, 63, 64, 38, 194, 227, 48, 103, 115, 132, 212
            ]),
            "Testing encoding of 32 bytes into a base58 string"
        );
    }
}
