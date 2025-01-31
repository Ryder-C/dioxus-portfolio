use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, token::Comma, LitStr};

struct MacroInput {
    category: LitStr,
    name: LitStr,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let category = input.parse()?;
        let _comma: Comma = input.parse()?;
        let name = input.parse()?;

        Ok(Self { category, name })
    }
}

#[proc_macro]
pub fn md(tokens: TokenStream) -> TokenStream {
    let input: MacroInput = parse_macro_input!(tokens as MacroInput);

    let path = format!(
        "/__build_{}_{}.rs",
        input.category.value(),
        input.name.value()
    );

    quote! {
        include!(concat!(env!("OUT_DIR"), #path))
    }
    .into()
}
