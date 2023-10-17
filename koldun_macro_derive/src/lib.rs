use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemStruct, Token};
use to_snake_case::ToSnakeCase;

struct Args {
    variants: Vec<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let variants = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            variants: variants.into_iter().collect(),
        })
    }
}

#[proc_macro_attribute]
pub fn render_tiles(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let args = parse_macro_input!(attr as Args);
    let name = &item.ident;

    let mut parse_methods = quote! {};

    for variant in &args.variants {
        let fn_name = format_ident!("{}", variant.to_string().to_snake_case());
        let fn_id = format_ident!("{}_id", variant.to_string().to_snake_case());

        let parse_method = quote! {
            pub fn #fn_name(fg: Rgb565, bg: Rgb565) -> [u8; 32 * 32 * 2] {
                let start = #variant.1 * 128;
                let finish = start + 128;
                let mut bits = [0u8; 128];
                bits.copy_from_slice(&TILEMAPS[#variant.0][start..finish]);
                #name::render(&bits, fg, bg)
            }

            pub fn #fn_id() -> usize{
                #variant.0 * 32 + #variant.1
            }
        };
        parse_methods.extend(parse_method);
    }

    let gen = quote! {
        #item

        impl #name{
            #parse_methods
        }
    };

    gen.into()
}
