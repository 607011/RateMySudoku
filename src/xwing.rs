use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    pub fn find_xwing_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for x-wings in rows
        for num in 1..=9 {
            for row1 in 0..8 {
                // We don't need to check the last row
                let mut cols1 = Vec::new();
                // Find columns with candidate `num` in this row
                for col in 0..9 {
                    if self.candidates[row1][col].contains(&num) {
                        cols1.push(col);
                    }
                }
                if cols1.len() != 2 {
                    continue;
                }
                // Find another row with the same columns
                for row2 in (row1 + 1)..9 {
                    let mut cols2 = Vec::new();
                    // Find columns with candidate `num` in this row
                    for col in 0..9 {
                        if self.candidates[row2][col].contains(&num) {
                            cols2.push(col);
                        }
                    }
                    // If we found another row with the same columns, we have an X-Wing
                    if cols2.len() != 2 || cols1 != cols2 {
                        continue;
                    }
                    log::info!(
                        "Found x-wing {:?} in rows {} and {} at columns {:?}",
                        num,
                        row1,
                        row2,
                        cols1
                    );
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: cols1[0],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: cols1[1],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: cols2[0],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: cols2[1],
                        num,
                    });
                    // Remove the candidate from other cells in the same columns
                    for row in 0..9 {
                        if row == row1 || row == row2 {
                            continue;
                        }
                        for &col in &cols1 {
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row,
                                    col,
                                    num,
                                });
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row1]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_xwing_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for x-wings in columns
        for num in 1..=9 {
            for col1 in 0..8 {
                // We don't need to check the last column
                let mut rows1 = Vec::new();

                // Find rows with candidate `num` in this column
                for row in 0..9 {
                    if self.candidates[row][col1].contains(&num) {
                        rows1.push(row);
                    }
                }
                if rows1.len() != 2 {
                    continue;
                }
                // Find another column with the same rows
                for col2 in (col1 + 1)..9 {
                    let mut rows2 = Vec::new();
                    // Find rows with candidate `num` in this column
                    for row in 0..9 {
                        if self.candidates[row][col2].contains(&num) {
                            rows2.push(row);
                        }
                    }
                    // If we found another column with the same rows, we have an X-Wing
                    if rows2.len() != 2 || rows1 != rows2 {
                        continue;
                    }
                    log::info!(
                        "Found X-Wing {:?} in columns {} and {} at rows {:?}",
                        num,
                        col1,
                        col2,
                        rows1
                    );
                    result.candidates_affected.push(Candidate {
                        row: rows1[0],
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows1[1],
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows2[0],
                        col: col2,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows2[1],
                        col: col2,
                        num,
                    });
                    // Mark removable candidates from other cells in the same rows
                    for &row in &rows1 {
                        for col in 0..9 {
                            if col == col1 || col == col2 {
                                continue;
                            }
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row,
                                    col,
                                    num,
                                });
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col1]);
                        return result;
                    }
                }
            }
        }
        result
    }

    /// Find and resolve X-Wing candidates.
    /// An X-Wing occurs when a digit can only go in two rows and two columns, forming a rectangle.
    /// In this case, the digit can be removed from all other cells in the same rows and columns.
    pub fn find_xwing(&self) -> StrategyResult {
        log::info!("Finding X-Wings in rows");
        let result = self.find_xwing_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::XWing,
                removals: result,
            };
        }
        log::info!("Finding X-Wings in columns");
        let result = self.find_xwing_in_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::XWing,
                removals: result,
            };
        }
        StrategyResult::empty()
    }
}
