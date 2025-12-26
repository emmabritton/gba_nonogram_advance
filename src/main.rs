#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

mod button_highlight;
mod direction;
mod gfx;
mod input;
mod nonos;
mod puzzle_size;
mod scenes;
mod settings_button_highlight;
mod settings_data;
mod sfx;

extern crate alloc;

use crate::puzzle_size::PuzzleSize;
use crate::scenes::scene_confirm::ConfirmScene;
use crate::scenes::scene_game_pause::GamePauseScene;
use crate::scenes::scene_game_puzzle::{GamePuzzleScene, Guess};
use crate::scenes::scene_game_win::GameWinScene;
use crate::scenes::scene_menu::MainMenuScene;
use crate::scenes::scene_puzzle_menu::PuzzleMenuScene;
use crate::scenes::scene_settings::SettingsScene;
use crate::settings_data::{HelpLevel, SAVE_DATA_SIZE, SettingsData};
use crate::sfx::start_track;
use agb::display::GraphicsFrame;
use agb::input::ButtonController;
use agb::sound::mixer::{ChannelId, Frequency, Mixer, SoundData};
use agb::{include_aseprite, include_background_gfx, include_wav};
use alloc::boxed::Box;
use alloc::vec::Vec;

static SFX_CURSOR: SoundData = include_wav!("sfx/cursor.wav");
static SFX_POSITIVE: SoundData = include_wav!("sfx/positive.wav");
static SFX_NEGATIVE: SoundData = include_wav!("sfx/negative.wav");
static SFX_CONGRATS: SoundData = include_wav!("sfx/congrats.wav");
static SFX_GAME: SoundData = include_wav!("sfx/game.wav");
static SFX_MENU: SoundData = include_wav!("sfx/menu.wav");

include_aseprite!(
    mod sprites,
    "gfx/common/sprite/sprites.aseprite",
    "gfx/game/sprite/number.aseprite",
    "gfx/menu/sprite/questionmark.aseprite",
    "gfx/game/sprite/congrats.aseprite",
    "gfx/menu/sprite/numbers.aseprite",
);

include_aseprite!(
    mod sq_nono_images,
    "gfx/game/sprite/nono_6x6.aseprite",
    "gfx/game/sprite/nono_8x8.aseprite",
    "gfx/game/sprite/nono_10x10.aseprite",
    "gfx/game/sprite/nono_12x12.aseprite",
);

include_aseprite!(
    mod rect_nono_images,
    "gfx/game/sprite/nono_20x10.aseprite",
    "gfx/game/sprite/nono_22x12.aseprite",
);

include_background_gfx!(
    mod bg_gfx,
    main => deduplicate "gfx/menu/bg/main.aseprite",
    board_sq => deduplicate "gfx/menu/bg/square.aseprite",
    board_rect => deduplicate "gfx/menu/bg/rect.aseprite",
    menu_6x6 => deduplicate "gfx/menu/bg/6x6.aseprite",
    menu_8x8 => deduplicate "gfx/menu/bg/8x8.aseprite",
    menu_10x10 => deduplicate "gfx/menu/bg/10x10.aseprite",
    menu_12x12 => deduplicate "gfx/menu/bg/12x12.aseprite",
    menu_20x10 => deduplicate "gfx/menu/bg/20x10.aseprite",
    menu_22x12 => deduplicate "gfx/menu/bg/22x12.aseprite",
    settings => deduplicate "gfx/menu/bg/settings.aseprite",
    dots => deduplicate "gfx/menu/bg/dots_bak.aseprite",
    polka => deduplicate "gfx/menu/bg/polka.aseprite",
    title => deduplicate "gfx/game/bg/title.aseprite",
    numbers => "gfx/game/bg/numbers.aseprite",
    game_6x6 => deduplicate "gfx/game/bg/6x6.aseprite",
    game_8x8 => deduplicate "gfx/game/bg/8x8.aseprite",
    game_10x10 => deduplicate "gfx/game/bg/10x10.aseprite",
    game_12x12 => deduplicate "gfx/game/bg/12x12.aseprite",
    game_20x10 => deduplicate "gfx/game/bg/20x10.aseprite",
    game_22x12 => deduplicate "gfx/game/bg/22x12.aseprite",
    grid_8x8 => deduplicate "gfx/game/bg/grid_8x8.aseprite",
    grid_10x10 => deduplicate "gfx/game/bg/grid_10x10.aseprite",
    grid_12x12 => deduplicate "gfx/game/bg/grid_12x12.aseprite",
    grid_20x10 => deduplicate "gfx/game/bg/grid_20x10.aseprite",
    grid_22x12 => deduplicate "gfx/game/bg/grid_22x12.aseprite",
    pause => deduplicate "gfx/game/bg/pause.aseprite",
    win => deduplicate "gfx/game/bg/win.aseprite",
    confirm => deduplicate "gfx/game/bg/confirm.aseprite",
    pieces => deduplicate "gfx/game/bg/board_pieces.aseprite",
);

#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    gba.save.init_sram();
    let mut mixer = gba.mixer.mixer(Frequency::Hz18157);

    let mut settings_data = match gba.save.access() {
        Ok(mut save_data) => {
            let mut save_bytes = [0_u8; SAVE_DATA_SIZE];
            if let Err(e) = save_data.read(0, &mut save_bytes) {
                panic!("Save read error: {:?}", e);
            }
            SettingsData::from_bytes(save_bytes)
        }
        Err(e) => {
            panic!("Save access error: {:?} (access)", e);
        }
    };

    let mut scene: Box<dyn Scene> =
        MainMenuScene::new(settings_data.music_enabled, settings_data.sfx_enabled);

    let mut gfx = gba.graphics.get();
    let mut button_controller = ButtonController::new();

    let mut bgm = Some(start_track(&mut mixer, SFX_MENU, SceneMusic::Menu));

    bgm = scene.init(bgm, &mut mixer);

    loop {
        let mut frame = gfx.frame();
        button_controller.update();

        if let Some(result) = scene.update(&button_controller, &mut mixer) {
            match result {
                SceneAction::Win(size, idx, grid_enabled) => {
                    if let Ok(mut save_data) = gba.save.access()
                        && let Ok(mut writer) = save_data.prepare_write(0..SAVE_DATA_SIZE)
                    {
                        settings_data.grid_enabled.insert(size, grid_enabled);
                        settings_data.set_completed(size, idx);
                        if let Err(e) = writer.write(0, &settings_data.as_bytes()) {
                            panic!("(win) Save write error: {:?}", e);
                        }
                    }
                    scene = GameWinScene::new(size, idx, settings_data.music_enabled);
                }
                SceneAction::MainMenu => {
                    scene =
                        MainMenuScene::new(settings_data.music_enabled, settings_data.sfx_enabled);
                }
                SceneAction::SettingsClose(music, sfx, help_level) => {
                    settings_data.music_enabled = music;
                    settings_data.sfx_enabled = sfx;
                    settings_data.help_level = help_level;
                    if let Ok(mut save_data) = gba.save.access()
                        && let Ok(mut writer) = save_data.prepare_write(0..SAVE_DATA_SIZE)
                        && let Err(e) = writer.write(0, &settings_data.as_bytes())
                    {
                        panic!("(settings) Save write error: {:?}", e);
                    }
                    scene =
                        MainMenuScene::new(settings_data.music_enabled, settings_data.sfx_enabled);
                }
                SceneAction::PuzzleMenu(size) => {
                    scene = PuzzleMenuScene::new(
                        size,
                        settings_data.is_completed_by_size(size),
                        settings_data.music_enabled,
                        settings_data.sfx_enabled,
                    );
                }
                SceneAction::Game(size, idx) => {
                    scene = GamePuzzleScene::new(
                        size,
                        idx,
                        None,
                        *settings_data
                            .grid_enabled
                            .get(&size)
                            .unwrap_or_else(|| panic!("size missing: {size:?}")),
                        settings_data.music_enabled,
                        settings_data.sfx_enabled,
                        settings_data.help_level,
                    );
                }
                SceneAction::RestoreGame(size, idx, grid_enabled, game_data) => {
                    scene = GamePuzzleScene::new(
                        size,
                        idx,
                        Some(game_data),
                        grid_enabled,
                        settings_data.music_enabled,
                        settings_data.sfx_enabled,
                        settings_data.help_level,
                    );
                }
                SceneAction::PauseMenu(size, idx, grid_enabled, game_data) => {
                    scene = GamePauseScene::new(
                        size,
                        idx,
                        grid_enabled,
                        game_data,
                        settings_data.sfx_enabled,
                    );
                }
                SceneAction::Confirm(positive, negative) => {
                    scene = ConfirmScene::new(positive, negative, settings_data.sfx_enabled);
                }
                SceneAction::Settings => {
                    scene = SettingsScene::new(
                        settings_data.music_enabled,
                        settings_data.sfx_enabled,
                        settings_data.help_level,
                    );
                }
            }
            bgm = scene.init(bgm, &mut mixer);
        }

        scene.show(&mut frame);

        mixer.frame();
        frame.commit();
    }
}

trait Scene {
    fn init(
        &mut self,
        bgm: Option<(SceneMusic, ChannelId)>,
        mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)>;
    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction>;
    fn show(&mut self, graphics: &mut GraphicsFrame);
}

#[derive(Debug, Eq, PartialEq)]
enum SceneAction {
    Win(PuzzleSize, usize, bool), //puzzle size, game idx, grid enabled
    MainMenu,
    PuzzleMenu(PuzzleSize),
    Game(PuzzleSize, usize), //puzzle size, game idx
    RestoreGame(PuzzleSize, usize, bool, Vec<Vec<Guess>>), //puzzle size, game idx, grid enabled, game data
    PauseMenu(PuzzleSize, usize, bool, Vec<Vec<Guess>>), //puzzle size, game idx, grid enabled, game data
    Confirm(Box<SceneAction>, Box<SceneAction>), //action to send if positive, action to send if negative
    Settings,
    SettingsClose(bool, bool, HelpLevel), //music enabled, sfx enabled, help level
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum SceneMusic {
    Menu,
    Game,
}
