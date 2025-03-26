use crate::{Candidate, EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};
use std::collections::HashSet;

impl Sudoku {
    pub fn find_hidden_triplet_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            // For each position in the row, track which positions have each digit as a candidate
            let mut positions_for_digit: [Vec<usize>; 10] = Default::default();

            // Fill the positions array
            for col in 0..9 {
                if self.board[row][col] == EMPTY {
                    for &num in &self.candidates[row][col] {
                        positions_for_digit[num as usize].push(col);
                    }
                }
            }

            // Look for triplets: digits that appear as candidates in exactly 3 cells
            for d1 in 1..=9 {
                if positions_for_digit[d1].len() != 3 {
                    continue;
                }
                for d2 in (d1 + 1)..=9 {
                    if positions_for_digit[d2].len() != 3 {
                        continue;
                    }
                    for d3 in (d2 + 1)..=9 {
                        if positions_for_digit[d3].len() != 3 {
                            continue;
                        }

                        // Check if these three digits appear in the same 3 cells
                        let pos1: HashSet<_> = positions_for_digit[d1].iter().cloned().collect();
                        let pos2: HashSet<_> = positions_for_digit[d2].iter().cloned().collect();
                        let pos3: HashSet<_> = positions_for_digit[d3].iter().cloned().collect();

                        let intersection: Vec<_> = pos1
                            .intersection(&pos2)
                            .cloned()
                            .collect::<HashSet<_>>()
                            .intersection(&pos3)
                            .cloned()
                            .collect();

                        // If all three digits appear in the same 3 cells, we have a hidden triplet
                        if intersection.len() == 3 {
                            let triplet_cols = intersection;
                            let triplet_digits = [d1 as u8, d2 as u8, d3 as u8];

                            // Mark these candidates as affected
                            for &col in &triplet_cols {
                                for &digit in &triplet_digits {
                                    result.candidates_affected.push(Candidate {
                                        row,
                                        col,
                                        num: digit,
                                    });
                                }
                            }

                            // Remove all other candidates from these cells
                            for &col in &triplet_cols {
                                for &num in &self.candidates[row][col] {
                                    if !triplet_digits.contains(&num) {
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
                                result.unit_index = Some(vec![row]);
                                return result;
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_triplet_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            // For each possible combination of three columns in the row
            for col1 in 0..7 {
                if self.board[row][col1] != EMPTY
                    || self.candidates[row][col1].is_empty()
                    || self.candidates[row][col1].len() > 3
                {
                    continue;
                }
                for col2 in (col1 + 1)..8 {
                    if self.board[row][col2] != EMPTY
                        || self.candidates[row][col2].is_empty()
                        || self.candidates[row][col2].len() > 3
                    {
                        continue;
                    }
                    for col3 in (col2 + 1)..9 {
                        if self.board[row][col3] != EMPTY
                            || self.candidates[row][col3].is_empty()
                            || self.candidates[row][col3].len() > 3
                        {
                            continue;
                        }
                        // Combine candidates from all three cells
                        let combined_candidates: HashSet<u8> = self.candidates[row][col1]
                            .union(&self.candidates[row][col2])
                            .cloned()
                            .collect::<HashSet<u8>>()
                            .union(&self.candidates[row][col3])
                            .cloned()
                            .collect();
                        // If we have exactly 3 unique candidates across these cells, we have a naked triplet
                        if combined_candidates.len() != 3 {
                            continue;
                        }
                        // Store the cells involved in the triplet
                        let triplet_cols = [col1, col2, col3];
                        // Record the candidates in these cells as affected
                        result
                            .candidates_affected
                            .extend(triplet_cols.iter().flat_map(|&col| {
                                combined_candidates
                                    .iter()
                                    .filter(move |&&num| self.candidates[row][col].contains(&num))
                                    .map(move |&num| Candidate { row, col, num })
                            }));
                        // Remove these candidates from other cells in the same row
                        for col in 0..9 {
                            if triplet_cols.contains(&col) {
                                continue; // Skip the triplet cells
                            }
                            combined_candidates
                                .iter()
                                .filter(|&&num| self.candidates[row][col].contains(&num))
                                .for_each(|&num| {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row,
                                        col,
                                        num,
                                    });
                                });
                        }
                        if result.will_remove_candidates() {
                            result.unit = Some(Unit::Row);
                            result.unit_index = Some(vec![row]);
                            return result;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_triplet_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for col in 0..9 {
            // For each possible combination of three rows in the column
            for row1 in 0..7 {
                if self.board[row1][col] != EMPTY
                    || self.candidates[row1][col].is_empty()
                    || self.candidates[row1][col].len() > 3
                {
                    continue;
                }
                for row2 in (row1 + 1)..8 {
                    if self.board[row2][col] != EMPTY
                        || self.candidates[row2][col].is_empty()
                        || self.candidates[row2][col].len() > 3
                    {
                        continue;
                    }
                    for row3 in (row2 + 1)..9 {
                        if self.board[row3][col] != EMPTY
                            || self.candidates[row3][col].is_empty()
                            || self.candidates[row3][col].len() > 3
                        {
                            continue;
                        }
                        // Combine candidates from all three cells
                        let combined_candidates: HashSet<u8> = self.candidates[row1][col]
                            .union(&self.candidates[row2][col])
                            .cloned()
                            .collect::<HashSet<u8>>()
                            .union(&self.candidates[row3][col])
                            .cloned()
                            .collect();
                        // If we have exactly 3 unique candidates across these cells, we have a naked triplet
                        if combined_candidates.len() != 3 {
                            continue;
                        }
                        // Store the cells involved in the triplet
                        let triplet_rows = [row1, row2, row3];
                        // Record the candidates in these cells as affected
                        result
                            .candidates_affected
                            .extend(triplet_rows.iter().flat_map(|&row| {
                                combined_candidates
                                    .iter()
                                    .filter(move |&&num| self.candidates[row][col].contains(&num))
                                    .map(move |&num| Candidate { row, col, num })
                            }));
                        // Remove these candidates from other cells in the same column
                        for row in 0..9 {
                            if triplet_rows.contains(&row) {
                                continue; // Skip the triplet cells
                            }
                            combined_candidates
                                .iter()
                                .filter(|&&num| self.candidates[row][col].contains(&num))
                                .for_each(|&num| {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row,
                                        col,
                                        num,
                                    });
                                });
                        }
                        if result.will_remove_candidates() {
                            result.unit = Some(Unit::Column);
                            result.unit_index = Some(vec![col]);
                            return result;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_triplet_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_idx in 0..9 {
            let start_row = 3 * (box_idx / 3);
            let start_col = 3 * (box_idx % 3);
            // Create a vector of all cells with their position and candidates
            let cells_with_candidates: Vec<_> = (0..3)
                .flat_map(|i| (0..3).map(move |j| (start_row + i, start_col + j)))
                .filter_map(|(row, col)| {
                    if self.board[row][col] == EMPTY
                        && !self.candidates[row][col].is_empty()
                        && self.candidates[row][col].len() <= 3
                    {
                        Some(((row, col), &self.candidates[row][col]))
                    } else {
                        None
                    }
                })
                .collect();
            // Find all combinations of three cells
            for i in 0..cells_with_candidates.len() {
                let ((row1, col1), cands1) = cells_with_candidates[i];
                for j in (i + 1)..cells_with_candidates.len() {
                    let ((row2, col2), cands2) = cells_with_candidates[j];
                    for (_k, ((row3, col3), cands3)) in
                        cells_with_candidates.iter().enumerate().skip(j + 1)
                    {
                        // Combine candidates from all three cells
                        let combined_candidates: HashSet<u8> = cands1
                            .union(cands2)
                            .cloned()
                            .collect::<HashSet<u8>>()
                            .union(cands3)
                            .cloned()
                            .collect();
                        // If we have exactly 3 unique candidates across these cells, we have a naked triplet
                        if combined_candidates.len() != 3 {
                            continue;
                        }
                        // Store the cells involved in the triplet
                        let triplet_cells = [(row1, col1), (row2, col2), (*row3, *col3)];
                        // Record the candidates in these cells as affected
                        result
                            .candidates_affected
                            .extend(triplet_cells.iter().flat_map(|&(row, col)| {
                                combined_candidates
                                    .iter()
                                    .filter(move |&&num| self.candidates[row][col].contains(&num))
                                    .map(move |&num| Candidate { row, col, num })
                            }));
                        // Remove these candidates from other cells in the same box
                        for r in 0..3 {
                            for c in 0..3 {
                                let row = start_row + r;
                                let col = start_col + c;
                                let cell = (row, col);
                                if triplet_cells.contains(&cell) {
                                    continue; // Skip the triplet cells
                                }
                                for &num in &combined_candidates {
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
                        if result.will_remove_candidates() {
                            result.unit = Some(Unit::Box);
                            result.unit_index = Some(vec![box_idx]);
                            return result;
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_triplet(&self) -> StrategyResult {
        log::info!("Finding naked triplets in rows");
        let result = self.find_obvious_triplet_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousTriplet,
                removals: result,
            };
        }
        log::info!("Finding naked triplets in columns");
        let result = self.find_obvious_triplet_in_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousTriplet,
                removals: result,
            };
        }
        log::info!("Finding naked triplets in boxes");
        let result = self.find_obvious_triplet_in_boxes();
        StrategyResult {
            strategy: Strategy::ObviousTriplet,
            removals: result,
        }
    }
}
