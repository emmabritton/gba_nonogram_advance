use crate::{bg_gfx, nonos};
use agb::display::object::Tag;
use agb::display::tile_data::TileData;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PuzzleSize {
    _6x6,
    _8x8,
    _10x10,
    _12x12,
    _20x10,
    _22x12,
}

pub const MAX_ROWS: usize = 12;
pub const MAX_COLS: usize = 22;
pub const CLUES_PER_ROW: usize = 8;
pub const CLUES_PER_COL: usize = 8;

#[derive(Debug)]
pub struct Puzzle {
    pub data: &'static [[u8; MAX_COLS]; MAX_ROWS],
    pub row_clues: &'static [[u8; CLUES_PER_ROW]; MAX_ROWS],
    pub col_clues: &'static [[u8; CLUES_PER_COL]; MAX_COLS],
    pub width: usize,
    pub height: usize,
}

impl PuzzleSize {
    pub fn save_idx(self) -> usize {
        match self {
            PuzzleSize::_6x6 => 0,
            PuzzleSize::_8x8 => 21,
            PuzzleSize::_10x10 => 42,
            PuzzleSize::_12x12 => 63,
            PuzzleSize::_20x10 => 84,
            PuzzleSize::_22x12 => 96,
        }
    }

    pub fn images(self) -> &'static Tag {
        match self {
            PuzzleSize::_6x6 => &nonos::_6x6::IMAGES,
            PuzzleSize::_8x8 => &nonos::_8x8::IMAGES,
            PuzzleSize::_10x10 => &nonos::_10x10::IMAGES,
            PuzzleSize::_12x12 => &nonos::_12x12::IMAGES,
            PuzzleSize::_20x10 => &nonos::_20x10::IMAGES,
            PuzzleSize::_22x12 => &nonos::_22x12::IMAGES,
        }
    }
    
    pub fn scale_in_menu(self) -> bool {
        matches!(self, PuzzleSize::_6x6 | PuzzleSize::_8x8)
    }

    pub fn game_count(self) -> usize {
        match self {
            PuzzleSize::_12x12 | PuzzleSize::_10x10 | PuzzleSize::_8x8 | PuzzleSize::_6x6 => 21,
            PuzzleSize::_20x10 | PuzzleSize::_22x12 => 12,
        }
    }

    pub fn bg_title(self) -> &'static TileData {
        match self {
            PuzzleSize::_6x6 => &bg_gfx::menu_6x6,
            PuzzleSize::_8x8 => &bg_gfx::menu_8x8,
            PuzzleSize::_10x10 => &bg_gfx::menu_10x10,
            PuzzleSize::_12x12 => &bg_gfx::menu_12x12,
            PuzzleSize::_20x10 => &bg_gfx::menu_20x10,
            PuzzleSize::_22x12 => &bg_gfx::menu_22x12,
        }
    }

    pub fn bg_game(self) -> &'static TileData {
        match self {
            PuzzleSize::_6x6 => &bg_gfx::game_6x6,
            PuzzleSize::_8x8 => &bg_gfx::game_8x8,
            PuzzleSize::_10x10 => &bg_gfx::game_10x10,
            PuzzleSize::_12x12 => &bg_gfx::game_12x12,
            PuzzleSize::_20x10 => &bg_gfx::game_20x10,
            PuzzleSize::_22x12 => &bg_gfx::game_22x12,
        }
    }

    pub fn games(self, idx: usize) -> Puzzle {
        match self {
            PuzzleSize::_6x6 => nonos::_6x6::game(idx),
            PuzzleSize::_8x8 => nonos::_8x8::game(idx),
            PuzzleSize::_10x10 => nonos::_10x10::game(idx),
            PuzzleSize::_12x12 => nonos::_12x12::game(idx),
            PuzzleSize::_20x10 => nonos::_20x10::game(idx),
            PuzzleSize::_22x12 => nonos::_22x12::game(idx),
        }
    }

    pub fn bg(self) -> &'static TileData {
        match self {
            PuzzleSize::_12x12 | PuzzleSize::_10x10 | PuzzleSize::_8x8 | PuzzleSize::_6x6 => {
                &bg_gfx::board_sq
            }
            PuzzleSize::_20x10 | PuzzleSize::_22x12 => &bg_gfx::board_rect,
        }
    }

    pub fn dimensions(self) -> (usize, usize) {
        match self {
            PuzzleSize::_6x6 => (6, 6),
            PuzzleSize::_8x8 => (8, 8),
            PuzzleSize::_10x10 => (10, 10),
            PuzzleSize::_12x12 => (12, 12),
            PuzzleSize::_20x10 => (20, 10),
            PuzzleSize::_22x12 => (22, 12),
        }
    }

    #[rustfmt::skip]
    pub fn buttons(self) -> &'static [&'static [(u8, u8)]] {
        match self {
            PuzzleSize::_12x12 | PuzzleSize::_10x10 | PuzzleSize::_8x8 | PuzzleSize::_6x6 => &[
                &[(1, 5), (5, 5), (9, 5), (13, 5), (17, 5), (21, 5), (25, 5)],
                &[(1, 10), (5, 10), (9, 10), (13, 10), (17, 10), (21, 10), (25, 10)],
                &[(1, 15), (5, 15), (9, 15), (13, 15),(17,15), (21, 15), (25, 15)],
            ],
            PuzzleSize::_20x10 | PuzzleSize::_22x12 => &[
                &[(1, 5), (8, 5), (16, 5), (23, 5)],
                &[(1, 10), (8, 10), (16, 10), (23, 10)],
                &[(1, 15), (8, 15), (16, 15), (23, 15)],
            ],
        }
    }

    pub fn button_size(self) -> (u8, u8) {
        match self {
            PuzzleSize::_12x12 | PuzzleSize::_10x10 | PuzzleSize::_8x8 | PuzzleSize::_6x6 => (3, 3),
            PuzzleSize::_20x10 | PuzzleSize::_22x12 => (5, 3),
        }
    }
}
