use  std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

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
                let cell_str = if cell == 0 { ".".to_string()} else {cell.to_string()};
                // Print the value of the cell, using "." for empty cells
                print!("{} ", cell_str);
            }
            // Move to the next line after printing each row
            println!();
        }
    }

    // Method to read a Sudoku puzzle from a text file
    fn read_from_file(filename: &str) -> Result<Self, Error> {
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
}

fn main() -> Result<(), Error> {
    let filename = "sudoku.txt";

    let sudoku_grid = match SudokuGrid::read_from_file(filename) {
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("Error reading sudoku puzzle: {}", e);
            return Err(Error::new(ErrorKind::Other, "Failed to read Sudoku puzzle"));
        }
    };

    println!("Sudou Puzzle::::");
    sudoku_grid.display();

    Ok(())
}
