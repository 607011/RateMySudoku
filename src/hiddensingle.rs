use crate::{EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    /// Finds and resolves "hidden single" candidates in the Sudoku puzzle.
    ///
    /// A hidden single occurs when a digit can only go in one cell within a group (row, column, or box),
    /// even though that cell may have multiple candidates.
    ///
    /// Returns the number of notes removed as a result of placing new digits.
    pub fn find_hidden_single(&self) -> StrategyResult {
        let mut result = StrategyResult::new(Strategy::HiddenSingle);
        let box_result = self.find_hidden_single_box();
        if box_result.will_remove_candidates() {
            result.removals = box_result;
            return result;
        }
        let row_result = self.find_hidden_single_row();
        if row_result.will_remove_candidates() {
            result.removals = row_result;
            return result;
        }
        let col_result = self.find_hidden_single_col();
        if col_result.will_remove_candidates() {
            result.removals = col_result;
            return result;
        }
        result
    }

    pub fn find_hidden_single_row(&self) -> RemovalResult {
        // Check for hidden singles in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] > 0 {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != col && self.candidates[row][i].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut result = self.collect_set_num(num, row, col);
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        RemovalResult::empty()
    }

    pub fn find_hidden_single_col(&self) -> RemovalResult {
        // Check for hidden singles in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != row && self.candidates[i][col].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut result = self.collect_set_num(num, row, col);
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        RemovalResult::empty()
    }

    pub fn find_hidden_single_box(&self) -> RemovalResult {
        // Check for hidden singles in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                for i in 0..3 {
                    for j in 0..3 {
                        let row = start_row + i;
                        let col = start_col + j;
                        if self.board[row][col] != EMPTY {
                            continue;
                        }
                        for &num in &self.candidates[row][col] {
                            let mut found = false;
                            'box_check: for r in 0..3 {
                                for c in 0..3 {
                                    let check_row = start_row + r;
                                    let check_col = start_col + c;
                                    if (check_row != row || check_col != col)
                                        && self.candidates[check_row][check_col].contains(&num)
                                    {
                                        found = true;
                                        break 'box_check;
                                    }
                                }
                            }
                            if !found {
                                let mut result = self.collect_set_num(num, row, col);
                                result.unit = Some(Unit::Box);
                                result.unit_index = Some(vec![3 * box_row + box_col]);
                                return result;
                            }
                        }
                    }
                }
            }
        }
        RemovalResult::empty()
    }
}
