use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::pixelcolor::Rgb565;
use koldun_macro_derive::render_tiles;

pub const TILE_SIZE_X: usize = 32;
pub const TILE_SIZE_Y: usize = 32;

pub const EMPTY: (usize, usize) = (0, 0);
pub const FLOOR: (usize, usize) = (0, 1);
pub const GROUND1: (usize, usize) = (0, 2);
pub const GROUND2: (usize, usize) = (0, 3);
pub const DOOR_OPEN: (usize, usize) = (0, 4);
pub const CHAIR: (usize, usize) = (0, 5);
pub const DEBRIS1: (usize, usize) = (0, 6);
pub const DEBRIS2: (usize, usize) = (0, 7);
pub const BRIDGE_WOOD1: (usize, usize) = (0, 8);
pub const BRIDGE_WOOD2: (usize, usize) = (0, 9);

pub const WALL1: (usize, usize) = (1, 0);
pub const WALL2: (usize, usize) = (1, 1);
pub const WALL3: (usize, usize) = (1, 2);
pub const WALL4: (usize, usize) = (1, 3);
pub const BRICK_WALL1: (usize, usize) = (1, 4);
pub const BRICK_WALL2: (usize, usize) = (1, 5);
pub const BRICK_WALL3: (usize, usize) = (1, 6);
pub const PLATE_WALL1: (usize, usize) = (1, 7);
pub const PLATE_WALL2: (usize, usize) = (1, 8);
pub const STONE1: (usize, usize) = (1, 9);
pub const STONE2: (usize, usize) = (1, 10);
pub const STONE3: (usize, usize) = (1, 11);
pub const FENCE: (usize, usize) = (1, 12);
pub const DOOR_FENCE: (usize, usize) = (1, 13);
pub const DOOR_WOOD: (usize, usize) = (1, 14);
pub const TABLE: (usize, usize) = (1, 15);
pub const PINES: (usize, usize) = (1, 16);
pub const PINE: (usize, usize) = (1, 17);
pub const TREE: (usize, usize) = (1, 18);
pub const TREES: (usize, usize) = (1, 19);
pub const MUSHROOM: (usize, usize) = (1, 20);
pub const MUSHROOMS: (usize, usize) = (1, 21);
pub const WEB: (usize, usize) = (1, 22);

pub const WIZARD_IDLE1: (usize, usize) = (2, 0);
pub const WIZARD_IDLE2: (usize, usize) = (2, 1);
pub const WIZARD_DOWN1: (usize, usize) = (2, 2);
pub const WIZARD_DOWN2: (usize, usize) = (2, 3);
pub const WIZARD_UP1: (usize, usize) = (2, 4);
pub const WIZARD_UP2: (usize, usize) = (2, 5);
pub const WIZARD_LEFT1: (usize, usize) = (2, 6);
pub const WIZARD_LEFT2: (usize, usize) = (2, 7);
pub const WIZARD_RIGHT1: (usize, usize) = (2, 8);
pub const WIZARD_RIGHT2: (usize, usize) = (2, 9);
pub const SPIDER1: (usize, usize) = (2, 30);
pub const SPIDER2: (usize, usize) = (2, 31);

pub const TILEMAPS: [&[u8; 4096]; 3] = [
    include_bytes!("..\\..\\resources\\tiles\\compressed\\tiles0.bin"),
    include_bytes!("..\\..\\resources\\tiles\\compressed\\tiles1.bin"),
    include_bytes!("..\\..\\resources\\tiles\\compressed\\tiles2.bin"),
];

#[render_tiles(
    EMPTY,
    FLOOR,
    GROUND1,
    GROUND2,
    DOOR_OPEN,
    CHAIR,
    DEBRIS1,
    DEBRIS2,
    BRIDGE_WOOD1,
    BRIDGE_WOOD2,
    WALL1,
    WALL2,
    WALL3,
    WALL4,
    BRICK_WALL1,
    BRICK_WALL2,
    BRICK_WALL3,
    PLATE_WALL1,
    PLATE_WALL2,
    STONE1,
    STONE2,
    STONE3,
    FENCE,
    DOOR_FENCE,
    DOOR_WOOD,
    TABLE,
    PINES,
    PINE,
    TREE,
    TREES,
    MUSHROOM,
    MUSHROOMS,
    WEB,
    WIZARD_IDLE1,
    WIZARD_IDLE2,
    WIZARD_DOWN1,
    WIZARD_DOWN2,
    WIZARD_UP1,
    WIZARD_UP2,
    WIZARD_LEFT1,
    WIZARD_LEFT2,
    WIZARD_RIGHT1,
    WIZARD_RIGHT2,
    SPIDER1,
    SPIDER2
)]
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
