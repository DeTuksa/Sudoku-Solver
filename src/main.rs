use std::io::{Error, ErrorKind};
use sudoku_solver::{SudokuGrid, Solver};

fn main() -> Result<(), Error> {
    let filename = "sudoku.txt";

    let mut sudoku_grid = match SudokuGrid::read_from_file(filename) {
        Ok(grid) => grid,
        Err(e) => {
            eprintln!("Error reading sudoku puzzle: {}", e);
            return Err(Error::new(ErrorKind::Other, "Failed to read Sudoku puzzle"));
        }
    };

    println!("Sudou Puzzle::::");
    sudoku_grid.display();

    let mut solver = Solver::new(&mut  sudoku_grid);

    if solver.solve() {
        println!("\n<<<<<<<<<<<<<<<Solved Sudoku Puzzle>>>>>>>>>>>>>>>>>>");
        sudoku_grid.display();
    } else {
        println!("\nNo solution found for this Sudoku puzzle");
    }

    Ok(())
}
