#![no_std]
#![no_main]
#![feature(async_fn_in_trait)]
#![feature(slice_pattern)]
#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]
#![feature(error_in_core)]
#![feature(impl_trait_in_assoc_type)]
#![feature(const_trait_impl)]
#![feature(slice_flatten)]
#![feature(exclusive_range_pattern)]

pub mod game;
pub mod heap;
pub mod ili9486;

#[macro_export]
macro_rules! h_vec {
    ($n: expr; $($arg:expr),*) => {
        {
            let mut vec: Vec<_, $n> = Vec::new();
            $(vec.push($arg).unwrap();)*
            vec
        }
    };
}

#[macro_export]
macro_rules! add_to_redraw {
    ($to_redraw: ident, $request: ident) => {
        if !$to_redraw.contains(&$request) {
            $to_redraw.push($request).unwrap();
        }
    };
}
