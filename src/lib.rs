use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::LazyLock;

mod claimingpair;
mod generator;
mod hiddenpair;
mod hiddensingle;
mod lastdigit;
mod lockedpair;
mod triplets;
mod obviouspair;
mod obvioussingle;
mod pointingpair;
mod skyscraper;
mod xwing;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    ObviousTriplet,
    Skyscraper,
    XWing,
}

impl Strategy {
    #[allow(clippy::wrong_self_convention)]
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
            Strategy::ObviousTriplet => "Obvious Triplet",
            Strategy::Skyscraper => "Skyscraper",
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
            Strategy::ObviousTriplet => 80,
            Strategy::Skyscraper => 130,
            Strategy::XWing => 140,
        }
    }
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

type StrategyApplicator = Vec<(Strategy, fn(&Sudoku) -> StrategyResult)>;
static STRATEGY_FUNCTIONS: LazyLock<StrategyApplicator> = LazyLock::new(|| {
    let mut strategies: StrategyApplicator = vec![
        (Strategy::LastDigit, Sudoku::find_last_digit),
        (Strategy::ObviousSingle, Sudoku::find_obvious_single),
        (Strategy::HiddenSingle, Sudoku::find_hidden_single),
        (Strategy::LockedPair, Sudoku::find_locked_pair),
        (Strategy::PointingPair, Sudoku::find_pointing_pair),
        (Strategy::ClaimingPair, Sudoku::find_claiming_pair),
        (Strategy::ObviousPair, Sudoku::find_obvious_pair),
        (Strategy::HiddenPair, Sudoku::find_hidden_pair),
        (Strategy::ObviousTriplet, Sudoku::find_obvious_triplet),
        (Strategy::Skyscraper, Sudoku::find_skyscraper),
        (Strategy::XWing, Sudoku::find_xwing),
    ];
    // Sort strategies by difficulty to pre-empt developers from adding strategies in the wrong order
    strategies.sort_by_key(|(strategy, _)| strategy.difficulty());
    strategies
});
pub const EMPTY: u8 = 0;
pub static ALL_DIGITS: LazyLock<HashSet<u8>> = LazyLock::new(|| (1..=9).collect());

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Candidate {
    pub row: usize,
    pub col: usize,
    pub num: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn nums_removed(&self) -> usize {
        self.nums_removed
    }
    pub fn strategy(&self) -> Strategy {
        self.strategy
    }
}

// Define a custom error for invalid Sudoku board
#[derive(Debug)]
pub struct SudokuError {
    pub message: String,
}

impl std::fmt::Display for SudokuError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SudokuError {}

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

    pub fn from_string(board_string: &str) -> Result<Sudoku, SudokuError> {
        let mut sudoku = Sudoku::new();
        sudoku.set_board_string(board_string)?;
        Ok(sudoku)
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

    pub fn from_zstd(binary: &[u8]) -> Result<Self, SudokuError> {
        let mut decoder = zstd::stream::Decoder::new(binary).unwrap();
        let mut decompressed_data = Vec::new();
        std::io::copy(&mut decoder, &mut decompressed_data).unwrap();
        Sudoku::from_binary(&decompressed_data)
    }

    pub fn to_zstd(&self) -> Vec<u8> {
        let mut compressed_data = Vec::new();
        let mut encoder = zstd::stream::Encoder::new(&mut compressed_data, 21).unwrap();
        std::io::Write::write_all(&mut encoder, &self.to_binary()).unwrap();
        encoder.finish().unwrap();
        compressed_data
    }

    pub fn to_binary(&self) -> Vec<u8> {
        let mut binary_representation = Vec::new();
        // Convert the board to binary representation using low and high nibbles
        for row in self.board.iter() {
            for chunk in row.chunks(2) {
                let byte = ((chunk[0] & 0xF) << 4) | (chunk.get(1).unwrap_or(&0) & 0xF);
                binary_representation.push(byte);
            }
        }

        // Convert the candidates to binary representation using low and high nibbles
        for row in self.candidates.iter() {
            for candidates in row.iter() {
                for digit in (1..=9).step_by(2) {
                    let high_nibble = if candidates.contains(&digit) {
                        digit
                    } else {
                        0
                    };
                    let low_nibble = if candidates.contains(&(digit + 1)) {
                        digit + 1
                    } else {
                        0
                    };
                    binary_representation.push((high_nibble << 4) | low_nibble);
                }
            }
        }
        binary_representation
    }

    /// Deserialize the Sudoku board from a binary representation.
    /// The binary representation is a sequence of bytes where each byte
    /// contains two cells or candidates.
    pub fn from_binary(binary: &[u8]) -> Result<Sudoku, SudokuError> {
        if binary.len() != 450 {
            return Err(SudokuError {
                message: "Invalid binary data: must contain exactly 450 bytes".to_string(),
            });
        }
        // First come 45 bytes for the board
        let mut sudoku = Sudoku::new();
        let mut board = [[0u8; 9]; 9];
        let mut idx = 0;
        for row in &mut board {
            for col in (0..9).step_by(2) {
                let byte = binary[idx];
                row[col] = (byte >> 4) & 0xF; // High nibble
                if col + 1 < 9 {
                    row[col + 1] = byte & 0xF; // Low nibble
                }
                idx += 1;
            }
        }
        sudoku.board = board;
        // Then come 405 bytes for the candidates
        let mut notes = std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new()));
        for row in &mut notes {
            for cell_notes in row.iter_mut() {
                let mut candidates = HashSet::new();
                for _ in (0..9).step_by(2) {
                    let byte = binary[idx];
                    let high_nibble = (byte >> 4) & 0xF;
                    let low_nibble = byte & 0xF;
                    if high_nibble != 0 {
                        candidates.insert(high_nibble);
                    }
                    if low_nibble != 0 {
                        candidates.insert(low_nibble);
                    }
                    idx += 1;
                }
                *cell_notes = candidates;
            }
        }
        sudoku.candidates = notes;
        Ok(sudoku)
    }

    pub fn to_board_string(&self) -> String {
        self.board
            .iter()
            .flatten()
            .map(|&digit| (digit + b'0') as char)
            .collect()
    }

    pub fn set_board_string(&mut self, board_string: &str) -> Result<String, SudokuError> {
        let board_string = if board_string.contains(' ') || board_string.contains('\n') {
            // Handle formatted board with spaces and newlines
            board_string
                .chars()
                .filter_map(|c| {
                    if c.is_ascii_digit() {
                        Some(c)
                    } else if c == '.' || c == '_' {
                        Some('0')
                    } else {
                        None
                    }
                })
                .collect::<String>()
        } else {
            // Handle compact board string
            board_string.replace(['.', '_'], "0")
        };
        if board_string.chars().filter(|c| c.is_ascii_digit()).count() != 81 {
            return Err(SudokuError {
                message:
                    "Invalid Sudoku board: board string must contain exactly 81 digits or dots"
                        .to_string(),
            });
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
        Ok(board_string)
    }

    /// Serialize the Sudoku board including all cells' candidates to a string
    pub fn to_json(&self) -> String {
        let mut json = String::from("{\"board\":[");
        for (i, row) in self.board.iter().enumerate() {
            json.push('[');
            for (j, &cell) in row.iter().enumerate() {
                json.push_str(&cell.to_string());
                if j < 8 {
                    json.push(',');
                }
            }
            json.push(']');
            if i < 8 {
                json.push(',');
            }
        }
        json.push_str("],\"candidates\":[");
        for (i, row) in self.candidates.iter().enumerate() {
            json.push('[');
            for (j, candidates) in row.iter().enumerate() {
                json.push('[');
                for (k, candidate) in candidates.iter().enumerate() {
                    json.push_str(&format!("{}", candidate));
                    if k < candidates.len() - 1 {
                        json.push(',');
                    }
                }
                json.push(']');
                if j < 8 {
                    json.push(',');
                }
            }
            json.push(']');
            if i < 8 {
                json.push(',');
            }
        }
        json.push_str("]}");
        json
    }

    pub fn from_json(json: &str) -> Result<Self, SudokuError> {
        let parsed: Value = serde_json::from_str(json).map_err(|e| SudokuError {
            message: format!("Failed to parse JSON: {}", e),
        })?;
        let mut sudoku = Sudoku::new();
        if let Some(board) = parsed.get("board").and_then(|b| b.as_array()) {
            for (i, row) in board.iter().enumerate() {
                if let Some(row_array) = row.as_array() {
                    for (j, cell) in row_array.iter().enumerate() {
                        if let Some(num) = cell.as_u64() {
                            sudoku.board[i][j] = num as u8;
                        } else {
                            return Err(SudokuError {
                                message: "Invalid board data in JSON".to_string(),
                            });
                        }
                    }
                } else {
                    return Err(SudokuError {
                        message: "Invalid board data in JSON".to_string(),
                    });
                }
            }
        } else {
            return Err(SudokuError {
                message: "Missing or invalid 'board' field in JSON".to_string(),
            });
        }
        if let Some(candidates) = parsed.get("candidates").and_then(|c| c.as_array()) {
            for (i, row) in candidates.iter().enumerate() {
                if let Some(row_array) = row.as_array() {
                    for (j, cell_candidates) in row_array.iter().enumerate() {
                        if let Some(candidate_array) = cell_candidates.as_array() {
                            sudoku.candidates[i][j] = candidate_array
                                .iter()
                                .filter_map(|c| c.as_u64().map(|n| n as u8))
                                .collect();
                        } else {
                            return Err(SudokuError {
                                message: "Invalid candidates data in JSON".to_string(),
                            });
                        }
                    }
                } else {
                    return Err(SudokuError {
                        message: "Invalid candidates data in JSON".to_string(),
                    });
                }
            }
        } else {
            return Err(SudokuError {
                message: "Missing or invalid 'candidates' field in JSON".to_string(),
            });
        }
        Ok(sudoku)
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
        println!("{}", self);
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

    pub fn has_candidates(&self) -> bool {
        self.candidates
            .iter()
            .flatten()
            .any(|cell| !cell.is_empty())
    }

    pub fn calc_candidates(&mut self) {
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

    pub fn get_candidates(&self, row: usize, col: usize) -> HashSet<u8> {
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
            strategy: strategy_result.strategy,
        };
        for note in &strategy_result.removals.candidates_about_to_be_removed {
            assert!(self.candidates[note.row][note.col].contains(&note.num));
            self.candidates[note.row][note.col].remove(&note.num);
        }
        if let Some(cell) = &strategy_result.removals.sets_cell {
            self.board[cell.row][cell.col] = cell.num;
            // Update rating for this strategy
            self.rating
                .entry(strategy_result.strategy)
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
        for (strategy, strategy_fn) in STRATEGY_FUNCTIONS.iter() {
            let result = (strategy_fn)(self);
            if !result.removals.will_remove_candidates() {
                continue;
            }
            let nums_removed = result.removals.candidates_about_to_be_removed.len();
            self.rating
                .entry(*strategy)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return StrategyResult {
                removals: result.removals,
                strategy: *strategy,
            };
        }
        StrategyResult::empty()
    }

    /// Build a list of all possible techniques to solve the Sudoku puzzle at the current state.
    pub fn all_possible_strategies(&self) -> Vec<StrategyResult> {
        let mut strategies = Vec::new();
        for (strategy, strategy_fn) in STRATEGY_FUNCTIONS.iter() {
            let result = (strategy_fn)(self);
            if !result.removals.will_remove_candidates() {
                continue;
            }
            strategies.push(StrategyResult {
                removals: result.removals,
                strategy: *strategy,
            });
        }
        strategies
    }

    /// Solve the Sudoku puzzle using human-like strategies
    #[cfg(feature = "dump")]
    fn solve_like_a_human(&mut self) -> bool {
        // The first step always is to calculate the notes
        self.calc_candidates();
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
        self.calc_candidates();
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
        let _ = self.set_board_string(&self.original_board());
    }
}
