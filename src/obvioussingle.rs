use crate::{EMPTY, Strategy, StrategyResult, Sudoku};

impl Sudoku {
    pub fn find_obvious_single(&self) -> StrategyResult {
        for row in 0..9 {
            for col in 0..9 {
                if self.candidates[row][col].len() != 1 {
                    continue;
                }
                log::info!(
                    "Found obvious single {} at ({}, {})",
                    self.board[row][col],
                    row,
                    col
                );
                assert_eq!(self.board[row][col], EMPTY);
                let &num = self.candidates[row][col].iter().next().unwrap();
                return StrategyResult {
                    strategy: Strategy::ObviousSingle,
                    removals: self.collect_set_num(num, row, col),
                };
            }
        }
        StrategyResult::new(Strategy::ObviousSingle)
    }
}
