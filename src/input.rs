use crate::direction::Direction;

pub fn calc_cursor_position(
    dir: Option<Direction>,
    cursor: &mut (usize, usize),
    size: (usize, usize),
    allow_wrapping: bool,
) -> bool {
    if let Some(dir) = dir {
        return match dir {
            Direction::Up => {
                if cursor.1 > 0 {
                    cursor.1 -= 1;
                } else if allow_wrapping {
                    cursor.1 = size.1 - 1;
                }
                true
            }
            Direction::Down => {
                if cursor.1 < size.1 - 1 {
                    cursor.1 += 1;
                } else if allow_wrapping {
                    cursor.1 = 0;
                }
                true
            }
            Direction::Left => {
                if cursor.0 > 0 {
                    cursor.0 -= 1;
                } else if allow_wrapping {
                    cursor.0 = size.0 - 1;
                }
                true
            }
            Direction::Right => {
                if cursor.0 < size.0 - 1 {
                    cursor.0 += 1;
                } else if allow_wrapping {
                    cursor.0 = 0;
                }
                true
            }
        };
    }
    false
}
