use crate::direction::Direction;
use crate::gfx::TILE_SIZE;
use crate::input::calc_cursor_position;
use crate::puzzle_size::{CLUES_PER_COL, CLUES_PER_ROW, MAX_COLS, MAX_ROWS, Puzzle};
use crate::settings_data::HelpLevel;
use crate::sfx::{init_bgm, play_sfx};
use crate::{PuzzleSize, SFX_CURSOR, SFX_GAME, Scene, SceneAction, SceneMusic, bg_gfx, sprites};
use agb::display::object::{GraphicsMode, Object, Sprite};
use agb::display::tiled::{
    RegularBackground, RegularBackgroundSize, TileEffect, TileFormat, TileSetting, VRAM_MANAGER,
};
use agb::display::{GraphicsFrame, Priority};
use agb::fixnum::{Num, vec2};
use agb::input::{Button, ButtonController};
use agb::sound::mixer::{ChannelId, Mixer};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use core::mem::swap;

const INPUT_DELAY: u8 = 6;

const BOARD_OFFSET: (i32, i32) = (8, 8);
const TEXT_OFFSET: (i32, i32) = (1, 4);
const TEXT_PX_OFFSET: (i32, i32) = (3, 1);
const PER_LINE_OFFSET: (i32, i32) = (0, 2);

const FIRST_COL_CLUE_POS: (usize, usize) = (8, 7);
const FIRST_ROW_CLUE_POS: (usize, usize) = (7, 8);

const NUMBERS_DEFAULT: usize = 0;
const NUMBERS_COMPLETE: usize = 23;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum GameState {
    WaitingForNoInput,
    Playing,
    Win,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Guess {
    Empty,
    Filled,
    Marked,
}

impl Guess {
    pub fn index(self) -> u16 {
        match self {
            Guess::Empty => 0,
            Guess::Filled => 1,
            Guess::Marked => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct Clock {
    frames: u8,
    seconds: u8,
    minutes: u8,
}

impl Clock {
    pub fn inc(&mut self) {
        self.frames += 1;
        if self.frames > 59 {
            self.seconds += 1;
            self.frames = 0;
            if self.seconds > 59 {
                self.seconds = 0;
                self.minutes += 1;
            }
        }
    }

    pub fn show(&self, graphics: &mut GraphicsFrame) {
        format!("{:02}:{:02}", self.minutes, self.seconds)
            .chars()
            .map(char_to_sprite)
            .map(Object::new)
            .enumerate()
            .for_each(|(x, mut obj)| {
                obj.set_pos(vec2(
                    (x as i32 + TEXT_OFFSET.0) * TILE_SIZE + TEXT_PX_OFFSET.0 + PER_LINE_OFFSET.0,
                    (2 + TEXT_OFFSET.1) * TILE_SIZE + TEXT_PX_OFFSET.1 + (PER_LINE_OFFSET.1 * 2),
                ))
                .show(graphics);
            })
    }
}

pub struct GamePuzzleScene {
    cursor: (usize, usize),
    background_hints: RegularBackground,
    background_title: RegularBackground,
    background_pieces: RegularBackground,
    background_grid: RegularBackground,
    cursor_sprite: Object,
    puzzle_size: PuzzleSize,
    next_input_frame: u8,
    puzzle: Puzzle,
    guesses: Vec<Vec<Guess>>,
    drag_mode: Option<(Guess, Guess)>,
    state: GameState,
    game_idx: usize,
    text: Vec<Vec<&'static Sprite>>,
    show_grid: bool,
    music_enabled: bool,
    sfx_enabled: bool,
    block: Object,
    clock: Clock,
    last_dpad: u8,
    row_complete: Vec<bool>,
    col_complete: Vec<bool>,
}

impl GamePuzzleScene {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        puzzle_size: PuzzleSize,
        game_idx: usize,
        init_game_data: Option<Vec<Vec<Guess>>>,
        grid_enabled: bool,
        music_enabled: bool,
        sfx_enabled: bool,
        help_level: HelpLevel,
    ) -> Box<dyn Scene> {
        let mut background_hints = RegularBackground::new(
            Priority::P3,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        background_hints.fill_with(puzzle_size.bg_game());

        let mut background_title = RegularBackground::new(
            Priority::P2,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        background_title.fill_with(&bg_gfx::title);

        let cursor_sprite = Object::new(sprites::SELECTOR.sprite(0));

        let mut background_pieces = RegularBackground::new(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            TileFormat::FourBpp,
        );

        let background_grid = build_grid(puzzle_size);

        let mut block = Object::new(sprites::BLOCK.sprite(0));
        block.set_graphics_mode(GraphicsMode::AlphaBlending);

        let guesses = if let Some(data) = init_game_data {
            if data.len() == puzzle_size.dimensions().1
                && data[0].len() == puzzle_size.dimensions().0
            {
                data
            } else {
                panic!(
                    "invalid restore game data got {}x{} for {:?} {}",
                    data.len(),
                    data[0].len(),
                    puzzle_size,
                    game_idx
                )
            }
        } else {
            let data =
                vec![vec![Guess::Empty; puzzle_size.dimensions().0]; puzzle_size.dimensions().1];
            pre_solve(
                data,
                help_level,
                puzzle_size.games(game_idx).row_clues,
                puzzle_size.games(game_idx).col_clues,
            )
        };

        for (y, row) in guesses.iter().enumerate() {
            for (x, &guess) in row.iter().enumerate() {
                set_piece(&mut background_pieces, (x, y), guess);
            }
        }

        let text = build_text(puzzle_size, game_idx);

        let col_complete = vec![false; puzzle_size.dimensions().0];
        let row_complete = vec![false; puzzle_size.dimensions().1];

        Box::new(Self {
            cursor: (0, 0),
            background_hints,
            background_title,
            background_grid,
            background_pieces,
            cursor_sprite,
            next_input_frame: 0,
            puzzle_size,
            block,
            guesses,
            game_idx,
            puzzle: puzzle_size.games(game_idx),
            drag_mode: None,
            state: GameState::WaitingForNoInput,
            text,
            show_grid: grid_enabled,
            music_enabled,
            sfx_enabled,
            clock: Clock::default(),
            last_dpad: 0,
            row_complete,
            col_complete,
        })
    }
}

impl GamePuzzleScene {
    fn set_piece(&mut self, guess: Guess) {
        if self.drag_mode.is_none()
            || self.drag_mode == Some((self.guesses[self.cursor.1][self.cursor.0], guess))
        {
            self.drag_mode = Some((self.guesses[self.cursor.1][self.cursor.0], guess));
            self.guesses[self.cursor.1][self.cursor.0] = guess;
            set_piece(&mut self.background_pieces, self.cursor, guess);
            self.refresh_row_clue(self.cursor.1);
            self.refresh_col_clue(self.cursor.0);
        }
    }

    fn refresh_row_clue(&mut self, y: usize) {
        let complete = self.is_row_complete(y);
        if self.row_complete[y] == complete {
            return;
        }
        self.row_complete[y] = complete;
        self.redraw_row_clue(y, complete);
    }

    fn refresh_col_clue(&mut self, x: usize) {
        let complete = self.is_col_complete(x);
        if self.col_complete[x] == complete {
            return;
        }
        self.col_complete[x] = complete;
        self.redraw_col_clue(x, complete);
    }

    fn validate(&mut self) {
        for x in 0..self.puzzle.width {
            for y in 0..self.puzzle.height {
                if self.puzzle.data[y][x] > 0 && self.guesses[y][x] != Guess::Filled {
                    return;
                }
            }
        }
        self.state = GameState::Win;
    }

    fn is_row_complete(&self, y: usize) -> bool {
        if self.puzzle.row_clues[y].iter().all(|&c| c == 0) {
            return true;
        }

        line_matches_hints(self.guesses[y].iter().copied(), &self.puzzle.row_clues[y])
    }

    fn is_col_complete(&self, x: usize) -> bool {
        if self.puzzle.col_clues[x].iter().all(|&c| c == 0) {
            return true;
        }

        let h = self.puzzle.height;
        line_matches_hints(
            (0..h).map(|y| self.guesses[y][x]),
            &self.puzzle.col_clues[x],
        )
    }

    fn redraw_col_clue(&mut self, x: usize, complete: bool) {
        let base = if complete {
            NUMBERS_COMPLETE
        } else {
            NUMBERS_DEFAULT
        };

        let tile_x = (FIRST_COL_CLUE_POS.0 + x) as i32;

        if self.puzzle.col_clues[x].iter().all(|&c| c == 0) {
            set_number_variant(&mut self.background_pieces, (tile_x, FIRST_COL_CLUE_POS.1 as i32), 0, base);
            return
        }

        let mut count = 0;
        for &num in self.puzzle.col_clues[x].iter().rev() {
            if num == 0 {
                continue;
            }
            let tile_y = FIRST_COL_CLUE_POS.1 as i32 - count;
            set_number_variant(&mut self.background_pieces, (tile_x, tile_y), num, base);
            count += 1;
        }
    }

    fn redraw_row_clue(&mut self, y: usize, complete: bool) {
        let base = if complete {
            NUMBERS_COMPLETE
        } else {
            NUMBERS_DEFAULT
        };

        let tile_y = (FIRST_ROW_CLUE_POS.1 + y) as i32;

        if self.puzzle.row_clues[y].iter().all(|&c| c == 0) {
            set_number_variant(&mut self.background_pieces, (FIRST_ROW_CLUE_POS.0 as i32, tile_y), 0, base);
            return
        }

        let mut count = 0;
        for &num in self.puzzle.row_clues[y].iter().rev() {
            if num == 0 {
                continue;
            }
            let tile_x = FIRST_ROW_CLUE_POS.0 as i32 - count;
            set_number_variant(&mut self.background_pieces, (tile_x, tile_y), num, base);
            count += 1;
        }
    }
}

fn line_runs<I>(line: I) -> Vec<usize>
where
    I: IntoIterator<Item = Guess>,
{
    let mut runs = Vec::new();
    let mut current = 0usize;
    for g in line {
        if g == Guess::Filled {
            current += 1;
        } else if current > 0 {
            runs.push(current);
            current = 0;
        }
    }
    if current > 0 {
        runs.push(current);
    }
    runs
}

fn expected_hints(hints: &[u8]) -> Vec<usize> {
    hints
        .iter()
        .take_while(|&&n| n != 0)
        .map(|&n| n as usize)
        .collect()
}

fn line_matches_hints<I>(line: I, hints: &[u8]) -> bool
where
    I: IntoIterator<Item = Guess>,
{
    let runs = line_runs(line);
    let exp = expected_hints(hints);
    runs == exp
}

fn set_number_variant(background: &mut RegularBackground, pos: (i32, i32), num: u8, base: usize) {
    background.set_tile(
        pos,
        &bg_gfx::numbers.tiles,
        TileSetting::new((base + num as usize) as u16, TileEffect::default()),
    );
}

impl Scene for GamePuzzleScene {
    fn init(
        &mut self,
        bgm: Option<(SceneMusic, ChannelId)>,
        mixer: &mut Mixer,
    ) -> Option<(SceneMusic, ChannelId)> {
        VRAM_MANAGER.set_background_palettes(bg_gfx::PALETTES);

        for y in 0..self.puzzle.height {
            self.row_complete[y] = self.is_row_complete(y);
            self.redraw_row_clue(y, self.row_complete[y]);
        }
        for x in 0..self.puzzle.width {
            self.col_complete[x] = self.is_col_complete(x);
            self.redraw_col_clue(x, self.col_complete[x]);
        }

        init_bgm(mixer, SFX_GAME, SceneMusic::Game, bgm, self.music_enabled)
    }

    fn update(&mut self, buttons: &ButtonController, mixer: &mut Mixer) -> Option<SceneAction> {
        self.clock.inc();
        match self.state {
            GameState::WaitingForNoInput => {
                if buttons.is_released(Button::A | Button::B) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Playing => {
                let dpad: u8 = (buttons.is_pressed(Button::UP) as u8)
                    | ((buttons.is_pressed(Button::DOWN) as u8) << 1)
                    | ((buttons.is_pressed(Button::LEFT) as u8) << 2)
                    | ((buttons.is_pressed(Button::RIGHT) as u8) << 3);

                if dpad != self.last_dpad {
                    self.next_input_frame = 0;
                    self.last_dpad = dpad;
                }

                let a_down = buttons.is_pressed(Button::A);
                let b_down = buttons.is_pressed(Button::B);

                if self.drag_mode.is_none() {
                    if a_down {
                        if self.guesses[self.cursor.1][self.cursor.0] == Guess::Filled {
                            self.set_piece(Guess::Empty);
                        } else {
                            self.set_piece(Guess::Filled);
                        }
                    } else if b_down {
                        if self.guesses[self.cursor.1][self.cursor.0] == Guess::Marked {
                            self.set_piece(Guess::Empty);
                        } else {
                            self.set_piece(Guess::Marked);
                        }
                    }
                }

                let mut moved = false;
                if dpad != 0 {
                    if self.next_input_frame == 0 {
                        moved = calc_cursor_position(
                            Direction::from_input(buttons),
                            &mut self.cursor,
                            self.puzzle_size.dimensions(),
                            self.drag_mode.is_none(),
                        );
                        if moved {
                            play_sfx(mixer, self.sfx_enabled, SFX_CURSOR);
                            self.next_input_frame = INPUT_DELAY;
                        }
                    } else {
                        self.next_input_frame -= 1;
                    }
                } else {
                    self.next_input_frame = 0;
                }

                if moved && let Some((_, target)) = self.drag_mode {
                    self.set_piece(target);
                }

                if buttons.is_just_pressed(Button::SELECT) {
                    self.show_grid = !self.show_grid;
                }

                if buttons.is_just_pressed(Button::START) {
                    let mut empty = vec![vec![]];
                    swap(&mut empty, &mut self.guesses);
                    return Some(SceneAction::PauseMenu(
                        self.puzzle_size,
                        self.game_idx,
                        self.show_grid,
                        empty,
                    ));
                }

                if !a_down && !b_down {
                    self.drag_mode = None;
                }

                self.validate();
            }
            GameState::Win => {
                return Some(SceneAction::Win(
                    self.puzzle_size,
                    self.game_idx,
                    self.show_grid,
                ));
            }
        }
        None
    }

    fn show(&mut self, graphics: &mut GraphicsFrame) {
        let id = self.background_hints.show(graphics);
        self.background_title.show(graphics);
        if self.show_grid {
            self.background_grid.show(graphics);
        }
        self.background_pieces.show(graphics);

        let pos = (
            (self.cursor.0 as i32 + BOARD_OFFSET.0) * TILE_SIZE,
            (self.cursor.1 as i32 + BOARD_OFFSET.1) * TILE_SIZE,
        );
        self.cursor_sprite.set_pos(pos).show(graphics);

        let highlight_x = (self.cursor.0 as i32 + BOARD_OFFSET.0) * TILE_SIZE;
        let highlight_y = (self.cursor.1 as i32 + BOARD_OFFSET.1) * TILE_SIZE;
        for i in 0..8 {
            let x = i * TILE_SIZE;
            self.block.set_pos((x, highlight_y)).show(graphics);
            let y = i * TILE_SIZE;
            self.block.set_pos((highlight_x, y)).show(graphics);
        }

        graphics
            .blend()
            .object_transparency(Num::from_f32(0.25), Num::from_f32(0.75))
            .enable_background(id);

        self.clock.show(graphics);

        for (y, line) in self.text.iter().enumerate() {
            for (x, &sprite) in line.iter().enumerate() {
                Object::new(sprite)
                    .set_pos(vec2(
                        (x as i32 + TEXT_OFFSET.0) * TILE_SIZE
                            + TEXT_PX_OFFSET.0
                            + (PER_LINE_OFFSET.0 * y as i32),
                        (y as i32 + TEXT_OFFSET.1) * TILE_SIZE
                            + TEXT_PX_OFFSET.1
                            + (PER_LINE_OFFSET.1 * y as i32),
                    ))
                    .show(graphics);
            }
        }
    }
}

fn build_grid(board_size: PuzzleSize) -> RegularBackground {
    let mut background = RegularBackground::new(
        Priority::P0,
        RegularBackgroundSize::Background32x32,
        TileFormat::FourBpp,
    );

    match board_size {
        PuzzleSize::_6x6 => {}
        PuzzleSize::_8x8 => {
            background.fill_with(&bg_gfx::grid_8x8);
        }
        PuzzleSize::_10x10 => {
            background.fill_with(&bg_gfx::grid_10x10);
        }
        PuzzleSize::_12x12 => {
            background.fill_with(&bg_gfx::grid_12x12);
        }
        PuzzleSize::_20x10 => {
            background.fill_with(&bg_gfx::grid_20x10);
        }
        PuzzleSize::_22x12 => {
            background.fill_with(&bg_gfx::grid_22x12);
        }
    }

    background
}

fn set_piece(background: &mut RegularBackground, pos: (usize, usize), guess: Guess) {
    background.set_tile(
        (pos.0 as i32 + BOARD_OFFSET.0, pos.1 as i32 + BOARD_OFFSET.1),
        &bg_gfx::pieces.tiles,
        TileSetting::new(guess.index(), TileEffect::default()),
    );
}

fn build_text(board_size: PuzzleSize, game_idx: usize) -> Vec<Vec<&'static Sprite>> {
    let mut text = Vec::new();

    let mut row1 = vec![];
    let mut row2 = vec![];

    let process_str = |s: String, v: &mut Vec<&Sprite>| {
        s.chars()
            .map(char_to_sprite)
            .for_each(|sprite| v.push(sprite));
    };

    process_str(board_size.dimensions().0.to_string(), &mut row1);
    row1.push(char_to_sprite('x'));
    process_str(board_size.dimensions().1.to_string(), &mut row1);

    process_str((game_idx + 1).to_string(), &mut row2);

    text.push(row1);
    text.push(row2);
    text
}

fn char_to_sprite(chr: char) -> &'static Sprite {
    match chr {
        '0'..='9' => sprites::NUMBERS.sprite((chr as u8 - b'0') as usize),
        'x' => sprites::X.sprite(0),
        '#' => sprites::OCTO.sprite(0),
        ':' => sprites::COLON.sprite(0),
        _ => sprites::UNKNOWN.sprite(0),
    }
}

fn pre_solve(
    mut data: Vec<Vec<Guess>>,
    help_level: HelpLevel,
    rows_hints: &[[u8; CLUES_PER_ROW]; MAX_ROWS],
    cols_hints: &[[u8; CLUES_PER_COL]; MAX_COLS],
) -> Vec<Vec<Guess>> {
    if help_level == HelpLevel::None {
        return data;
    }

    let height = data.len();
    let width = data[0].len();

    for (y, row) in data.iter_mut().enumerate().take(height) {
        let first = rows_hints[y][0];
        if help_level.zeros() && first == 0 {
            row.fill(Guess::Marked);
        } else if help_level.full() && first == width as u8 {
            row.fill(Guess::Filled);
        } else if help_level.solvable() && hint_calc(&rows_hints[y]) == width {
            solve_row(&rows_hints[y], row, width);
        }
    }

    for (x, col_hints) in cols_hints.iter().enumerate().take(width) {
        let first = col_hints[0];
        if help_level.zeros() && first == 0 {
            fill_col(&mut data, x, Guess::Marked);
        } else if help_level.full() && first == height as u8 {
            fill_col(&mut data, x, Guess::Filled);
        } else if help_level.solvable() && hint_calc(col_hints) == width {
            solve_col(&mut data, x, col_hints, height);
        }
    }

    data
}

fn fill_col(data: &mut [Vec<Guess>], x: usize, guess: Guess) {
    data.iter_mut().for_each(|row| row[x] = guess);
}

fn hint_calc(hints: &[u8]) -> usize {
    let mut sum = 0;
    let mut blocks = 0;
    for &n in hints {
        if n == 0 {
            break;
        }
        sum += n as usize;
        blocks += 1;
    }

    sum + blocks - 1
}

fn solve_row(hints: &[u8], row: &mut [Guess], width: usize) {
    let mut offset = 0;
    for &n in hints {
        if n == 0 {
            break;
        }
        let n = n as usize;
        for slot in row.iter_mut().skip(offset).take(n) {
            *slot = Guess::Filled;
        }
        offset += n;

        if offset < width {
            row[offset] = Guess::Marked;
            offset += 1;
        }
    }
}

fn solve_col(data: &mut [Vec<Guess>], x: usize, hints: &[u8], height: usize) {
    let mut offset = 0usize;
    for &n in hints.iter() {
        if n == 0 {
            break;
        }
        let n = n as usize;
        for slot in data.iter_mut().skip(offset).take(n) {
            slot[x] = Guess::Filled;
        }
        offset += n;
        if offset < height {
            data[offset][x] = Guess::Marked;
            offset += 1;
        }
    }
}
