use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;
use core::mem::transmute;
use embassy_rp::flash::{Async, Flash as RPFlash, Instance};
extern crate alloc;

const ADDR_OFFSET: usize = 0x100000; // 1Mb offset
const FLASH_SIZE: usize = 2 * 1024 * 1024;

#[async_trait]
pub trait Flash {
    async fn load(&mut self, offset: usize, buf: &mut [u32]);
    async fn load_tga<const SIZE: usize, const SIZE2: usize>(&mut self, offset: usize) -> Vec<u8>;
}

pub struct FlashAccess<'a, T: Instance> {
    flash: RPFlash<'a, T, Async, FLASH_SIZE>,
}

impl<'a, T: Instance> FlashAccess<'a, T> {
    pub fn new(flash: RPFlash<'a, T, Async, FLASH_SIZE>) -> Self {
        FlashAccess { flash }
    }
}

#[async_trait]
impl<'a, T: Instance + Send> Flash for FlashAccess<'a, T> {
    async fn load(&mut self, offset: usize, buf: &mut [u32]) {
        self.flash
            .background_read((ADDR_OFFSET + offset) as u32, buf)
            .unwrap()
            .await;
    }

    async fn load_tga<const SIZE_U32: usize, const SIZE_U8: usize>(
        &mut self,
        offset: usize,
    ) -> Vec<u8> {
        assert!(SIZE_U32 * 4 == SIZE_U8);
        let data_u32 = &mut [0u32; SIZE_U32];
        self.load(offset, data_u32).await;
        let data_u8 = unsafe { transmute::<&[u32; SIZE_U32], &[u8; SIZE_U8]>(data_u32) };
        let mut vec_u8: Vec<u8> = Vec::new();
        vec_u8.extend_from_slice(&data_u8[..]);
        vec_u8
    }
}
