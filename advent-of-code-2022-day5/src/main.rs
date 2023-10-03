use std::backtrace::BacktraceStatus::Captured;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;
use regex::Regex;
use crate::stack::Stack;

mod stack;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day5/src/data.txt") {
        let mut stacks: Vec<Stack> = Vec::new();
        let mut building_stacks = true;
        for line in lines {
            if let Ok(line) = line {
                if line.is_empty() {
                    building_stacks = false;
                    continue;
                }

                if building_stacks {
                    build_stacks(&mut stacks, line);
                } else {
                    handle_move_command(&mut stacks, line);
                }
            }
        }

        print_result(&stacks);
    }
}

fn print_result(stacks: &Vec<Stack>) {
    let mut result = "".to_string();
    let mut position = 1;
    while let Some(stack) = stacks.iter().find(|stack| stack.position() == position) {
        let c = stack.get_crate_on_top();
        result += &c.to_string();
        position += 1;
    }

    println!("Result: {}", result);
}

fn handle_move_command(mut stacks: &mut Vec<Stack>, line: String) {
    let re = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    let caps = re.captures(&line).unwrap();
    let amount_of_moves = &caps["amount"].parse::<u32>().unwrap();
    let from = &caps["from"].parse::<u32>().unwrap();
    let to = &caps["to"].parse::<u32>().unwrap();

    let mut removed = Vec::new();

    // Remove
    let mut stack = stacks.iter_mut().find(|stack| stack.position() == *from).unwrap();
    for _ in 0..*amount_of_moves {
        removed.push(stack.remove_one_crate());
    }

    // Add
    stack = stacks.iter_mut().find(|stack| stack.position() == *to).unwrap();
    for item in removed.iter().rev() {
        stack.add_one_crate(*item);
    }
}

fn build_stacks(mut stacks: &mut Vec<Stack>, line: String) {
    let re = Regex::new(r"\[(.)]").unwrap();
    let caps = re.find_iter(&line);
    for capture in caps {
        let stack_number = (capture.start() / 4) as u32 + 1;
        let name_of_crate = capture.as_str().replace('[', "").replace(']', "").chars().next().unwrap();

        let stack = find_or_insert(&mut stacks, stack_number);
        Stack::add_to_create(stack, name_of_crate);
    }
}

fn find_or_insert(stacks: &mut Vec<Stack>, position: u32) -> &mut Stack {
    if let Some(i) = stacks.iter().position(|stack| stack.position() == position) {
        &mut stacks[i]
    } else {
        let stack = Stack::new(position);
        stacks.push(stack);
        stacks.last_mut().unwrap()
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}