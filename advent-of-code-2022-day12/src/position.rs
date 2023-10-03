use crate::position::Position::{Goal, Point, Start};

// Maps characters to numbers. 'a' is 0, 'b' is 1, 'c' is 2 and so on
fn get_char_value(c: char) -> u32 {
    (c as u8 - 'a' as u8) as u32
}

const GOAL_HEIGHT: char = 'z';
const START_HEIGHT: char = 'a';

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum Position {
    Point {x: u32, y: u32, height: char},
    Goal {x: u32, y: u32},
    Start {x: u32, y: u32}
}

impl Position {
    pub fn x(&self) -> u32 {
        match self {
            Point{x, y, height} => *x,
            Goal {x, y} => *x,
            Start {x, y} => *x
        }
    }

    pub fn y(&self) -> u32 {
        match self {
            Point{x, y, height} => *y,
            Goal {x, y} => *y,
            Start {x, y} => *y
        }
    }

    pub fn height(&self) -> char {
        match self {
            Point{x, y, height} => *height,
            Goal {x, y} => GOAL_HEIGHT,
            Start {x, y} => START_HEIGHT
        }
    }

    pub fn is_goal(&self) -> bool {
        match self {
            Goal { .. } => true,
            _ => false
        }
    }

    pub fn is_start(&self) -> bool {
        match self {
            Start { .. } => true,
            _ => false
        }
    }

    pub fn can_walk_to(&self, reference_height: char) -> bool {
        match self {
            Point { x, y, height } => get_char_value(reference_height) as i32 - get_char_value(*height) as i32 >= -1,
            Goal { x, y } => get_char_value(reference_height) as i32 - get_char_value(GOAL_HEIGHT) as i32 >= -1,
            Start {x, y} => get_char_value(reference_height) as i32 - get_char_value(START_HEIGHT) as i32 >= -1,
        }
    }
}