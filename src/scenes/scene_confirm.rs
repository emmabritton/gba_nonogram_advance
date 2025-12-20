use crate::button_highlight::Highlight;
use crate::direction::Direction;
use crate::gfx::{background_stack, button_sprites};
use crate::sfx::play_sfx;
use crate::{SFX_CURSOR, SFX_POSITIVE, Scene, SceneAction, SceneMusic, bg_gfx};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, VRAM_MANAGER};
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;
use core::mem::swap;

const BUTTON_INDEXES: [(u8, u8); 2] = [(4, 10), (18, 10)];
const BUTTON_SIZE: (u8, u8) = (7, 3);

pub struct ConfirmScene {
    button_idx: usize,
    on_positive: Box<SceneAction>,
    on_negative: Box<SceneAction>,
    backgrounds: [RegularBackground; 2],
    button_highlight_sprites: [Object; 3],
    sfx_enabled: bool,
    button_highlight: Highlight,
}

impl ConfirmScene {
    pub fn new(
        positive: Box<SceneAction>,
        negative: Box<SceneAction>,
        sfx_enabled: bool,
    ) -> Box<Self> {
        Box::new(Self {
            button_idx: 0,
            backgrounds: background_stack([&bg_gfx::dots, &bg_gfx::confirm]),
            button_highlight_sprites: button_sprites(),
            on_positive: positive,
            on_negative: negative,
            sfx_enabled,
            button_highlight: Highlight::new(BUTTON_INDEXES[0].0, BUTTON_INDEXES[0].1),
        })
    }
}

impl Scene for ConfirmScene {
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
                Direction::Left => {
                    if self.button_idx > 0 {
                        self.button_idx -= 1;
                    }
                    let pos = BUTTON_INDEXES[self.button_idx];
                    self.button_highlight.set_target(pos.0, pos.1);
                    play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                }
                Direction::Right => {
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
            match self.button_idx {
                0 => {
                    play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
                    let mut temp = Box::new(SceneAction::MainMenu);
                    swap(&mut temp, &mut self.on_positive);
                    return Some(*temp);
                }
                1 => {
                    let mut temp = Box::new(SceneAction::MainMenu);
                    swap(&mut temp, &mut self.on_negative);
                    return Some(*temp);
                }
                _ => {}
            }
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
