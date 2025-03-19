use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    None,
    LastDigit,
    ObviousSingle,
    HiddenSingle,
    ObviousPair,
    HiddenPair,
    PointingPair,
    XWing,
}

impl Strategy {
    fn to_string(&self) -> &str {
        match self {
            Strategy::None => "none",
            Strategy::LastDigit => "last digit",
            Strategy::ObviousSingle => "obvious single",
            Strategy::HiddenSingle => "hidden single",
            Strategy::PointingPair => "pointing pair",
            Strategy::ObviousPair => "obvious pair",
            Strategy::HiddenPair => "hidden pair",
            Strategy::XWing => "x-wing",
        }
    }

    fn difficulty(&self) -> i32 {
        match self {
            Strategy::None => 0,
            Strategy::LastDigit => 4,
            Strategy::ObviousSingle => 5,
            Strategy::HiddenSingle => 14,
            Strategy::PointingPair => 50,
            Strategy::ObviousPair => 60,
            Strategy::HiddenPair => 70,
            Strategy::XWing => 140,
        }
    }
}

const EMPTY: u8 = 0;

#[allow(dead_code)]
pub struct Note {
    row: usize,
    col: usize,
    num: u8,
}

#[allow(dead_code)]
pub struct Coord {
    row: usize,
    col: usize,
}

#[allow(dead_code)]
pub struct StrategyResult {
    strategy: Strategy,
    cell_affected: Coord,
    candidates_affected: Vec<Note>,
    candidates_about_to_be_removed: Vec<Note>,
}

pub struct Resolution {
    nums_removed: usize,
    strategy: Strategy,
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    board: [[u8; 9]; 9],
    original_board: [[u8; 9]; 9],
    notes: [[HashSet<u8>; 9]; 9],
    rating: HashMap<Strategy, usize>,
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

/// Represents a Sudoku puzzle and provides methods for solving and manipulating it.
///
/// The `Sudoku` struct contains the following fields:
/// - `board`: A 2D array representing the Sudoku board, where each element is a u8 representing the number in that cell (0 for empty).
/// - `notes`: A 2D array of HashSets, where each HashSet contains the possible numbers (notes) for that cell.
/// - `nums_in_row`: An array of HashSets, where each HashSet contains the numbers already present in that row.
/// - `nums_in_col`: An array of HashSets, where each HashSet contains the numbers already present in that column.
/// - `nums_in_box`: An array of HashSets, where each HashSet contains the numbers already present in that 3x3 box.
/// - `rating`: A HashMap to store the rating of the Sudoku puzzle (not currently used).
impl Sudoku {
    pub fn new() -> Sudoku {
        Sudoku {
            board: [[EMPTY; 9]; 9],
            original_board: [[EMPTY; 9]; 9],
            notes: std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new())),
            rating: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.notes = std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new()));
        self.board = [[EMPTY; 9]; 9];
        self.rating.clear();
    }

    fn original_empty_cells(&self) -> usize {
        self.original_board
            .iter()
            .flatten()
            .filter(|&&cell| cell == EMPTY)
            .count()
    }

    pub fn dump_rating(&self) {
        println!("Rating:");
        let candidates_removed = self
            .rating
            .iter()
            .map(|(_, &count)| count)
            .sum::<usize>();
        let total_rating: i32 = self
            .rating
            .iter()
            .map(|(strategy, &count)| strategy.difficulty() * count as i32)
            .sum();
        let difficulty = (total_rating as f64) / (self.original_empty_cells() as f64);
        println!("  Difficulty: {:.2}", difficulty);
        println!("  Total candidates removed: {}; by …", candidates_removed);
        let mut strategies: Vec<(&Strategy, &usize)> = self.rating.iter().collect();
        strategies.sort_by_key(|(strategy, _)| strategy.difficulty());
        for (strategy, count) in strategies {
            println!("  - {} ({}): {}", strategy.to_string(), strategy.difficulty(), count);
        }
    }

    fn unsolved(&self) -> bool {
        self.board.iter().any(|row| row.contains(&EMPTY))
    }

    #[allow(dead_code)]
    pub fn rating(&self) -> HashMap<Strategy, usize>{
        self.rating.clone()
    } 

    pub fn serialized(&self) -> String {
        self.board
            .iter()
            .flatten()
            .map(|&digit| (digit as u8 + b'0') as char)
            .collect()
    }

    /// print the board
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
                        if self.notes[i][j].contains(&num) {
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
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
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
                self.notes[row][col] = notes;
            }
        }
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
        return self.solve();
    }

    /// Check if there are last digits in any of the rows.
    /// If so, remove it from the notes in the row, column, and box where we've found it.
    /// Set the respective cell to the digit.
    fn resolve_last_digit_in_rows(&mut self) -> usize {
        for row in 0..9 {
            // Find the only empty cell in the row, if there's exactly one
            let empty_cells = (0..9)
                .filter(|&col| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let col = empty_cells[0];
            assert!(self.notes[row][col].len() == 1);
            let &digit = self.notes[row][col].iter().next().unwrap();
            return self.set_num(digit, row, col);
        }
        0
    }

    fn resolve_last_digit_in_cols(&mut self) -> usize {
        for col in 0..9 {
            let empty_cells = (0..9)
                .filter(|&row| self.board[row][col] == EMPTY)
                .collect::<Vec<_>>();
            if empty_cells.len() != 1 {
                continue;
            }
            let row = empty_cells[0];
            assert!(self.notes[row][col].len() == 1);
            // if there's exactly one empty cell, set the digit
            let &digit = self.notes[row][col].iter().next().unwrap();
            return self.set_num(digit, row, col);
        }
        0
    }

    fn resolve_last_digit_in_boxes(&mut self) -> usize {
        for box_index in 0..9 {
            let start_row = 3 * (box_index / 3);
            let start_col = 3 * (box_index % 3);
            let mut count = 0;
            let mut empty_row = 0;
            let mut empty_col = 0;

            // count the number of empty cells in the box and track the location of the empty cell
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

                    // If we already found more than one empty cell, we can break early
                    if count > 1 {
                        break 'box_search;
                    }
                }
            }
            if count != 1 {
                continue;
            }
            assert!(self.notes[empty_row][empty_col].len() == 1);
            // There's exactly one empty cell, so we can set the digit
            let &digit = self.notes[empty_row][empty_col].iter().next().unwrap();
            return self.set_num(digit, empty_row, empty_col);
        }
        0
    }

    fn resolve_last_digit(&mut self) -> usize {
        let count = self.resolve_last_digit_in_rows();
        if count > 0 {
            return count;
        }
        let count = self.resolve_last_digit_in_cols();
        if count > 0 {
            return count;
        }
        self.resolve_last_digit_in_boxes()
    }

    fn resolve_obvious_single(&mut self) -> usize {
        for row in 0..9 {
            for col in 0..9 {
                if self.notes[row][col].len() != 1 {
                    continue;
                }
                assert_eq!(self.board[row][col], EMPTY);
                let &num = self.notes[row][col].iter().next().unwrap();
                return self.set_num(num, row, col);
            }
        }
        0
    }

    /// Finds and resolves "hidden single" candidates in the Sudoku puzzle.
    ///
    /// A hidden single occurs when a digit can only go in one cell within a group (row, column, or box),
    /// even though that cell may have multiple candidates.
    ///
    /// Returns the number of notes removed as a result of placing new digits.
    fn resolve_hidden_single(&mut self) -> usize {
        let row_result = self.resolve_hidden_single_row();
        if row_result > 0 {
            println!("Hidden single in row");
            return row_result;
        }
        let col_result = self.resolve_hidden_single_col();
        if col_result > 0 {
            println!("Hidden single in column");
            return col_result;
        }
        let box_result = self.resolve_hidden_single_box();
        if box_result > 0 {
            println!("Hidden single in box");
            return box_result;
        }
        0
    }

    fn resolve_hidden_single_row(&mut self) -> usize {
        // Check for hidden singles in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] > 0 {
                    continue;
                }
                for &num in &self.notes[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != col && self.notes[row][i].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return self.set_num(num, row, col);
                    }
                }
            }
        }
        0
    }

    fn resolve_hidden_single_col(&mut self) -> usize {
        // Check for hidden singles in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.notes[row][col] {
                    let mut found = false;
                    for i in 0..9 {
                        if i != row && self.notes[i][col].contains(&num) {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return self.set_num(num, row, col);
                    }
                }
            }
        }
        0
    }

    fn resolve_hidden_single_box(&mut self) -> usize {
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
                        for &num in &self.notes[row][col] {
                            let mut found = false;
                            'box_check: for r in 0..3 {
                                for c in 0..3 {
                                    let check_row = start_row + r;
                                    let check_col = start_col + c;
                                    if (check_row != row || check_col != col)
                                        && self.notes[check_row][check_col].contains(&num)
                                    {
                                        found = true;
                                        break 'box_check;
                                    }
                                }
                            }
                            if !found {
                                return self.set_num(num, row, col);
                            }
                        }
                    }
                }
            }
        }
        0
    }

    fn resolve_pointing_pair_in_rows(&mut self) -> usize {
        let mut count = 0;

        for row in 0..9 {
            for num in 1..=9 {
                // Track cells with candidate `num` in this row
                let mut cells_with_num = Vec::new();

                for col in 0..9 {
                    if !self.notes[row][col].contains(&num) {
                        continue;
                    }
                    cells_with_num.push(col);
                }

                // Need exactly 2 cells with this candidate
                if cells_with_num.len() != 2 {
                    continue;
                }

                let col1 = cells_with_num[0];
                let col2 = cells_with_num[1];

                // They must be in the same box
                if col1 / 3 != col2 / 3 {
                    continue;
                }

                let box_col = col1 / 3;
                let start_row = 3 * (row / 3);

                println!(
                    "Found pointing pair {:?} in row {} at columns {} and {}",
                    num, row, col1, col2
                );

                // Remove this candidate from other cells in the same box but different row
                for r in start_row..start_row + 3 {
                    if r == row {
                        continue; // Skip the original row
                    }

                    for c in (box_col * 3)..(box_col * 3 + 3) {
                        if self.notes[r][c].remove(&num) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn resolve_pointing_pair_in_cols(&mut self) -> usize {
        let mut count = 0;
        for col in 0..9 {
            for num in 1..=9 {
                // Find cells in this column that contain the number as a candidate
                let mut cells_with_num = Vec::new();
                for row in 0..9 {
                    if !self.notes[row][col].contains(&num) {
                        continue;
                    }
                    cells_with_num.push(row);
                }

                // Check if exactly two cells with this candidate are in the same box
                if cells_with_num.len() != 2 {
                    continue;
                }

                let row1 = cells_with_num[0];
                let row2 = cells_with_num[1];

                // Check if they're in the same box
                if row1 / 3 != row2 / 3 {
                    continue;
                }

                let box_idx = row1 / 3;
                let start_col = 3 * (col / 3);
                println!(
                    "Found pointing pair {:?} in column {} at rows {} and {}",
                    num, col, row1, row2
                );

                // Remove this candidate from other cells in the same box but different column
                for c in start_col..start_col + 3 {
                    if c == col {
                        continue; // Skip the original column
                    }

                    for r in (box_idx * 3)..(box_idx * 3 + 3) {
                        if self.notes[r][c].remove(&num) {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn resolve_pointing_pair(&mut self) -> usize {
        let count = self.resolve_pointing_pair_in_rows();
        if count > 0 {
            return count;
        }
        self.resolve_pointing_pair_in_cols()
    }

    fn resolve_obvious_pair_in_rows(&mut self) -> usize {
        // Check for obvious pairs in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.notes[row][col].len() != 2 {
                    continue;
                }

                let pair = self.notes[row][col].clone();

                // Find pair in same row
                for i in (col + 1)..9 {
                    if self.notes[row][i] != pair {
                        continue;
                    }
                    let mut count = 0;
                    // Found a pair, remove these candidates from other cells in the same row
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != col && j != i {
                            for &num in &nums {
                                if self.notes[row][j].remove(&num) {
                                    count += 1;
                                }
                            }
                        }
                    }
                    if count > 0 {
                        println!(
                            "Found obvious pair {:?} in row {} at columns {} and {}",
                            nums, row, col, i
                        );
                        return count;
                    }
                }
            }
        }
        0
    }

    fn resolve_obvious_pair_in_cols(&mut self) -> usize {
        // Check for obvious pairs in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.notes[row][col].len() != 2 {
                    continue;
                }

                let pair = self.notes[row][col].clone();

                // Find pair in same column
                for i in (row + 1)..9 {
                    if self.notes[i][col] != pair {
                        continue;
                    }
                    let mut count = 0;
                    // Found a pair, remove these candidates from other cells in the same column
                    let nums: Vec<u8> = pair.iter().cloned().collect();
                    for j in 0..9 {
                        if j != row && j != i {
                            for &num in &nums {
                                if self.notes[j][col].remove(&num) {
                                    count += 1;
                                }
                            }
                        }
                    }
                    if count > 0 {
                        println!(
                            "Found obvious pair {:?} in column {} at rows {} and {}",
                            nums, col, row, i
                        );
                        return count;
                    }
                }
            }
        }
        0
    }

    fn resolve_obvious_pair_in_boxes(&mut self) -> usize {
        // Check for obvious pairs in boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let start_row = box_row * 3;
                let start_col = box_col * 3;

                for r1 in 0..3 {
                    for c1 in 0..3 {
                        let row1 = start_row + r1;
                        let col1 = start_col + c1;

                        if self.notes[row1][col1].len() != 2 {
                            continue;
                        }

                        let pair = self.notes[row1][col1].clone();

                        for r2 in 0..3 {
                            for c2 in 0..3 {
                                let row2 = start_row + r2;
                                let col2 = start_col + c2;

                                // Skip same cell or already checked pairs
                                if (row1 == row2 && col1 == col2) || (r2 * 3 + c2 <= r1 * 3 + c1) {
                                    continue;
                                }

                                if self.notes[row2][col2] != pair {
                                    continue;
                                }

                                let mut count = 0;
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
                                                if self.notes[row][col].remove(&num) {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                                if count > 0 {
                                    println!(
                                        "Found obvious pair {:?} in box at ({},{}) and ({},{})",
                                        nums, row1, col1, row2, col2
                                    );
                                    return count;
                                }
                            }
                        }
                    }
                }
            }
        }
        0
    }

    fn resolve_obvious_pair(&mut self) -> usize {
        let count = self.resolve_obvious_pair_in_rows();
        if count > 0 {
            return count;
        }
        let count = self.resolve_obvious_pair_in_cols();
        if count > 0 {
            return count;
        }
        self.resolve_obvious_pair_in_boxes()
    }

    fn resolve_hidden_pair_in_rows(&mut self) -> usize {
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
                        if self.board[row][col] != 0 {
                            continue;
                        }
                        for &num in &self.notes[row][col] {
                            digit_locations.entry(num).or_default().push((row, col));
                        }
                    }
                }

                // Find pairs of digits that appear in exactly the same two cells
                let mut digit_pairs: Vec<(u8, u8, (usize, usize), (usize, usize))> = Vec::new();
                let candidates: Vec<(u8, &Vec<(usize, usize)>)> = digit_locations
                    .iter()
                    .filter(|(_, cells)| cells.len() == 2)
                    .map(|(&digit, cells)| (digit, cells))
                    .collect();

                for i in 0..candidates.len() {
                    let (digit1, cells1) = &candidates[i];
                    for j in (i + 1)..candidates.len() {
                        let (digit2, cells2) = &candidates[j];
                        if cells1 == cells2 {
                            digit_pairs.push((*digit1, *digit2, cells1[0], cells1[1]));
                        }
                    }
                }

                // Apply the strategy: for each hidden pair, remove all other digits from those cells
                for (digit1, digit2, cell1, cell2) in digit_pairs {
                    let mut count = 0;
                    // Remove all other digits from these two cells
                    for &(row, col) in &[cell1, cell2] {
                        for digit in 1..=9 {
                            if digit != digit1
                                && digit != digit2
                                && self.notes[row][col].remove(&digit)
                            {
                                count += 1;
                            }
                        }
                    }
                    if count > 0 {
                        println!(
                            "Found hidden pair ({},{}) in box at cells ({},{}) and ({},{})",
                            digit1, digit2, cell1.0, cell1.1, cell2.0, cell2.1
                        );
                        return count;
                    }
                }
            }
        }
        0
    }

    fn resolve_hidden_pair_in_cols(&mut self) -> usize {
        // Check for hidden pairs in rows
        for row in 0..9 {
            // Find which digits appear in exactly two cells in the row
            let mut digit_locations: HashMap<u8, Vec<usize>> = HashMap::new();
            for col in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.notes[row][col] {
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

            for i in 0..candidates.len() {
                let (digit1, cols1) = &candidates[i];
                for j in (i + 1)..candidates.len() {
                    let (digit2, cols2) = &candidates[j];
                    if cols1 == cols2 {
                        digit_pairs.push((*digit1, *digit2, cols1[0], cols1[1]));
                    }
                }
            }

            // Apply the strategy: for each hidden pair, remove all other digits from those cells
            for (digit1, digit2, col1, col2) in digit_pairs {
                let mut count = 0;
                // Remove all other digits from these two cells
                for &col in &[col1, col2] {
                    for digit in 1..=9 {
                        if digit != digit1 && digit != digit2 && self.notes[row][col].remove(&digit)
                        {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    println!(
                        "Found hidden pair ({},{}) in row {} at columns {} and {}",
                        digit1, digit2, row, col1, col2
                    );
                    return count;
                }
            }
        }
        0
    }

    fn resolve_hidden_pair_in_boxes(&mut self) -> usize {
        // Check for hidden pairs in columns
        for col in 0..9 {
            // Find which digits appear in exactly two cells in the column
            let mut digit_locations: HashMap<u8, Vec<usize>> = HashMap::new();
            for row in 0..9 {
                if self.board[row][col] != EMPTY {
                    continue;
                }
                for &num in &self.notes[row][col] {
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

            for i in 0..candidates.len() {
                let (digit1, rows1) = &candidates[i];
                for j in (i + 1)..candidates.len() {
                    let (digit2, rows2) = &candidates[j];
                    if rows1 == rows2 {
                        digit_pairs.push((*digit1, *digit2, rows1[0], rows1[1]));
                    }
                }
            }

            // Apply the strategy: for each hidden pair, remove all other digits from those cells
            for (digit1, digit2, row1, row2) in digit_pairs {
                let mut count = 0;
                // Remove all other digits from these two cells
                for &row in &[row1, row2] {
                    for digit in 1..=9 {
                        if digit != digit1 && digit != digit2 && self.notes[row][col].remove(&digit)
                        {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    println!(
                        "Found hidden pair ({},{}) in column {} at rows {} and {}",
                        digit1, digit2, col, row1, row2
                    );
                    return count;
                }
            }
        }
        0
    }

    fn resolve_hidden_pair(&mut self) -> usize {
        let count = self.resolve_hidden_pair_in_rows();
        if count > 0 {
            return count;
        }
        let count = self.resolve_hidden_pair_in_cols();
        if count > 0 {
            return count;
        }
        self.resolve_hidden_pair_in_boxes()
    }

    fn resolve_xwing_in_rows(&mut self) -> usize {
        // Check for x-wings in rows
        for num in 1..=9 {
            for row1 in 0..8 {
                // We don't need to check the last row
                let mut cols1 = Vec::new();
                // Find columns with candidate `num` in this row
                for col in 0..9 {
                    if self.notes[row1][col].contains(&num) {
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
                        if self.notes[row2][col].contains(&num) {
                            cols2.push(col);
                        }
                    }
                    // If we found another row with the same columns, we have an X-Wing
                    if cols2.len() != 2 || cols1 != cols2 {
                        continue;
                    }
                    println!(
                        "Found x-wing {:?} in rows {} and {} at columns {:?}",
                        num, row1, row2, cols1
                    );
                    let mut count = 0;
                    // Remove the candidate from other cells in the same columns
                    for row in 0..9 {
                        if row == row1 || row == row2 {
                            continue;
                        }

                        for &col in &cols1 {
                            if self.notes[row][col].remove(&num) {
                                count += 1;
                                println!("Removed candidate {:?} from cell ({},{})", num, row, col);
                            }
                        }
                    }
                    if count > 0 {
                        return count;
                    }
                }
            }
        }
        0
    }

    fn resolve_xwing_in_cols(&mut self) -> usize {
        // Check for x-wings in columns
        for num in 1..=9 {
            for col1 in 0..8 {
                // We don't need to check the last column
                let mut rows1 = Vec::new();

                // Find rows with candidate `num` in this column
                for row in 0..9 {
                    if self.notes[row][col1].contains(&num) {
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
                        if self.notes[row][col2].contains(&num) {
                            rows2.push(row);
                        }
                    }
                    // If we found another column with the same rows, we have an X-Wing
                    if rows2.len() != 2 || rows1 != rows2 {
                        continue;
                    }
                    println!(
                        "Found x-wing {:?} in columns {} and {} at rows {:?}",
                        num, col1, col2, rows1
                    );
                    let mut count = 0;
                    // Remove candidates from other cells in the same rows
                    for &row in &rows1 {
                        for col in 0..9 {
                            if col == col1 || col == col2 {
                                continue;
                            }

                            if self.notes[row][col].remove(&num) {
                                count += 1;
                                println!("Removed candidate {:?} from cell ({},{})", num, row, col);
                            }
                        }
                    }
                    if count > 0 {
                        return count;
                    }
                }
            }
        }
        0
    }

    /// Find and resolve X-Wing candidates.
    /// An X-Wing occurs when a digit can only go in two rows and two columns, forming a rectangle.
    /// In this case, the digit can be removed from all other cells in the same rows and columns.
    fn resolve_xwing(&mut self) -> usize {
        let count = self.resolve_xwing_in_rows();
        if count > 0 {
            return count;
        }
        self.resolve_xwing_in_cols()
    }

    fn remove_notes_in_row(&mut self, nums: &[u8], row: usize) -> usize {
        (0..9)
            .map(|col| {
                nums.iter()
                    .filter(|&num| self.notes[row][col].remove(num))
                    .count()
            })
            .sum()
    }

    fn remove_notes_in_col(&mut self, nums: &[u8], col: usize) -> usize {
        (0..9)
            .map(|row| {
                nums.iter()
                    .filter(|&num| self.notes[row][col].remove(num))
                    .count()
            })
            .sum()
    }

    fn remove_notes_in_box(&mut self, nums: &[u8], row: usize, col: usize) -> usize {
        let start_row = 3 * (row / 3);
        let start_col = 3 * (col / 3);
        (0..3)
            .flat_map(|i| (0..3).map(move |j| (start_row + i, start_col + j)))
            .map(|(r, c)| {
                nums.iter()
                    .filter(|&num| self.notes[r][c].remove(num))
                    .count()
            })
            .sum()
    }

    /// Remove candidates from the notes in the same row, column, and box where we've set a digit.
    fn remove_notes(&mut self, nums: &[u8], row: usize, col: usize) -> usize {
        let mut count = 0;
        count += self.remove_notes_in_row(nums, row);
        count += self.remove_notes_in_col(nums, col);
        count += self.remove_notes_in_box(nums, row, col);
        println!("Candidates removed: {}", count);
        count
    }

    pub fn get_num(&self, row: usize, col: usize) -> u8 {
        self.board[row][col]
    }

    #[allow(dead_code)]
    pub fn get_notes(&self, row: usize, col: usize) -> HashSet<u8> {
        self.notes[row][col].clone()
    }

    /// Set a digit in the Sudoku board and remove its candidates from the notes.
    /// Return the number of notes removed.
    pub fn set_num(&mut self, num: u8, row: usize, col: usize) -> usize {
        println!("Setting num {} in row {}, col {}", num, row, col);
        self.board[row][col] = num;
        let mut count = self.notes[row][col].len();
        self.notes[row][col].clear();
        count += self.remove_notes(&[num], row, col);
        count
    }

    pub fn prev_step(&mut self) -> Resolution {
        Resolution { nums_removed: 0, strategy: Strategy::None }
    }

    pub fn next_step(&mut self) -> Resolution {
        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::LastDigit.to_string()
        );
        let mut nums_removed = self.resolve_last_digit();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::LastDigit)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::LastDigit,
            };
        }

        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::ObviousSingle.to_string()
        );
        nums_removed = self.resolve_obvious_single();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::ObviousSingle)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::ObviousSingle,
            };
        }

        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::HiddenSingle.to_string()
        );
        nums_removed = self.resolve_hidden_single();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::HiddenSingle)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::HiddenSingle,
            };
        }

        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::PointingPair.to_string()
        );
        nums_removed = self.resolve_pointing_pair();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::PointingPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::PointingPair,
            };
        }

        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::ObviousPair.to_string()
        );
        nums_removed = self.resolve_obvious_pair();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::ObviousPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::ObviousPair,
            };
        }

        print!(
            "\nApplying strategy '{}' ... ",
            Strategy::HiddenPair.to_string()
        );
        nums_removed = self.resolve_hidden_pair();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::HiddenPair)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::HiddenPair,
            };
        }

        print!("\nApplying strategy '{}' ... ", Strategy::XWing.to_string());
        nums_removed = self.resolve_xwing();
        if nums_removed > 0 {
            self.rating
                .entry(Strategy::XWing)
                .and_modify(|count| *count += nums_removed)
                .or_insert(nums_removed);
            return Resolution {
                nums_removed,
                strategy: Strategy::XWing,
            };
        }

        Resolution {
            nums_removed: 0,
            strategy: Strategy::LastDigit,
        }
    }

    /// Solve the Sudoku puzzle using human-like strategies
    fn solve_like_a_human(&mut self) {
        // the first step always is to calculate the notes
        self.calc_all_notes();
        // since we're starting from scratch, we clear the rating
        self.rating.clear();
        while self.unsolved() {
            let result: Resolution = self.next_step();
            if result.nums_removed == 0 && result.strategy == Strategy::None {
                println!("\nNo more strategies to apply");
                break;
            }
            self.print();
            self.dump_notes();
        }
    }

    pub fn solve_puzzle(&mut self) {
        let mut sudoku = self.clone();
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

        let start = std::time::Instant::now();
        sudoku.solve_by_backtracking();

        if self.serialized() != sudoku.serialized() {
            println!("\nSOLUTIONS DIFFER\n");
            println!("Human-like solver:");
            self.print();
            println!("Backtracking solver:");
            sudoku.print();
        }

        let duration = start.elapsed();
        println!(
            "For comparison: time to solve with backtracker: {} µs",
            duration.as_micros()
        );
    }

    pub fn restore(&mut self) {
        self.board = self.original_board;
        self.calc_all_notes();
    }

    pub fn from_string(&mut self, board_string: &str) {
        if board_string.chars().filter(|c| c.is_digit(10)).count() != 81 {
            eprintln!("Invalid Sudoku board: must contain exactly 81 numeric characters");
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_solver() {
        let board_string =
            "860001000009250006000000008010020760040000000608000053080075024050002000300000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.solve_by_backtracking();

        assert_eq!(
            sudoku.serialized(),
            "865431297479258316231697548513824769947563182628719453186375924754982631392146875"
        );
    }

    #[test]
    fn test_from_string() {
        let board_string =
            "123456789000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        for i in 0..9 {
            assert_eq!(sudoku.board[0][i], (i + 1) as u8);
        }
    }

    #[test]
    fn test_serialized() {
        let board_string =
            "123456789000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        assert_eq!(sudoku.serialized(), board_string);
    }

    #[test]
    fn test_unsolved() {
        let board_string =
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);

        assert!(sudoku.unsolved());

        let board_string =
            "123456789123456789123456789123456789123456789123456789123456789123456789123456789"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        assert!(!sudoku.unsolved());
    }

    #[test]
    fn test_can_place() {
        let board_string =
            "123456789000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);

        for j in 0..9 {
            for i in 0..9 {
                assert!(!sudoku.can_place(j, i, i as u8 + 1));
            }
        }
    }

    #[test]
    fn test_calc_all_notes() {
        let board_string =
            "120000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.calc_all_notes();

        // Cell (0,0) has value 1, so notes should be empty
        assert_eq!(sudoku.notes[0][0].len(), 0);

        // Cell (0,1) has value 2, so notes should be empty
        assert_eq!(sudoku.notes[0][1].len(), 0);

        // Cell (0,2) should not have 1 or 2 in notes (same row)
        assert!(!sudoku.notes[0][2].contains(&1));
        assert!(!sudoku.notes[0][2].contains(&2));

        // Cell (1,0) should not have 1 in notes (same column)
        assert!(!sudoku.notes[1][0].contains(&1));

        // Cell (1,1) should not have 2 in notes (same column)
        assert!(!sudoku.notes[1][1].contains(&2));

        // Cell (1,1) should not have 1 in notes (same box)
        assert!(!sudoku.notes[1][1].contains(&1));
    }

    #[test]
    fn test_set_num() {
        let board_string =
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.calc_all_notes();

        let notes_removed = sudoku.set_num(1, 0, 0);
        assert_eq!(sudoku.board[0][0], 1);
        assert!(notes_removed > 0);
        assert!(!sudoku.notes[0][1].contains(&1)); // removed from row
        assert!(!sudoku.notes[1][0].contains(&1)); // removed from column
        assert!(!sudoku.notes[1][1].contains(&1)); // removed from box
    }

    #[test]
    fn test_resolve_obvious_single() {
        let board_string =
            "120000000000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.calc_all_notes();

        // Manually set up a situation where there's an obvious single
        for num in 1..=9 {
            if num != 3 {
                sudoku.notes[0][2].remove(&num);
            }
        }

        let notes_removed = sudoku.resolve_obvious_single();
        assert_eq!(notes_removed, 19);
        assert_eq!(sudoku.board[0][2], 3);
    }

    #[test]
    fn test_resolve_last_digit() {
        let board_string =
            "123456780000000000000000000000000000000000000000000000000000000000000000000000000"
                .to_string();
        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.calc_all_notes();

        let notes_removed = sudoku.resolve_last_digit();
        assert_eq!(notes_removed, 13);
        assert_eq!(sudoku.board[0][8], 9);
    }

    #[test]
    fn test_strategy_enum() {
        assert_eq!(Strategy::LastDigit.to_string(), "last digit");
        assert_eq!(Strategy::ObviousSingle.to_string(), "obvious single");
        assert_eq!(Strategy::HiddenSingle.to_string(), "hidden single");

        assert_eq!(Strategy::LastDigit.difficulty(), 4);
        assert_eq!(Strategy::ObviousSingle.difficulty(), 5);
        assert_eq!(Strategy::XWing.difficulty(), 140);
    }

    #[test]
    fn test_simple_sudoku_solution() {
        // This is a very simple Sudoku that can be solved with just obvious singles
        let board_string =
            "123456789456789123789123456234567891567891234891234567345678912678912345912345678"
                .to_string();
        // Change one cell to empty
        let mut chars: Vec<char> = board_string.chars().collect();
        chars[0] = '0';
        let board_string: String = chars.into_iter().collect();

        let mut sudoku = Sudoku::new();
        sudoku.from_string(&board_string);
        sudoku.solve_puzzle();
        assert_eq!(sudoku.board[0][0], 1);
        assert!(!sudoku.unsolved());
    }

    #[test]
    fn test_resolve_hidden_single() {
        let mut sudoku = Sudoku::new();
        sudoku.from_string(
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000",
        );
        sudoku.calc_all_notes();

        // Set up a hidden single in row 0
        for i in 1..9 {
            sudoku.notes[0][i].remove(&1);
        }

        let notes_removed = sudoku.resolve_hidden_single();
        assert!(notes_removed > 0);
        assert_eq!(sudoku.board[0][0], 1);
    }
}
