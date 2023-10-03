use std::fs::File;
use std::{cmp, io};
use std::collections::hash_set::Difference;
use std::io::BufRead;
use std::ops::Deref;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum Move {
    Left(u32),
    Right(u32),
    Up(u32),
    Down(u32)
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new() -> Self {
        Point {
            x: 0,
            y: 0
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct RopeEnd {
    position: Point,
    points_visited: Vec<Point>,
}

impl RopeEnd {
    pub fn new() -> Self {
        let point = Point::new();
        RopeEnd {
            position: point,
            points_visited: vec![point.clone()],
        }
    }

    pub fn make_move_with_cb(&mut self, direction: Move, mut notify: impl FnMut(&RopeEnd)) {
        match direction {
            Move::Left(amount) => {
                for _ in 0..amount {
                    self.position.x -= 1;
                    notify(&self);
                }
            },
            Move::Right(amount) => {
                for _ in 0..amount {
                    self.position.x += 1;
                    notify(&self);
                }
            },
            Move::Up(amount) => {
                for _ in 0..amount {
                    self.position.y -= 1;
                    notify(&self);
                }
            },
            Move::Down(amount) => {
                for _ in 0..amount {
                    self.position.y += 1;
                    notify(&self);
                }
            }
        };
    }

    pub fn make_move(&mut self, direction: Move) {
        self.make_move_with_cb(direction, |_| {});
    }

    pub fn get_points_visited(&self) -> u32 {
        self.points_visited.len() as u32
    }

    fn mark_as_visited(&mut self, point: Point) {
        if !self.points_visited.contains(&point) {
            self.points_visited.push(point);
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut head = RopeEnd::new();
    let mut tails = vec![RopeEnd::new(); 9];

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day9/src/data.txt") {
        for line in lines {
            let line = line.unwrap();
            let head_move = get_move_from_line(&line).unwrap();
            let mut head_positions = Vec::new();
            head.make_move_with_cb(head_move, |modified| {
                head_positions.push(modified.position);

                for i in 0..tails.len() {
                    let clone = match i {
                        0 => modified.clone(),
                        _ => tails.get(i - 1).unwrap().clone()
                    };
                    let moved_done = move_tail_if_needed(tails.get_mut(i).unwrap(), &clone);
                }
            });
            head_positions.iter().for_each(|position| head.mark_as_visited(*position));
        }
    }

    let last_tail_position_visisted = tails.last().unwrap().get_points_visited();
    println!("Result: {}", last_tail_position_visisted);

}

/*

 . . . .   . . . .    . . . .  0
 . . . .   . . H .    . . H .  1
 . . H .   . . . .    . . T .  2
 . T . .   . T . .    . . . .  3

 */

fn get_tail_move_based_on_head(tail: &RopeEnd, head: &RopeEnd) -> Vec<Move> {
    let is_head_to_the_right = head.position.x > tail.position.x;
    let is_head_to_the_left = head.position.x < tail.position.x;
    let is_head_above = head.position.y < tail.position.y;
    let is_head_below = head.position.y > tail.position.y;

    let mut result = Vec::new();
    if head.position.x > tail.position.x + 1 { // If head is 2 steps to the right of the tail
        result.push(Move::Right(1)); // Move one step to the right
        if is_head_above {
            result.push(Move::Up(1));
        }
        if is_head_below {
            result.push(Move::Down(1));
        }
    } else if head.position.x < tail.position.x - 1 { // If head is 2 steps to the left of the tail
        result.push(Move::Left(1)); // Move one step to the left
        if is_head_above {
            result.push(Move::Up(1));
        }
        if is_head_below {
            result.push(Move::Down(1));
        }
    } else if head.position.y > tail.position.y + 1 { // If head is 2 steps below the tail
        result.push(Move::Down(1)); // Move one step down
        if is_head_to_the_right {
            result.push(Move::Right(1));
        }
        if is_head_to_the_left {
            result.push(Move::Left(1))
        }
    } else if head.position.y < tail.position.y - 1 { // If head is 2 steps above the tail
        result.push(Move::Up(1)); // Move one step up
        if is_head_to_the_right {
            result.push(Move::Right(1));
        }
        if is_head_to_the_left {
            result.push(Move::Left(1))
        }
    }
    result
}

fn move_tail_if_needed(tail: &mut RopeEnd, head: &RopeEnd) -> Vec<Move> {
    let moves_needed = get_tail_move_based_on_head(&tail, &head);
    moves_needed.iter().for_each(|move_to_do| {
        tail.make_move(*move_to_do);
    });
    tail.mark_as_visited(tail.position);
    moves_needed
}

fn get_move_from_line(line: &str) -> Option<Move> {
    let re = Regex::new(r"(?<direction>\w) (?<amount>\d+)").unwrap();
    let caps = re.captures(line).unwrap();
    let direction = &caps["direction"];
    let amount = caps["amount"].parse::<u32>().unwrap();
    match direction {
        "R" => Some(Move::Right(amount)),
        "L" => Some(Move::Left(amount)),
        "U" => Some(Move::Up(amount)),
        "D" => Some(Move::Down(amount)),
        _ => None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}