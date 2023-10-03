#[derive(Debug)]
pub struct Grid<T> where T: Copy {
    data: Vec<T>,
    rows: usize,
    cols: usize
}

impl<T> Grid<T> where T: Copy {
    pub fn new(rows: usize, cols: usize, default_value: T) -> Self {
        Grid {
            rows,
            cols,
            data: vec![default_value; rows * cols]
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.get_index(row, col);
        self.data[index] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let index = self.get_index(row, col);
        self.data[index]
    }

    pub fn get_amount_of_rows(&self) -> usize {
        self.rows
    }

    pub fn get_amount_of_cols(&self) -> usize {
        self.cols
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}