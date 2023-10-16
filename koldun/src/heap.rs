use core::mem::MaybeUninit;
use defmt::info;
use embedded_alloc::Heap;

#[global_allocator]
pub static HEAP: Heap = Heap::empty();

pub fn init() {
    info!("Initializing heap...");
    const HEAP_SIZE: usize = 1024 * 70; //kB
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

pub fn used() -> usize {
    HEAP.used()
}
