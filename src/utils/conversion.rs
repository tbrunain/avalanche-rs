use std::convert::TryInto;

pub fn pop_i64(barry: &[u8]) -> i64 {
    let x: [u8; 8] = barry.try_into().expect("slice with incorrect length");
    i64::from_be_bytes(x)
}

pub fn pop_u32(barry: &[u8]) -> u32 {
    let x: [u8; 4] = barry.try_into().expect("slice with incorrect length");
    u32::from_be_bytes(x)
}

pub fn pop_i32(barry: &[u8]) -> i32 {
    let x: [u8; 4] = barry.try_into().expect("slice with incorrect length");
    i32::from_be_bytes(x)
}

pub fn pop_u16(barry: &[u8]) -> u16 {
    let x: [u8; 2] = barry.try_into().expect("slice with incorrect length");
    u16::from_be_bytes(x)
}

pub fn pop_i16(barry: &[u8]) -> i16 {
    let x: [u8; 2] = barry.try_into().expect("slice with incorrect length");
    i16::from_be_bytes(x)
}

pub fn pop_u8(barry: &[u8]) -> u8 {
    let x: [u8; 1] = barry.try_into().expect("slice with incorrect length");
    u8::from_be_bytes(x)
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn convert_u8_01() {
        let raw_bytes: Vec<u8> = Vec::from([0]);
        let result = pop_u8(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into u8 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_u8_02() {
        let raw_bytes: Vec<u8> = Vec::from([255]);
        let result = pop_u8(&raw_bytes);
        assert_eq!(
            result, 255,
            "Converting vec {:?} into u8 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i16_01() {
        let raw_bytes: Vec<u8> = Vec::from([0, 0]);
        let result = pop_i16(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into i16 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i16_02() {
        let raw_bytes: Vec<u8> = Vec::from([255, 255]);
        let result = pop_i16(&raw_bytes);
        assert_eq!(
            result, -1,
            "Converting vec {:?} into i16 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_u16_01() {
        let raw_bytes: Vec<u8> = Vec::from([0, 0]);
        let result = pop_u16(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into u16 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_u16_02() {
        let raw_bytes: Vec<u8> = Vec::from([255, 255]);
        let result = pop_u16(&raw_bytes);
        assert_eq!(
            result, 65535,
            "Converting vec {:?} into u16 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i32_01() {
        let raw_bytes: Vec<u8> = Vec::from([0, 0, 0, 0]);
        let result = pop_i32(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into i32 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i32_02() {
        let raw_bytes: Vec<u8> = Vec::from([254, 255, 255, 255]);
        let result = pop_i32(&raw_bytes);
        assert_eq!(
            result, -16777217,
            "Converting vec {:?} into i32 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_u32_01() {
        let raw_bytes: Vec<u8> = Vec::from([0, 0, 0, 0]);
        let result = pop_u32(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into u32 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_u32_02() {
        let raw_bytes: Vec<u8> = Vec::from([255, 255, 255, 255]);
        let result = pop_u32(&raw_bytes);
        assert_eq!(
            result, 4294967295,
            "Converting vec {:?} into u32 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i64_01() {
        let raw_bytes: Vec<u8> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0]);
        let result = pop_i64(&raw_bytes);
        assert_eq!(
            result, 0,
            "Converting vec {:?} into i64 {:?}",
            raw_bytes, result
        );
    }

    #[test]
    fn convert_i64_02() {
        let raw_bytes: Vec<u8> = Vec::from([1, 255, 255, 255, 255, 255, 255, 255]);
        let result = pop_i64(&raw_bytes);
        assert_eq!(
            result, 144115188075855871,
            "Converting vec {:?} into i64 {:?}",
            raw_bytes, result
        );
    }
}
