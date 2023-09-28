pub fn total_bits(used_bits: u8) -> syn::Result<u8> {
    match power_of_two_bit_length(used_bits) {
        Some(n) => Ok(n),
        None => Err(syn::Error::new_spanned(
            used_bits.to_string(),
            format!("{} is larger than 64 bits", used_bits),
        )),
    }
}

/// Return number of bits (corresponding to a power of two) required to
/// represent `bits`. Successful output can be 8, 16, 32, or 64.
#[rustfmt::skip]
pub fn power_of_two_bit_length(bits: u8) -> Option<u8> {
    match bits {
         0 ..=  8 => Some(8),
         9 ..= 16 => Some(16),
        17 ..= 32 => Some(32),
        33 ..= 64 => Some(64),
                _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::power_of_two_bit_length;

    #[test]
    fn power_of_two_bit_length_near_8() {
        assert_eq!(power_of_two_bit_length(7), Some(8));
        assert_eq!(power_of_two_bit_length(8), Some(8));
        assert_eq!(power_of_two_bit_length(9), Some(16));
    }

    #[test]
    fn power_of_two_bit_length_near_16() {
        assert_eq!(power_of_two_bit_length(15), Some(16));
        assert_eq!(power_of_two_bit_length(16), Some(16));
        assert_eq!(power_of_two_bit_length(17), Some(32));
    }

    #[test]
    fn power_of_two_bit_length_near_32() {
        assert_eq!(power_of_two_bit_length(31), Some(32));
        assert_eq!(power_of_two_bit_length(32), Some(32));
        assert_eq!(power_of_two_bit_length(33), Some(64));
    }

    #[test]
    fn power_of_two_bit_length_near_64() {
        assert_eq!(power_of_two_bit_length(63), Some(64));
        assert_eq!(power_of_two_bit_length(64), Some(64));
        assert_eq!(power_of_two_bit_length(65), None);
    }
}
