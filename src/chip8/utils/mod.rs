pub fn bits_from_u8(byte: u8) -> Vec<bool> {
    let mut bits = Vec::new();
    for i in 0..8 {
        bits.push(bool::from_u8((byte>>i)&1));
    }
    bits.reverse();
    bits
}

trait FromInteger {
    fn from_u8(val: u8) -> bool;
}

impl FromInteger for bool {
    fn from_u8(val: u8) -> bool {
        if val > 0 {true} else {false}
    }
}

#[cfg(test)]
mod test {
    use super::bits_from_u8;

    #[test]
    fn from_u8() {
        let val = 253;
        let bits = bits_from_u8(val);
        assert_eq!(bits, Vec::from([true, true, true, true, true, true, false, true]));
    }
}