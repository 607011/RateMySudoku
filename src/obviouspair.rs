use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    fn remove_box_candidates(&self, result: &mut RemovalResult) -> bool {
        if result.candidates_affected.is_empty() {
            return false;
        }
        // Check if all candidates are in the same box
        let first_box_index =
            (result.candidates_affected[0].row / 3) * 3 + (result.candidates_affected[0].col / 3);
        if result.candidates_affected.iter().all(|candidate| {
            let box_index = (candidate.row / 3) * 3 + (candidate.col / 3);
            box_index == first_box_index
        }) {
            let candidate = result.candidates_affected[0];
            let (row_start, col_start) = Self::get_box_start(candidate.row, candidate.col);
            for row in row_start..row_start + 3 {
                for col in col_start..col_start + 3 {
                    // Skip the cells that are part of the locked pair
                    if result
                        .candidates_affected
                        .iter()
                        .any(|candidate| candidate.col == col)
                        && result
                            .candidates_affected
                            .iter()
                            .any(|candidate| candidate.row == row)
                    {
                        continue;
                    }
                    // Mark the candidates from the cells in the same box as removable
                    for &num in &result
                        .candidates_affected
                        .iter()
                        .map(|c| c.num)
                        .collect::<Vec<_>>()
                    {
                        if self.candidates[row][col].contains(&num) {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col,
                                num,
                            });
                        }
                    }
                }
            }
            return true;
        }
        false
    }

    pub fn find_obvious_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.candidates[row][col].is_empty() || self.candidates[row][col].len() != 2 {
                    continue;
                }

                let pair = self.candidates[row][col].clone();

                // If the pair is not found in exactly two cells, skip
                if self.candidates[row]
                    .iter()
                    .filter(|&cell| pair.is_subset(cell))
                    .count()
                    != 2
                {
                    continue;
                }

                // Find pair in the same row
                for i in (col + 1)..9 {
                    if self.candidates[row][i] != pair {
                        continue;
                    }
                    // Found a pair, mark these candidates from other cells
                    // in the same row as about to be removed
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != col && j != i {
                            for &num in &nums {
                                if self.candidates[row][j].contains(&num) {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row,
                                        col: j,
                                        num,
                                    });
                                }
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col, num }));
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col: i, num }));
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.candidates[row][col].len() != 2 {
                    continue;
                }

                let pair = self.candidates[row][col].clone();

                // If the pair is not found in exactly two cells, skip
                if self
                    .candidates
                    .iter()
                    .filter(|row| pair.is_subset(&row[col]))
                    .count()
                    != 2
                {
                    continue;
                }

                // Find pair in same column
                for i in (row + 1)..9 {
                    if self.candidates[i][col].is_empty() || self.candidates[i][col] != pair {
                        continue;
                    }
                    // Found a pair, mark these candidates from other cells
                    // in the same column as about to be removed
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != row && j != i {
                            for &num in &nums {
                                if self.candidates[j][col].contains(&num) {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row: j,
                                        col,
                                        num,
                                    });
                                }
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col, num }));
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row: i, col, num }));
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                for r1 in 0..3 {
                    for c1 in 0..3 {
                        let row1 = start_row + r1;
                        let col1 = start_col + c1;

                        if self.candidates[row1][col1].len() != 2 {
                            continue;
                        }

                        let pair = self.candidates[row1][col1].clone();

                        for r2 in 0..3 {
                            for c2 in 0..3 {
                                let row2 = start_row + r2;
                                let col2 = start_col + c2;

                                // Skip same cell or already checked pairs
                                if (row1 == row2 && col1 == col2) || (r2 * 3 + c2 <= r1 * 3 + c1) {
                                    continue;
                                }

                                if self.candidates[row2][col2] != pair {
                                    continue;
                                }

                                // Found a pair, remove these candidates from other cells in the same box
                                let nums: Vec<u8> = pair.iter().cloned().collect();
                                for r in 0..3 {
                                    for c in 0..3 {
                                        let row = start_row + r;
                                        let col = start_col + c;
                                        if (row != row1 || col != col1)
                                            && (row != row2 || col != col2)
                                        {
                                            for &num in &nums {
                                                if self.candidates[row][col].contains(&num) {
                                                    result
                                                        .candidates_about_to_be_removed
                                                        .insert(Candidate { row, col, num });
                                                }
                                            }
                                        }
                                    }
                                }
                                if result.will_remove_candidates() {
                                    result.candidates_affected.extend(pair.iter().map(|&num| {
                                        Candidate {
                                            row: row1,
                                            col: col1,
                                            num,
                                        }
                                    }));
                                    result.candidates_affected.extend(
                                        self.candidates[row2][col2].iter().map(|&num| Candidate {
                                            row: row2,
                                            col: col2,
                                            num,
                                        }),
                                    );
                                    result.unit = Some(Unit::Box);
                                    result.unit_index = Some(vec![box_row * 3 + box_col]);
                                    return result;
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair(&self) -> StrategyResult {
        log::info!("Finding obvious pairs in rows");
        let removal_result = self.find_obvious_pair_in_rows();
        // Check if the pair exists in the same box (locked pair)
        let mut row_removal = removal_result.clone();
        if !row_removal.candidates_affected.is_empty()
            && self.remove_box_candidates(&mut row_removal)
        {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: row_removal,
            };
        }
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousPair,
                removals: removal_result,
            };
        }
        log::info!("Finding obvious pairs in columns");
        let removal_result = self.find_obvious_pair_in_cols();
        // Check if the pair exists in the same box (locked pair)
        let mut row_removal = removal_result.clone();
        if !row_removal.candidates_affected.is_empty()
            && self.remove_box_candidates(&mut row_removal)
        {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: row_removal,
            };
        }
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousPair,
                removals: removal_result,
            };
        }
        log::info!("Finding obvious pairs in boxes");
        let removal_result = self.find_obvious_pair_in_boxes();
        StrategyResult {
            strategy: Strategy::ObviousPair,
            removals: removal_result,
        }
    }
}
