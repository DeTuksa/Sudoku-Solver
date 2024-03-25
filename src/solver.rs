use super::SudokuGrid;

pub struct  Solver<'a> {
    sudoku_grid: &'a mut SudokuGrid
}

impl <'a> Solver<'a> {
    pub fn new(sudoku_grid: &'a mut SudokuGrid) -> Self {
        Solver { sudoku_grid }
    }

    pub fn solve(&mut self) -> bool {
        self.solve_recursive()
    }

    // Method to solve the Sudoku puzzle using backtracing
    fn solve_recursive(&mut self) -> bool {
        // Find the next empty cell (if any)
        if let Some((row, col)) = self.sudoku_grid.find_empty_cell() {
            // Try filling the cell with each possible digit
            for num in 1..=9  {
                // Check if the current digit is valid for the cell
                if self.sudoku_grid.is_valid_move(row, col, num as u8) {
                    // If valid, set the cell value and recursively solve
                    self.sudoku_grid.set_cell(row, col, num as u8);
                    // Recursively solve the puzzle
                    if self.solve() {
                        return  true; // Solution found
                    }
                    // If the recursive call returns false, backtrack and try the next digit
                    self.sudoku_grid.set_cell(row, col, 0); // Reset the cell
                }
            }
            // If no valid digit leads to a solution, backtrack
            return  false;
        }
        true // If there are no empty cells left, the puzzle is solved
    }
}