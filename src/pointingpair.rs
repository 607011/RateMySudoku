use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};
use std::collections::HashSet;

impl Sudoku {
    pub fn find_pointing_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // First iterate over possible digits
        for num in 1..=9 {
            // Then iterate over all rows
            for row in 0..9 {
                // Check each box that intersects with this row
                let box_row = (row / 3) * 3;
                // Find all cells in this row and within the box that have this candidate
                let cells_with_num: Vec<(usize, usize)> = (0..9)
                    .filter(|&col| self.candidates[row][col].contains(&num))
                    .map(|col| (row, col))
                    .filter(|&(_, col)| (col / 3) * 3 == box_row)
                    .collect();
                // For a pointing pair, we need exactly 2 cells in same box
                if cells_with_num.len() != 2 {
                    continue;
                }
                // Check if both cells are in the same box
                let box_indices: HashSet<usize> =
                    cells_with_num.iter().map(|&(_, col)| col / 3).collect();
                if box_indices.len() != 1 {
                    continue;
                }
                // Check there are no other cells in the box with this candidate
                let (box_row, box_col) = Self::get_box_start(row, cells_with_num[0].1);
                // Check if any other cells in the box outside our row have this candidate
                let has_other_cells_with_num = (box_row..box_row + 3)
                    .flat_map(|r| (box_col..box_col + 3).map(move |c| (r, c)))
                    .filter(|&(r, _)| r != row) // Skip cells in our row
                    .any(|(r, c)| self.candidates[r][c].contains(&num));
                if has_other_cells_with_num {
                    continue;
                }
                let box_col = (cells_with_num[0].1 / 3) * 3;
                // Check if there are candidates to remove in the same row outside the box
                for col in 0..9 {
                    if (col < box_col || col >= box_col + 3)
                        && self.candidates[row][col].contains(&num)
                    {
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
                    result.unit = Some(Unit::Row);
                    result.unit_index = Some(vec![row]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // First iterate over possible digits
        for num in 1..=9 {
            // Then iterate over all columns
            for col in 0..9 {
                // Check each box that intersects with this column
                let box_col = (col / 3) * 3;
                // Find all cells in this column and within the box that have this candidate
                let cells_with_num: Vec<(usize, usize)> = (0..9)
                    .filter(|&row| self.candidates[row][col].contains(&num))
                    .map(|row| (row, col))
                    .filter(|&(row, _)| (row / 3) * 3 == box_col)
                    .collect();
                // For a pointing pair, we need exactly 2 cells in same box
                if cells_with_num.len() != 2 {
                    continue;
                }
                // Check if both cells are in the same box
                let box_indices: HashSet<usize> =
                    cells_with_num.iter().map(|&(row, _)| row / 3).collect();
                if box_indices.len() != 1 {
                    continue;
                }
                // Check there are no other cells in the box with this candidate
                let (box_row, box_col) = Self::get_box_start(cells_with_num[0].0, col);
                // Check if any other cells in the box have this candidate
                let has_other_cells_with_num = (box_row..box_row + 3)
                    .flat_map(|r| (box_col..box_col + 3).map(move |c| (r, c)))
                    .filter(|&(_, c)| c != col) // Skip cells in our column
                    .any(|(r, c)| self.candidates[r][c].contains(&num));
                if has_other_cells_with_num {
                    continue;
                }
                let box_row = (cells_with_num[0].0 / 3) * 3;
                // Check if there are candidates to remove in the same column outside the box
                for row in 0..9 {
                    if (row < box_row || row >= box_row + 3)
                        && self.candidates[row][col].contains(&num)
                    {
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
                    result.unit = Some(Unit::Column);
                    result.unit_index = Some(vec![col]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check each 3x3 box
        for box_idx in 0..9 {
            let (box_row, box_col) = Self::get_box_start_from_index(box_idx);
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
