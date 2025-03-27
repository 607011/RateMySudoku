use crate::{Candidate, EMPTY, RemovalResult, Strategy, StrategyResult, Sudoku, Unit};

#[derive(Debug, Clone, Copy)]
struct StrongLink {
    base: Candidate,
    top: Candidate,
}

impl Sudoku {
    pub fn find_skyscraper_in_rows(&self) -> RemovalResult {
        // The Skyscraper pattern occurs when the same candidate appears exactly twice in two rows
        // The pattern forms a rectangle where the candidates in corners can see each other
        let mut result = RemovalResult::empty();
        for num in 1..=9 {
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
            let mut skyscraper_candidates: Vec<StrongLink> = Vec::new();
            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    let row1 = &candidates[i];
                    let row2 = &candidates[j];
                    // If the two candidates share the same column we have a potential strong link
                    // Check if bases share the same column
                    if row1[0].col == row2[0].col {
                        if (row1[1].col < row1[0].col && row2[1].col < row1[0].col)
                            || (row1[1].col > row1[0].col && row2[1].col > row1[0].col)
                        {
                            // Tops are on the same side (either left or right)
                            let link1 = StrongLink {
                                base: row1[0],
                                top: row1[1],
                            };
                            let link2 = StrongLink {
                                base: row2[0],
                                top: row2[1],
                            };
                            skyscraper_candidates.push(link1);
                            skyscraper_candidates.push(link2);
                        }
                    }
                    // Check if tops share the same column
                    else if row1[1].col == row2[1].col {
                        // Tops share the same column
                        if (row1[0].col < row1[1].col && row2[0].col < row2[1].col)
                            || (row1[0].col > row1[1].col && row2[0].col > row2[1].col)
                        {
                            // Bases are on the same side (either left or right) of their tops
                            let link1 = StrongLink {
                                base: row1[1],
                                top: row1[0],
                            };
                            let link2 = StrongLink {
                                base: row2[1],
                                top: row2[0],
                            };
                            skyscraper_candidates.push(link1);
                            skyscraper_candidates.push(link2);
                        }
                    }
                }
            }

            // Filter out candidates where a base and its corresponding top are in the same box
            skyscraper_candidates.retain(|link| {
                let base_box = Self::get_box_index(link.base.row, link.base.col);
                let top_box = Self::get_box_index(link.top.row, link.top.col);
                base_box != top_box
            });

            // Filter out candidates where the bases are in the same box
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    let base1_box = Self::get_box_index(
                        skyscraper_candidates[i].base.row,
                        skyscraper_candidates[i].base.col,
                    );
                    let base2_box = Self::get_box_index(
                        skyscraper_candidates[j].base.row,
                        skyscraper_candidates[j].base.col,
                    );
                    if base1_box == base2_box {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            // Filter out candidates where the tops are in the same column
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    if skyscraper_candidates[i].top.col == skyscraper_candidates[j].top.col {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            // Filter out candidates where tops are not in the same box column
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    let top1_box_col = skyscraper_candidates[i].top.col / 3;
                    let top2_box_col = skyscraper_candidates[j].top.col / 3;
                    if top1_box_col != top2_box_col {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            if skyscraper_candidates.is_empty() {
                continue;
            }

            for i in 0..skyscraper_candidates.len() {
                for j in i + 1..skyscraper_candidates.len() {
                    let link1 = &skyscraper_candidates[i];
                    let link2 = &skyscraper_candidates[j];

                    if link1.base.col == link2.base.col {
                        // The bases share the same column
                        let base1 = &link1.base;
                        let base2 = &link2.base;
                        let top1 = &link1.top;
                        let top2 = &link2.top;

                        log::info!("Skyscraper with candidate {}", num);
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

                        if result.will_remove_candidates() {
                            result.unit = Some(Unit::Row);
                            result.unit_index = Some(vec![base1.row, base2.row]);
                            return result;
                        }
                    }
                }
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
            let mut skyscraper_candidates: Vec<StrongLink> = Vec::new();
            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    let col1 = &candidates[i];
                    let col2 = &candidates[j];
                    if col1[0].row == col2[0].row {
                        // Bases share the same row
                        if (col1[1].row < col1[0].row && col2[1].row < col1[0].row)
                            || (col1[1].row > col1[0].row && col2[1].row > col1[0].row)
                        {
                            // Tops are on the same side (either above or below)
                            let link1 = StrongLink {
                                base: col1[0],
                                top: col1[1],
                            };
                            let link2 = StrongLink {
                                base: col2[0],
                                top: col2[1],
                            };
                            skyscraper_candidates.push(link1);
                            skyscraper_candidates.push(link2);
                        }
                    } else if col1[1].row == col2[1].row {
                        // Tops share the same row
                        if (col1[0].row < col1[1].row && col2[0].row < col2[1].row)
                            || (col1[0].row > col1[1].row && col2[0].row > col2[1].row)
                        {
                            // Bases are on the same side (either above or below) of their tops
                            let link1 = StrongLink {
                                base: col1[1],
                                top: col1[0],
                            };
                            let link2 = StrongLink {
                                base: col2[1],
                                top: col2[0],
                            };
                            skyscraper_candidates.push(link1);
                            skyscraper_candidates.push(link2);
                        }
                    }
                }
            }
            // Filter out candidates where a base and its corresponding top are in the same box
            // Filter out candidates where a base and its corresponding top are in the same box
            skyscraper_candidates.retain(|link| {
                let base_box = Self::get_box_index(link.base.row, link.base.col);
                let top_box = Self::get_box_index(link.top.row, link.top.col);
                base_box != top_box
            });

            // Filter out candidates where the bases are in the same box
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    let base1_box = Self::get_box_index(
                        skyscraper_candidates[i].base.row,
                        skyscraper_candidates[i].base.col,
                    );
                    let base2_box = Self::get_box_index(
                        skyscraper_candidates[j].base.row,
                        skyscraper_candidates[j].base.col,
                    );
                    if base1_box == base2_box {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            // Filter out candidates where the tops are in the same column
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    if skyscraper_candidates[i].top.row == skyscraper_candidates[j].top.row {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            // Filter out candidates where tops are not in the same box row
            for i in (0..skyscraper_candidates.len()).rev() {
                for j in (i + 1..skyscraper_candidates.len()).rev() {
                    let top1_box_row = skyscraper_candidates[i].top.row / 3;
                    let top2_box_row = skyscraper_candidates[j].top.row / 3;
                    if top1_box_row != top2_box_row {
                        skyscraper_candidates.remove(j);
                        skyscraper_candidates.remove(i);
                        break;
                    }
                }
            }

            if skyscraper_candidates.is_empty() {
                continue;
            }

            for i in 0..skyscraper_candidates.len() {
                for j in i + 1..skyscraper_candidates.len() {
                    let link1 = &skyscraper_candidates[i];
                    let link2 = &skyscraper_candidates[j];

                    if link1.base.row == link2.base.row {
                        // The bases share the same row
                        let base1 = &link1.base;
                        let base2 = &link2.base;
                        let top1 = &link1.top;
                        let top2 = &link2.top;

                        log::info!("Skyscraper with candidate {}", num);
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

                        if result.will_remove_candidates() {
                            result.unit = Some(Unit::Column);
                            result.unit_index = Some(vec![base1.col, base2.col]);
                            return result;
                        }
                    }
                }
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
