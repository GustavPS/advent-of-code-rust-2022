use std::collections::HashSet;
use std::fs::{File};
use std::io;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

#[derive(Debug,PartialEq, Eq, Hash, Clone)]
enum Item {
    Rock,
    Air,
    Sand,
    SandSpawner
}

/*
    [
        [0, 0, 0]
        [0, 0, 0]
        [0, 0, 0]
    ]

 */
#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T> Grid<T>
where
    T: Clone
{
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        Self {
            data: vec![default_value; rows * cols],
            rows,
            cols
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(self.get_index(row, col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = {
            self.get_index(row, col)
        };
        self.data[index] = value;
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}
/*

    [
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0],
        [0,0,0,0]
    ]

 */

#[derive(Debug)]
struct World {
    grid: Grid<Item>
}

impl World {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols, Item::Air)
        }
    }

}
/*

fn build_rocks<P>(filename: P) -> Vec<Item> where P: AsRef<Path> {
    let mut rocks = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        rocks.extend(parse_line(&line));
    }

    // Remove duplicates. TODO: Can be better so we don't save them from the start..
    rocks.into_iter().collect::<HashSet<_>>().into_iter().collect()
}
fn parse_line(line: &str) -> Vec<Item> {
    let mut rocks = Vec::new();
    let re = Regex::new(r"(?<x>\d+),(?<y>\d+)").unwrap();

    let splitted: Vec<&str> = line.split(" -> ").collect();
    for window in splitted.windows(2) {
        let captures = re.captures(window[0]).unwrap();
        let from_x = captures["x"].parse::<usize>().unwrap();
        let from_y = captures["y"].parse::<usize>().unwrap();

        let captures = re.captures(window[1]).unwrap();
        let to_x = captures["x"].parse::<usize>().unwrap();
        let to_y = captures["y"].parse::<usize>().unwrap();

        let (start_x, end_x) = (std::cmp::min(from_x, to_x), std::cmp::max(from_x, to_x));
        let (start_y, end_y) = (std::cmp::min(from_y, to_y), std::cmp::max(from_y, to_y));

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                rocks.push(Item::Rock {x, y});
            }
        }
    }
    rocks
}*/


fn main() {
    println!("Hello, world!");

    let mut world = World::new(0, 0);
/*
    let rocks = build_rocks("/home/acorn/RustroverProjects/advent-of-code-2022-day14/src/data.txt");
    let min_x = rocks.iter().map(|rock| rock.x()).min().unwrap();
    let max_x = rocks.iter().map(|rock| rock.x()).max().unwrap();
    let max_y = rocks.iter().map(|rock| rock.y()).max().unwrap();
    let min_y = rocks.iter().map(|rock| rock.y()).min().unwrap();

    println!("{:?}, min_x {} max_x {} min_y {} max_y {}", rocks, min_x, max_x, min_y, max_y);

 */
}

