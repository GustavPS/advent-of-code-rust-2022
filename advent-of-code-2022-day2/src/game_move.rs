use crate::match_result::MatchResult;

#[derive(Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors
}

impl Move {
    pub fn new(c: &char) -> Result<Move, String> {
        match c {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err("Invalid character".to_string())
        }
    }

    pub fn get_winning_move_against(other: &Move) -> Move {
        match other {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock
        }
    }

    pub fn get_loosing_move_against(other: &Move) -> Move {
        match other {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper
        }
    }

    pub fn get_points(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3
        }
    }
}