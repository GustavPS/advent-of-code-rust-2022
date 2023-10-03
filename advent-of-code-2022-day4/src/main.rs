use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Elf {
    sectors: HashMap<u32, u32>
}

impl Elf {
    pub fn new(sectors_as_string: String) -> Elf {
        let mut splitted = sectors_as_string.split('-');
        let starting_sector = splitted.next().unwrap();
        let starting_sector = starting_sector.parse::<u32>().unwrap();
        let ending_sector = splitted.next().unwrap();
        let ending_sector = ending_sector.parse::<u32>().unwrap();

        let mut sectors = HashMap::new();
        for i in starting_sector..ending_sector + 1 {
            sectors.insert(i, 1);
        }
        Elf {
            sectors
        }
    }

    pub fn contains(&self, other: &Elf) -> bool {
        for (sector, _) in self.sectors.iter() {
            if !other.sectors.contains_key(&sector) {
                return false;
            }
        }
        return true;
    }

    pub fn overlap(&self, other: &Elf) -> bool {
        for (sector, _) in self.sectors.iter() {
            if other.sectors.contains_key(&sector) {
                return true;
            }
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day4/src/data.txt") {
        let mut total_contain = 0;
        let mut total_overlap = 0;
        for line in lines {
            if let Ok(pair) = line {
                let elfs = split_pair_to_elfs(pair);
                if elfs[0].contains(&elfs[1]) || elfs[1].contains(&elfs[0]) {
                    total_contain += 1;
                }
                if elfs[0].overlap(&elfs[1]) {
                    total_overlap += 1;
                }
            }
        }
        println!("Total contain: {}, Total overlap: {}", total_contain, total_overlap);
    }
}

fn split_pair_to_elfs(pair_as_string: String) -> Vec<Elf> {
    let mut result = Vec::new();
    for part in pair_as_string.split(',') {
        result.push(Elf::new(part.to_string()));
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
