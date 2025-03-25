use crate::{Candidate, RemovalResult, Strategy, StrategyResult, Sudoku};

impl Sudoku {
    fn remove_box_candidates(&self, result: &mut RemovalResult) -> bool {
        if result.candidates_affected.iter().all(|candidate| {
            let box_index = (candidate.row / 3) * 3 + (candidate.col / 3);
            let first_box_index = (result.candidates_affected[0].row / 3) * 3
                + (result.candidates_affected[0].col / 3);
            box_index == first_box_index
        }) {
            for row in (result.candidates_affected[0].row / 3) * 3
                ..(result.candidates_affected[0].row / 3) * 3 + 3
            {
                for col in (result.candidates_affected[0].col / 3) * 3
                    ..(result.candidates_affected[0].col / 3) * 3 + 3
                {
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

    pub fn find_locked_pair_in_rows(&self) -> RemovalResult {
        let mut result = self.find_obvious_pair_in_rows();
        if self.remove_box_candidates(&mut result) {
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_locked_pair_in_cols(&self) -> RemovalResult {
        let mut result = self.find_obvious_pair_in_cols();
        if self.remove_box_candidates(&mut result) {
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_locked_pair(&self) -> StrategyResult {
        log::info!("Finding locked pairs in rows");
        let result = self.find_locked_pair_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: result,
            };
        }
        log::info!("Finding locked pairs in columns");
        let result = self.find_locked_pair_in_cols();
        StrategyResult {
            strategy: Strategy::LockedPair,
            removals: result,
        }
    }
}
