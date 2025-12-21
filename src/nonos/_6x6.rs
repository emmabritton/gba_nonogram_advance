use crate::nono_images;
use crate::nonos::calc::{Clues, compute_all_clues, parse_grids};
use crate::puzzle_size::{MAX_COLS, MAX_ROWS, Puzzle};
use agb::display::object::Tag;

const N: usize = 21;
const W: usize = 6;
const H: usize = 6;

pub const IMAGES: &Tag = &nono_images::_6X6;

const GAMES: [[[u8; MAX_COLS]; MAX_ROWS]; N] =
    parse_grids::<N>(include_bytes!("../../assets/6x6.nonos"), W, H);

const CLUES: [Clues; N] = compute_all_clues(GAMES, W, H);

pub fn game(idx: usize) -> Puzzle {
    Puzzle {
        data: &GAMES[idx],
        row_clues: &CLUES[idx].rows,
        col_clues: &CLUES[idx].cols,
        width: W,
        height: H,
    }
}
