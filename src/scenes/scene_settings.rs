use crate::direction::Direction;
use crate::gfx::{TILE_SIZE, background_stack};
use crate::settings_button_highlight::SettingsHighlight;
use crate::settings_data::HelpLevel;
use crate::sfx::{play_sfx, stop_bgm, update_bgm};
use crate::{SFX_CURSOR, SFX_MENU, SFX_POSITIVE, Scene, SceneAction, SceneMusic, bg_gfx, sprites};
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, RegularBackgroundSize, TileFormat, VRAM_MANAGER};
use agb::display::{GraphicsFrame, Priority};
use agb::fixnum::vec2;
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;
use core::mem::swap;

const CHECKMARK_INDEXES: [(u8, u8); 2] = [(18, 6), (18, 10)];
const SETTINGS_INDEXES: [(u8, u8); 4] = [(12, 14), (15, 14), (18, 14), (21, 14)];

const IDX_SETTINGS: usize = 2;

const WARNING_COUNT_MAX: u8 = 30;
const WARNING_TIME: u8 = 8;

pub struct SettingsScene {
    button_idx: usize,
    backgrounds: [RegularBackground; 2],
    warning_bg: RegularBackground,
    warning_count: u8,
    warning_time: u8,
    music_enabled: bool,
    sfx_enabled: bool,
    bgm: Option<(SceneMusic, ChannelId)>,
    help_level: HelpLevel,
    button_gfx: [Object; 4],
    button_highlight: SettingsHighlight,
}

impl SettingsScene {
    pub fn new(music_enabled: bool, sfx_enabled: bool, help_level: HelpLevel) -> Box<Self> {
        let mut button_gfx = [
            Object::new(sprites::SETTINGS_TOP.sprite(0)),
            Object::new(sprites::SETTINGS_TOP.sprite(0)),
            Object::new(sprites::SETTINGS_BOTTOM.sprite(0)),
            Object::new(sprites::SETTINGS_BOTTOM.sprite(0)),
        ];
        button_gfx[1].set_hflip(true);
        button_gfx[3].set_hflip(true);

        let mut warning_bg = RegularBackground::new(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );
        warning_bg.fill_with(&bg_gfx::delete_save);

        Box::new(Self {
            button_idx: 0,
            backgrounds: background_stack([&bg_gfx::dots, &bg_gfx::settings]),
            music_enabled,
            sfx_enabled,
            warning_bg,
            bgm: None,
            warning_count: WARNING_COUNT_MAX,
            warning_time: 0,
            help_level,
            button_gfx,
            button_highlight: SettingsHighlight::new(
                CHECKMARK_INDEXES[0].0,
                CHECKMARK_INDEXES[0].1,
            ),
        })
    }
}

impl Scene for SettingsScene {
    fn init(
        &mut self,
        mut bgm: Option<(SceneMusic, ChannelId)>,
        mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)> {
        VRAM_MANAGER.set_background_palettes(bg_gfx::PALETTES);

        if self.music_enabled {
            if bgm.is_some() {
                swap(&mut self.bgm, &mut bgm);
            } else {
                self.bgm = Some(update_bgm(mixer, SFX_MENU, SceneMusic::Menu, None));
            }
        } else if let Some(bgm) = bgm {
            stop_bgm(mixer, bgm);
        }

        None
    }

    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction> {
        self.button_highlight.update();
        if let Some(dir) = Direction::from_recent_input(buttons) {
            match dir {
                Direction::Up => {
                    if self.button_idx > 0 {
                        self.button_idx -= 1;
                        play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                        let pos = CHECKMARK_INDEXES[self.button_idx];
                        self.button_highlight.set_target(pos.0, pos.1);
                    }
                }
                Direction::Down => {
                    if self.button_idx < IDX_SETTINGS {
                        self.button_idx += 1;
                        if self.button_idx < IDX_SETTINGS {
                            let pos = CHECKMARK_INDEXES[self.button_idx];
                            self.button_highlight.set_target(pos.0, pos.1);
                        } else {
                            let pos = SETTINGS_INDEXES[self.help_level.to_byte() as usize];
                            self.button_highlight.set_target(pos.0, pos.1);
                        }
                        play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                    }
                }
                Direction::Left => {
                    if self.button_idx == IDX_SETTINGS && self.help_level > HelpLevel::None {
                        self.help_level = self.help_level.prev();
                        let pos = SETTINGS_INDEXES[self.help_level.to_byte() as usize];
                        self.button_highlight.set_target(pos.0, pos.1);
                        play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                    }
                }
                Direction::Right => {
                    if self.button_idx == IDX_SETTINGS && self.help_level < HelpLevel::Solvable {
                        self.help_level = self.help_level.next();
                        let pos = SETTINGS_INDEXES[self.help_level.to_byte() as usize];
                        self.button_highlight.set_target(pos.0, pos.1);
                        play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                    }
                }
            }
        } else if buttons.is_just_pressed(Button::A) {
            match self.button_idx {
                0 => {
                    self.sfx_enabled = !self.sfx_enabled;
                    play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
                }
                1 => {
                    self.music_enabled = !self.music_enabled;
                    play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
                    let bgm = self.bgm.take();
                    if self.music_enabled {
                        self.bgm = Some(update_bgm(mixer, SFX_MENU, SceneMusic::Menu, bgm));
                    } else if let Some(bgm) = bgm {
                        stop_bgm(mixer, bgm);
                    }
                }
                _ => {}
            }
        } else if buttons.is_just_pressed(Button::START) {
            play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
            let bgm = self.bgm.take();
            if let Some(bgm) = bgm {
                stop_bgm(mixer, bgm);
            }
            return Some(SceneAction::SettingsClose(
                self.music_enabled,
                self.sfx_enabled,
                self.help_level,
            ));
        } else {
            #[allow(clippy::collapsible_else_if)]
            if buttons.is_pressed(Button::L | Button::R) {
                if self.warning_time == 0 {
                    self.warning_time = WARNING_TIME;
                    self.warning_count = self.warning_count.saturating_sub(1);
                    if self.warning_count == 0 {
                        let bgm = self.bgm.take();
                        if let Some(bgm) = bgm {
                            stop_bgm(mixer, bgm);
                        }
                        return Some(SceneAction::DeleteSave);
                    }
                } else {
                    self.warning_time -= 1;
                }
            } else {
                self.warning_count = WARNING_COUNT_MAX;
                self.warning_time = 0;
            }
        }
        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        if self.warning_count < WARNING_COUNT_MAX {
            self.warning_bg.show(graphics);
            let count = (WARNING_COUNT_MAX - self.warning_count) as i32;
            for x in 0..count {
                Object::new(sprites::WARNING_BLOCK.sprite(0))
                    .set_pos(vec2(x * TILE_SIZE, 11 * TILE_SIZE) - vec2(0, 2))
                    .show(graphics);
            }
        } else {
            self.backgrounds.iter().for_each(|bg| {
                bg.show(graphics);
            });

            let help_idx = self.help_level.to_byte() as usize;
            let help_pos = SETTINGS_INDEXES[help_idx];

            self.button_highlight.show(graphics, &mut self.button_gfx);

            Object::new(sprites::SETTINGS_NUMBERS.sprite(help_idx))
                .set_pos(vec2(
                    help_pos.0 as i32 * TILE_SIZE,
                    help_pos.1 as i32 * TILE_SIZE,
                ))
                .show(graphics);

            if self.sfx_enabled {
                show_checkmark(CHECKMARK_INDEXES[0], graphics);
            }
            if self.music_enabled {
                show_checkmark(CHECKMARK_INDEXES[1], graphics);
            }
        }
    }
}

fn show_checkmark(pos: (u8, u8), graphics: &mut GraphicsFrame) {
    let mut obj = Object::new(sprites::CHECKMARK.sprite(0));
    obj.set_pos(vec2(pos.0 as i32 * TILE_SIZE, pos.1 as i32 * TILE_SIZE))
        .show(graphics);
}
