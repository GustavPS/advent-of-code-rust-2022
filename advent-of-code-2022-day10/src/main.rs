mod screen;
mod grid;

use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::rc::Rc;
use regex::Regex;
use crate::screen::Screen;

#[derive(Debug)]
enum Instruction {
    Add(char, i32), // Registry, value
    Noop
}

impl Instruction {
    pub fn get_cycles(&self) -> u32 {
        match self {
            Instruction::Add(_, _) => 2,
            Instruction::Noop => 1
        }
    }
}

#[derive(Debug)]
struct Registry {
    name: char,
    value: i32
}

impl Registry {
    pub fn new(name: char) -> Self {
        Registry {
            name,
            value: 1
        }
    }
}

struct Circuit {
    registries: Vec<Rc<RefCell<Registry>>>,
    watchers: Vec<(Rc<RefCell<Registry>>, u32, u32, i32)>, // Registry, start_cycle, interval, value
    screen: Screen,
    current_cycle: u32
}

impl Circuit  {
    const SCREEN_ROWS: usize = 6;
    const SCREEN_COLUMNS: usize = 40;

    pub fn new() -> Self {
        Circuit {
            registries: Vec::new(),
            watchers: Vec::new(),
            screen: Screen::new(Self::SCREEN_ROWS, Self::SCREEN_COLUMNS),
            current_cycle: 0
        }
    }

    pub fn add_registry(&mut self, registry: Registry) {
        match self.get_registry(registry.name) {
            Some(_) => panic!(), // Don't allow registries with same names
            None => self.registries.push(Rc::new(RefCell::new(registry)))
        };
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {
        println!("Running instruction {:?}", instruction);
        match instruction {
            Instruction::Add(registry, value) => {
                let cycles = instruction.get_cycles();
                for _ in 0..cycles {
                    self.tick();
                }
                let registry = match self.get_registry(registry) {
                    Some(registry) => registry,
                    None => panic!("Trying to execute instruction {:?}: No registry named {}", instruction, registry)
                };
                let mut registry = registry.borrow_mut();
                registry.value += value;
            }
            Instruction::Noop => self.tick()
        }
    }

    pub fn watch_registry(&mut self, registry_name: char, start_cycle: u32, interval: u32) {
        let registry = match self.get_registry(registry_name) {
            Some(registry) => registry,
            None => panic!("Registry {} does not exist", registry_name)
        };
        self.watchers.push((Rc::clone(registry), start_cycle, interval, 0));
    }

    pub fn draw_screen(&self) {
        self.screen.draw();
    }

    fn tick(&mut self) {
        self.update_screen();
        self.current_cycle += 1;
        // Send notifications if an interrupt has been registered for this cycle

        for (registry, start_cycle, interval, value) in &mut self.watchers {
            let registry = registry.borrow();
            if self.current_cycle == *start_cycle {
                *value += registry.value * self.current_cycle as i32;
            } else if (self.current_cycle as i32 - *start_cycle as i32) % *interval as i32 == 0 {
                *value += registry.value * self.current_cycle as i32;
            }
        }
    }

    fn update_screen(&mut self) {
        let column = (self.current_cycle % Self::SCREEN_COLUMNS as u32) as i32;
        let row = (self.current_cycle / Self::SCREEN_COLUMNS as u32) as i32;

        let value = {
            let registry = self.get_registry('x').unwrap(); // X is screen registrý
            let registry = registry.borrow();
            registry.value
        };
        let lit_pixel = value >= column - 1 && value <= column + 1;
        self.screen.set_pixel(row as usize, column as usize, lit_pixel);
    }

    fn get_registry(&self, name: char) -> Option<&Rc<RefCell<Registry>>> {
        self.registries.iter().find(|registry| {
            let registry = registry.borrow();
            registry.name == name
        })
    }
}

fn main() {
    println!("Hello, world!");
    let mut circuit = Circuit::new();
    circuit.add_registry(Registry::new('x'));
    circuit.watch_registry('x', 20, 40);

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day10/src/data.txt") {
        for line in lines {
            let line = line.unwrap();
            let instruction = parse_instruction_from_line(&line).unwrap();
            circuit.run_instruction(instruction);
        }
    }
    println!("result: {:?}", circuit.watchers.get(0).unwrap());
    circuit.draw_screen();

}

fn parse_instruction_from_line(line: &str) -> Option<Instruction> {
    let re= Regex::new(r"(?<instruction>noop|add)(?<registry>\w)?\s?(?<amount>-?\d+)?").unwrap();
    let caps = re.captures(line).unwrap();

    match &caps["instruction"] {
        "add" => {
            let registry = caps["registry"].chars().next().unwrap();
            let amount = caps["amount"].parse::<i32>().unwrap();
            Some(Instruction::Add(registry, amount))
        },
        "noop" => Some(Instruction::Noop),
        _ => None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}