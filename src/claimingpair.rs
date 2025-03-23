use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    fn is_claiming_pair(cells_with_num: &[usize]) -> bool {
        cells_with_num.len() == 2 && (cells_with_num[0] / 3 == cells_with_num[1] / 3)
    }

    pub fn find_claiming_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            for num in 1..=9 {
                // Track cells with candidate `num` in this row
                let cells_with_num: Vec<_> = (0..9)
                    .filter(|&col| self.candidates[row][col].contains(&num))
                    .collect();
                if !Self::is_claiming_pair(&cells_with_num) {
                    continue;
                }
                let col1 = cells_with_num[0];
                let col2 = cells_with_num[1];
                let box_col = col1 / 3;
                let start_row = 3 * (row / 3);
                // Remove this candidate from other cells in the same box but different row
                for r in start_row..start_row + 3 {
                    if r == row {
                        continue; // Skip the original row
                    }
                    for c in (box_col * 3)..(box_col * 3 + 3) {
                        if self.candidates[r][c].contains(&num) {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: r,
                                col: c,
                                num,
                            });
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row,
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row,
                        col: col2,
                        num,
                    });
                    result.unit = Some(Unit::Row);
                    result.unit_index = Some(vec![row]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_claiming_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for col in 0..9 {
            for num in 1..=9 {
                let cells_with_num: Vec<_> = (0..9)
                    .filter(|&row| self.candidates[row][col].contains(&num))
                    .collect();
                if !Self::is_claiming_pair(&cells_with_num) {
                    continue;
                }
                let row1 = cells_with_num[0];
                let row2 = cells_with_num[1];
                let box_idx = row1 / 3;
                let start_col = 3 * (col / 3);
                // Remove this candidate from other cells in the same box but different column
                for c in start_col..start_col + 3 {
                    if c == col {
                        continue; // Skip the original column
                    }
                    for r in (box_idx * 3)..(box_idx * 3 + 3) {
                        if self.candidates[r][c].contains(&num) {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: r,
                                col: c,
                                num,
                            });
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col,
                        num,
                    });
                    result.unit = Some(Unit::Column);
                    result.unit_index = Some(vec![col]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_claiming_pair(&self) -> StrategyResult {
        log::info!("Finding claiming pairs in rows");
        let result = self.find_claiming_pair_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ClaimingPair,
                removals: result,
            };
        }
        log::info!("Finding claiming pairs in columns");
        let result = self.find_claiming_pair_in_cols();
        StrategyResult {
            strategy: Strategy::ClaimingPair,
            removals: result,
        }
    }
}
