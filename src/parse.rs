use proc_macro2::Span;
use syn::parse;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitInt, Token, Visibility};

pub struct Input {
    pub visibility: Option<Visibility>,
    pub name: Ident,
    pub signed: bool,
    pub int_bits: u8,
    pub frac_bits: u8,
}

/// Parses, for example:
/// - `define_q_num!(MyNum, Q10.4)'
/// - `define_q_num!(pub MyNum, UQ10.4)'
///
/// "UQ" -> signed = false
/// "Q" -> signed = true
impl Parse for Input {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let visibility = if input.peek(Token![pub]) {
            Some(input.parse()?)
        } else {
            None
        };
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let next_token = input.parse::<Ident>()?.to_string();
        let (signed, int_bits) = if next_token.starts_with("UQ") {
            let rest = next_token.strip_prefix("UQ").unwrap();
            (false, parse_int_bits(rest)?)
        } else if next_token.starts_with('Q') {
            let rest = next_token.strip_prefix('Q').unwrap();
            (true, parse_int_bits(rest)?)
        } else {
            return Err(parse::Error::new(Span::call_site(), "Expected UQ or Q"));
        };
        input.parse::<Token![.]>()?;
        let frac_bits = input.parse::<LitInt>()?.base10_parse()?;
        Ok(Input {
            visibility,
            name,
            signed,
            int_bits,
            frac_bits,
        })
    }
}

fn parse_int_bits(input: &str) -> Result<u8, parse::Error> {
    match input.parse() {
        Ok(x) => Ok(x),
        Err(_) => Err(parse::Error::new(Span::call_site(), "Expected integer")),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_int_bits;

    #[test]
    fn test_parse_int_bits_1() {
        let x = parse_int_bits("10");
        assert!(x.is_ok());
    }

    #[test]
    fn test_parse_int_bits_2() {
        let x = parse_int_bits("31x");
        assert!(x.is_err());
    }
}
