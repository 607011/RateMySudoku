use crate::{ALL_DIGITS, EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};
use std::collections::HashSet;

impl Sudoku {
    /// Check if there are last digits in any of the rows.
    /// If so, remove it from the notes in the row, column, and box where we've found it.
    /// Set the respective cell to the digit.
    pub fn find_last_digit_in_rows(&self) -> RemovalResult {
        for row in 0..9 {
            // Find the only empty cell in the row, if there's exactly one
            let empty_cells = (0..9)
                .filter(|&col| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.collect_nums_in_row(row))
                .cloned()
                .collect();
            assert_eq!(missing_digits.len(), 1);
            let num = *missing_digits.iter().next().unwrap();
            let col = empty_cells[0];
            let mut result = self.collect_set_num(num, row, col);
            result.unit = Some(Unit::Row);
            result.unit_index = Some(vec![row]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit_in_cols(&self) -> RemovalResult {
        for col in 0..9 {
            let empty_cells = (0..9)
                .filter(|&row| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let row = empty_cells[0];
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.collect_nums_in_col(col))
                .cloned()
                .collect();
            assert_eq!(missing_digits.len(), 1);
            let num = *missing_digits.iter().next().unwrap();
            let mut result = self.collect_set_num(num, row, col);
            result.unit = Some(Unit::Column);
            result.unit_index = Some(vec![col]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit_in_boxes(&self) -> RemovalResult {
        for box_index in 0..9 {
            let start_row = 3 * (box_index / 3);
            let start_col = 3 * (box_index % 3);
            let mut count = 0;
            let mut empty_row = 0;
            let mut empty_col = 0;
            'box_search: for i in 0..3 {
                for j in 0..3 {
                    let row = start_row + i;
                    let col = start_col + j;
                    if self.board[row][col] != EMPTY {
                        continue;
                    }
                    count += 1;
                    empty_row = row;
                    empty_col = col;
                    break 'box_search;
                }
            }
            if count != 1 {
                continue;
            }
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.collect_nums_in_box(box_index))
                .cloned()
                .collect();
            if missing_digits.len() != 1 {
                continue;
            }
            let num = *missing_digits.iter().next().unwrap();
            let mut result = self.collect_set_num(num, empty_row, empty_col);
            result.unit = Some(Unit::Box);
            result.unit_index = Some(vec![box_index]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit(&self) -> StrategyResult {
        let mut result = StrategyResult::new(Strategy::LastDigit);
        log::info!("Finding last digits in rows");
        let removal_result = self.find_last_digit_in_rows();
        if removal_result.will_remove_candidates() {
            result.removals = removal_result;
            return result;
        }
        log::info!("Finding last digits in columns");
        let removal_result = self.find_last_digit_in_cols();
        if removal_result.will_remove_candidates() {
            result.removals = removal_result;
            return result;
        }
        log::info!("Finding last digits in boxes");
        let removal_result = self.find_last_digit_in_boxes();
        result.removals = removal_result;
        result
    }
}
