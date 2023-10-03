extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::os::unix::raw::ino_t;
use std::path::Path;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(PartialEq, Debug)]
enum CompareResult {
    Correct,
    Incorrect,
    NotKnown
}

#[derive(Debug, PartialEq)]
enum Item {
    Number(u64),
    List(Vec<Item>)
}

impl Clone for Item {
    fn clone(&self) -> Self {
        match self {
            Item::List(list) => Item::List(list.clone()),
            Item::Number(val) => Item::Number(val.clone())
        }
    }
}

impl Item {
    pub fn to_list(self) -> Self {
        match self {
            Item::Number(_) => Item::List(vec![self]),
            Item::List(_) => self
        }
    }
}

impl<'de> Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;

        match value {
            serde_json::Value::Number(n) => Ok(Item::Number(n.as_u64().unwrap())),
            serde_json::Value::Array(items) => {
                let parsed_items: Result<Vec<Item>, _> = items
                    .into_iter()
                    .map(serde_json::from_value)
                    .collect();
                Ok(parsed_items.map(Item::List).unwrap())
            },
            _ => Err(serde::de::Error::custom("Invalid value"))
        }
    }
}

fn compare_numbers(left: u64, right: u64) -> CompareResult {
    if left < right {
        return CompareResult::Correct
    }
    if left == right {
        return CompareResult::NotKnown
    }
    CompareResult::Incorrect
}

fn compare_list(left: &Vec<Item>, right: &Vec<Item>) -> CompareResult {
    for (i, item) in left.iter().enumerate() {
        if i >= right.len() {
            return CompareResult::Incorrect;
        }

        let result = compare(item, &right[i]);

        if result != CompareResult::NotKnown {
            return result;
        }
    }

    if left.len() < right.len() {
        return CompareResult::Correct;
    }

    CompareResult::NotKnown
}

fn compare(left: &Item, right: &Item) -> CompareResult {
    println!("Comparing: {:?} with {:?}", left, right);
    match (left, right) {
        (Item::Number(left_value), Item::Number(right_value)) => compare_numbers(*left_value, *right_value),
        (Item::List(left_list), Item::List(right_list)) => compare_list(left_list, right_list),
        (_,_) => compare(&left.clone().to_list(), &right.clone().to_list()) // Mixed types, convert both to list
    }
}


fn main() {
    println!("Hello, world!");

    let mut items = build_items().unwrap();
    items.sort_by(|a, b| {
        if compare(a, b) == CompareResult::Correct {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let decoders = vec![Item::List(vec![Item::List(vec![Item::Number(2)])]), Item::List(vec![Item::List(vec![Item::Number(6)])])];
    let product: usize = items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| if decoders.contains(item) { Some(index+1) } else { None })
        .product();

    println!("   - Result: {}", product);
}

fn build_items() -> Result<Vec<Item>, String> {
    let mut result = Vec::new();

    let file = File::open("/home/acorn/RustroverProjects/advent-of-code-2022-day13/src/data.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().filter_map(|line| line.ok());

    while let(Some(line)) = lines.next() {
        if !line.trim().is_empty() {
            result.push(serde_json::from_str(&line).unwrap());
        }
    }
    println!("{:?}", result);
    Ok(result)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
