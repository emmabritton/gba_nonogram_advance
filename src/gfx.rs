use crate::sprites;
use agb::display::object::Object;
use agb::display::tile_data::TileData;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat};
use agb::display::{GraphicsFrame, Priority};

pub const TILE_SIZE: i32 = 8;

const SPR_IDX_CORNER: usize = 0;
const SPR_IDX_HORZ: usize = 1;
const SPR_IDX_VERT: usize = 2;

pub fn lvl_button_sprites() -> [Object; 3] {
    [
        Object::new(sprites::LVL_SELECT_CORNER.sprite(0)),
        Object::new(sprites::LVL_SELECT_HORZ.sprite(0)),
        Object::new(sprites::LVL_SELECT_VERT.sprite(0)),
    ]
}

pub fn button_sprites() -> [Object; 3] {
    [
        Object::new(sprites::BUTTON_SELECT_CORNER.sprite(0)),
        Object::new(sprites::BUTTON_SELECT_HORZ.sprite(0)),
        Object::new(sprites::BUTTON_SELECT_VERT.sprite(0)),
    ]
}

pub fn draw_button_highlight(
    at: (i32, i32),
    sprites: &mut [Object; 3],
    button_size: (u8, u8),
    graphics: &mut GraphicsFrame,
) {
    draw_corners(at, &mut sprites[SPR_IDX_CORNER], button_size, graphics);
    draw_vert(at, &mut sprites[SPR_IDX_VERT], false, button_size, graphics);
    draw_vert(at, &mut sprites[SPR_IDX_VERT], true, button_size, graphics);
    draw_horz(at, &mut sprites[SPR_IDX_HORZ], false, button_size, graphics);
    draw_horz(at, &mut sprites[SPR_IDX_HORZ], true, button_size, graphics);
}

fn draw_vert(
    at: (i32, i32),
    sprite: &mut Object,
    flip: bool,
    button_size: (u8, u8),
    graphics: &mut GraphicsFrame,
) {
    let x = at.0
        + (if flip {
            TILE_SIZE * button_size.0 as i32
        } else {
            0
        });
    let y = at.1 + TILE_SIZE;
    for i in 0..(button_size.1 - 1) {
        let pos = (x, y + (i as i32 * TILE_SIZE));
        sprite.set_hflip(flip).set_pos(pos).show(graphics);
    }
}

fn draw_horz(
    at: (i32, i32),
    sprite: &mut Object,
    flip: bool,
    button_size: (u8, u8),
    graphics: &mut GraphicsFrame,
) {
    let x = at.0 + TILE_SIZE;
    let y = at.1
        + (if flip {
            TILE_SIZE * button_size.1 as i32
        } else {
            0
        });
    for i in 0..(button_size.0 - 1) {
        let pos = (x + (i as i32 * TILE_SIZE), y);
        sprite.set_vflip(flip).set_pos(pos).show(graphics);
    }
}

fn draw_corners(
    at: (i32, i32),
    sprite: &mut Object,
    button_size: (u8, u8),
    graphics: &mut GraphicsFrame,
) {
    draw_corner(at, sprite, false, false, graphics);
    let x = at.0 + (button_size.0 as i32 * TILE_SIZE);
    draw_corner((x, at.1), sprite, false, true, graphics);
    let y = at.1 + (button_size.1 as i32 * TILE_SIZE);
    draw_corner((at.0, y), sprite, true, false, graphics);
    draw_corner((x, y), sprite, true, true, graphics);
}

fn draw_corner(
    at: (i32, i32),
    sprite: &mut Object,
    vflip: bool,
    hflip: bool,
    graphics: &mut GraphicsFrame,
) {
    sprite
        .set_hflip(hflip)
        .set_vflip(vflip)
        .set_pos(at)
        .show(graphics);
}

pub fn background(data: &'static TileData, priority: Priority) -> RegularBackground {
    let mut background = RegularBackground::new(
        priority,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    background.fill_with(data);

    background
}

/// Background first
/// then extras
/// then ui
pub fn background_stack<const N: usize>(layers: [&'static TileData; N]) -> [RegularBackground; N] {
    assert!(N > 0, "at least 1 background required");
    assert!(N <= 4, "max 4 layers");

    let priorities = [Priority::P3, Priority::P2, Priority::P1, Priority::P0];

    core::array::from_fn(|i| background(layers[i], priorities[i]))
}
