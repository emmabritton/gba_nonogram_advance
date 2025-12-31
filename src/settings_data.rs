use crate::nonos::TOTAL_GAME_COUNT;
use crate::puzzle_size::PuzzleSize;
use agb::eprintln;
use agb::hash_map::HashMap;

const VERSION: u8 = 3;

const SAVE_IDX_VERSION: usize = 0;
const SAVE_IDX_GRID_6X6: usize = 1;
const SAVE_IDX_GRID_8X8: usize = 2;
const SAVE_IDX_GRID_10X10: usize = 3;
const SAVE_IDX_GRID_12X12: usize = 4;
const SAVE_IDX_GRID_20X10: usize = 5;
const SAVE_IDX_GRID_22X12: usize = 6;
const SAVE_IDX_MUSIC: usize = 7;
const SAVE_IDX_SFX: usize = 8;
const SAVE_IDX_HELP: usize = 9;
//const RESERVED: usize = 10;
//const RESERVED: usize = 11;
const SAVE_IDX_GAME_DATA: usize = 12;

pub const SAVE_DATA_SIZE: usize = TOTAL_GAME_COUNT + SAVE_IDX_GAME_DATA;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HelpLevel {
    None = 0,     //no help
    Zeros = 1,    //fill empty row/cols
    Full = 2,     //fill full row/cols
    Solvable = 3, //complete solvable row/cols
}

impl HelpLevel {
    pub fn from_byte(byte: u8) -> Self {
        match byte {
            1 => HelpLevel::Zeros,
            2 => HelpLevel::Full,
            3 => HelpLevel::Solvable,
            _ => HelpLevel::None,
        }
    }

    pub fn to_byte(self) -> u8 {
        match self {
            HelpLevel::None => 0,
            HelpLevel::Zeros => 1,
            HelpLevel::Full => 2,
            HelpLevel::Solvable => 3,
        }
    }

    pub const fn zeros(self) -> bool {
        matches!(
            self,
            HelpLevel::Zeros | HelpLevel::Full | HelpLevel::Solvable
        )
    }

    pub const fn full(self) -> bool {
        matches!(self, HelpLevel::Full | HelpLevel::Solvable)
    }

    pub const fn solvable(self) -> bool {
        matches!(self, HelpLevel::Solvable)
    }

    pub const fn prev(self) -> HelpLevel {
        match self {
            HelpLevel::None => panic!("prev called on None"),
            HelpLevel::Zeros => HelpLevel::None,
            HelpLevel::Full => HelpLevel::Zeros,
            HelpLevel::Solvable => HelpLevel::Full,
        }
    }

    pub const fn next(self) -> HelpLevel {
        match self {
            HelpLevel::None => HelpLevel::Zeros,
            HelpLevel::Zeros => HelpLevel::Full,
            HelpLevel::Full => HelpLevel::Solvable,
            HelpLevel::Solvable => panic!("next called on Solvable"),
        }
    }
}

pub struct SettingsData {
    pub grid_enabled: HashMap<PuzzleSize, bool>,
    pub music_enabled: bool,
    pub sfx_enabled: bool,
    completed_games: [u8; TOTAL_GAME_COUNT],
    pub help_level: HelpLevel,
}

impl SettingsData {
    pub fn from_bytes(bytes: [u8; SAVE_DATA_SIZE]) -> SettingsData {
        if bytes[SAVE_IDX_VERSION] != VERSION {
            eprintln!("Invalid save data (magic num)");
            let mut grid_enabled = HashMap::new();
            grid_enabled.insert(PuzzleSize::_6x6, true);
            grid_enabled.insert(PuzzleSize::_8x8, true);
            grid_enabled.insert(PuzzleSize::_10x10, true);
            grid_enabled.insert(PuzzleSize::_12x12, true);
            grid_enabled.insert(PuzzleSize::_20x10, true);
            grid_enabled.insert(PuzzleSize::_22x12, true);
            SettingsData {
                grid_enabled,
                music_enabled: true,
                sfx_enabled: true,
                completed_games: [0; TOTAL_GAME_COUNT],
                help_level: HelpLevel::Full,
            }
        } else {
            let mut grid_enabled = HashMap::new();
            grid_enabled.insert(PuzzleSize::_6x6, bytes[SAVE_IDX_GRID_6X6] > 0);
            grid_enabled.insert(PuzzleSize::_8x8, bytes[SAVE_IDX_GRID_8X8] > 0);
            grid_enabled.insert(PuzzleSize::_10x10, bytes[SAVE_IDX_GRID_10X10] > 0);
            grid_enabled.insert(PuzzleSize::_12x12, bytes[SAVE_IDX_GRID_12X12] > 0);
            grid_enabled.insert(PuzzleSize::_20x10, bytes[SAVE_IDX_GRID_20X10] > 0);
            grid_enabled.insert(PuzzleSize::_22x12, bytes[SAVE_IDX_GRID_22X12] > 0);
            SettingsData {
                grid_enabled,
                help_level: HelpLevel::from_byte(bytes[SAVE_IDX_HELP]),
                music_enabled: bytes[SAVE_IDX_MUSIC] > 0,
                sfx_enabled: bytes[SAVE_IDX_SFX] > 0,
                completed_games: bytes[SAVE_IDX_GAME_DATA..]
                    .try_into()
                    .expect("Invalid save data (slicing)"),
            }
        }
    }

    pub fn as_bytes(&self) -> [u8; SAVE_DATA_SIZE] {
        let mut output = [0; SAVE_DATA_SIZE];
        output[SAVE_IDX_VERSION] = VERSION;
        output[SAVE_IDX_GRID_6X6] = self.grid_enabled[&PuzzleSize::_6x6] as u8;
        output[SAVE_IDX_GRID_8X8] = self.grid_enabled[&PuzzleSize::_8x8] as u8;
        output[SAVE_IDX_GRID_10X10] = self.grid_enabled[&PuzzleSize::_10x10] as u8;
        output[SAVE_IDX_GRID_12X12] = self.grid_enabled[&PuzzleSize::_12x12] as u8;
        output[SAVE_IDX_GRID_20X10] = self.grid_enabled[&PuzzleSize::_20x10] as u8;
        output[SAVE_IDX_GRID_22X12] = self.grid_enabled[&PuzzleSize::_22x12] as u8;
        output[SAVE_IDX_MUSIC] = self.music_enabled as u8;
        output[SAVE_IDX_SFX] = self.sfx_enabled as u8;
        output[SAVE_IDX_GAME_DATA..].copy_from_slice(&self.completed_games);
        output[SAVE_IDX_HELP] = self.help_level.to_byte();
        output
    }

    pub fn set_completed(&mut self, size: PuzzleSize, idx: usize) {
        self.completed_games[size.save_idx() + idx] = 1;
    }

    pub fn is_completed_by_size(&self, size: PuzzleSize) -> &[u8] {
        &self.completed_games[size.save_idx()..size.save_idx() + size.game_count()]
    }

    pub fn reset(&mut self) {
        self.grid_enabled.insert(PuzzleSize::_6x6, true);
        self.grid_enabled.insert(PuzzleSize::_8x8, true);
        self.grid_enabled.insert(PuzzleSize::_10x10, true);
        self.grid_enabled.insert(PuzzleSize::_12x12, true);
        self.grid_enabled.insert(PuzzleSize::_20x10, true);
        self.grid_enabled.insert(PuzzleSize::_22x12, true);
        self.music_enabled = true;
        self.sfx_enabled = true;
        self.completed_games = [0; TOTAL_GAME_COUNT];
        self.help_level = HelpLevel::Full;
    }
}
