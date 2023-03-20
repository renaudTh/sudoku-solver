use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Sudoku {
    size: u8,
    grid: Vec<u8>,
}

impl Sudoku{
    pub fn get_coordinates(&self, index: u8) -> (u8, u8) {
        ((index / self.size), (index % self.size))
    }
}

#[wasm_bindgen]
impl Sudoku {
    pub fn new(existing_grid: Vec<u8>) -> Sudoku {
        Sudoku {
            size: existing_grid.len() as u8,
            grid: Vec::from(existing_grid),
        }
    }
    pub fn new_empty(size: u8) -> Sudoku {
        Sudoku {
            size,
            grid: vec![0;(size as usize) * (size as usize)]
        }
    }
    pub fn new_for_test() -> Sudoku {
        Sudoku {
            size: 9,
            grid: vec![
                3, 0, 0, 1, 6, 0, 8, 9, 4, 0, 9, 0, 5, 0, 0, 0, 3, 0, 0, 4, 0, 9, 3, 8, 2, 5, 1, 9,
                0, 0, 8, 1, 6, 5, 4, 0, 4, 0, 0, 3, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 2, 8,
                9, 0, 0, 0, 4, 1, 0, 6, 1, 4, 2, 0, 5, 0, 0, 0, 0, 0, 0, 0, 9, 1, 6, 0, 8,
            ],
        }
    }
    pub fn get_grid_ptr(&self) -> *const u8 {
        self.grid.as_ptr()
    }
    pub fn set(&mut self, row:u8, col:u8, value: u8) {
        let index = self.get(row, col);
        self.grid[index as usize] = value;
    }
    pub fn create_empty(size: u8) -> Sudoku {
        Sudoku {
            size: size * size,
            grid: vec![0; size as usize],
        }
    }
    pub fn get_size(&self) -> u8{
        self.size
    }
    pub fn get(&self, row: u8, col: u8) -> u8 {
        row * self.size + col
    }
    pub fn check_row(&self, row: u8, value: u8) -> bool {
        let mut k = row * self.size;
        while k < self.size * (row + 1) {
            if self.grid[k as usize] == value {
                return false;
            }
            k += 1
        }
        true
    }
    pub fn check_column(&self, col: u8, value: u8) -> bool {
        let mut k = col;
        while k < col + self.size * (self.size - 1) {
            if self.grid[k as usize] == value {
                return false;
            }
            k += self.size;
        }
        true
    }
    pub fn check_square(&self, row: u8, col: u8, value: u8) -> bool {
        let (i0, j0) = self.get_coordinates(self.get_square_origin(row, col));
        for i in 0..3 {
            for j in 0..3 {
                let index = self.get(i0 + i, j0 + j) as usize;
                if self.grid[index] == value {
                    return false;
                }
            }
        }
        true
    }
    pub fn get_square(&self, row: u8, col: u8) -> u8 {
        let sqrt_s = (self.size as f64).sqrt() as u8;
        col / sqrt_s + sqrt_s * (row / sqrt_s)
    }
    pub fn get_square_origin(&self, row: u8, col: u8) -> u8 {
        let k = self.get_square(row, col);
        let sqrt_s = (self.size as f64).sqrt() as u8;
        (k / sqrt_s) * sqrt_s * self.size + sqrt_s * (k % sqrt_s)
    }

    pub fn is_valid(&self, row: u8, col: u8, value: u8) -> bool {
        self.grid[self.get(row, col) as usize] == 0
            && self.check_row(row, value)
            && self.check_column(col, value)
            && self.check_square(row, col, value)
    }
    pub fn print(&self) {
        let mut i = 0;
        for number in &self.grid {
            if i > 0 && i % 9 == 0 {
                println!(" ");
            }
            print!("{} ", number);
            i += 1;
        }
        println!(" ");
    }
    pub fn solve(&mut self, index: u8) -> bool {
        if index == self.size * self.size {
            return true;
        }
        if self.grid[index as usize] > 0 {
            return self.solve(index + 1);
        }
        for value in 1..10 {
            let (row, col) = self.get_coordinates(index);
            if self.is_valid(row, col, value) {
                self.grid[index as usize] = value;
                let a = self.solve(index + 1);
                if a {
                    return true;
                }
            }
            self.grid[index as usize] = 0;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert_eq!(0, s.get(0, 0));
        assert_eq!(80, s.get(8, 8));
        assert_eq!(39, s.get(4, 3));
        assert_eq!(30, s.get(3, 3));
    }
    #[test]
    fn get_square_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert_eq!(0, s.get_square(0, 0));
        assert_eq!(1, s.get_square(1, 4));
        assert_eq!(8, s.get_square(8, 8));
    }
    #[test]
    fn check_row_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert!(s.check_row(0, 2));
        assert!(!s.check_row(2, 9));
        assert!(!s.check_row(7, 4));
    }
    #[test]
    fn check_col_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert!(s.check_column(0, 1));
        assert!(!s.check_column(2, 9));
        assert!(!s.check_column(7, 4));
    }
    #[test]
    fn check_square_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert!(s.check_square(0, 0, 1));
        assert!(!s.check_square(3, 3, 1));
        assert!(!s.check_square(8, 8, 4));
        assert!(!s.check_square(8, 5, 1));
    }
    #[test]
    fn is_valid_test() {
        let s: Sudoku = Sudoku::new_for_test();
        assert!(s.is_valid(1, 2, 1));
        assert!(!s.is_valid(0, 0, 3));
        assert!(!s.is_valid(8, 5, 4));
    }
    #[test]
    fn solve_test() {
        let mut s: Sudoku = Sudoku::new_for_test();
        assert!(s.solve(0));
        s.print();
    }
}
