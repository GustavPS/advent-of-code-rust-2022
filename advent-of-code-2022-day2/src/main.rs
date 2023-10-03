
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::game_move::Move;
use crate::match_result::MatchResult;

mod game_move;
mod match_result;


fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day2/src/data.txt") {
        let mut total_points: u32 = 0;
        for mut line in lines {
            if let Ok(battle) = line {
                let battle = remove_whitespace(&battle);
                let opponent = Move::new(&battle.chars().nth(0).unwrap()).unwrap();
                let needed_result = MatchResult::new(&battle.chars().nth(1).unwrap()).unwrap();
                print!("opponent: {:?}", opponent);

                let me = match needed_result {
                    MatchResult::Win => Move::get_winning_move_against(&opponent),
                    MatchResult::Draw => opponent,
                    MatchResult::Loose => Move::get_loosing_move_against(&opponent)
                };
                println!(" me: {:?}, needed_result: {:?}", me, needed_result);
                total_points += needed_result.get_points() as u32 + me.get_points() as u32;
            }
        }

        println!("I Scored: {}", total_points);
    } else {
        println!("File not found");
    }
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
