use crate::button_highlight::Highlight;
use crate::direction::Direction;
use crate::gfx::{TILE_SIZE, background_stack, lvl_button_sprites};
use crate::input::calc_cursor_position;
use crate::puzzle_size::PuzzleSize;
use crate::sfx::{init_bgm, play_sfx};
use crate::{
    SFX_CURSOR, SFX_MENU, SFX_NEGATIVE, SFX_POSITIVE, Scene, SceneAction, SceneMusic, bg_gfx,
    sprites,
};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, VRAM_MANAGER};
use agb::fixnum::vec2;
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

pub struct PuzzleMenuScene {
    cursor: (usize, usize),
    backgrounds: [RegularBackground; 3],
    button_highlight_sprites: [Object; 3],
    size: PuzzleSize,
    empty_sprite: Vec<Object>,
    is_completed: Vec<bool>,
    sfx_enabled: bool,
    music_enabled: bool,
    button_highlight: Highlight,
}

impl PuzzleMenuScene {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        size: PuzzleSize,
        completed_games: &[u8],
        music_enabled: bool,
        sfx_enabled: bool,
    ) -> Box<dyn Scene> {
        let is_completed: Vec<bool> = completed_games.iter().map(|&v| v > 0).collect();
        let cursor = if let Some(pos) = is_completed.iter().position(|&v| !v) {
            let x = pos % size.buttons()[0].len();
            let y = pos / size.buttons()[0].len();
            (x, y)
        } else {
            (0, 0)
        };
        let pos = size.buttons()[cursor.1][cursor.0];
        let button_highlight = Highlight::new(pos.0, pos.1);

        let mut empty_sprite = vec![];
        match size {
            PuzzleSize::_12x12 | PuzzleSize::_8x8 | PuzzleSize::_10x10 | PuzzleSize::_6x6 => {
                empty_sprite.push(Object::new(sprites::QUESTION_SQ.sprite(0)));
            }
            PuzzleSize::_20x10 | PuzzleSize::_22x12 => {
                empty_sprite.push(Object::new(sprites::QUESTION_RECT_L.sprite(0)));
                empty_sprite.push(Object::new(sprites::QUESTION_RECT_R.sprite(0)));
            }
        }

        Box::new(Self {
            cursor,
            backgrounds: background_stack([&bg_gfx::dots, size.bg(), size.bg_title()]),
            button_highlight_sprites: lvl_button_sprites(),
            size,
            empty_sprite,
            is_completed,
            sfx_enabled,
            music_enabled,
            button_highlight,
        })
    }
}

impl Scene for PuzzleMenuScene {
    fn init(
        &mut self,
        bgm: Option<(SceneMusic, ChannelId)>,
        mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)> {
        VRAM_MANAGER.set_background_palettes(bg_gfx::PALETTES);

        init_bgm(mixer, SFX_MENU, SceneMusic::Menu, bgm, self.music_enabled)
    }

    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction> {
        self.button_highlight.update();
        if buttons.is_just_pressed(Button::A) {
            play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
            let idx = self.cursor.1 * self.size.buttons()[0].len() + self.cursor.0;
            return Some(SceneAction::Game(self.size, idx));
        } else if buttons.is_just_pressed(Button::B) {
            play_sfx(mixer, self.sfx_enabled, SFX_NEGATIVE);
            return Some(SceneAction::MainMenu);
        }
        if calc_cursor_position(
            Direction::from_recent_input(buttons),
            &mut self.cursor,
            (self.size.buttons()[0].len(), self.size.buttons().len()),
            true,
        ) {
            play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
            let pos = self.size.buttons()[self.cursor.1][self.cursor.0];
            self.button_highlight.set_target(pos.0, pos.1);
            return None;
        }
        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        self.backgrounds.iter().for_each(|bg| {
            bg.show(graphics);
        });

        self.button_highlight.show(
            graphics,
            &mut self.button_highlight_sprites,
            self.size.button_size(),
        );

        for (iy, row) in self.size.buttons().iter().enumerate() {
            for (ix, (x, y)) in row.iter().enumerate() {
                let i = iy * self.size.buttons()[0].len() + ix;
                let y = (*y as i32 + 1) * TILE_SIZE;
                let start_x = (*x as i32 + 1) * TILE_SIZE;
                if self.is_completed[i] {
                    Object::new(self.size.images().sprite(i))
                        .set_pos(vec2(start_x, y))
                        .show(graphics);
                } else {
                    for (i, sprite) in self.empty_sprite.iter_mut().enumerate() {
                        let x = start_x + ((i as i32 * TILE_SIZE) * 2);
                        sprite.set_pos(vec2(x, y)).show(graphics);
                    }
                }
            }
        }
    }
}
