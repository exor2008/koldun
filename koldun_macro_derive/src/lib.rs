use proc_macro::TokenStream;
use quote::{format_ident, quote};
use to_snake_case::ToSnakeCase;

#[proc_macro]
pub fn load_tga(input: TokenStream) -> TokenStream {
    // let ast: DeriveInput = parse(input).unwrap();
    let input = input.to_string();
    let input: Vec<_> = input.split(",").map(|s| s.trim()).collect();
    let hashmap = input[0];
    let tile = input[1];
    let color1 = format_ident!("{}", input[2]);
    let color2 = format_ident!("{}", input[3]);
    let tile_upper = format_ident!("{}", tile.to_uppercase());
    let tile_snake = format_ident!("{}", tile.to_snake_case());
    let size = format_ident!("SIZE_{}", tile.to_uppercase());
    let offset = format_ident!("OFFSET_{}", tile.to_uppercase());
    let hasmap = format_ident!("{}", hashmap);

    let gen = quote! {
        {
            let #tile_snake = flash
                .load_tga::<{ #size / 4 + 1 }, { #size }>(#offset)
                .await;
            let #tile_snake = D::render_bin_tga(#tile_snake.as_slice(), colors::#color1, colors::#color2);
            #hasmap.insert(#tile_upper, #tile_snake).unwrap();
        }
    };
    gen.into()
}
