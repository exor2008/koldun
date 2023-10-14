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

pub mod control;
pub mod game;
pub mod heap;
pub mod ili9486;
