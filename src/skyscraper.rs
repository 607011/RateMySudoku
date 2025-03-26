use crate::{Candidate, EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    pub fn find_skyscraper_in_rows(&self) -> RemovalResult {
        // The Skyscraper pattern occurs when the same candidate appears exactly twice in two rows
        // The pattern forms a rectangle where the candidates in corners can see each other
        let mut result = RemovalResult::empty();
        for num in 1..=9 {
            // Find all candidates for the number that appear exactly twice in a row
            let candidates: Vec<Vec<Candidate>> = (0..9)
                .filter_map(|row| {
                    let row_candidates: Vec<Candidate> = (0..9)
                        .filter_map(|col| {
                            if self.candidates[row][col].contains(&num) {
                                Some(Candidate { row, col, num })
                            } else {
                                None
                            }
                        })
                        .take(3) // Limit to at most 3 candidates
                        .collect();
                    if row_candidates.len() == 2 {
                        Some(row_candidates)
                    } else {
                        None
                    }
                })
                .collect();
            // Only keep the pairs of candidates that form a Skyscraper
            let mut skyscraper_candidates = Vec::new();
            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    let base1 = &candidates[i];
                    let base2 = &candidates[j];
                    if base1[0].col == base2[0].col {
                        // Bases share the same column
                        if (base1[1].col < base1[0].col && base2[1].col < base1[0].col)
                            || (base1[1].col > base1[0].col && base2[1].col > base1[0].col)
                        {
                            // Tops are on the same side (either left or right)
                            skyscraper_candidates.push((base1.clone(), base2.clone()));
                        }
                    } else if base1[1].col == base2[1].col {
                        // Bases share the same column for the other pair
                        if (base1[0].col < base1[1].col && base2[0].col < base1[1].col)
                            || (base1[0].col > base1[1].col && base2[0].col > base1[1].col)
                        {
                            // Tops are on the same side (either left or right)
                            skyscraper_candidates.push((base1.clone(), base2.clone()));
                        }
                    }
                }
            }
            if skyscraper_candidates.is_empty() {
                continue;
            }
            let row_pair = &skyscraper_candidates[0];
            let row1 = &row_pair.0;
            let row2 = &row_pair.1;
            if row1[0].col == row2[0].col {
                // The two candidates are in the same column; we found the base
                if row1[1].col != row2[1].col {
                    let base1 = &row1[0];
                    let base2 = &row2[0];
                    let top1 = &row1[1];
                    let top2 = &row2[1];
                    log::info!(
                        "Skyscraper with candidate {} in {:?}",
                        num,
                        skyscraper_candidates
                    );
                    result.candidates_affected.push(*base1);
                    result.candidates_affected.push(*base2);
                    result.candidates_affected.push(*top1);
                    result.candidates_affected.push(*top2);
                    // Mark candidates as removable that are in the same column as top1 and in the same box as top2
                    let box_start_row = (top2.row / 3) * 3;
                    for row in box_start_row..box_start_row + 3 {
                        if self.candidates[row][top1.col].contains(&num) {
                            assert!(self.board[row][top1.col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col: top1.col,
                                num,
                            });
                        }
                    }
                    // Mark candidates as removable that are in the same column as top2 and in the same box as top1
                    let box_start_row = (top1.row / 3) * 3;
                    for row in box_start_row..box_start_row + 3 {
                        if self.candidates[row][top2.col].contains(&num) {
                            assert!(self.board[row][top2.col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col: top2.col,
                                num,
                            });
                        }
                    }
                }
            } else if row1[1].col == row2[1].col {
                // The two candidates are in the same column; we found the base
                if row1[0].col != row2[0].col {
                    let base1 = &row1[1];
                    let base2 = &row2[1];
                    let top1 = &row1[0];
                    let top2 = &row2[0];
                    log::info!("Skyscraper with candidate {} in {:?}", num, candidates);
                    result.candidates_affected.push(*base1);
                    result.candidates_affected.push(*base2);
                    result.candidates_affected.push(*top1);
                    result.candidates_affected.push(*top2);
                    let box_start_row = (top2.row / 3) * 3;
                    // Mark candidates as removable that are in the same column as top1 and in the same box as top2
                    for row in box_start_row..box_start_row + 3 {
                        if self.candidates[row][top1.col].contains(&num) {
                            assert!(self.board[row][top1.col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col: top1.col,
                                num,
                            });
                        }
                    }
                    // Mark candidates as removable that are in the same column as top2 and in the same box as top1
                    let box_start_row = (top1.row / 3) * 3;
                    for row in box_start_row..box_start_row + 3 {
                        if self.candidates[row][top2.col].contains(&num) {
                            assert!(self.board[row][top2.col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col: top2.col,
                                num,
                            });
                        }
                    }
                }
            }
            if result.will_remove_candidates() {
                result.unit = Some(Unit::Row);
                result.unit_index = Some(vec![row1[0].row, row2[0].row]);
                return result;
            }
        }

        result
    }

    pub fn find_skyscraper_in_cols(&self) -> RemovalResult {
        // The Skyscraper pattern occurs when the same candidate appears exactly twice in two columns
        // The pattern forms a rectangle where the candidates in corners can see each other
        let mut result = RemovalResult::empty();
        for num in 1..=9 {
            let candidates: Vec<Vec<Candidate>> = (0..9)
                .filter_map(|col| {
                    let col_candidates: Vec<Candidate> = (0..9)
                        .filter_map(|row| {
                            if self.candidates[row][col].contains(&num) {
                                Some(Candidate { row, col, num })
                            } else {
                                None
                            }
                        })
                        .take(3) // Limit to at most 3 candidates
                        .collect();
                    if col_candidates.len() == 2 {
                        Some(col_candidates)
                    } else {
                        None
                    }
                })
                .collect();

            // Only keep the pairs of candidates that form a Skyscraper
            let mut skyscraper_candidates = Vec::new();
            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    let base1 = &candidates[i];
                    let base2 = &candidates[j];
                    if base1[0].row == base2[0].row {
                        // Bases share the same row
                        if (base1[1].row < base1[0].row && base2[1].row < base1[0].row)
                            || (base1[1].row > base1[0].row && base2[1].row > base1[0].row)
                        {
                            // Tops are on the same side (either above or below)
                            skyscraper_candidates.push((base1.clone(), base2.clone()));
                        }
                    } else if base1[1].row == base2[1].row {
                        // Bases share the same row for the other pair
                        if (base1[0].row < base1[1].row && base2[0].row < base1[1].row)
                            || (base1[0].row > base1[1].row && base2[0].row > base1[1].row)
                        {
                            // Tops are on the same side (either above or below)
                            skyscraper_candidates.push((base1.clone(), base2.clone()));
                        }
                    }
                }
            }
            if skyscraper_candidates.is_empty() {
                continue;
            }
            let col_pair = &skyscraper_candidates[0];
            let col1 = &col_pair.0;
            let col2 = &col_pair.1;
            if col1[0].row == col2[0].row {
                // The two candidates are in the same row; we found the base
                if col1[1].row != col2[1].row {
                    let base1 = &col1[0];
                    let base2 = &col2[0];
                    let top1 = &col1[1];
                    let top2 = &col2[1];
                    log::info!("Skyscraper with candidate {} in {:?}", num, candidates);
                    result.candidates_affected.push(*base1);
                    result.candidates_affected.push(*base2);
                    result.candidates_affected.push(*top1);
                    result.candidates_affected.push(*top2);
                    // Mark candidates as removable that are in the same row as top1 and in the same box as top2
                    let box_start_col = (top2.col / 3) * 3;
                    for col in box_start_col..box_start_col + 3 {
                        if self.candidates[top1.row][col].contains(&num) {
                            assert!(self.board[top1.row][col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: top1.row,
                                col,
                                num,
                            });
                        }
                    }
                    // Mark candidates as removable that are in the same row as top2 and in the same box as top1
                    let box_start_col = (top1.col / 3) * 3;
                    for col in box_start_col..box_start_col + 3 {
                        if self.candidates[top2.row][col].contains(&num) {
                            assert!(self.board[top2.row][col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: top2.row,
                                col,
                                num,
                            });
                        }
                    }
                }
            } else if col1[1].row == col2[1].row {
                // The two candidates are in the same row; we found the base
                if col1[0].row != col2[0].row {
                    let base1 = &col1[1];
                    let base2 = &col2[1];
                    let top1 = &col1[0];
                    let top2 = &col2[0];
                    log::info!("Skyscraper with candidate {} in {:?}", num, candidates);
                    result.candidates_affected.push(*base1);
                    result.candidates_affected.push(*base2);
                    result.candidates_affected.push(*top1);
                    result.candidates_affected.push(*top2);
                    let box_start_col = (top2.col / 3) * 3;
                    // Mark candidates as removable that are in the same row as top1 and in the same box as top2
                    for col in box_start_col..box_start_col + 3 {
                        if self.candidates[top1.row][col].contains(&num) {
                            assert!(self.board[top1.row][col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: top1.row,
                                col,
                                num,
                            });
                        }
                    }
                    // Mark candidates as removable that are in the same row as top2 and in the same box as top1
                    let box_start_col = (top1.col / 3) * 3;
                    for col in box_start_col..box_start_col + 3 {
                        if self.candidates[top2.row][col].contains(&num) {
                            assert!(self.board[top2.row][col] == EMPTY);
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row: top2.row,
                                col,
                                num,
                            });
                        }
                    }
                }
            }
            if result.will_remove_candidates() {
                result.unit = Some(Unit::Column);
                result.unit_index = Some(vec![col1[0].col, col2[0].col]);
                return result;
            }
        }
        result
    }

    pub fn find_skyscraper(&self) -> StrategyResult {
        log::info!("Finding Skyscraper in rows");
        let result = self.find_skyscraper_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::Skyscraper,
                removals: result,
            };
        }
        log::info!("Finding Skyscraper in columns");
        let result = self.find_skyscraper_in_cols();
        StrategyResult {
            strategy: Strategy::Skyscraper,
            removals: result,
        }
    }
}
