mod position;
mod graph;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Index;
use std::path::Path;
use crate::graph::Graph;
use crate::position::Position;

fn main() {

    if let Ok(lines) = read_lines("/home/acorn/RustroverProjects/advent-of-code-2022-day12/src/data.txt") {
        let graph = build_grid(lines);
        let starting_nodes = graph.get_start_nodes();

        let mut shortest_amount_of_steps: Option<u32> = None;
        for position in starting_nodes {
            let steps = graph.bfs(position.x(), position.y());
            if steps.is_err() {
                println!("x: {}, y: {} can't find path", position.x(), position.y());
                continue;
            }
            let steps = steps.unwrap();

            if let Some(current_shortest) = shortest_amount_of_steps {
                if steps < current_shortest {
                    shortest_amount_of_steps = Some(steps);
                }
            }  else {
                shortest_amount_of_steps = Some(steps);
            }
        }

        println!("Result: {}", shortest_amount_of_steps.unwrap());
    }
}

fn build_grid(lines: io::Lines<io::BufReader<File>>) -> Graph {
    let mut graph = Graph::new();

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, height) in line.chars().enumerate() {
            let position = match height {
                'S' => Position::Start {x: x as u32, y: y as u32},
                'a' => Position::Start {x: x as u32, y: y as u32},
                'E' => Position::Goal {x: x as u32, y: y as u32},
                _ => Position::Point {x: x as u32, y: y as u32, height}
            };
            graph.add_node(position);
        }
    }
    graph
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}