use std::collections::VecDeque;

#[derive(Debug)]
pub struct Stack {
    crates: VecDeque<char>,
    position: u32
}

impl Stack {
    pub fn new(position: u32) -> Stack {
        Stack {
            position,
            crates: VecDeque::new()
        }
    }

    pub fn position(&self) -> u32 {
        self.position
    }

    // Used for building the stack
    pub fn add_to_create(stack: &mut Stack, c: char) {
        stack.crates.push_front(c);
    }

    pub fn remove_one_crate(&mut self) -> char {
        let removed = self.crates.pop_back().unwrap();
        removed
    }

    pub fn add_one_crate(&mut self, item: char) {
        self.crates.push_back(item);
    }

    pub fn get_crate_on_top(&self) -> &char {
        self.crates.get(self.crates.len() - 1).unwrap()
    }
}