use crate::math::power_of_two_bit_length;
use syn::Type;

// Signed integer qualified type
pub fn signed_int_qualified(bits: u8) -> syn::Result<Type> {
    core_primitive_type(&signed_integer_type_string(bits)?)
}

// Unsigned integer qualified type
pub fn unsigned_int_qualified(bits: u8) -> syn::Result<Type> {
    core_primitive_type(&unsigned_integer_type_string(bits)?)
}

/// Adds prefix to make a hygienic type name.
pub fn core_primitive_type(s: &str) -> syn::Result<Type> {
    syn::parse_str(&format!("::core::primitive::{s}"))
}

/// Signed integer unqualified string
fn signed_integer_type_string(bits: u8) -> syn::Result<String> {
    integer_type_string(bits, 'i')
}

/// Unsigned integer unqualified string
fn unsigned_integer_type_string(bits: u8) -> syn::Result<String> {
    integer_type_string(bits, 'u')
}

/// Returns the narrowest integer type that can hold `bits` bits.
/// ('i' prefix -> signed integer; 'u' prefix -> unsigned integer)
fn integer_type_string(bits: u8, prefix: char) -> syn::Result<String> {
    assert!(prefix == 'i' || prefix == 'u');
    match power_of_two_bit_length(bits) {
        Some(bits) => Ok(format!("{prefix}{bits}")),
        None => Err(syn::Error::new_spanned(
            bits.to_string(),
            format!("{prefix}64 is the largest supported type"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_signed_int_qualified() {
        let ty = signed_int_qualified(13).unwrap();
        let s = quote! { #ty }.to_string();
        assert_eq!(s, ":: core :: primitive :: i16");
    }

    #[test]
    fn test_unsigned_int_qualified() {
        let ty = unsigned_int_qualified(55).unwrap();
        let s = quote! { #ty }.to_string();
        assert_eq!(s, ":: core :: primitive :: u64");
    }

    #[test]
    fn test_signed_integer_type_string() {
        assert_eq!(signed_integer_type_string(10).unwrap(), "i16");
    }

    #[test]
    fn test_unsigned_integer_type_string() {
        assert_eq!(unsigned_integer_type_string(6).unwrap(), "u8");
    }

    #[test]
    fn test_integer_type_string() {
        use super::integer_type_string as f;
        assert_eq!(f(8, 'i').unwrap(), "i8");
        assert_eq!(f(9, 'i').unwrap(), "i16");
        assert_eq!(f(15, 'u').unwrap(), "u16");
        assert_eq!(f(16, 'u').unwrap(), "u16");
        assert_eq!(f(17, 'u').unwrap(), "u32");
    }
}
