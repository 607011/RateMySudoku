use crate::{EMPTY, Sudoku};
use rand::prelude::ThreadRng;
use rand::{Rng, seq::SliceRandom};
use std::fmt::{Display, Formatter};

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
pub enum FillAlgorithm {
    DiagonalThinOut,
    Incremental,
}

impl Display for FillAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FillAlgorithm::DiagonalThinOut => write!(f, "diagonal_thin_out"),
            FillAlgorithm::Incremental => write!(f, "incremental"),
        }
    }
}

pub struct SudokuGenerator {
    fill_algorithm: FillAlgorithm,
    max_filled_cells: usize,
    solutions_iter: std::vec::IntoIter<[[u8; 9]; 9]>,
    rng: ThreadRng,
}

/// A generator for Sudoku puzzles.
/// This struct implements the Iterator trait, allowing it to be used in a loop
/// to generate Sudoku puzzles with a specified number of filled cells.
/// The `max_filled_cells` parameter specifies how many cells should remain filled
/// at most. The minimum number of cells to fill is 17 (God's Number).
impl SudokuGenerator {
    pub fn new(fill_algorithm: FillAlgorithm, max_filled_cells: usize) -> Self {
        let mut rng = rand::rng();
        let solutions = match fill_algorithm {
            FillAlgorithm::DiagonalThinOut => {
                let mut all_digits: Vec<u8> = (1..=9).collect();
                let mut sudoku = Sudoku::new();
                // Fill the 3 diagonal boxes (top-left, middle, bottom-right)
                for box_idx in 0..3 {
                    let start_row = box_idx * 3;
                    let start_col = box_idx * 3;
                    all_digits.shuffle(&mut rng);
                    for (i, &num) in all_digits.iter().enumerate() {
                        sudoku.board[start_row + i / 3][start_col + i % 3] = num;
                    }
                }
                sudoku.all_solutions()
            }
            FillAlgorithm::Incremental => {
                if let Some(solution) = Self::generate_incrementally(max_filled_cells) {
                    vec![solution.board]
                } else {
                    Vec::new()
                }
            }
        };
        SudokuGenerator {
            fill_algorithm,
            max_filled_cells,
            solutions_iter: solutions.into_iter(),
            rng,
        }
    }
}

impl Iterator for SudokuGenerator {
    type Item = Sudoku;

    fn next(&mut self) -> Option<Self::Item> {
        match self.fill_algorithm {
            FillAlgorithm::DiagonalThinOut => {
                while let Some(board) = self.solutions_iter.next() {
                    let sudoku = Sudoku::from_board(board);
                    // Try to reduce this solution
                    if let Some(reduced) = self.try_reduce_puzzle(sudoku) {
                        return Some(reduced);
                    }
                }
            }
            FillAlgorithm::Incremental => {
                if let Some(sudoku) = Self::generate_incrementally(self.max_filled_cells) {
                    return Some(sudoku);
                }
            }
        }
        None
    }
}

impl SudokuGenerator {
    fn try_reduce_puzzle(&mut self, mut sudoku: Sudoku) -> Option<Sudoku> {
        let mut available_cells: Vec<(usize, usize)> = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .collect();
        available_cells.shuffle(&mut self.rng);
        let mut filled_cells = 81;
        while let Some((row, col)) = available_cells.pop() {
            let cell = sudoku.board[row][col];
            sudoku.board[row][col] = EMPTY;
            if Sudoku::has_unique_solution(&sudoku) {
                filled_cells -= 1;
                if filled_cells <= self.max_filled_cells {
                    sudoku.original_board = sudoku.board;
                    return Some(sudoku);
                }
            } else {
                // Cell removal created a Sudoku with multiple solutions,
                // so we need to backtrack
                sudoku.board[row][col] = cell;
            }
        }
        None
    }

    /// Generate a Sudoku puzzle by filling cells incrementally.
    /// This method fills cells one by one, ensuring that the
    /// puzzle has a unique solution.
    /// The `filled_cells` parameter specifies how many cells should remain filled.
    /// The minimum number of cells to fill is 17.
    /// If the puzzle cannot be generated with the specified number of filled cells,
    /// it returns `None`.
    pub fn generate_incrementally(max_cells_to_fill: usize) -> Option<Sudoku> {
        assert!(
            max_cells_to_fill <= 81,
            "Filled cells must be less than or equal to 81"
        );
        assert!(
            max_cells_to_fill >= 17,
            "Filled cells must be greater than or equal to 17"
        );
        let mut rng: ThreadRng = rand::rng();
        let mut sudoku = Sudoku::new();
        let mut available_cells: Vec<(usize, usize)> = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .collect();
        available_cells.shuffle(&mut rng);
        let mut filled = 0;
        while filled < max_cells_to_fill {
            if let Some((row, col)) = available_cells.pop() {
                let digit = rng.random_range(1..=9);
                if sudoku.can_place(row, col, digit) {
                    sudoku.board[row][col] = digit;
                    filled += 1;
                }
            } else {
                return None;
            }
        }
        sudoku.original_board = sudoku.board;
        Sudoku::has_unique_solution(&sudoku).then_some(sudoku)
    }
}
