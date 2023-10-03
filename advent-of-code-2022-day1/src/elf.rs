#[derive(Debug)]
#[derive(Clone)]
pub struct Elf {
    food: Vec<u32>
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            food: Vec::new()
        }
    }

    pub fn get_combined_calories(elves: Vec<Elf>) -> u32 {
        elves.iter().map(|elf| elf.get_total_calories()).sum()
    }

    pub fn add_food(&mut self, calories: u32) {
        self.food.push(calories);
    }

    pub fn get_total_calories(&self) -> u32 {
        return self.food.iter().sum()
    }
}
