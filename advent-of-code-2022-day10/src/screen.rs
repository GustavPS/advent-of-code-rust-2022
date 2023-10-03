use crate::grid::Grid;

pub struct Screen {
    pixels: Grid<bool>
}

impl Screen {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            pixels: Grid::new(rows, columns, false)
        }
    }

    pub fn set_pixel(&mut self, row: usize, col: usize, value: bool) {
        self.pixels.set(row, col, value);
    }

    pub fn draw(&self) {
        let rows = self.pixels.get_amount_of_rows();
        let cols = self.pixels.get_amount_of_cols();

        for row in 0..rows {
            for col in 0..cols {
                if self.pixels.get(row, col) {
                    // If the pixel is on
                    print!("#");
                } else {
                    // If the pixel is off
                    print!(".")
                }
            }
            println!();
        }
    }
}