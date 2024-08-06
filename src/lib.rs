//! # Q Number
//!
//! This library provides a macro to define a binary fixed-point number type for
//! a specified number of integer bits and fractional bits.
//!
//! ## Summary
//!
//! This library provides the `define_q_num!` procedural macro (evaluated at
//! compile time) to define a signed/unsigned binary fixed-point number type. It
//! uses ARM-style Q notation: `Qm.n` (signed) or `UQm.n` (unsigned), where:
//!
//! - **m** is the number of integer bits, and
//! - **n** is the number of fractional bits.
//!
//! Internally, the macro chooses the narrowest primitive integer type that can
//! hold `m + n` bits, up to `u64` (unsigned) and `i64` (signed). More internal
//! details are discussed below.
//!
//! ## Q Number Value
//!
//! A Q number's value is the ratio of the stored number (having n + m bits)
//! and a fixed denominator (equal to 2 ^ n).
//!
//! For example, using the UQ3.2 specification, the bit pattern 0b10111
//! represents the value 5.75. Keeping in mind the denominator is 2 ^ 2 = 4,
//! there are two ways to see this:
//!
//! - 0b10111 / 4 == 23 / 4 == 5.75
//! - 0b101 + 0b11 / 4 == 5 + 3/4 == 5.75
//!
//! ## Example Macro Usage
//!
//! Here is one example:
//!
//! ```
//! # use q_num::define_q_num;
//! define_q_num!(X, Q6.2);
//! let a = X::try_from(13.75).unwrap();
//! let b = X::try_from(-2.25).unwrap();
//! let c = X::try_from(11.5).unwrap();
//! assert_eq!(a + b, c);
//! ```
//!
//! This defines a new type named `MyQ`, a signed fixed-point number represented
//! internally with 8 bits:
//!
//! - 6 bits for the integer part
//! - 2 bits for the fractional part
//!
//! ## Also Defined: Associated Methods
//!
//! The example above also defines the following floating-point conversions:
//!
//! - `MyQ::from(f64) -> MyQ`
//! - `f64::from(MyQ) -> f64`
//!
//! It also defines the following getter and setter to access the internal
//! representation:
//!
//! - `MyQ.to_bits() -> i8`
//! - `MyQ::from_bits(i8) -> MyQ`
//!
//! ## Macro Variations
//!
//! Variations include (a) signed vs. unsigned, and (b) visibility.
//!
//! ### Unsigned Variation
//!
//! Use `UQ` to define an _unsigned_ fixed-point number:
//!
//! ```
//! # use q_num::define_q_num;
//! define_q_num!(MyNum, UQ11.5);
//! ```
//!
//! The integer part uses two's complement representation.
//!
//! ### Visibility
//!
//! A caller can optionally include a visibility modifier for the new type:
//!
//! ```
//! # use q_num::define_q_num;
//! define_q_num!(pub MyNum, UQ11.5);
//! ```
//!
//! ## Numerical Properties
//!
//! The value of a Q number is the ratio of the storage number and a fixed
//! denominator.
//!
//! ## Internals
//!
//! The `define_q!` procedural macro defines a struct wrapping a primitive
//! integer. This is an example of the [newtype pattern].
//!
//! For example, `define_q!(MyNum, Q6.2)` defines `struct MyNum(i8)`.
//!
//! The macro selects the narrowest primitive integer type that can hold the
//! necessary number of bits. (For signed numbers, the integer part uses two's
//! complement representation, meaning that the sign bit is already counted
//! towards the integer part.)
//!
//! [newtype pattern]:
//!     https://doc.rust-lang.org/rust-by-example/generics/new_types.html
//!
//! ## See Also
//!
//! https://en.wikipedia.org/wiki/Q_(number_format)

mod gen;
mod literal;
mod math;
mod parse;
mod types;

use crate::gen::generate;
use crate::parse::Input;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_q_num(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    match generate(input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}
