use agb::input::{Button, ButtonController};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_input(buttons: &ButtonController) -> Option<Direction> {
        if buttons.is_pressed(Button::UP) {
            Some(Direction::Up)
        } else if buttons.is_pressed(Button::DOWN) {
            Some(Direction::Down)
        } else if buttons.is_pressed(Button::LEFT) {
            Some(Direction::Left)
        } else if buttons.is_pressed(Button::RIGHT) {
            Some(Direction::Right)
        } else {
            None
        }
    }

    pub fn from_recent_input(buttons: &ButtonController) -> Option<Direction> {
        if buttons.is_just_pressed(Button::UP) {
            Some(Direction::Up)
        } else if buttons.is_just_pressed(Button::DOWN) {
            Some(Direction::Down)
        } else if buttons.is_just_pressed(Button::LEFT) {
            Some(Direction::Left)
        } else if buttons.is_just_pressed(Button::RIGHT) {
            Some(Direction::Right)
        } else {
            None
        }
    }
}
