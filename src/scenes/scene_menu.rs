use crate::button_highlight::Highlight;
use crate::direction::Direction;
use crate::gfx::{background_stack, button_sprites};
use crate::input::calc_cursor_position;
use crate::puzzle_size::PuzzleSize;
use crate::sfx::{init_bgm, play_sfx};
use crate::{SFX_CURSOR, SFX_MENU, SFX_POSITIVE, Scene, SceneAction, SceneMusic, bg_gfx};
use agb::display::GraphicsFrame;
use agb::display::object::Object;
use agb::display::tiled::{RegularBackground, VRAM_MANAGER};
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;

const BUTTON_INDEXES: [[(u8, u8); 3]; 2] =
    [[(2, 8), (11, 8), (20, 8)], [(2, 13), (11, 13), (20, 13)]];
const BUTTON_SIZE: (u8, u8) = (7, 3);
const BUTTON_BOARDS: [[PuzzleSize; 3]; 2] = [
    [PuzzleSize::_6x6, PuzzleSize::_8x8, PuzzleSize::_10x10],
    [PuzzleSize::_12x12, PuzzleSize::_20x10, PuzzleSize::_22x12],
];

pub struct MainMenuScene {
    cursor: (usize, usize),
    backgrounds: [RegularBackground; 2],
    button_highlight_sprites: [Object; 3],
    sfx_enabled: bool,
    music_enabled: bool,
    button_highlight: Highlight,
}

impl MainMenuScene {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(music_enabled: bool, sfx_enabled: bool) -> Box<dyn Scene> {
        let backgrounds = background_stack([&bg_gfx::dots, &bg_gfx::main]);

        Box::new(Self {
            cursor: (0, 0),
            backgrounds,
            button_highlight_sprites: button_sprites(),
            sfx_enabled,
            music_enabled,
            button_highlight: Highlight::new(BUTTON_INDEXES[0][0].0, BUTTON_INDEXES[0][0].1),
        })
    }
}

impl Scene for MainMenuScene {
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
        if calc_cursor_position(
            Direction::from_recent_input(buttons),
            &mut self.cursor,
            (BUTTON_BOARDS[0].len(), BUTTON_BOARDS.len()),
            true,
        ) {
            let pos = BUTTON_INDEXES[self.cursor.1][self.cursor.0];
            self.button_highlight.set_target(pos.0, pos.1);
            play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
        } else if buttons.is_just_pressed(Button::A) {
            play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
            let board_size = BUTTON_BOARDS[self.cursor.1][self.cursor.0];
            return Some(SceneAction::PuzzleMenu(board_size));
        } else if buttons.is_just_pressed(Button::SELECT) {
            play_sfx(mixer, self.sfx_enabled, SFX_POSITIVE);
            return Some(SceneAction::Settings);
        }
        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        self.button_highlight
            .show(graphics, &mut self.button_highlight_sprites, BUTTON_SIZE);

        self.backgrounds.iter().for_each(|bg| {
            bg.show(graphics);
        });
    }
}
