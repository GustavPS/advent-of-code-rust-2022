use std::arch::x86_64::_mm_test_all_ones;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::os::unix::raw::mode_t;
use std::path::Path;
use std::ptr::addr_of;
use regex::{Captures, Regex};
use crate::operation::{MathOperation, Modifier, Operation, TestOperation};
use crate::operation::MathOperation::Modulo;
use crate::operation::Modifier::Value;

mod operation;

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
struct Item {
    worry_level: usize
}

impl Item {
    pub fn new(worry_level: usize) -> Self {
        Self {
            worry_level
        }
    }

    pub fn calculate_new_worry_level(&self, operation: &Operation) -> usize {
        operation.run(self.worry_level)
    }

    pub fn set_worry_level(&mut self, worry_level: usize) {
        self.worry_level = worry_level;
    }
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: Vec<Item>,
    calculate_worry: Operation,
    test_operation: TestOperation, // Operation, item if not equal 0, item if equal 0
    items_inspected: u32
}

impl Monkey {
    pub fn new(id: u32, calculate_worry: Operation, test_operation: TestOperation) -> Self {
        Self {
            id,
            items: Vec::new(),
            calculate_worry,
            test_operation,
            items_inspected: 0
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    pub fn run(&mut self, total_product: u32) -> Vec<(Item, u32)> {
        let mut result: Vec<(Item, u32)> = Vec::new();

        self.items.iter_mut().for_each(|item| {
            let new_worry_level = item.calculate_new_worry_level(&self.calculate_worry) % total_product as usize;
            self.items_inspected += 1;

            let monkey_id = self.test_operation.test(new_worry_level);

            item.set_worry_level(new_worry_level);
            result.push((item.clone(), monkey_id));
        });
        self.clear_items();
        result
    }

    pub fn print_items(&self) {
        println!("{:?}", self.items);
    }
}

#[derive(Debug)]
struct Game {
    monkeys: Vec<Monkey>
}

impl Game {
    pub fn new() -> Self {
        Self {
            monkeys: Vec::new()
        }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    pub fn run_round(&mut self, total_product: u32) {
        let mut i = 0;
        while i < self.monkeys.len() {
            let mut moves = self.monkeys.get_mut(i).unwrap().run(total_product);

            for result in moves {
                let item = result.0;
                let monkey_idx = result.1 as usize;
                self.monkeys.get_mut(monkey_idx).unwrap().add_item(item);
            }
            i += 1;
        }
    }

    pub fn print_all_monkeys(&self) {
        self.monkeys.iter().for_each(|monkey| {
            println!("Monkey id: {}, items inspected: {}", monkey.id, monkey.items_inspected);
            //monkey.print_items();
        });
    }


}

fn main() {
    println!("Hello, world!");
/*
    let mut monkey = Monkey::new(
        0,
        Operation::new(MathOperation::Multiply, Modifier::Value(19)),
        TestOperation::new(Operation::new(MathOperation::Modulo, Modifier::Value(23)), 0, 1, 1)
    );
    monkey.add_item(Item::new(79));
    monkey.add_item(Item::new(98));

    let mut monkey2 = Monkey::new(
        1,
        Operation::new(MathOperation::Add, Modifier::Value(6)),
        TestOperation::new(Operation::new(MathOperation::Modulo, Modifier::Value(19)), 0, 1, 1)
    );*/

    let mut game = Game::new();
    let mut total_product = 0;

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day11/src/data.txt") {
        let build_result = build_monkeys(lines);
        let monkeys = build_result.0;
        total_product = build_result.1;
        for monkey in monkeys {
            game.add_monkey(monkey);
        }
    }
    for _ in 0..10000 {
        game.run_round(total_product);
    }
    game.print_all_monkeys();
}

fn build_monkeys(lines: Lines<BufReader<File>>) -> (Vec<Monkey>, u32) {
    let mut monkeys = Vec::new();

    let mut monkey_id = None;
    let mut monkey_items = None;
    let mut test_value = None;
    let mut math_operator = None;
    let mut math_modifier = None;
    let mut monkey_id_if_true = None;
    let mut monkey_id_if_false = None;
    let mut total_product: u32 = 1;


    for line in lines {
        let line = line.unwrap();
        let re = Regex::new(r"Monkey (?<monkey_id>\d+)").unwrap();
        // Try to match monkey_id
        match re.captures(&line) {
            Some(captures) => {
                monkey_id = Some(captures["monkey_id"].parse::<u32>().unwrap());
                continue;
            },
            None => {}
        }

        // Try to match monkey items
        let re = Regex::new(r"Starting items: (?<items>\d+(?:, \d+)*)$").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                let items = &captures["items"];
                let items: Vec<usize> = items.split(", ").map(|item| item.parse::<usize>().unwrap()).collect();
                monkey_items = Some(items);
            },
            None => {}
        }

        // Try to match oepration
        let re = Regex::new(r"Operation: new = old (?<operation>[+-/%*]) (?<amount>old|\d+)").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                math_operator = match &captures["operation"] {
                    "+" => Some(MathOperation::Add),
                    "-" => Some(MathOperation::Subtract),
                    "/" => Some(MathOperation::Divide),
                    "*" => Some(MathOperation::Multiply),
                    "%" => Some(MathOperation::Modulo),
                    _ => panic!("Can't parse {} as math operation", &captures["operation"])
                };
                math_modifier = match &captures["amount"] {
                    "old" => Some(Modifier::OldValue),
                    _ => Some(Modifier::Value(captures["amount"].parse::<usize>().unwrap()))
                };
            },
            None => {}
        }

        // Try to match test
        let re = Regex::new(r"Test: divisible by (?<test_amount>\d+)").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                let divide_by = captures["test_amount"].parse::<usize>().unwrap();
                test_value = Some(divide_by);
                total_product *= divide_by as u32;
            },
            None => {}
        }

        // Try to match if true
        let re = Regex::new(r"If true: throw to monkey (?<monkey_id>\d+)").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                let monkey_id = captures["monkey_id"].parse::<u32>().unwrap();
                monkey_id_if_true = Some(monkey_id);
            },
            None => {}
        }

        // Try to match if false
        let re = Regex::new(r"If false: throw to monkey (?<monkey_id>\d+)").unwrap();
        match re.captures(&line) {
            Some(captures) => {
                let monkey_id = captures["monkey_id"].parse::<u32>().unwrap();
                monkey_id_if_false = Some(monkey_id);
            },
            None => {}
        }

        // If we have all info we need
        if monkey_id.is_some() && monkey_items.is_some() && test_value.is_some() && monkey_id_if_true.is_some() && monkey_id_if_false.is_some() && math_operator.is_some() && math_modifier.is_some() {
            let test_operation = TestOperation::new(
                Operation::new(
                    MathOperation::Modulo,
                    Modifier::Value(test_value.unwrap())
                ),
                0,
                monkey_id_if_true.unwrap(),
                monkey_id_if_false.unwrap()
            );
            let math_operation = Operation::new(math_operator.unwrap(), math_modifier.unwrap());
            let mut monkey = Monkey::new(monkey_id.unwrap(), math_operation, test_operation);
            for worry_level in monkey_items.unwrap() {
                monkey.add_item(Item::new(worry_level));
            }
            monkeys.push(monkey);

            // Reset values
            monkey_id = None;
            monkey_items = None;
            test_value = None;
            monkey_id_if_true = None;
            monkey_id_if_false = None;
            math_operator = None;
            math_modifier = None;
        }
    }
    (monkeys, total_product)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
