use crate::int::power_of_two_bit_length;
use crate::int::{signed_int_qualified, unsigned_int_qualified};
use crate::parse::Input;
use proc_macro2::{Literal, TokenStream};
use quote::quote;

pub fn generate(input: Input) -> syn::Result<TokenStream> {
    assert!(input.int_bits >= 2);
    let int_bits = input.int_bits;
    let frac_bits = input.frac_bits;
    let used_bits = int_bits + frac_bits;
    let total_bits = match power_of_two_bit_length(used_bits) {
        Some(n) => n,
        None => {
            return Err(syn::Error::new_spanned(
                used_bits.to_string(),
                format!("{} is larger than 64 bits", used_bits),
            ))
        }
    };
    let pad_bits = total_bits - used_bits;

    let denominator = (1 << frac_bits) as f64;
    let conversion_factor = (1 << (frac_bits + pad_bits)) as f64;
    
    let signed = input.signed;
    let q_notation = if signed {
        format!("Q{int_bits}.{frac_bits}")
    } else {
        format!("UQ{int_bits}.{frac_bits}")
    };
    let ty = if signed {
        signed_int_qualified(used_bits)?
    } else {
        unsigned_int_qualified(used_bits)?
    };
    let (min_float, max_float) = if signed {
        let x = (1 << (int_bits - 1)) as f64;
        (-x, x - 1.0 / denominator)
    } else {
        (0.0, ((1 << used_bits) - 1) as f64 / denominator)
    };
    let (min_inner, max_inner) = if signed {
        let n = (1 << (used_bits - 1)) as u64;
        (hex_literal(n), hex_literal(n - 1))
    } else {
        let n = (1 << used_bits) as u64;
        (hex_literal(0), hex_literal(n - 1))
    };
    let name = input.name;

    let used_mask = used_mask_literal(total_bits, pad_bits);
    Ok(quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub struct #name(#ty);

        impl #name {
            pub const Q_NOTATION: &'static str = #q_notation;
            pub const SIGNED: bool = #signed;

            pub const TOTAL_BITS: core::primitive::u8 = #total_bits;
            pub const USED_BITS: core::primitive::u8 = #used_bits;
            pub const INT_BITS: core::primitive::u8 = #int_bits;
            pub const FRAC_BITS: core::primitive::u8 = #frac_bits;
            pub const PAD_BITS: core::primitive::u8 = #pad_bits;

            pub const USED_MASK: #ty = #used_mask;

            pub const MIN_FLOAT: core::primitive::f64 = #min_float;
            pub const MAX_FLOAT: core::primitive::f64 = #max_float;
            pub const MIN: Self = Self(#min_inner);
            pub const MAX: Self = Self(#max_inner);

            pub const DENOMINATOR: core::primitive::f64 = #denominator;
            pub const CONVERSION_FACTOR: core::primitive::f64 = #conversion_factor;

            /// Returns the inner value.
            pub fn to_bits(self) -> #ty {
                self.0
            }

            /// Builds a new instance using the provided bits;
            ///
            /// Note: ensures unused bits (the padding) are zeroed out.
            pub fn from_bits(bits: #ty) -> Self {
                Self(bits & Self::USED_MASK)
            }
        }

        impl From<core::primitive::f64> for #name {
            fn from(value: core::primitive::f64) -> Self {
                if !(Self::MIN_FLOAT..=Self::MAX_FLOAT).contains(&value) {
                    panic!("{} is out of range for {}", value, Self::Q_NOTATION);
                }
                let n = (value * Self::CONVERSION_FACTOR) as #ty;
                Self(n & Self::USED_MASK)
            }
        }

        impl From<#name> for core::primitive::f64 {
            fn from(value: #name) -> Self {
                (value.0 as core::primitive::f64) / #name::CONVERSION_FACTOR
            }
        }

        impl core::ops::Add for #name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

    })
}

/// Converts the provided value to a hex literal.
///
/// Note: this function will panic if the value cannot be parsed as a
/// `proc_macro2::Literal`. This is the best choice, because that would be a bug
/// outside of this crate's control.
fn hex_literal(value: u64) -> Literal {
    format!("{:#x}", value).parse().unwrap()
}

/// e.g. 0b1111_1000 if used_bits is 5 and pad_bits is 3
fn used_mask_literal(total_bits: u8, pad_bits: u8) -> Literal {
    match total_bits {
        8 => {
            let mut x: u8 = 0xFF;
            x >>= pad_bits;
            x <<= pad_bits;
            hex_literal(x as u64)
        }
        16 => {
            let mut x: u16 = 0xFFFF;
            x >>= pad_bits;
            x <<= pad_bits;
            hex_literal(x as u64)
        }
        32 => {
            let mut x: u32 = 0xFFFF_FFFF;
            x >>= pad_bits;
            x <<= pad_bits;
            hex_literal(x as u64)
        }
        64 => {
            let mut x: u64 = 0xFFFF_FFFF_FFFF_FFFF;
            x >>= pad_bits;
            x <<= pad_bits;
            hex_literal(x)
        }
        _ => panic!(),
    }
}