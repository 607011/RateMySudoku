use crate::{EMPTY, Sudoku};
use rand::seq::SliceRandom;

impl Sudoku {
    fn count_solutions(sudoku: &mut Sudoku, count: &mut usize, max_count: usize) -> bool {
        if *count >= max_count {
            return true; // Early return if we already found enough solutions
        }
        let mut found_empty = false;
        let mut empty_row = 0;
        let mut empty_col = 0;
        'find_empty: for row in 0..9 {
            for col in 0..9 {
                if sudoku.board[row][col] == EMPTY {
                    empty_row = row;
                    empty_col = col;
                    found_empty = true;
                    break 'find_empty;
                }
            }
        }
        // If no empty cell is found, we have a solution
        if !found_empty {
            *count += 1;
            return *count >= max_count;
        }
        // Try each possible value
        for num in 1..=9 {
            if !sudoku.can_place(empty_row, empty_col, num) {
                continue;
            }
            // Place and recurse
            sudoku.board[empty_row][empty_col] = num;
            if Self::count_solutions(sudoku, count, max_count) {
                return true;
            }
            // Backtrack
            sudoku.board[empty_row][empty_col] = EMPTY;
        }
        false
    }

    /// Generate a Sudoku puzzle by filling cells incrementally.
    /// This method fills cells one by one, ensuring that the
    /// puzzle has a unique solution.
    /// The `filled_cells` parameter specifies how many cells should remain filled.
    /// The minimum number of cells to fill is 17.
    /// If the puzzle cannot be generated with the specified number of filled cells,
    /// it returns `None`.
    pub fn generate_incrementally(filled_cells: usize) -> Option<Self> {
        let min_cells_to_fill = 17;
        let mut rng = rand::rng();
        let mut all_digits: Vec<u8> = (1..=9).collect();
        let mut sudoku = Sudoku::new();
        let mut filled = 0;
        let mut positions: Vec<(usize, usize)> = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .collect();
        positions.shuffle(&mut rng);

        // Fill cells one by one
        while filled < filled_cells.max(min_cells_to_fill) && !positions.is_empty() {
            let (row, col) = positions.pop().unwrap();
            if sudoku.board[row][col] != EMPTY {
                continue;
            }
            // Shuffle the digits for each attempt
            all_digits.shuffle(&mut rng);
            // Try placing each digit
            for &digit in &all_digits {
                if sudoku.can_place(row, col, digit) {
                    sudoku.board[row][col] = digit;
                    filled += 1;
                    break;
                }
            }
        }
        if filled == filled_cells.max(min_cells_to_fill) {
            sudoku.original_board = sudoku.board;
            let mut solution_count = 0;
            Self::count_solutions(&mut sudoku, &mut solution_count, 2);
            if solution_count != 1 {
                return None;
            }
            return Some(sudoku);
        }

        None
    }

    /// Generate a new Sudoku puzzle with a given number of filled cells.
    /// This method fills the diagonal boxes first, then solves the Sudoku,
    /// and finally removes cells while ensuring a unique solution.
    /// The `filled_cells` parameter specifies how many cells should remain filled.
    pub fn generate_diagonal_fill(filled_cells: usize) -> Option<Self> {
        let mut rng = rand::rng();
        let mut all_digits: Vec<u8> = (1..=9).collect();
        let mut sudoku = Sudoku::new();
        // Fill the 3 diagonal boxes (top-left, middle, bottom-right)
        for box_idx in 0..3 {
            let start_row = box_idx * 3;
            let start_col = box_idx * 3;
            // Fill the box with a shuffled sequence of 1-9
            all_digits.shuffle(&mut rng);
            for (i, &num) in all_digits.iter().enumerate() {
                sudoku.board[start_row + i / 3][start_col + i % 3] = num;
            }
        }
        sudoku.solve_by_backtracking();
        // Get all filled cells that haven't been removed yet
        let mut available_cells: Vec<(usize, usize)> = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .filter(|&(row, col)| sudoku.board[row][col] != EMPTY)
            .collect();
        available_cells.shuffle(&mut rng);
        available_cells.truncate(81 - filled_cells);
        while let Some((row, col)) = available_cells.pop() {
            sudoku.board[row][col] = EMPTY;
            // Check if the puzzle still has a unique solution
            let mut test_sudoku = sudoku.clone();
            let mut solution_count = 0;
            Self::count_solutions(&mut test_sudoku, &mut solution_count, 2);
            if solution_count != 1 {
                return None;
            }
        }
        // Store the current board state as the original board string
        sudoku.original_board = sudoku.board;
        Some(sudoku)
    }
}
