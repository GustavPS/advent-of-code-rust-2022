use std::collections::VecDeque;
use crate::position::Position;

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<(Position, Vec<Position>)>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn add_node(&mut self, position: Position) {
        if self.node_exists(position.x(), position.y()) {
            panic!("Position already added");
        }

        let neighbors = self.add_neighbors(position);

        self.nodes.push((position, neighbors))
    }

    pub fn get_start_nodes(&self) -> Vec<Position> {
        self.nodes
            .iter()
            .filter(|position| position.0.is_start())
            .map(|position| position.0)
            .collect()
    }

    pub fn bfs(&self, start_x: u32, start_y: u32) -> Result<u32, &str>{
        let mut steps: u32 = 0;

        // List of visited nodes (the index of the node inside the self.nodes ist)
        let mut visited: Vec<bool> = vec![false; self.nodes.len()];

        // BFS Queue
        let mut queue: VecDeque<usize> = VecDeque::new();

        // Find the start index
        let start_index = self.get_node_index(start_x, start_y);
        // Mark it as visited
        visited[start_index] = true;
        // Queue it!
        queue.push_back(start_index);

        while !queue.is_empty() {
            let mut queue_size = queue.len();

            // Handle all nodes in this level
            while queue_size != 0 {
                let index = queue.pop_front().unwrap();
                let node = &self.nodes[index].0;
                if node.is_goal() {
                    println!("Found goal! {:?}", node);
                    return Ok(steps);
                }

                let neighbors = &self.nodes[index].1;
                for neighbor in neighbors {
                    // TODO: Not so good since we have to get the index A LOT of times
                    let index = self.get_node_index(neighbor.x(), neighbor.y());
                    if !visited[index] && neighbor.can_walk_to(node.height()) {
                        visited[index] = true;
                        queue.push_back(index);
                    }
                }
                queue_size -= 1;
            }
            steps += 1;
        }
        Err("No path found")
    }

    fn add_neighbors(&mut self, new_node: Position) -> Vec<Position> {
        let mut result = Vec::new();
        if new_node.x() > 0 {
            if let Some(node) = self.get_node_mut(new_node.x() - 1, new_node.y()) {
                let mut neighbors = &mut node.1;
                neighbors.push(new_node);
                result.push(node.0);
            }
        }

        if let Some(node) = self.get_node_mut(new_node.x(), new_node.y()) {
            let mut neighbors = &mut node.1;
            neighbors.push(new_node);
            result.push(node.0);
        }

        if new_node.y() > 0 {
            if let Some(node) = self.get_node_mut(new_node.x(), new_node.y() - 1) {
                let mut neighbors = &mut node.1;
                neighbors.push(new_node);
                result.push(node.0);
            }
        }

        if let Some(node) = self.get_node_mut(new_node.x(), new_node.y() + 1) {
            let mut neighbors = &mut node.1;
            neighbors.push(new_node);
            result.push(node.0);
        }
        result
    }

    fn get_node_index(&self, x: u32, y: u32) -> usize {
        self.nodes.iter().position(|position| position.0.x() == x && position.0.y() == y).unwrap()
    }

    fn get_node_mut(&mut self, x: u32, y: u32) -> Option<&mut (Position, Vec<Position>)> {
        self.nodes.iter_mut().find(|position| position.0.x() == x && position.0.y() == y)
    }

    fn node_exists(&self, x: u32, y: u32) -> bool {
        self.nodes.iter().find(|position| position.0.x() == x && position.0.y() == y).is_some()
    }
}