use crate::button_highlight::Highlight;
use crate::direction::Direction;
use crate::gfx::{background_stack, button_sprites};
use crate::puzzle_size::PuzzleSize;
use crate::scenes::scene_game_puzzle::Guess;
use crate::sfx::play_sfx;
use crate::{SFX_CURSOR, SFX_NEGATIVE, SFX_POSITIVE, Scene, SceneAction, SceneMusic, bg_gfx};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, VRAM_MANAGER};
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::swap;

const BUTTON_INDEXES: [(u8, u8); 2] = [(11, 7), (11, 12)];
const BUTTON_SIZE: (u8, u8) = (7, 3);

pub struct GamePauseScene {
    button_idx: usize,
    backgrounds: [RegularBackground; 2],
    button_highlight_sprites: [Object; 3],
    puzzle_size: PuzzleSize,
    game_idx: usize,
    grid_enabled: bool,
    game_data: Vec<Vec<Guess>>,
    sfx_enabled: bool,
    button_highlight: Highlight,
}

impl GamePauseScene {
    pub fn new(
        puzzle_size: PuzzleSize,
        game_idx: usize,
        grid_enabled: bool,
        game_data: Vec<Vec<Guess>>,
        sfx_enabled: bool,
    ) -> Box<Self> {
        Box::new(Self {
            button_idx: 0,
            backgrounds: background_stack([&bg_gfx::dots, &bg_gfx::pause]),
            button_highlight_sprites: button_sprites(),
            puzzle_size,
            grid_enabled,
            game_data,
            game_idx,
            sfx_enabled,
            button_highlight: Highlight::new(BUTTON_INDEXES[0].0, BUTTON_INDEXES[0].1),
        })
    }
}

impl Scene for GamePauseScene {
    fn init(
        &mut self,
        bgm: Option<(SceneMusic, ChannelId)>,
        _mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)> {
        VRAM_MANAGER.set_background_palettes(bg_gfx::PALETTES);

        bgm
    }

    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction> {
        self.button_highlight.update();
        if let Some(dir) = Direction::from_recent_input(buttons) {
            match dir {
                Direction::Up => {
                    if self.button_idx > 0 {
                        self.button_idx -= 1;
                    }
                    let pos = BUTTON_INDEXES[self.button_idx];
                    self.button_highlight.set_target(pos.0, pos.1);
                    play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                }
                Direction::Down => {
                    if self.button_idx < 1 {
                        self.button_idx += 1;
                    }
                    let pos = BUTTON_INDEXES[self.button_idx];
                    self.button_highlight.set_target(pos.0, pos.1);
                    play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                }
                _ => {}
            }
        } else if buttons.is_just_pressed(Button::A) {
            let mut empty = vec![vec![]];
            swap(&mut empty, &mut self.game_data);
            let negative = Box::new(SceneAction::RestoreGame(
                self.puzzle_size,
                self.game_idx,
                self.grid_enabled,
                empty,
            ));
            play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
            let positive = match self.button_idx {
                0 => SceneAction::Game(self.puzzle_size, self.game_idx),
                1 => SceneAction::PuzzleMenu(self.puzzle_size),
                _ => panic!("invalid button_idx in pause: {}", self.button_idx),
            };
            return Some(SceneAction::Confirm(Box::new(positive), negative));
        } else if buttons.is_just_pressed(Button::START) {
            play_sfx(mixer, self.sfx_enabled, SFX_NEGATIVE);
            let mut empty = vec![vec![]];
            swap(&mut empty, &mut self.game_data);
            return Some(SceneAction::RestoreGame(
                self.puzzle_size,
                self.game_idx,
                self.grid_enabled,
                empty,
            ));
        }

        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        self.backgrounds.iter().for_each(|bg| {
            bg.show(graphics);
        });

        self.button_highlight
            .show(graphics, &mut self.button_highlight_sprites, BUTTON_SIZE);
    }
}
