use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::LazyLock;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Unit {
    Row,
    Column,
    Box,
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Row => write!(f, "Row"),
            Unit::Column => write!(f, "Column"),
            Unit::Box => write!(f, "Box"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    None,
    LastDigit,
    ObviousSingle,
    HiddenSingle,
    ObviousPair,
    HiddenPair,
    LockedPair,
    PointingPair,
    ClaimingPair,
    NakedTriplet,
    XWing,
}

impl Strategy {
    fn to_string(&self) -> &str {
        match self {
            Strategy::None => "None",
            Strategy::LastDigit => "Last Digit",
            Strategy::ObviousSingle => "Obvious Single",
            Strategy::HiddenSingle => "Hidden Single",
            Strategy::LockedPair => "Locked Pair",
            Strategy::PointingPair => "Pointing Pair",
            Strategy::ClaimingPair => "Claiming Pair",
            Strategy::ObviousPair => "Obvious Pair",
            Strategy::HiddenPair => "Hidden Pair",
            Strategy::NakedTriplet => "Naked Triplet",
            Strategy::XWing => "X-Wing",
        }
    }

    fn difficulty(&self) -> i32 {
        match self {
            Strategy::None => 0,
            Strategy::LastDigit => 4,
            Strategy::ObviousSingle => 5,
            Strategy::HiddenSingle => 14,
            Strategy::LockedPair => 40,
            Strategy::PointingPair => 50,
            Strategy::ClaimingPair => 50,
            Strategy::ObviousPair => 60,
            Strategy::HiddenPair => 70,
            Strategy::NakedTriplet => 80,
            Strategy::XWing => 140,
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
pub const EMPTY: u8 = 0;
pub static ALL_DIGITS: LazyLock<HashSet<u8>> = LazyLock::new(|| (1..=9).collect());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Candidate {
    pub row: usize,
    pub col: usize,
    pub num: u8,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub num: u8,
}

#[derive(Debug)]
pub struct RemovalResult {
    pub sets_cell: Option<Cell>,
    pub cells_affected: Vec<Cell>,
    pub candidates_affected: Vec<Candidate>,
    pub candidates_about_to_be_removed: HashSet<Candidate>,
    pub unit: Option<Unit>,
    pub unit_index: Option<Vec<usize>>,
}

impl RemovalResult {
    pub fn empty() -> Self {
        RemovalResult {
            sets_cell: None,
            cells_affected: Vec::new(),
            candidates_affected: Vec::new(),
            candidates_about_to_be_removed: HashSet::new(),
            unit: None,
            unit_index: None,
        }
    }
    pub fn will_remove_candidates(&self) -> bool {
        !self.candidates_about_to_be_removed.is_empty()
    }
    pub fn clear(&mut self) {
        self.sets_cell = None;
        self.cells_affected.clear();
        self.candidates_affected.clear();
        self.candidates_about_to_be_removed.clear();
        self.unit = None;
        self.unit_index = None;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct StrategyResult {
    pub strategy: Strategy,
    pub removals: RemovalResult,
}

impl StrategyResult {
    fn new(strategy: Strategy) -> Self {
        StrategyResult {
            strategy,
            removals: RemovalResult::empty(),
        }
    }
    pub fn empty() -> Self {
        StrategyResult {
            strategy: Strategy::None,
            removals: RemovalResult::empty(),
        }
    }
    pub fn clear(&mut self) {
        self.strategy = Strategy::None;
        self.removals.clear();
    }
}

#[derive(Debug)]
pub struct Resolution {
    pub nums_removed: usize,
    pub strategy: Strategy,
}

impl Resolution {
    #[allow(dead_code)]
    pub fn nums_removed(&self) -> usize {
        self.nums_removed
    }
    #[allow(dead_code)]
    pub fn strategy(&self) -> Strategy {
        self.strategy.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub board: [[u8; 9]; 9],
    pub original_board: [[u8; 9]; 9],
    pub candidates: [[HashSet<u8>; 9]; 9],
    pub rating: HashMap<Strategy, usize>,
    pub undo_stack: Vec<Sudoku>,
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..9 {
            for col in 0..9 {
                write!(f, "{} ", self.board[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Sudoku::new()
    }
}

impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            board: [[EMPTY; 9]; 9],
            original_board: [[EMPTY; 9]; 9],
            candidates: std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new())),
            rating: HashMap::new(),
            undo_stack: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from_string(s: &str) -> Sudoku {
        let mut sudoku = Sudoku::new();
        sudoku.set_board_string(s);
        sudoku
    }

    pub fn clear(&mut self) {
        self.candidates = std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new()));
        self.board = [[EMPTY; 9]; 9];
        self.rating.clear();
    }

    pub fn undo(&mut self) {
        if let Some(state) = self.undo_stack.pop() {
            self.board = state.board;
            self.candidates = state.candidates;
            self.rating = state.rating;
        }
    }

    pub fn original_board(&self) -> String {
        self.original_board
            .iter()
            .flatten()
            .map(|&digit| (digit + b'0') as char)
            .collect()
    }

    #[cfg(feature = "dump")]
    pub fn dump_rating(&self) {
        println!("Rating:");
        let candidates_removed = self.rating.iter().map(|(_, &count)| count).sum::<usize>();
        let total_rating: i32 = self
            .rating
            .iter()
            .map(|(strategy, &count)| strategy.difficulty() * count as i32)
            .sum();
        let difficulty = (total_rating as f64) / (candidates_removed as f64);
        println!("  Difficulty: {:.2}", difficulty);
        println!("  Total candidates removed: {}; by …", candidates_removed);
        let mut strategies: Vec<(&Strategy, &usize)> = self.rating.iter().collect();
        strategies.sort_by_key(|(strategy, _)| strategy.difficulty());
        for (strategy, count) in strategies {
            println!(
                "  - {} ({}): {}",
                strategy.to_string(),
                strategy.difficulty(),
                count
            );
        }
    }

    #[cfg(feature = "dump")]
    pub fn dump_notes(&self) {
        println!();
        println!("     0     1     2     3     4     5     6     7     8");
        println!("  ╔═════╤═════╤═════╦═════╤═════╤═════╦═════╤═════╤═════╗");
        for i in 0..9 {
            for line in 0..3 {
                if line == 1 {
                    print!("{} ║ ", i);
                } else {
                    print!("  ║ ");
                }
                for j in 0..9 {
                    for k in 0..3 {
                        let num = 3 * line + k + 1;
                        if self.candidates[i][j].contains(&num) {
                            print!("{}", num);
                        } else {
                            print!(".");
                        }
                    }
                    if (j + 1) % 3 == 0 {
                        print!(" ║ ");
                    } else {
                        print!(" │ ");
                    }
                }
                println!();
            }
            if i == 8 {
                println!("  ╚═════╧═════╧═════╩═════╧═════╧═════╩═════╧═════╧═════╝");
            } else if (i + 1) % 3 == 0 {
                println!("  ╠═════╪═════╪═════╬═════╪═════╪═════╬═════╪═════╪═════╣");
            } else {
                println!("  ╟─────┼─────┼─────╫─────┼─────┼─────╫─────┼─────┼─────╢");
            }
        }
    }

    pub fn effort(&self) -> f64 {
        let candidates_removed = self.rating.iter().map(|(_, &count)| count).sum::<usize>();
        let total_rating: i32 = self
            .rating
            .iter()
            .map(|(strategy, &count)| strategy.difficulty() * count as i32)
            .sum();
        (total_rating as f64) / (candidates_removed as f64)
    }

    pub fn unsolved(&self) -> bool {
        self.board.iter().any(|row| row.contains(&EMPTY))
    }

    pub fn is_solved(&self) -> bool {
        !self.unsolved()
    }

    #[allow(dead_code)]
    pub fn rating(&self) -> HashMap<Strategy, usize> {
        self.rating.clone()
    }

    pub fn difficulty(&self) -> f64 {
        let candidates_removed = self.rating.iter().map(|(_, &count)| count).sum::<usize>();
        let total_rating: i32 = self
            .rating
            .iter()
            .map(|(strategy, &count)| strategy.difficulty() * count as i32)
            .sum();
        (total_rating as f64) / (candidates_removed as f64)
    }

    pub fn serialized(&self) -> String {
        self.board
            .iter()
            .flatten()
            .map(|&digit| (digit + b'0') as char)
            .collect()
    }

    /// print the board
    #[cfg(feature = "dump")]
    pub fn print(&self) {
        for row in 0..9 {
            for col in 0..9 {
                print!("{} ", self.board[row][col]);
            }
            println!();
        }
        println!("{}", self.serialized());
    }

    fn calc_nums_in_row(&self, row: usize) -> HashSet<u8> {
        let mut nums = HashSet::new();
        for col in 0..9 {
            if self.board[row][col] != EMPTY {
                nums.insert(self.board[row][col]);
            }
        }
        nums
    }

    fn calc_nums_in_col(&self, col: usize) -> HashSet<u8> {
        let mut nums = HashSet::new();
        for row in 0..9 {
            if self.board[row][col] != EMPTY {
                nums.insert(self.board[row][col]);
            }
        }
        nums
    }

    fn calc_nums_in_box(&self, box_index: usize) -> HashSet<u8> {
        let mut nums = HashSet::new();
        let start_row = 3 * (box_index / 3);
        let start_col = 3 * (box_index % 3);
        for i in 0..3 {
            for j in 0..3 {
                if self.board[start_row + i][start_col + j] != EMPTY {
                    nums.insert(self.board[start_row + i][start_col + j]);
                }
            }
        }
        nums
    }

    pub fn calc_all_notes(&mut self) {
        // First calculate all the "used numbers" sets
        let mut nums_in_row: [HashSet<u8>; 9] = std::array::from_fn(|_| HashSet::new());
        let mut nums_in_col: [HashSet<u8>; 9] = std::array::from_fn(|_| HashSet::new());
        let mut nums_in_box: [HashSet<u8>; 9] = std::array::from_fn(|_| HashSet::new());
        for i in 0..9 {
            nums_in_row[i] = self.calc_nums_in_row(i);
            nums_in_col[i] = self.calc_nums_in_col(i);
            nums_in_box[i] = self.calc_nums_in_box(i);
        }

        // Then populate notes for empty cells
        (0..9).for_each(|row| {
            (0..9).for_each(|col| {
                if self.board[row][col] != EMPTY {
                    return;
                }
                let box_idx = 3 * (row / 3) + col / 3;
                let mut notes = (1..=9).collect::<HashSet<u8>>();
                // Remove numbers already present in row, column, and box
                for &num in &nums_in_row[row] {
                    notes.remove(&num);
                }
                for &num in &nums_in_col[col] {
                    notes.remove(&num);
                }
                for &num in &nums_in_box[box_idx] {
                    notes.remove(&num);
                }
                self.candidates[row][col] = notes;
            })
        });
    }

    /// Check if `num` can be placed in row `row` and column `col`
    pub fn can_place(&self, row: usize, col: usize, num: u8) -> bool {
        if self.board[row][col] != EMPTY {
            return false;
        }
        for i in 0..9 {
            // this is faster than using `nums_in_row`, `nums_in_col`, and `nums_in_box`
            // because these sets have to be recalculated every time a number is placed,
            // and backtracked when a number is removed
            if self.board[row][i] == num {
                return false;
            }
            if self.board[i][col] == num {
                return false;
            }
            if self.board[3 * (row / 3) + i / 3][3 * (col / 3) + i % 3] == num {
                return false;
            }
        }
        true
    }

    /// Solve the Sudoku the "computer" way by backtracking recursively
    fn solve(&mut self) -> bool {
        // Find empty cell
        let mut empty_found = false;
        let mut row = 0;
        let mut col = 0;
        'find_empty: for r in 0..9 {
            for c in 0..9 {
                if self.board[r][c] == EMPTY {
                    row = r;
                    col = c;
                    empty_found = true;
                    break 'find_empty;
                }
            }
        }
        // If no empty cell was found, the board is solved
        if !empty_found {
            return true;
        }
        // Try placing digits 1-9 in the empty cell
        for num in 1..=9 {
            if !self.can_place(row, col, num) {
                continue;
            }
            self.board[row][col] = num;
            if self.solve() {
                return true;
            }
            self.board[row][col] = EMPTY;
        }
        false
    }

    pub fn solve_by_backtracking(&mut self) -> bool {
        self.solve()
    }

    /// Check if there are last digits in any of the rows.
    /// If so, remove it from the notes in the row, column, and box where we've found it.
    /// Set the respective cell to the digit.
    pub fn find_last_digit_in_rows(&self) -> RemovalResult {
        for row in 0..9 {
            // Find the only empty cell in the row, if there's exactly one
            let empty_cells = (0..9)
                .filter(|&col| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.calc_nums_in_row(row))
                .cloned()
                .collect();
            assert_eq!(missing_digits.len(), 1);
            let num = *missing_digits.iter().next().unwrap();
            let col = empty_cells[0];
            let mut result = self.collect_set_num(num, row, col);
            result.unit = Some(Unit::Row);
            result.unit_index = Some(vec![row]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit_in_cols(&self) -> RemovalResult {
        for col in 0..9 {
            let empty_cells = (0..9)
                .filter(|&row| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let row = empty_cells[0];
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.calc_nums_in_col(col))
                .cloned()
                .collect();
            assert_eq!(missing_digits.len(), 1);
            let num = *missing_digits.iter().next().unwrap();
            let mut result = self.collect_set_num(num, row, col);
            result.unit = Some(Unit::Column);
            result.unit_index = Some(vec![col]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit_in_boxes(&self) -> RemovalResult {
        for box_index in 0..9 {
            let start_row = 3 * (box_index / 3);
            let start_col = 3 * (box_index % 3);
            let mut count = 0;
            let mut empty_row = 0;
            let mut empty_col = 0;
            'box_search: for i in 0..3 {
                for j in 0..3 {
                    let row = start_row + i;
                    let col = start_col + j;
                    if self.board[row][col] != EMPTY {
                        continue;
                    }
                    count += 1;
                    empty_row = row;
                    empty_col = col;
                    break 'box_search;
                }
            }
            if count != 1 {
                continue;
            }
            let missing_digits: HashSet<u8> = ALL_DIGITS
                .difference(&self.calc_nums_in_box(box_index))
                .cloned()
                .collect();
            if missing_digits.len() != 1 {
                continue;
            }
            let num = *missing_digits.iter().next().unwrap();
            let mut result = self.collect_set_num(num, empty_row, empty_col);
            result.unit = Some(Unit::Box);
            result.unit_index = Some(vec![box_index]);
            return result;
        }
        RemovalResult::empty()
    }

    pub fn find_last_digit(&self) -> StrategyResult {
        let mut result = StrategyResult::new(Strategy::LastDigit);
        log::info!("Finding last digits in rows");
        let removal_result = self.find_last_digit_in_rows();
        if removal_result.will_remove_candidates() {
            result.removals = removal_result;
            return result;
        }
        log::info!("Finding last digits in columns");
        let removal_result = self.find_last_digit_in_cols();
        if removal_result.will_remove_candidates() {
            result.removals = removal_result;
            return result;
        }
        log::info!("Finding last digits in boxes");
        let removal_result = self.find_last_digit_in_boxes();
        result.removals = removal_result;
        result
    }

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

    /// Finds and resolves "hidden single" candidates in the Sudoku puzzle.
    ///
    /// A hidden single occurs when a digit can only go in one cell within a group (row, column, or box),
    /// even though that cell may have multiple candidates.
    ///
    /// Returns the number of notes removed as a result of placing new digits.
    pub fn find_hidden_single(&self) -> StrategyResult {
        let mut result = StrategyResult::new(Strategy::HiddenSingle);
        log::info!("Finding hidden singles in boxes");
        let box_result = self.find_hidden_single_box();
        log::info!("{:?}", box_result);
        if box_result.will_remove_candidates() {
            result.removals = box_result;
            return result;
        }
        log::info!("Finding hidden singles in rows");
        let row_result = self.find_hidden_single_row();
        log::info!("{:?}", row_result);
        if row_result.will_remove_candidates() {
            result.removals = row_result;
            return result;
        }
        log::info!("Finding hidden singles in columns");
        let col_result = self.find_hidden_single_col();
        log::info!("{:?}", col_result);
        if col_result.will_remove_candidates() {
            result.removals = col_result;
            return result;
        }
        result
    }

    pub fn find_hidden_single_row(&self) -> RemovalResult {
        // Check for hidden singles in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] > 0 {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != col && self.candidates[row][i].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut result = self.collect_set_num(num, row, col);
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        RemovalResult::empty()
    }

    pub fn find_hidden_single_col(&self) -> RemovalResult {
        // Check for hidden singles in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.candidates[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != row && self.candidates[i][col].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        let mut result = self.collect_set_num(num, row, col);
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        RemovalResult::empty()
    }

    pub fn find_hidden_single_box(&self) -> RemovalResult {
        // Check for hidden singles in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                for i in 0..3 {
                    for j in 0..3 {
                        let row = start_row + i;
                        let col = start_col + j;
                        if self.board[row][col] != EMPTY {
                            continue;
                        }
                        for &num in &self.candidates[row][col] {
                            let mut found = false;
                            'box_check: for r in 0..3 {
                                for c in 0..3 {
                                    let check_row = start_row + r;
                                    let check_col = start_col + c;
                                    if (check_row != row || check_col != col)
                                        && self.candidates[check_row][check_col].contains(&num)
                                    {
                                        found = true;
                                        break 'box_check;
                                    }
                                }
                            }
                            if !found {
                                let mut result = self.collect_set_num(num, row, col);
                                result.unit = Some(Unit::Box);
                                result.unit_index = Some(vec![3 * box_row + box_col]);
                                return result;
                            }
                        }
                    }
                }
            }
        }
        RemovalResult::empty()
    }

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

    pub fn find_pointing_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                for num in 1..=9 {
                    // Collect unique rows where candidate `num` appears in this box
                    let rows_with_num: HashSet<usize> = (0..3)
                        .flat_map(|i| (0..3).map(move |j| (box_row + i, box_col + j)))
                        .filter(|&(row, col)| self.candidates[row][col].contains(&num))
                        .map(|(row, _)| row)
                        .collect();
                    // `num` must appear in exactly one row within the box
                    if rows_with_num.len() != 1 {
                        continue;
                    }
                    let row = *rows_with_num.iter().next().unwrap();
                    // Check how many cells in this box's row contain the candidate
                    let box_cells_with_num = (box_col..box_col + 3)
                        .filter(|&col| self.candidates[row][col].contains(&num))
                        .count();
                    // For a proper pointing pair, we want 2 cells in this box's row
                    if box_cells_with_num != 2 {
                        continue;
                    }
                    // Check if there are candidates to remove outside the box
                    for col in 0..9 {
                        if (col < box_col || col >= box_col + 3)
                            && self.candidates[row][col].contains(&num)
                        {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col,
                                num,
                            });
                        }
                    }
                    if result.will_remove_candidates() {
                        // For each cell with the candidate in this box and row, add it to affected candidates
                        for col in box_col..box_col + 3 {
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_affected.push(Candidate { row, col, num });
                            }
                        }
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                for num in 1..=9 {
                    // Collect unique columns where candidate `num` appears in this box
                    let cols_with_num: HashSet<usize> = (0..3)
                        .flat_map(|i| (0..3).map(move |j| (box_row + j, box_col + i)))
                        .filter(|&(row, col)| self.candidates[row][col].contains(&num))
                        .map(|(_, col)| col)
                        .collect();
                    // `num` must appear exactly one column within the box
                    if cols_with_num.len() != 1 {
                        continue;
                    }
                    let col = *cols_with_num.iter().next().unwrap();
                    // Check how many cells in this box's row contain the candidate
                    let box_cells_with_num = (box_col..box_col + 3)
                        .filter(|&row| self.candidates[row][col].contains(&num))
                        .count();
                    // For a proper pointing pair, we want 2 cells in this box's column
                    if box_cells_with_num != 2 {
                        continue;
                    }
                    for row in 0..9 {
                        if (row < box_row || row >= box_row + 3)
                            && self.candidates[row][col].contains(&num)
                        {
                            result.candidates_about_to_be_removed.insert(Candidate {
                                row,
                                col,
                                num,
                            });
                        }
                    }
                    if result.will_remove_candidates() {
                        // For each cell with the candidate in this box and column, add it to affected candidates
                        for row in box_row..box_row + 3 {
                            if self.candidates[row][col].contains(&num) {
                                result.candidates_affected.push(Candidate { row, col, num });
                            }
                        }
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_pointing_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check each 3x3 box
        for box_idx in 0..9 {
            let box_row = (box_idx / 3) * 3;
            let box_col = (box_idx % 3) * 3;
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

    pub fn find_obvious_pair_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.candidates[row][col].len() != 2 {
                    continue;
                }

                let pair = self.candidates[row][col].clone();

                // Find pair in same row
                for i in (col + 1)..9 {
                    if self.candidates[row][i] != pair {
                        continue;
                    }
                    // Found a pair, mark these candidates from other cells
                    // in the same row as about to be removed
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != col && j != i {
                            for &num in &nums {
                                if self.candidates[row][j].contains(&num) {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row,
                                        col: j,
                                        num,
                                    });
                                }
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col, num }));
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col: i, num }));
                        result.unit = Some(Unit::Row);
                        result.unit_index = Some(vec![row]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.candidates[row][col].len() != 2 {
                    continue;
                }

                let pair = self.candidates[row][col].clone();
                log::info!("Found pair {:?} at ({}, {})", pair, row, col);

                // Find pair in same column
                for i in (row + 1)..9 {
                    if self.candidates[i][col] != pair {
                        continue;
                    }
                    // Found a pair, mark these candidates from other cells
                    // in the same column as about to be removed
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != row && j != i {
                            for &num in &nums {
                                if self.candidates[j][col].contains(&num) {
                                    result.candidates_about_to_be_removed.insert(Candidate {
                                        row: j,
                                        col,
                                        num,
                                    });
                                }
                            }
                        }
                    }
                    if result.will_remove_candidates() {
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row, col, num }));
                        result
                            .candidates_affected
                            .extend(pair.iter().map(|&num| Candidate { row: i, col, num }));
                        result.unit = Some(Unit::Column);
                        result.unit_index = Some(vec![col]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair_in_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for obvious pairs in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                for r1 in 0..3 {
                    for c1 in 0..3 {
                        let row1 = start_row + r1;
                        let col1 = start_col + c1;

                        if self.candidates[row1][col1].len() != 2 {
                            continue;
                        }

                        let pair = self.candidates[row1][col1].clone();

                        for r2 in 0..3 {
                            for c2 in 0..3 {
                                let row2 = start_row + r2;
                                let col2 = start_col + c2;

                                // Skip same cell or already checked pairs
                                if (row1 == row2 && col1 == col2) || (r2 * 3 + c2 <= r1 * 3 + c1) {
                                    continue;
                                }

                                if self.candidates[row2][col2] != pair {
                                    continue;
                                }

                                // Found a pair, remove these candidates from other cells in the same box
                                let nums: Vec<u8> = pair.iter().cloned().collect();
                                for r in 0..3 {
                                    for c in 0..3 {
                                        let row = start_row + r;
                                        let col = start_col + c;
                                        if (row != row1 || col != col1)
                                            && (row != row2 || col != col2)
                                        {
                                            for &num in &nums {
                                                if self.candidates[row][col].contains(&num) {
                                                    result
                                                        .candidates_about_to_be_removed
                                                        .insert(Candidate { row, col, num });
                                                }
                                            }
                                        }
                                    }
                                }
                                if result.will_remove_candidates() {
                                    result.candidates_affected.extend(pair.iter().map(|&num| {
                                        Candidate {
                                            row: row1,
                                            col: col1,
                                            num,
                                        }
                                    }));
                                    result.candidates_affected.extend(
                                        self.candidates[row2][col2].iter().map(|&num| Candidate {
                                            row: row2,
                                            col: col2,
                                            num,
                                        }),
                                    );
                                    result.unit = Some(Unit::Box);
                                    result.unit_index = Some(vec![box_row * 3 + box_col]);
                                    return result;
                                }
                            }
                        }
                    }
                }
            }
        }
        result
    }

    pub fn find_obvious_pair(&self) -> StrategyResult {
        log::info!("Finding obvious pairs in rows");
        let removal_result = self.find_obvious_pair_in_rows();
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousPair,
                removals: removal_result,
            };
        }
        log::info!("Finding obvious pairs in columns");
        let removal_result = self.find_obvious_pair_in_cols();
        if removal_result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::ObviousPair,
                removals: removal_result,
            };
        }
        log::info!("Finding obvious pairs in boxes");
        let removal_result = self.find_obvious_pair_in_boxes();
        StrategyResult {
            strategy: Strategy::ObviousPair,
            removals: removal_result,
        }
    }

    pub fn find_hidden_pair_in_rows(&self) -> RemovalResult {
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
                log::info!("Hidden pair in {:?} / {:?}", digit_locations, digit_pairs);
                result.unit = Some(Unit::Row);
                result.unit_index = Some(vec![]);

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
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_hidden_pair_in_cols(&self) -> RemovalResult {
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
                    result.unit = Some(Unit::Column);
                    result.unit_index = Some(vec![col1, col2]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_hidden_pair_in_boxes(&self) -> RemovalResult {
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
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![row1 / 3 * 3 + col / 3]);
                    return result;
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

    pub fn find_xwing_in_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for x-wings in rows
        for num in 1..=9 {
            for row1 in 0..8 {
                // We don't need to check the last row
                let mut cols1 = Vec::new();
                // Find columns with candidate `num` in this row
                for col in 0..9 {
                    if self.candidates[row1][col].contains(&num) {
                        cols1.push(col);
                    }
                }
                if cols1.len() != 2 {
                    continue;
                }
                // Find another row with the same columns
                for row2 in (row1 + 1)..9 {
                    let mut cols2 = Vec::new();
                    // Find columns with candidate `num` in this row
                    for col in 0..9 {
                        if self.candidates[row2][col].contains(&num) {
                            cols2.push(col);
                        }
                    }
                    // If we found another row with the same columns, we have an X-Wing
                    if cols2.len() != 2 || cols1 != cols2 {
                        continue;
                    }
                    log::info!(
                        "Found x-wing {:?} in rows {} and {} at columns {:?}",
                        num,
                        row1,
                        row2,
                        cols1
                    );
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: cols1[0],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: cols1[1],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: cols2[0],
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: cols2[1],
                        num,
                    });
                    // Remove the candidate from other cells in the same columns
                    for row in 0..9 {
                        if row == row1 || row == row2 {
                            continue;
                        }
                        for &col in &cols1 {
                            if self.candidates[row][col].contains(&num) {
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
                        result.unit_index = Some(vec![row1]);
                        return result;
                    }
                }
            }
        }
        result
    }

    pub fn find_xwing_in_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        // Check for x-wings in columns
        for num in 1..=9 {
            for col1 in 0..8 {
                // We don't need to check the last column
                let mut rows1 = Vec::new();

                // Find rows with candidate `num` in this column
                for row in 0..9 {
                    if self.candidates[row][col1].contains(&num) {
                        rows1.push(row);
                    }
                }
                if rows1.len() != 2 {
                    continue;
                }
                // Find another column with the same rows
                for col2 in (col1 + 1)..9 {
                    let mut rows2 = Vec::new();
                    // Find rows with candidate `num` in this column
                    for row in 0..9 {
                        if self.candidates[row][col2].contains(&num) {
                            rows2.push(row);
                        }
                    }
                    // If we found another column with the same rows, we have an X-Wing
                    if rows2.len() != 2 || rows1 != rows2 {
                        continue;
                    }
                    log::info!(
                        "Found X-Wing {:?} in columns {} and {} at rows {:?}",
                        num,
                        col1,
                        col2,
                        rows1
                    );
                    result.candidates_affected.push(Candidate {
                        row: rows1[0],
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows1[1],
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows2[0],
                        col: col2,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: rows2[1],
                        col: col2,
                        num,
                    });
                    // Mark removable candidates from other cells in the same rows
                    for &row in &rows1 {
                        for col in 0..9 {
                            if col == col1 || col == col2 {
                                continue;
                            }
                            if self.candidates[row][col].contains(&num) {
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
                        result.unit_index = Some(vec![col1]);
                        return result;
                    }
                }
            }
        }
        result
    }

    /// Find and resolve X-Wing candidates.
    /// An X-Wing occurs when a digit can only go in two rows and two columns, forming a rectangle.
    /// In this case, the digit can be removed from all other cells in the same rows and columns.
    pub fn find_xwing(&self) -> StrategyResult {
        log::info!("Finding X-Wings in rows");
        let result = self.find_xwing_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::XWing,
                removals: result,
            };
        }
        log::info!("Finding X-Wings in columns");
        let result = self.find_xwing_in_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::XWing,
                removals: result,
            };
        }
        StrategyResult::empty()
    }

    pub fn find_naked_triplet_in_rows(&self) -> RemovalResult {
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
                        let triplet_cols = vec![col1, col2, col3];
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

    pub fn find_naked_triplet_in_cols(&self) -> RemovalResult {
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
                        let triplet_rows = vec![row1, row2, row3];
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

    pub fn find_naked_triplet_in_boxes(&self) -> RemovalResult {
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
                    for k in (j + 1)..cells_with_candidates.len() {
                        let ((row3, col3), cands3) = cells_with_candidates[k];
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
                        let triplet_cells = vec![(row1, col1), (row2, col2), (row3, col3)];
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

    pub fn find_naked_triplet(&self) -> StrategyResult {
        log::info!("Finding naked triplets in rows");
        let result = self.find_naked_triplet_in_rows();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::NakedTriplet,
                removals: result,
            };
        }
        log::info!("Finding naked triplets in columns");
        let result = self.find_naked_triplet_in_cols();
        if result.will_remove_candidates() {
            return StrategyResult {
                strategy: Strategy::NakedTriplet,
                removals: result,
            };
        }
        log::info!("Finding naked triplets in boxes");
        let result = self.find_naked_triplet_in_boxes();
        StrategyResult {
            strategy: Strategy::NakedTriplet,
            removals: result,
        }
    }

    #[allow(dead_code)]
    fn find_cells_with_candidate_in_box(&self, box_idx: usize, num: u8) -> Vec<(usize, usize)> {
        let start_row = 3 * (box_idx / 3);
        let start_col = 3 * (box_idx % 3);
        (0..3)
            .flat_map(|r| (0..3).map(move |c| (start_row + r, start_col + c)))
            .filter(|&(row, col)| self.candidates[row][col].contains(&num))
            .collect()
    }

    /// Check if all cells are in the same row.
    #[allow(dead_code)]
    fn cells_in_same_row(cells: &[(usize, usize)]) -> Option<usize> {
        let rows: HashSet<_> = cells.iter().map(|&(row, _)| row).collect();
        if rows.len() == 1 {
            Some(*rows.iter().next().unwrap())
        } else {
            None
        }
    }

    /// Check if all cells are in the same column.
    #[allow(dead_code)]
    fn cells_in_same_column(cells: &[(usize, usize)]) -> Option<usize> {
        let cols: HashSet<_> = cells.iter().map(|&(_, col)| col).collect();
        if cols.len() == 1 {
            Some(*cols.iter().next().unwrap())
        } else {
            None
        }
    }

    pub fn find_locked_pair_rows(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            for num in 1..=9 {
                // Find all columns in this row that have this candidate
                let cols: Vec<_> = (0..9)
                    .filter(|&col| self.candidates[row][col].contains(&num))
                    .collect();
                if cols.len() != 2 {
                    continue;
                }
                // Check if any other row has this candidate in either of our two columns
                let has_other_row_with_candidate = (0..9).filter(|&r| r != row).any(|r| {
                    cols.iter()
                        .any(|&col| self.candidates[r][col].contains(&num))
                });
                if has_other_row_with_candidate {
                    continue;
                }
                log::info!("Found locked pair {:?} in row {}", num, row);
                result.candidates_affected.push(Candidate {
                    row,
                    col: cols[0],
                    num,
                });
                result.candidates_affected.push(Candidate {
                    row,
                    col: cols[1],
                    num,
                });
                // Remove this candidate from other cells in the same row
                for col in 0..9 {
                    if !cols.contains(&col) && self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    result.unit = Some(Unit::Row);
                    result.unit_index = Some(vec![row]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_locked_pair_cols(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for col in 0..9 {
            for num in 1..=9 {
                // Find all rows in this column that have this candidate
                let rows: Vec<_> = (0..9)
                    .filter(|&row| self.candidates[row][col].contains(&num))
                    .collect();
                if rows.len() != 2 {
                    continue;
                }
                // Check if any other column has this candidate in either of our two rows
                let has_other_col_with_candidate = (0..9).filter(|&c| c != col).any(|c| {
                    rows.iter()
                        .any(|&row| self.candidates[row][c].contains(&num))
                });
                if has_other_col_with_candidate {
                    continue;
                }
                log::info!("Found locked pair {:?} in column {}", num, col);
                result.candidates_affected.push(Candidate {
                    row: rows[0],
                    col,
                    num,
                });
                result.candidates_affected.push(Candidate {
                    row: rows[1],
                    col,
                    num,
                });
                // Remove this candidate from other cells in the same column
                for row in 0..9 {
                    if !rows.contains(&row) && self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
                if result.will_remove_candidates() {
                    result.unit = Some(Unit::Column);
                    result.unit_index = Some(vec![col]);
                    return result;
                }
            }
        }
        result
    }

    pub fn find_locked_pair_boxes(&self) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for box_idx in 0..9 {
            let start_row = 3 * (box_idx / 3);
            let start_col = 3 * (box_idx % 3);
            for num in 1..=9 {
                // Find all cells in this box that have this candidate
                let mut cells_with_num = Vec::new();
                for r in 0..3 {
                    for c in 0..3 {
                        let row = start_row + r;
                        let col = start_col + c;
                        if self.candidates[row][col].contains(&num) {
                            cells_with_num.push((row, col));
                        }
                    }
                }
                if cells_with_num.len() != 2 {
                    continue;
                }
                let (row1, col1) = cells_with_num[0];
                let (row2, col2) = cells_with_num[1];
                // Check if the cells are in the same row
                if row1 != row2 {
                    continue;
                }
                // Check if any other box in the same row has this candidate
                let has_other_box_with_candidate = (0..9)
                    .filter(|&c| c < start_col || c >= start_col + 3)
                    .any(|c| self.candidates[row1][c].contains(&num));
                if has_other_box_with_candidate {
                    continue;
                }
                // Remove this candidate from other cells in the same box but different row
                for r in start_row..start_row + 3 {
                    if r != row1 {
                        for c in start_col..start_col + 3 {
                            if self.candidates[r][c].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row: r,
                                    col: c,
                                    num,
                                });
                            }
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: col2,
                        num,
                    });
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
                // Check if the cells are in the same column
                if col1 != col2 {
                    continue;
                }
                // Check if any other box in the same column has this candidate
                let has_other_box_with_candidate = (0..9)
                    .filter(|&r| r < start_row || r >= start_row + 3)
                    .any(|r| self.candidates[r][col1].contains(&num));
                if has_other_box_with_candidate {
                    continue;
                }
                // Remove this candidate from other cells in the same box but different column
                for c in start_col..start_col + 3 {
                    if c != col1 {
                        for r in start_row..start_row + 3 {
                            if self.candidates[r][c].contains(&num) {
                                result.candidates_about_to_be_removed.insert(Candidate {
                                    row: r,
                                    col: c,
                                    num,
                                });
                            }
                        }
                    }
                }
                if result.will_remove_candidates() {
                    result.candidates_affected.push(Candidate {
                        row: row1,
                        col: col1,
                        num,
                    });
                    result.candidates_affected.push(Candidate {
                        row: row2,
                        col: col2,
                        num,
                    });
                    result.unit = Some(Unit::Box);
                    result.unit_index = Some(vec![box_idx]);
                    return result;
                }
            }
        }
        result
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

    /// Collect all candidates in a row that contain a given digit.
    fn collect_candidates_in_row(&self, nums: &[u8], row: usize) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for col in 0..9 {
            for &num in nums {
                if self.candidates[row][col].contains(&num) {
                    result
                        .candidates_about_to_be_removed
                        .insert(Candidate { row, col, num });
                }
            }
        }
        result
    }

    /// Collect all candidates in a column that contain a given digit.
    fn collect_candidates_in_col(&self, nums: &[u8], col: usize) -> RemovalResult {
        let mut result = RemovalResult::empty();
        for row in 0..9 {
            for &num in nums {
                if self.candidates[row][col].contains(&num) {
                    result
                        .candidates_about_to_be_removed
                        .insert(Candidate { row, col, num });
                }
            }
        }
        result
    }

    /// Collect all candidates in a box that contain a given digit.
    fn collect_candidates_in_box(&self, nums: &[u8], row: usize, col: usize) -> RemovalResult {
        let mut result = RemovalResult::empty();
        let start_row = 3 * (row / 3);
        let start_col = 3 * (col / 3);
        for i in 0..3 {
            for j in 0..3 {
                let row = start_row + i;
                let col = start_col + j;
                for &num in nums {
                    if self.candidates[row][col].contains(&num) {
                        result
                            .candidates_about_to_be_removed
                            .insert(Candidate { row, col, num });
                    }
                }
            }
        }
        result
    }

    /// Remove candidates from the notes in the same row, column, and box where we've set a digit.
    fn collect_candidates(&self, nums: &[u8], row: usize, col: usize) -> RemovalResult {
        let mut result = RemovalResult::empty();
        let remove_in_row = self.collect_candidates_in_row(nums, row);
        let remove_in_col = self.collect_candidates_in_col(nums, col);
        let remove_in_box = self.collect_candidates_in_box(nums, row, col);
        result
            .candidates_about_to_be_removed
            .extend(remove_in_row.candidates_about_to_be_removed);
        result
            .candidates_about_to_be_removed
            .extend(remove_in_col.candidates_about_to_be_removed);
        result
            .candidates_about_to_be_removed
            .extend(remove_in_box.candidates_about_to_be_removed);
        result
            .candidates_affected
            .extend(remove_in_row.candidates_affected);
        result
            .candidates_affected
            .extend(remove_in_col.candidates_affected);
        result
            .candidates_affected
            .extend(remove_in_box.candidates_affected);
        result
    }

    pub fn get_num(&self, row: usize, col: usize) -> u8 {
        self.board[row][col]
    }

    #[allow(dead_code)]
    pub fn get_notes(&self, row: usize, col: usize) -> HashSet<u8> {
        self.candidates[row][col].clone()
    }

    /// Collect all candidates that are about to be removed when setting a digit in a cell.
    pub fn collect_set_num(&self, num: u8, row: usize, col: usize) -> RemovalResult {
        let cell = Cell { row, col, num };
        let removal_result = self.collect_candidates(&[num], row, col);
        RemovalResult {
            sets_cell: Some(cell.clone()),
            cells_affected: vec![cell],
            candidates_affected: vec![Candidate { row, col, num }],
            candidates_about_to_be_removed: {
                let mut candidates = removal_result.candidates_about_to_be_removed;
                candidates.insert(Candidate { row, col, num });
                for &n in &self.candidates[row][col] {
                    if n != num {
                        candidates.insert(Candidate { row, col, num: n });
                    }
                }
                candidates
            },
            unit: None,
            unit_index: None,
        }
    }

    /// Apply the strategy result to the Sudoku board.
    pub fn apply(&mut self, strategy_result: &StrategyResult) -> Resolution {
        log::info!("Applying strategy: {:?}", strategy_result.strategy);
        let start = std::time::Instant::now();
        let mut clone = self.clone();
        clone.undo_stack = Vec::new(); // Don't clone the undo stack
        self.undo_stack.push(clone);
        let elapsed = start.elapsed().as_millis();
        log::info!("Cloning and pushing to undo stack took {} ms", elapsed);
        let result = Resolution {
            nums_removed: strategy_result
                .removals
                .candidates_about_to_be_removed
                .len(),
            strategy: strategy_result.strategy.clone(),
        };
        for note in &strategy_result.removals.candidates_about_to_be_removed {
            assert!(self.candidates[note.row][note.col].contains(&note.num));
            self.candidates[note.row][note.col].remove(&note.num);
        }
        if let Some(cell) = &strategy_result.removals.sets_cell {
            self.board[cell.row][cell.col] = cell.num;
            // Update rating for this strategy
            self.rating
                .entry(strategy_result.strategy.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        // self.dump_notes();
        result
    }

    /// Undo the last step.
    pub fn prev_step(&mut self) -> Resolution {
        self.undo();
        Resolution {
            nums_removed: 0,
            strategy: Strategy::None,
        }
    }

    /// Find the next step to solve the Sudoku puzzle.
    pub fn next_step(&mut self) -> StrategyResult {
        // last digit
        let result = self.find_last_digit();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::LastDigit)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::LastDigit,
            };
        }

        // obvious single
        let result = self.find_obvious_single();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::ObviousSingle)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::ObviousSingle,
            };
        }

        // hidden single
        let result = self.find_hidden_single();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::HiddenSingle)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::HiddenSingle,
            };
        }

        // locked pair
        let result = self.find_locked_pair();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::LockedPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::LockedPair,
            };
        }

        // pointing pair
        let result = self.find_pointing_pair();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::PointingPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::PointingPair,
            };
        }

        // claiming pair
        let result = self.find_claiming_pair();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::ClaimingPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::ClaimingPair,
            };
        }

        // obvious pair
        let result = self.find_obvious_pair();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::ObviousPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::ObviousPair,
            };
        }

        // hidden pair
        let result = self.find_hidden_pair();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::HiddenPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::HiddenPair,
            };
        }

        // naked triplet
        let result = self.find_naked_triplet();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::NakedTriplet)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::NakedTriplet,
            };
        }

        // x-wing
        let result = self.find_xwing();
        if result.removals.will_remove_candidates() {
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(Strategy::XWing)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: Strategy::XWing,
            };
        }

        StrategyResult::empty()
    }

    /// Solve the Sudoku puzzle using human-like strategies
    #[cfg(feature = "dump")]
    fn solve_like_a_human(&mut self) -> bool {
        // The first step always is to calculate the notes
        self.calc_all_notes();
        // Since we're starting from scratch, we clear the rating
        self.rating.clear();
        while self.unsolved() {
            let result = self.next_step();
            if result.strategy == Strategy::None {
                // No applicable strategy found or Sudoku is solved
                break;
            }
            self.apply(&result);
            self.print();
            self.dump_notes();
        }
        self.is_solved()
    }

    pub fn solve_human_like(&mut self) -> bool {
        // The first step always is to calculate the notes
        self.calc_all_notes();
        // Since we're starting from scratch, we clear the rating
        self.rating.clear();
        while self.unsolved() {
            let result = self.next_step();
            if result.strategy == Strategy::None {
                // No applicable strategy found or Sudoku is solved
                break;
            }
            self.apply(&result);
        }
        self.is_solved()
    }

    #[cfg(feature = "dump")]
    pub fn solve_puzzle(&mut self) {
        self.solve_like_a_human();
        println!();
        self.print();
        if self.unsolved() {
            println!("\n**** SUDOKU NOT SOLVED ****\n");
            self.dump_notes();
        } else {
            println!("\n**** SUDOKU SOLVED ****\n");
        }
        self.dump_rating();
    }

    pub fn restore(&mut self) {
        self.set_board_string(&self.original_board());
    }

    pub fn set_board_string(&mut self, board_string: &str) {
        if board_string.chars().filter(|c| c.is_ascii_digit()).count() != 81 {
            log::error!("Invalid Sudoku board: must contain exactly 81 numeric characters");
            return;
        }
        self.clear();
        let digits = board_string
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .take(81);
        self.original_board = [[EMPTY; 9]; 9];
        for (idx, digit) in digits.enumerate() {
            let row = idx / 9;
            let col = idx % 9;
            self.board[row][col] = digit;
            self.original_board[row][col] = digit;
        }
    }

    /// Generates a new Sudoku puzzle with a given number of filled cells.
    /// The puzzle is guaranteed to have a unique solution.
    pub fn generate(filled_cells: usize) -> Option<Self> {
        let mut rng = rand::rng();
        let mut all_digits: Vec<u8> = (1..=9).collect();
        let mut sudoku = Sudoku::new();
        // Fill the 3 diagonal boxes (top-left, middle, bottom-right)
        for box_idx in 0..3 {
            let start_row = box_idx * 3;
            let start_col = box_idx * 3;
            // Fill the box with a shuffled sequence of 1-9
            all_digits.shuffle(&mut rng);
            for (i, &num) in all_digits.iter().enumerate() {
                sudoku.board[start_row + i / 3][start_col + i % 3] = num;
            }
        }
        sudoku.solve_by_backtracking();
        // Get all filled cells that haven't been removed yet
        let mut available_cells: Vec<(usize, usize)> = (0..9)
            .flat_map(|row| (0..9).map(move |col| (row, col)))
            .filter(|&(row, col)| sudoku.board[row][col] != EMPTY)
            .collect();
        available_cells.shuffle(&mut rng);
        available_cells.truncate(81 - filled_cells);
        while !available_cells.is_empty() {
            // Take the last cell from the shuffled list
            let (row, col) = available_cells.pop().unwrap();
            sudoku.board[row][col] = EMPTY;
            // Check if the puzzle still has a unique solution
            let mut test_sudoku = sudoku.clone();

            // Count solutions using backtracking (up to 2)
            fn count_solutions(sudoku: &mut Sudoku, count: &mut usize, max_count: usize) -> bool {
                if *count >= max_count {
                    return true; // Early return if we already found enough solutions
                }
                let mut found_empty = false;
                let mut empty_row = 0;
                let mut empty_col = 0;
                'find_empty: for r in 0..9 {
                    for c in 0..9 {
                        if sudoku.board[r][c] == EMPTY {
                            empty_row = r;
                            empty_col = c;
                            found_empty = true;
                            break 'find_empty;
                        }
                    }
                }
                // If no empty cell is found, we have a solution
                if !found_empty {
                    *count += 1;
                    return *count >= max_count;
                }
                // Try each possible value
                for num in 1..=9 {
                    if !sudoku.can_place(empty_row, empty_col, num) {
                        continue;
                    }
                    // Place and recurse
                    sudoku.board[empty_row][empty_col] = num;
                    if count_solutions(sudoku, count, max_count) {
                        return true;
                    }
                    // Backtrack
                    sudoku.board[empty_row][empty_col] = EMPTY;
                }
                false
            }

            let mut solution_count = 0;
            count_solutions(&mut test_sudoku, &mut solution_count, 2);

            if solution_count != 1 {
                return None;
            }
        }
        Some(sudoku)
    }
}
