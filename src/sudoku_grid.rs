// struct to represent a Sudoku puzzle
struct SudokuGrid {
    cells: [[u8; 9]; 9] // 9x9 grid of cells, each containing a number. Is 0 if empty
}

impl SudokuGrid {

    // Method to create a new empty Sudoku puzzle
    fn new() -> Self {
        SudokuGrid { cells: [[0; 9]; 9]}
    }

    // Method to set the value of a cell
    fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        // Update the value of the cell at the specified row and column
        self.cells[row][col] = value;
    }

    // Method to get the value of a cell
    fn get_cell(&self, row: usize, col: usize) -> u8 {
        // Retrieve the value of the cell at the specified row and column
        self.cells[row][col]
    }

    // Method to display the Sudoku puzzle
    fn display(&self) {
        // Iterate over each row in the Sudoku grid
        for row in &self.cells {
            // Iterate over each cell in the current row
            for &cell in row {
                // Print the value of the cell, using "." for empty cells
                print!("{} ", if cell == 0 {"."} else {&cell.to_string() });
            }
            // Move to the next line after printing each row
            println!();
        }
    }
}