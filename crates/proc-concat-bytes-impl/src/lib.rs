extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_hack::proc_macro_hack;

use quote::ToTokens;
use syn::parse_macro_input;

struct ByteConcat(Vec<u8>);

enum Param {
    Byte(u8),
    ByteStr(Vec<u8>),
}

impl Param {
    fn len(&self) -> usize {
        match self {
            Param::Byte(_) => 1,
            Param::ByteStr(vec) => vec.len(),
        }
    }
}

impl syn::parse::Parse for ByteConcat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut args = input.parse_terminated::<_, syn::Token![,]>(|input| {
            if input.peek(syn::LitByte) {
                Ok(Param::Byte(input.parse::<syn::LitByte>()?.value()))
            } else {
                Ok(Param::ByteStr(input.parse::<syn::LitByteStr>()?.value()))
            }
        })?;

        let mut bytes = Vec::with_capacity(args.iter().map(|slice| slice.len()).sum());

        for param in args.iter_mut() {
            match param {
                Param::Byte(b) => bytes.push(*b),
                Param::ByteStr(ref mut vec) => bytes.append(vec),
            }
        }

        Ok(ByteConcat(bytes))
    }
}

#[proc_macro_hack]
pub fn concat_bytes(input: TokenStream) -> TokenStream {
    let bytes = parse_macro_input!(input as ByteConcat);

    let literal = syn::LitByteStr::new(&bytes.0, Span::call_site());
    literal.into_token_stream().into()
}
