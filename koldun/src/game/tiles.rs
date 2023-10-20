use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::pixelcolor::Rgb565;
use koldun_macro_derive::render_tiles;

pub const EMPTY: (usize, usize) = (0, 0);
pub const FLOOR: (usize, usize) = (0, 1);
pub const WALL1: (usize, usize) = (0, 2);
pub const WALL2: (usize, usize) = (0, 3);
pub const WALL3: (usize, usize) = (0, 4);
pub const WALL4: (usize, usize) = (0, 5);
pub const WIZARD1: (usize, usize) = (0, 6);
pub const WIZARD2: (usize, usize) = (0, 7);

pub const TILEMAPS: [&[u8; 4096]; 1] = [include_bytes!(
    "..\\..\\resources\\tiles\\compressed\\tiles0.bin"
)];

#[render_tiles(EMPTY, FLOOR, WALL1, WALL2, WALL3, WALL4, WIZARD1, WIZARD2)]
pub struct Tile {}

impl Tile {
    fn render(data: &[u8; 128], fg: Rgb565, bg: Rgb565) -> [u8; 32 * 32 * 2] {
        let mut colors = [0; 32 * 32 * 2];
        for (i, byte) in data.iter().enumerate() {
            for j in (0..8).rev() {
                let bit = (byte >> j) & 1;

                let c: [u8; 2];
                match bit == 0 {
                    true => {
                        c = Tile::color_to_data(bg);
                    }
                    false => {
                        c = Tile::color_to_data(fg);
                    }
                }
                let start = (i * 16) + j * 2;
                let end = start + 2;
                colors[start..end].copy_from_slice(&c);
            }
        }
        colors
    }

    fn color_to_data(color: Rgb565) -> [u8; 2] {
        let b = color.to_ne_bytes();
        [b[1], b[0]]
    }
}
