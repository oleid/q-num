use crate::math::power_of_two_bit_length;
use syn::Type;

// Unsigned integer qualified type
pub fn signed_int_qualified(bits: u8) -> syn::Result<Type> {
    core_primitive_type(&signed_integer_type_string(bits)?)
}

// Unsigned integer qualified type
pub fn unsigned_int_qualified(bits: u8) -> syn::Result<Type> {
    core_primitive_type(&unsigned_integer_type_string(bits)?)
}

pub fn core_primitive_type(s: &str) -> syn::Result<Type> {
    syn::parse_str(&format!("::core::primitive::{s}"))
}

// Signed integer unqualified string
fn signed_integer_type_string(bits: u8) -> syn::Result<String> {
    integer_type_string(bits, 'i')
}

// Unsigned integer unqualified string
fn unsigned_integer_type_string(bits: u8) -> syn::Result<String> {
    integer_type_string(bits, 'u')
}

fn integer_type_string(bits: u8, prefix: char) -> syn::Result<String> {
    match power_of_two_bit_length(bits) {
        Some(bits) => Ok(format!("{prefix}{bits}")),
        None => Err(syn::Error::new_spanned(
            bits.to_string(),
            format!("{prefix}64 is the largest supported type"),
        )),
    }
}
