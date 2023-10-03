use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
const EMPTY_CHAR: char = '\0';

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day6/src/data.txt") {
        for line in lines {
            if let Ok(line) = line {
                match find_start_of_message_marker(line.as_str()) {
                    Some(starting_number) => println!("Result: {}", starting_number),
                    None => println!("ERROR")
                }
            }
        }
    }
}

fn find_start_of_packet_marker(data: &str) -> Option<u32> {
    find_unique_characters(data, 4)
}

fn find_start_of_message_marker(data: &str) -> Option<u32> {
    find_unique_characters(data, 14)
}

fn find_unique_characters(data: &str, amount: usize) -> Option<u32>  {
    let mut data_buffer = vec![EMPTY_CHAR; amount];

    let mut count = 0;
    for c in data.chars() {
        data_buffer[count % amount] = c;

        let res = data_buffer
            .clone()
            .into_iter()
            .filter(|v| v != &EMPTY_CHAR)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        if res.len() == data_buffer.len() {
            return Some((count + 1) as u32);
        }

        count += 1;
    }
    None
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}