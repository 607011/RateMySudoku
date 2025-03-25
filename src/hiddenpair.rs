use crate::{Candidate, EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};
use std::collections::HashMap;

impl Sudoku {
    pub fn find_hidden_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for hidden pairs in rows
        for row in 0..9 {
            // Find which digits appear in exactly two cells in the row
            let mut digit_locations: HashMap<u8, Vec<usize>> = HashMap::new();
            for col in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    digit_locations.entry(num).or_default().push(col);
                }
            }

            // Find pairs of digits that appear in exactly the same two cells
            let mut digit_pairs: Vec<(u8, u8, usize, usize)> = Vec::new();
            let candidates: Vec<(u8, &Vec<usize>)> = digit_locations
                .iter()
                .filter(|(_, cols)| cols.len() == 2)
                .map(|(&digit, cols)| (digit, cols))
                .collect();

            for (i, (digit1, cols1)) in candidates.iter().enumerate() {
                for (digit2, cols2) in candidates.iter().skip(i + 1) {
                    if cols1 == cols2 {
                        digit_pairs.push((*digit1, *digit2, cols1[0], cols1[1]));
                    }
                }
            }
            result
                .candidates_affected
                .extend(
                    digit_pairs
                        .iter()
                        .flat_map(|&(digit1, digit2, col1, col2)| {
                            vec![
                                Candidate {
                                    row,
                                    col: col1,
                                    num: digit1,
                                },
                                Candidate {
                                    row,
                                    col: col1,
                                    num: digit2,
                                },
                                Candidate {
                                    row,
                                    col: col2,
                                    num: digit1,
                                },
                                Candidate {
                                    row,
                                    col: col2,
                                    num: digit2,
                                },
                            ]
                        }),
                );
            // Apply the strategy: for each hidden pair, remove all other digits from those cells
            for (digit1, digit2, col1, col2) in digit_pairs {
                // Remove all other digits from these two cells
                for &col in &[col1, col2] {
                    for num in 1..=9 {
                        if num != digit1
                            && num != digit2
                            && self.candidates[row][col].contains(&num)
                        {
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
                    // self.remove_box_candidates(&mut result);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_hidden_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for hidden pairs in columns
        for col in 0..9 {
            // Find which digits appear in exactly two cells in the column
            let mut digit_locations: HashMap<u8, Vec<usize>> = HashMap::new();
            for row in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    digit_locations.entry(num).or_default().push(row);
                }
            }

            // Find pairs of digits that appear in exactly the same two cells
            let mut digit_pairs: Vec<(u8, u8, usize, usize)> = Vec::new();
            let candidates: Vec<(u8, &Vec<usize>)> = digit_locations
                .iter()
                .filter(|(_, rows)| rows.len() == 2)
                .map(|(&digit, rows)| (digit, rows))
                .collect();

            for (i, (digit1, rows1)) in candidates.iter().enumerate() {
                for (digit2, rows2) in candidates.iter().skip(i + 1) {
                    if rows1 == rows2 {
                        digit_pairs.push((*digit1, *digit2, rows1[0], rows1[1]));
                    }
                }
            }
            result
                .candidates_affected
                .extend(
                    digit_pairs
                        .iter()
                        .flat_map(|&(digit1, digit2, row1, row2)| {
                            vec![
                                Candidate {
                                    row: row1,
                                    col,
                                    num: digit1,
                                },
                                Candidate {
                                    row: row1,
                                    col,
                                    num: digit2,
                                },
                                Candidate {
                                    row: row2,
                                    col,
                                    num: digit1,
                                },
                                Candidate {
                                    row: row2,
                                    col,
                                    num: digit2,
                                },
                            ]
                        }),
                );
            // Apply the strategy: for each hidden pair, remove all other digits from those cells
            for (digit1, digit2, row1, row2) in digit_pairs {
                // Remove all other digits from these two cells
                for &row in &[row1, row2] {
                    for num in 1..=9 {
                        if num != digit1
                            && num != digit2
                            && self.candidates[row][col].contains(&num)
                        {
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
                    result.unit_index = Some(vec![col]);
                    // self.remove_box_candidates(&mut result);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_hidden_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for hidden pairs in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                // Find which digits appear in exactly two cells in the box
                let mut digit_locations: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
                for r in 0..3 {
                    for c in 0..3 {
                        let row = start_row + r;
                        let col = start_col + c;
                        if self.board[row][col] != EMPTY {
                            continue;
                        }
                        for &num in &self.candidates[row][col] {
                            digit_locations.entry(num).or_default().push((row, col));
                        }
                    }
                }

                // Find pairs of digits that appear in exactly the same two cells
                type DigitPairs = Vec<(u8, u8, (usize, usize), (usize, usize))>;
                let mut digit_pairs: DigitPairs = Vec::new();
                let candidates: Vec<(u8, &Vec<(usize, usize)>)> = digit_locations
                    .iter()
                    .filter(|(_, cells)| cells.len() == 2)
                    .map(|(&digit, cells)| (digit, cells))
                    .collect();

                for (i, (digit1, cells1)) in candidates.iter().enumerate() {
                    for (digit2, cells2) in candidates.iter().skip(i + 1) {
                        if cells1 == cells2 {
                            digit_pairs.push((*digit1, *digit2, cells1[0], cells1[1]));
                        }
                    }
                }
                result
                    .candidates_affected
                    .extend(digit_pairs.iter().flat_map(
                        |&(digit1, digit2, (row1, col1), (row2, col2))| {
                            vec![
                                Candidate {
                                    row: row1,
                                    col: col1,
                                    num: digit1,
                                },
                                Candidate {
                                    row: row1,
                                    col: col1,
                                    num: digit2,
                                },
                                Candidate {
                                    row: row2,
                                    col: col2,
                                    num: digit1,
                                },
                                Candidate {
                                    row: row2,
                                    col: col2,
                                    num: digit2,
                                },
                            ]
                        },
                    ));
                // Apply the strategy: for each hidden pair, remove all other digits from those cells
                for (digit1, digit2, cell1, cell2) in digit_pairs {
                    // Remove all other digits from these two cells
                    for &(row, col) in &[cell1, cell2] {
                        for num in 1..=9 {
                            if num != digit1
                                && num != digit2
                                && self.candidates[row][col].contains(&num)
                            {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row,
                                    col,
                                    num,
                                });
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result.unit_index = Some(vec![box_row * 3 + box_col]);
                        result.unit = Some(Unit::Box);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_hidden_pair(&self) -> StrategyResult {
        log::info!("Finding hidden pairs in rows");
        let removal_result = self.find_hidden_pair_in_rows();
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::HiddenPair,
                removals: removal_result,
            };
        }
        log::info!("Finding hidden pairs in columns");
        let removal_result = self.find_hidden_pair_in_cols();
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::HiddenPair,
                removals: removal_result,
            };
        }
        log::info!("Finding hidden pairs in boxes");
        let removal_result = self.find_hidden_pair_in_boxes();
        StrategyResult {
            strategy: Strategy::HiddenPair,
            removals: removal_result,
        }
    }
}
