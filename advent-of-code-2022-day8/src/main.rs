use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Range;
use std::path::Path;

#[derive(Debug)]
struct Grid<T> where T: Copy {
    data: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T> Grid<T> where T: Copy {
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        Grid {
            rows,
            cols,
            data: vec![default_value; rows * cols]
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.data[index] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let index = self.get_index(row, col);
        self.data[index]
    }

    pub fn get_amount_of_rows(&self) -> usize {
        self.rows
    }

    pub fn get_amount_of_cols(&self) -> usize {
        self.cols
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

fn main() {
    println!("Hello, world!");
    let filename = "/home/acorn/RustroverProjects/advent-of-code-2022-day8/src/data.txt";
    let (num_rows, num_columns) = count_rows_and_columns_in_file(filename);
    println!("Num rows: {}, num columns: {}", num_rows, num_columns);

    let mut grid = Grid::new(num_rows, num_columns, 0);

    // Build grid
    if let Ok(lines) = read_lines(filename) {
        for (row_idx, line) in lines.enumerate() {
            let line = line.unwrap();
            for (column_idx, value) in line.chars().enumerate() {
                const RADIX: u32 = 10;
                let value = value.to_digit(RADIX).unwrap();
                grid.set(row_idx, column_idx, value);
            }
        }
    }

    // Calculate
    //calculate_visible_trees(&grid);
    let highest_score = calculate_highest_scenic_score(&grid);
    println!("Highest score: {}", highest_score);
}

fn calculate_highest_scenic_score(grid: &Grid<u32>) -> u32 {
    let rows = grid.get_amount_of_rows();
    let cols = grid.get_amount_of_cols();

    let mut result = 0;
    let mut highest_score = 0u32;

    // Skip first and last since they are on the edge
    for row in 1..rows - 1 {
        for col in 1..cols - 1  {

            if col == 2 && row == 3 {
                println!("break");
            }

            let mut scores: Vec<u32> = Vec::new();
            let grid_value = grid.get(row, col);

            let steps_until_higher_left = match find((0..col).rev(), |col| grid.get(row, col) >= grid_value) {
                None => col as u32, // If we didn't find any, view is not blocked so the score is our current column
                Some(steps) => steps + 1 // If it was blocked, steps is the amount of steps before we where blocked
            };
            scores.push(steps_until_higher_left);

            let steps_until_higher_right = match find(col+1..cols, |col| grid.get(row, col) >= grid_value) {
                None => cols as u32 - col as u32 - 1, // if we didn't get blocked, the score is the amount of columns to our right
                Some(steps) => steps + 1 // + 1 since we count the tree that stopped us
            };
            scores.push(steps_until_higher_right);

            let steps_until_higher_down = match find(row+1..rows, |row| grid.get(row, col) >= grid_value) {
                None => rows as u32 - row as u32 - 1, // If we didn't get blocked the score is the amount of rows beneth us
                Some(steps) => steps + 1
            };
            scores.push(steps_until_higher_down);

            let steps_until_higher_up = match find((0..row).rev(), |row| grid.get(row, col) >= grid_value) {
                None => row as u32, // If we didn't find any, view is not blocked so the score is our current row
                Some(steps) => steps + 1
            };
            scores.push(steps_until_higher_up);

            let total_score = scores.iter().fold(1, |res, a| res * a);
            if total_score > highest_score {
                println!("New highest, row: {}, col: {}, value: {}", row, col, total_score);
                highest_score = total_score;
            }

        }
    }
    highest_score
}

fn calculate_visible_trees(grid: &Grid<u32>) -> u32 {
    let rows = grid.get_amount_of_rows();
    let cols = grid.get_amount_of_cols();

    let mut result = 0;

    // Skip first and last since they are on the edge
    for row in 1..rows - 1 {
        let mut highest_value_left = grid.get(row, 0);

        for col in 1..cols - 1  {
            let grid_value = grid.get(row, col);

            // If it's higher than everything to the left, it's visible
            if grid_value > highest_value_left {
                highest_value_left = grid_value;
                result += 1;
                continue;
            }

            let higher_right = find(col+1..cols, |col| grid.get(row, col) >= grid_value).is_some();
            // If we don't have anything higher to the right, it's visible
            if !higher_right {
                result += 1;
                continue;
            }

            let higher_down = find(row + 1..rows, |row| grid.get(row, col) >= grid_value).is_some();
            // If we don't have anything higher down, it's visible
            if !higher_down {
                result += 1;
                continue;
            }

            let higher_up = find(0..row, |row| grid.get(row, col) >= grid_value).is_some();
            // Check if we have higher up
            if !higher_up {
                result += 1;
                continue;
            }
        }
    }

    let outside = cols * 2 + rows * 2 - 4; // -4 so we don't count edges twice
    println!("outside: {}, inside: {}, result: {}", outside, result, result + outside);
    result as u32 + outside as u32
}

fn find<F, T>(range: T, comparer: F) -> Option<u32>
where
    F: Fn(usize) -> bool,
T: Iterator<Item = usize>
{
    let mut count = 0;
    for i in range {
        if comparer(i) {
            return Some(count);
        }
        count += 1;
    }
    None
}

fn count_rows_and_columns_in_file<P>(filename: P) -> (usize, usize) where P: AsRef<Path> {
    let lines = read_lines(filename).unwrap();
    let mut rows = 0;
    let mut columns = 0;
    for line in lines {
        columns = line.unwrap().len();
        rows += 1;
    }
    (rows, columns)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
