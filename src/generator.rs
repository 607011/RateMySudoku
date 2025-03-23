use crate::{EMPTY, Sudoku};
use rand::seq::SliceRandom;

impl Sudoku {
    /// Generates a new Sudoku puzzle with a given number of filled cells.
    /// The puzzle is guaranteed to have a unique solution.
    pub fn generate(filled_cells: usize) -> Option<Self> {
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

            // Count solutions using backtracking (up to 2)
            fn count_solutions(sudoku: &mut Sudoku, count: &mut usize, max_count: usize) -> bool {
                if *count >= max_count {
                    return true; // Early return if we already found enough solutions
                }
                let mut found_empty = false;
                let mut empty_row = 0;
                let mut empty_col = 0;
                'find_empty: for r in 0..9 {
                    for c in 0..9 {
                        if sudoku.board[r][c] == EMPTY {
                            empty_row = r;
                            empty_col = c;
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
                    if count_solutions(sudoku, count, max_count) {
                        return true;
                    }
                    // Backtrack
                    sudoku.board[empty_row][empty_col] = EMPTY;
                }
                false
            }

            let mut solution_count = 0;
            count_solutions(&mut test_sudoku, &mut solution_count, 2);

            if solution_count != 1 {
                return None;
            }
        }
        Some(sudoku)
    }
}
