use crate::gfx::{TILE_SIZE, draw_button_highlight};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::fixnum::Num;

pub struct Highlight {
    pos_x: Num<i32, 4>,
    pos_y: Num<i32, 4>,
    target_x: Num<i32, 4>,
    target_y: Num<i32, 4>,
}

impl Highlight {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            pos_x: Num::from(x as i32 * TILE_SIZE),
            pos_y: Num::from(y as i32 * TILE_SIZE),
            target_x: Num::from(x as i32 * TILE_SIZE),
            target_y: Num::from(y as i32 * TILE_SIZE),
        }
    }

    pub fn set_target(&mut self, x: u8, y: u8) {
        self.target_x = Num::from(x as i32 * TILE_SIZE);
        self.target_y = Num::from(y as i32 * TILE_SIZE);
    }

    pub fn update(&mut self) {
        let factor = Num::from_f32(0.35);
        self.pos_x += (self.target_x - self.pos_x) * factor;
        self.pos_y += (self.target_y - self.pos_y) * factor;

        if (self.target_x - self.pos_x).abs() < Num::from_f32(0.5) {
            self.pos_x = self.target_x;
        }
        if (self.target_y - self.pos_y).abs() < Num::from_f32(0.5) {
            self.pos_y = self.target_y;
        }
    }

    pub fn show(
        &self,
        graphics: &mut GraphicsFrame,
        sprites: &mut [Object; 3],
        button_size: (u8, u8),
    ) {
        draw_button_highlight(
            (self.pos_x.round(), self.pos_y.round()),
            sprites,
            button_size,
            graphics,
        );
    }
}
