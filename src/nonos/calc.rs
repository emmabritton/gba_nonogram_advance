use crate::puzzle_size::{CLUES_PER_COL, CLUES_PER_ROW, MAX_COLS, MAX_ROWS};

pub const fn parse_grids<const N: usize>(
    bytes: &[u8],
    width: usize,
    height: usize,
) -> [[[u8; MAX_COLS]; MAX_ROWS]; N] {
    let mut grids = [[[0u8; MAX_COLS]; MAX_ROWS]; N];

    let mut puzzle = 0;
    let mut y = 0;
    let mut x = 0;

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        let val = if b == b'#' {
            1u8
        } else if b == b'_' {
            0u8
        } else {
            i += 1;
            continue;
        };

        grids[puzzle][y][x] = val;

        x += 1;
        if x == width {
            x = 0;
            y += 1;
            if y == height {
                y = 0;
                puzzle += 1;
                if puzzle == N {
                    break;
                }
            }
        }

        i += 1;
    }

    grids
}

#[derive(Copy, Clone)]
pub struct Clues {
    pub rows: [[u8; CLUES_PER_ROW]; MAX_ROWS],
    pub cols: [[u8; CLUES_PER_COL]; MAX_COLS],
}

pub const fn compute_clues(grid: [[u8; MAX_COLS]; MAX_ROWS], width: usize, height: usize) -> Clues {
    let mut rows = [[0u8; CLUES_PER_ROW]; MAX_ROWS];
    let mut cols = [[0u8; CLUES_PER_COL]; MAX_COLS];

    let mut y = 0;
    while y < height {
        let mut x = 0;
        let mut run: u8 = 0;
        let mut out = 0;
        while x < width {
            if grid[y][x] != 0 {
                run += 1;
            } else if run != 0 {
                rows[y][out] = run;
                out += 1;
                run = 0;
            }
            x += 1;
        }
        if run != 0 {
            rows[y][out] = run;
        }
        y += 1;
    }

    let mut x = 0;
    while x < width {
        let mut y = 0;
        let mut run: u8 = 0;
        let mut out = 0;
        while y < height {
            if grid[y][x] != 0 {
                run += 1;
            } else if run != 0 {
                cols[x][out] = run;
                out += 1;
                run = 0;
            }
            y += 1;
        }
        if run != 0 {
            cols[x][out] = run;
        }
        x += 1;
    }

    Clues { rows, cols }
}

pub const fn compute_all_clues<const N: usize>(
    grids: [[[u8; MAX_COLS]; MAX_ROWS]; N],
    width: usize,
    height: usize,
) -> [Clues; N] {
    let mut out = [Clues {
        rows: [[0u8; CLUES_PER_ROW]; MAX_ROWS],
        cols: [[0u8; CLUES_PER_COL]; MAX_COLS],
    }; N];

    let mut i = 0;
    while i < N {
        out[i] = compute_clues(grids[i], width, height);
        i += 1;
    }

    out
}
