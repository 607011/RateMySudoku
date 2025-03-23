use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};
use std::collections::HashSet;

impl Sudoku {
    pub fn find_pointing_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                for num in 1..=9 {
                    // Collect unique rows where candidate `num` appears in this box
                    let rows_with_num: HashSet<usize> = (0..3)
                        .flat_map(|i| (0..3).map(move |j| (box_row + i, box_col + j)))
                        .filter(|&(row, col)| self.candidates[row][col].contains(&num))
                        .map(|(row, _)| row)
                        .collect();
                    // `num` must appear in exactly one row within the box
                    if rows_with_num.len() != 1 {
                        continue;
                    }
                    let row = *rows_with_num.iter().next().unwrap();
                    // Check how many cells in this box's row contain the candidate
                    let box_cells_with_num = (box_col..box_col + 3)
                        .filter(|&col| self.candidates[row][col].contains(&num))
                        .count();
                    // For a proper pointing pair, we want 2 cells in this box's row
                    if box_cells_with_num != 2 {
                        continue;
                    }
                    // Check if there are candidates to remove outside the box
                    for col in 0..9 {
                        if (col < box_col || col >= box_col + 3)
                            && self.candidates[row][col].contains(&num)
                        {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col,
                                num,
                            });
                        }
                    }
                    if result.will_remove_candidates() {
                        // For each cell with the candidate in this box and row, add it to affected candidates
                        for col in box_col..box_col + 3 {
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_affected.push(Candidate { row, col, num });
                            }
                        }
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                for num in 1..=9 {
                    // Collect unique columns where candidate `num` appears in this box
                    let cols_with_num: HashSet<usize> = (0..3)
                        .flat_map(|i| (0..3).map(move |j| (box_row + j, box_col + i)))
                        .filter(|&(row, col)| self.candidates[row][col].contains(&num))
                        .map(|(_, col)| col)
                        .collect();
                    // `num` must appear exactly one column within the box
                    if cols_with_num.len() != 1 {
                        continue;
                    }
                    let col = *cols_with_num.iter().next().unwrap();
                    // Check how many cells in this box's row contain the candidate
                    let box_cells_with_num = (box_col..box_col + 3)
                        .filter(|&row| self.candidates[row][col].contains(&num))
                        .count();
                    // For a proper pointing pair, we want 2 cells in this box's column
                    if box_cells_with_num != 2 {
                        continue;
                    }
                    for row in 0..9 {
                        if (row < box_row || row >= box_row + 3)
                            && self.candidates[row][col].contains(&num)
                        {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col,
                                num,
                            });
                        }
                    }
                    if result.will_remove_candidates() {
                        // For each cell with the candidate in this box and column, add it to affected candidates
                        for row in box_row..box_row + 3 {
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_affected.push(Candidate { row, col, num });
                            }
                        }
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check each 3x3 box
        for box_idx in 0..9 {
            let box_row = (box_idx / 3) * 3;
            let box_col = (box_idx % 3) * 3;
            // For each possible digit 1-9
            for num in 1..=9 {
                // Find all cells in this box containing the candidate
                let mut cells_with_num = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        let row = box_row + r;
                        let col = box_col + c;
                        if self.candidates[row][col].contains(&num) {
                            cells_with_num.push((row, col));
                        }
                    }
                }
                // Skip if none or too many cells have this candidate
                if cells_with_num.len() != 2 {
                    continue;
                }
                // Check if all cells with this candidate are in the same row
                let rows: HashSet<_> = cells_with_num.iter().map(|&(r, _)| r).collect();
                if rows.len() != 1 {
                    continue;
                }
                let row = *rows.iter().next().unwrap();
                // See if we can remove this candidate from other cells in the same row
                for col in 0..9 {
                    // Skip cells in the current box
                    if col >= box_col && col < box_col + 3 {
                        continue;
                    }

                    if self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    // Add the source cells as affected candidates
                    for &(_, col) in &cells_with_num {
                        result.candidates_affected.push(Candidate { row, col, num });
                    }
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
                // Check if all cells with this candidate are in the same column
                let cols: HashSet<_> = cells_with_num.iter().map(|&(_, c)| c).collect();
                if cols.len() != 1 {
                    continue;
                }
                let col = *cols.iter().next().unwrap();
                // See if we can remove this candidate from other cells in the same column
                for row in 0..9 {
                    // Skip cells in the current box
                    if row >= box_row && row < box_row + 3 {
                        continue;
                    }
                    if self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    // Add the source cells as affected candidates
                    for &(row, _) in &cells_with_num {
                        result.candidates_affected.push(Candidate { row, col, num });
                    }
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_pointing_pair(&self) -> StrategyResult {
        log::info!("Finding pointing pair in rows");
        let result = self.find_pointing_pair_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::PointingPair,
                removals: result,
            };
        }
        log::info!("Finding pointing pair in columns");
        let result = self.find_pointing_pair_in_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::PointingPair,
                removals: result,
            };
        }
        log::info!("Finding pointing pair in boxes");
        let result = self.find_pointing_pair_in_boxes();
        StrategyResult {
            strategy: Strategy::PointingPair,
            removals: result,
        }
    }
}
