use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

impl Sudoku {
    pub fn find_skyscraper_rows(&self) -> RemovalResult {
        // The Skyscraper pattern occurs when the same candidate appears exactly twice in two rows
        // The pattern forms a rectangle where the candidates in corners can see each other
        let mut result = RemovalResult::empty();

        for num in 1..=9 {
            let rows_with_digit: Vec<(usize, Vec<usize>)> = (0..9)
                .map(|row| {
                    let positions: Vec<usize> = (0..9)
                        .filter(|&col| self.candidates[row][col].contains(&num))
                        .collect();
                    (row, positions)
                })
                .filter(|(_, positions)| positions.len() == 2)
                .collect();
            for i in 0..rows_with_digit.len() {
                for j in i + 1..rows_with_digit.len() {
                    let (row1, pos1) = &rows_with_digit[i];
                    let (row2, pos2) = &rows_with_digit[j];
                    // Check if we have a valid skyscraper pattern:
                    // - One common column
                    // - One different column for each row
                    if pos1[0] == pos2[0] && pos1[1] != pos2[1] {
                        let common_col = pos1[0];
                        let diff_col1 = pos1[1];
                        let diff_col2 = pos2[1];
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: diff_col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: diff_col2,
                            num,
                        });
                        result.unit_index = Some(vec![*row1]);
                    } else if pos1[0] == pos2[1] && pos1[1] != pos2[0] {
                        let common_col = pos1[0];
                        let diff_col1 = pos1[1];
                        let diff_col2 = pos2[0];
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: diff_col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: diff_col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: common_col,
                            num,
                        });
                        result.unit_index = Some(vec![*row1]);
                    } else if pos1[1] == pos2[0] && pos1[0] != pos2[1] {
                        let common_col = pos1[1];
                        let diff_col1 = pos1[0];
                        let diff_col2 = pos2[1];
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: diff_col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: diff_col2,
                            num,
                        });
                        result.unit_index = Some(vec![*row1]);
                    } else if pos1[1] == pos2[1] && pos1[0] != pos2[0] {
                        let common_col = pos1[1];
                        let diff_col1 = pos1[0];
                        let diff_col2 = pos2[0];
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: diff_col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row1,
                            col: common_col,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: diff_col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: *row2,
                            col: common_col,
                            num,
                        });
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![*row1]);
                    }
                }
            }
        }
        result
    }

    pub fn find_skyscraper_cols(&self) -> RemovalResult {
        // The Skyscraper pattern occurs when the same candidate appears exactly twice in two columns
        // The pattern forms a rectangle where the candidates in corners can see each other
        let mut result = RemovalResult::empty();

        for num in 1..=9 {
            let cols_with_digit: Vec<(usize, Vec<usize>)> = (0..9)
                .map(|col| {
                    let positions: Vec<usize> = (0..9)
                        .filter(|&row| self.candidates[row][col].contains(&num))
                        .collect();
                    (col, positions)
                })
                .filter(|(_, positions)| positions.len() == 2)
                .collect();

            for i in 0..cols_with_digit.len() {
                for j in i + 1..cols_with_digit.len() {
                    let (col1, pos1) = &cols_with_digit[i];
                    let (col2, pos2) = &cols_with_digit[j];

                    // Check if we have a valid skyscraper pattern:
                    // - One common row
                    // - One different row for each column
                    if pos1[0] == pos2[0] && pos1[1] != pos2[1] {
                        let common_row = pos1[0];
                        let diff_row1 = pos1[1];
                        let diff_row2 = pos2[1];
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row1,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row2,
                            col: *col2,
                            num,
                        });
                        result.unit_index = Some(vec![*col1]);
                    } else if pos1[0] == pos2[1] && pos1[1] != pos2[0] {
                        let common_row = pos1[0];
                        let diff_row1 = pos1[1];
                        let diff_row2 = pos2[0];
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row1,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row2,
                            col: *col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col2,
                            num,
                        });
                        result.unit_index = Some(vec![*col1]);
                    } else if pos1[1] == pos2[0] && pos1[0] != pos2[1] {
                        let common_row = pos1[1];
                        let diff_row1 = pos1[0];
                        let diff_row2 = pos2[1];
                        result.candidates_affected.push(Candidate {
                            row: diff_row1,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row2,
                            col: *col2,
                            num,
                        });
                        result.unit_index = Some(vec![*col1]);
                    } else if pos1[1] == pos2[1] && pos1[0] != pos2[0] {
                        let common_row = pos1[1];
                        let diff_row1 = pos1[0];
                        let diff_row2 = pos2[0];
                        result.candidates_affected.push(Candidate {
                            row: diff_row1,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col1,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: diff_row2,
                            col: *col2,
                            num,
                        });
                        result.candidates_affected.push(Candidate {
                            row: common_row,
                            col: *col2,
                            num,
                        });
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![*col1]);
                    }
                }
            }
        }
        result
    }

    pub fn find_skyscraper(&self) -> StrategyResult {
        log::info!("Finding Skyscraper in rows");
        let result = self.find_skyscraper_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::Skyscraper,
                removals: result,
            };
        }
        log::info!("Finding Skyscraper in columns");
        let result = self.find_skyscraper_cols();
        StrategyResult {
            strategy: Strategy::Skyscraper,
            removals: result,
        }
    }
}
