#[derive(Debug)]
pub enum MatchResult {
    Win,
    Loose,
    Draw
}

impl MatchResult {
    pub fn new(c: &char) -> Result<MatchResult, String> {
        match c {
            'X' => Ok(MatchResult::Loose),
            'Y' => Ok(MatchResult::Draw),
            'Z' => Ok(MatchResult::Win),
            _ => Err("Invalid character".to_string())
        }
    }

    pub fn get_points(&self) -> u8 {
        match self {
            MatchResult::Win => 6,
            MatchResult::Draw => 3,
            MatchResult::Loose => 0
        }
    }
}