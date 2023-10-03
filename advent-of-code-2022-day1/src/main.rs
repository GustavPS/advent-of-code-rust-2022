use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::elf::Elf;

mod elf;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022/src/data.txt") {
        let mut elves: Vec<Elf> = Vec::new();
        elves.push(Elf::new());

        for line in lines {
            if let Ok(calories) = line {
                if calories.is_empty() {
                    elves.push(Elf::new());
                    continue;
                }
                let num_elves = elves.len();
                let elf = elves.get_mut(num_elves - 1).unwrap();

                let calories = calories.parse::<u32>().unwrap();
                elf.add_food(calories);
            }
        }

        elves.sort_by(|a, b| b.get_total_calories().cmp(&a.get_total_calories()));

        let most_calories = elves.get(0).unwrap().get_total_calories();
        let top_three_combined = Elf::get_combined_calories(elves[0..3].to_vec());

        println!("Most: {}", most_calories);
        println!("3 most combined: {}", top_three_combined)
    } else {
        println!("Couldn't read file")
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
