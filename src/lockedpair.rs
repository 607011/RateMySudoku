use crate::{RemovalResult, Strategy, StrategyResult, Sudoku};

impl Sudoku {
    pub fn find_locked_pair_rows(&self) -> RemovalResult {
        RemovalResult::empty()
    }

    pub fn find_locked_pair_cols(&self) -> RemovalResult {
        RemovalResult::empty()
    }

    pub fn find_locked_pair_boxes(&self) -> RemovalResult {
        RemovalResult::empty()
    }

    pub fn find_locked_pair(&self) -> StrategyResult {
        log::info!("Finding locked pairs in rows");
        let result = self.find_locked_pair_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: result,
            };
        }
        log::info!("Finding locked pairs in columns");
        let result = self.find_locked_pair_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::LockedPair,
                removals: result,
            };
        }
        log::info!("Finding locked pairs in boxes");
        let result = self.find_locked_pair_boxes();
        StrategyResult {
            strategy: Strategy::LockedPair,
            removals: result,
        }
    }
}
