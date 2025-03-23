use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    pub fn find_locked_pair_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            for num in 1..=9 {
                // Find all columns in this row that have this candidate
                let cols: Vec<_> = (0..9)
                    .filter(|&col| self.candidates[row][col].contains(&num))
                    .collect();
                if cols.len() != 2 {
                    continue;
                }
                // Check if any other row has this candidate in either of our two columns
                let has_other_row_with_candidate = (0..9).filter(|&r| r != row).any(|r| {
                    cols.iter()
                        .any(|&col| self.candidates[r][col].contains(&num))
                });
                if has_other_row_with_candidate {
                    continue;
                }
                log::info!("Found locked pair {:?} in row {}", num, row);
                result.candidates_affected.push(Candidate {
                    row,
                    col: cols[0],
                    num,
                });
                result.candidates_affected.push(Candidate {
                    row,
                    col: cols[1],
                    num,
                });
                // Remove this candidate from other cells in the same row
                for col in 0..9 {
                    if !cols.contains(&col) && self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    result.unit = Some(Unit::Row);
                    result.unit_index = Some(vec![row]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_locked_pair_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for col in 0..9 {
            for num in 1..=9 {
                // Find all rows in this column that have this candidate
                let rows: Vec<_> = (0..9)
                    .filter(|&row| self.candidates[row][col].contains(&num))
                    .collect();
                if rows.len() != 2 {
                    continue;
                }
                // Check if any other column has this candidate in either of our two rows
                let has_other_col_with_candidate = (0..9).filter(|&c| c != col).any(|c| {
                    rows.iter()
                        .any(|&row| self.candidates[row][c].contains(&num))
                });
                if has_other_col_with_candidate {
                    continue;
                }
                log::info!("Found locked pair {:?} in column {}", num, col);
                result.candidates_affected.push(Candidate {
                    row: rows[0],
                    col,
                    num,
                });
                result.candidates_affected.push(Candidate {
                    row: rows[1],
                    col,
                    num,
                });
                // Remove this candidate from other cells in the same column
                for row in 0..9 {
                    if !rows.contains(&row) && self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    result.unit = Some(Unit::Column);
                    result.unit_index = Some(vec![col]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_locked_pair_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_idx in 0..9 {
            let start_row = 3 * (box_idx / 3);
            let start_col = 3 * (box_idx % 3);
            for num in 1..=9 {
                // Find all cells in this box that have this candidate
                let mut cells_with_num = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        let row = start_row + r;
                        let col = start_col + c;
                        if self.candidates[row][col].contains(&num) {
                            cells_with_num.push((row, col));
                        }
                    }
                }
                if cells_with_num.len() != 2 {
                    continue;
                }
                let (row1, col1) = cells_with_num[0];
                let (row2, col2) = cells_with_num[1];
                // Check if the cells are in the same row
                if row1 != row2 {
                    continue;
                }
                // Check if any other box in the same row has this candidate
                let has_other_box_with_candidate = (0..9)
                    .filter(|&c| c < start_col || c >= start_col + 3)
                    .any(|c| self.candidates[row1][c].contains(&num));
                if has_other_box_with_candidate {
                    continue;
                }
                // Remove this candidate from other cells in the same box but different row
                for r in start_row..start_row + 3 {
                    if r != row1 {
                        for c in start_col..start_col + 3 {
                            if self.candidates[r][c].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row: r,
                                    col: c,
                                    num,
                                });
                            }
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: col2,
                        num,
                    });
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
                // Check if the cells are in the same column
                if col1 != col2 {
                    continue;
                }
                // Check if any other box in the same column has this candidate
                let has_other_box_with_candidate = (0..9)
                    .filter(|&r| r < start_row || r >= start_row + 3)
                    .any(|r| self.candidates[r][col1].contains(&num));
                if has_other_box_with_candidate {
                    continue;
                }
                // Remove this candidate from other cells in the same box but different column
                for c in start_col..start_col + 3 {
                    if c != col1 {
                        for r in start_row..start_row + 3 {
                            if self.candidates[r][c].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row: r,
                                    col: c,
                                    num,
                                });
                            }
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: col2,
                        num,
                    });
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_locked_pair(&self) -> StrategyResult {
        log::info!("Finding locked pairs in rows");
        let result = self.find_locked_pair_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: result,
            };
        }
        log::info!("Finding locked pairs in columns");
        let result = self.find_locked_pair_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: result,
            };
        }
        log::info!("Finding locked pairs in boxes");
        let result = self.find_locked_pair_boxes();
        StrategyResult {
            strategy: Strategy::LockedPair,
            removals: result,
        }
    }
}
