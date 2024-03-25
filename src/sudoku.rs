use  std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// struct to represent a Sudoku puzzle
#[derive(Clone)]
pub struct SudokuGrid {
    cells: [[u8; 9]; 9] // 9x9 grid of cells, each containing a number. Is 0 if empty
}

impl SudokuGrid {

    // Method to create a new empty Sudoku puzzle
    pub fn new() -> Self {
        SudokuGrid { cells: [[0; 9]; 9]}
    }

    // Method to set the value of a cell
    pub fn set_cell(&mut self, row: usize, col: usize, value: u8) {
        // Update the value of the cell at the specified row and column
        self.cells[row][col] = value;
    }

    // Method to get the value of a cell
    pub fn get_cell(&self, row: usize, col: usize) -> u8 {
        // Retrieve the value of the cell at the specified row and column
        self.cells[row][col]
    }

    // Method to display the Sudoku puzzle
    pub fn display(&self) {
        // Iterate over each row in the Sudoku grid
        for row in &self.cells {
            // Iterate over each cell in the current row
            for &cell in row {
                let cell_str = if cell == 0 { ".".to_string()} else {cell.to_string()};
                // Print the value of the cell, using "." for empty cells
                print!("{} ", cell_str);
            }
            // Move to the next line after printing each row
            println!();
        }
    }

    // Method to read a Sudoku puzzle from a text file
    pub fn read_from_file(filename: &str) -> Result<Self, Error> {
        // Open the file in read mode
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        // Create a new empty SudokuGrid
        let mut sudoku_grid = SudokuGrid::new();

        for (row, line) in reader.lines().enumerate() {
            let line = line?;
            // Split the line into individual characters
            let chars: Vec<char> = line.chars().collect();
            // Iterate over each character to set the cell values
            for (col, ch) in chars.iter().enumerate() {
                if let Some(num) = ch.to_digit(10) {
                    sudoku_grid.set_cell(row, col, num as u8);
                } else if *ch != '.' {
                    return  Err(Error::new(ErrorKind::InvalidData, "Invalid character in input"));
                }
            }
        }

        Ok(sudoku_grid)
    }

    // Helper method to find the next empty cell
    pub fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.get_cell(row, col) == 0 {
                    return Some((row, col));
                }
            }
        }
        None // Return None if no empty cell is found
    }

    // Helper method to check if the number is used in the same column
    pub fn used_in_col(&self, col: usize, num: u8) -> bool {
        for row in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }

    // Helper method to check if the number is used in the 3x3 subgrid
    pub fn used_in_subgrid(&self, start_row: usize, start_col: usize, num: u8) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.get_cell(row + start_row, col + start_col) == num {
                    return true;
                }
            }
        }
        false
    }

    // Helper method to check if a move is valid
    pub fn is_valid_move(&self, row: usize, col: usize, num: u8) -> bool {
        // Check if the number is not present in the same row, column, or 3x3 subgrid
        !self.used_in_row(row, num) && !self.used_in_col(col, num) && !self.used_in_subgrid(row - row % 3, col - col % 3, num)
    }

    // Helper method to check if the number is used in the same row
    pub fn used_in_row(&self, row: usize, num: u8) -> bool {
        for col in 0..9 {
            if self.get_cell(row, col) == num {
                return true;
            }
        }
        false
    }
}