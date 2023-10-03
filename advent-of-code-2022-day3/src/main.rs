use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct Rucksack {
    compartments: [String; 2]
}

impl Rucksack {
    pub fn new(compartments: [String; 2]) -> Rucksack {
        Rucksack {
            compartments
        }
    }

    pub fn find_common_item_in_compartment(&self) -> Result<char, String> {
        let mut map: HashMap<char, u32> = HashMap::new();
        for char in self.compartments[0].chars() {
            map.entry(char).or_insert(1);
        }

        for char in self.compartments[1].chars() {
            if map.contains_key(&char) {
                return Ok(char);
            }
        }
        Err("No common character found".to_string())
    }

    pub fn get_compartments_as_string(&self) -> String {
        return self.compartments[0].to_string() + &self.compartments[1];
    }

    pub fn find_group(sacks: &Vec<Rucksack>) -> Result<char, String> {
        let mut str = sacks.get(0).unwrap().get_compartments_as_string();

        for sack in sacks.iter().skip(1) {
            str = find_common_characters_in_string(str, sack.get_compartments_as_string())
        }

        if str.len() == 1 {
            return Ok(str.chars().next().unwrap());
        }
        Err("Couldn't find group name".to_string())
    }
}

fn find_common_characters_in_string(str1: String, str2: String) -> String {
    let mut map: HashMap<char, u32> = HashMap::new();
    for char in str1.chars() {
        map.entry(char).or_insert(1);
    }

    for char in str2.chars() {
        map.entry(char).and_modify(|value| *value += 1);
    }
    map.iter().filter(|(key, value)| **value > 1u32).map(|(key, value)| key).collect()
}


fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day3/src/data.txt") {
        let mut total: u32 = 0;
        let mut total_priority_for_groups = 0;

        let mut sacks = Vec::new();
        let mut counter = 0;
        for line in lines {
            if let Ok(items) = line {

                let compartments = create_compartments(items);
                let sack = Rucksack::new(compartments);
                let common_item = sack.find_common_item_in_compartment().unwrap();
                let priority = get_priority(&common_item).unwrap();
                total += priority;
                println!("Common item: {}, priority: {}", common_item, priority);

                sacks.push(sack);
                counter += 1;
                if counter % 3 == 0 {
                    let group = Rucksack::find_group(&sacks).unwrap();
                    sacks = Vec::new();
                    total_priority_for_groups += get_priority(&group).unwrap();
                }
            }
        }
        println!("Total: {}, Total group: {}", total, total_priority_for_groups);
    }
}

fn get_priority(c: &char) -> Result<u32, String> {
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let index = alphabet.iter().position(|r| r == c);
    match index {
        Some(value) => Ok(value as u32 + 1),
        None => Err("Not found in the alphabet".to_string())
    }
}

fn create_compartments(items: String) -> [String; 2] {
    let values = items.split_at(items.len() / 2);
    return [values.0.to_string(), values.1.to_string()];
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
