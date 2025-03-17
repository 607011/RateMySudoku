use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SolvingStrategy {
    LastDigitInRow,
    LastDigitInCol,
    LastDigitInBox,
    ObviousSingle,
    HiddenSingle,
    ObviousPair,
    HiddenPair,
    PointingPair,
}

impl SolvingStrategy {
    fn to_string(&self) -> &str {
        match self {
            SolvingStrategy::LastDigitInRow => "last digit in row",
            SolvingStrategy::LastDigitInCol => "last digit in column",
            SolvingStrategy::LastDigitInBox => "last digit in box",
            SolvingStrategy::ObviousSingle => "obvious single",
            SolvingStrategy::HiddenSingle => "hidden single",
            SolvingStrategy::PointingPair => "pointing pair",
            SolvingStrategy::ObviousPair => "obvious pair",
            SolvingStrategy::HiddenPair => "hidden pair",
        }
    }

    fn difficulty(&self) -> i32 {
        match self {
            SolvingStrategy::LastDigitInRow => 1,
            SolvingStrategy::LastDigitInCol => 1,
            SolvingStrategy::LastDigitInBox => 1,
            SolvingStrategy::ObviousSingle => 2,
            SolvingStrategy::HiddenSingle => 3,
            SolvingStrategy::PointingPair => 4,
            SolvingStrategy::ObviousPair => 5,
            SolvingStrategy::HiddenPair => 6,
        }
    }
}

const EMPTY: u8 = 0;

#[derive(Debug, Clone)]
pub struct Sudoku {
    board: [[u8; 9]; 9],
    notes: [[HashSet<u8>; 9]; 9],
    nums_in_row: [HashSet<u8>; 9],
    nums_in_col: [HashSet<u8>; 9],
    nums_in_box: [HashSet<u8>; 9],
    rating: HashMap<SolvingStrategy, usize>,
    original_empty_cells: usize,
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
    pub fn new(board_string: &String) -> Sudoku {
        if board_string.chars().filter(|c| c.is_digit(10)).count() != 81 {
            panic!("Invalid Sudoku board: must contain exactly 81 numeric characters");
        }
        let mut b = Sudoku {
            board: [[0; 9]; 9],
            notes: std::array::from_fn(|_| std::array::from_fn(|_| HashSet::new())),
            nums_in_row: std::array::from_fn(|_| HashSet::new()),
            nums_in_col: std::array::from_fn(|_| HashSet::new()),
            nums_in_box: std::array::from_fn(|_| HashSet::new()),
            rating: HashMap::new(),
            original_empty_cells: board_string.chars().filter(|&c| c == '0').count(),
        };
        b.from_string(&board_string);
        b
    }

    pub fn dump_rating(&self) {
        println!("Rating:");
        let total_rating: i32 = self
            .rating
            .iter()
            .map(|(strategy, &count)| strategy.difficulty() * count as i32)
            .sum();
        let difficulty = (total_rating as f64) / (self.original_empty_cells as f64);
        println!("  Difficulty: {:.2}", difficulty);
        println!("  Total candidates removed: {}", total_rating);
        for (strategy, count) in &self.rating {
            println!("  - {}: {}", strategy.to_string(), count);
        }
    }

    fn unsolved(&self) -> bool {
        self.board.iter().any(|row| row.contains(&0))
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
        for i in 0..9 {
            self.nums_in_row[i] = self.calc_nums_in_row(i);
            self.nums_in_col[i] = self.calc_nums_in_col(i);
            self.nums_in_box[i] = self.calc_nums_in_box(i);
        }
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == EMPTY {
                    let mut notes = (1..=9).collect::<HashSet<u8>>();
                    for &num in &self.nums_in_row[i] {
                        notes.remove(&num);
                    }
                    for &num in &self.nums_in_col[j] {
                        notes.remove(&num);
                    }
                    for &num in &self.nums_in_box[3 * (i / 3) + j / 3] {
                        notes.remove(&num);
                    }
                    self.notes[i][j] = notes;
                }
            }
        }
    }

    /// Check if `num` can be placed in row `row` and column `col`
    pub fn can_place(&self, row: usize, col: usize, num: u8) -> bool {
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
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] == EMPTY {
                    for num in 1..=9 {
                        if self.can_place(row, col, num) {
                            self.board[row][col] = num;
                            if self.solve() {
                                return true;
                            }
                            self.board[row][col] = EMPTY;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn solve_by_backtracking(&mut self) -> bool {
        return self.solve();
    }

    /// Check if there's only one digit missing in a certain row.
    /// If so, return the digit besides the column where we've found it.
    fn find_last_digit_in_row(&self, row: usize) -> Option<(u8, usize)> {
        // Find the only empty cell in the row, if there's exactly one
        let empty_cells = (0..9)
            .filter(|&col| self.board[row][col] == 0)
            .collect::<Vec<_>>();
        if empty_cells.len() != 1 {
            return None;
        }
        let col = empty_cells[0];
        assert!(self.notes[row][col].len() == 1);
        let &digit = self.notes[row][col].iter().next().unwrap();
        Some((digit, col))
    }

    /// Check if there's only one digit missing in a certain column.
    /// If so, return the digit besides the row where we've found it.
    fn find_last_digit_in_col(&self, col: usize) -> Option<(u8, usize)> {
        let empty_cells = (0..9)
            .filter(|&row| self.board[row][col] == 0)
            .collect::<Vec<_>>();
        if empty_cells.len() != 1 {
            return None;
        }
        let row = empty_cells[0];
        assert!(self.notes[row][col].len() == 1);
        let &digit = self.notes[row][col].iter().next().unwrap();
        Some((digit, row))
    }

    /// Check if there's only one digit missing in a certain 3x3 box.
    /// If so, return the digit besides the row and column where we've found it.
    fn find_last_digit_in_box(&self, box_index: usize) -> Option<(u8, usize, usize)> {
        let start_row = 3 * (box_index / 3);
        let start_col = 3 * (box_index % 3);
        let mut count = 0;
        let mut row = 0;
        let mut col = 0;
        for i in 0..3 {
            for j in 0..3 {
                if self.board[start_row + i][start_col + j] == 0 {
                    count += 1;
                    row = start_row + i;
                    col = start_col + j;
                }
            }
        }
        if count != 1 {
            return None;
        }
        let &digit = self.notes[row][col].iter().next().unwrap();
        Some((digit, row, col))
    }

    /// Check if there are last digits in any of the rows.
    /// If so, return the digit besides the row and column where we've found it.
    fn find_last_digit_in_rows(&self) -> Option<(u8, usize, usize)> {
        for row in 0..9 {
            if let Some((num, col)) = self.find_last_digit_in_row(row) {
                return Some((num, row, col));
            }
        }
        None
    }

    /// Check if there are last digits in any of the boxes.
    /// If so, return the digit besides the row and column where we've found it.
    fn find_last_digit_in_boxes(&self) -> Option<(u8, usize, usize)> {
        for box_index in 0..9 {
            if let Some((num, row, col)) = self.find_last_digit_in_box(box_index) {
                return Some((num, row, col));
            }
        }
        None
    }

    /// Check if there are last digits in any of the columns.
    /// If so, return the digit besides the row and column where we've found it.
    fn find_last_digit_in_cols(&self) -> Option<(u8, usize, usize)> {
        for col in 0..9 {
            if let Some((num, row)) = self.find_last_digit_in_col(col) {
                return Some((num, row, col));
            }
        }
        None
    }

    /// Check if there are last digits in any of the rows.
    /// If so, remove it from the notes in the row, column, and box where we've found it.
    /// Set the respective cell to the digit.
    fn resolve_last_digit_in_rows(&mut self) -> usize {
        let mut count = 0;
        if let Some((num, row, col)) = self.find_last_digit_in_rows() {
            count += self.set_num(num, row, col);
        }
        count
    }

    fn resolve_last_digit_in_cols(&mut self) -> usize {
        let mut count = 0;
        if let Some((num, row, col)) = self.find_last_digit_in_cols() {
            count += self.set_num(num, row, col);
        }
        count
    }

    fn resolve_last_digit_in_boxes(&mut self) -> usize {
        let mut count = 0;
        if let Some((num, row, col)) = self.find_last_digit_in_boxes() {
            count += self.set_num(num, row, col);
        }
        count
    }

    fn resolve_obvious_single(&mut self) -> usize {
        for row in 0..9 {
            for col in 0..9 {
                if self.notes[row][col].len() == 1 {
                    assert_eq!(self.board[row][col], EMPTY);
                    let &num = self.notes[row][col].iter().next().unwrap();
                    return self.set_num(num, row, col);
                }
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
                if self.board[row][col] == 0 {
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

                        if self.board[row][col] == 0 {
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
        }
        0
    }

    fn resolve_pointing_pair_in_rows(&mut self) -> usize {
        let mut count = 0;
        for row in 0..9 {
            for num in 1..=9 {
                // Track cells with candidate 'num' in this row
                let mut cells_with_num = Vec::new();
                for col in 0..9 {
                    if self.notes[row][col].contains(&num) {
                        cells_with_num.push(col);
                    }
                }

                // Check if exactly 2 cells with this candidate are in the same box
                if cells_with_num.len() != 2 {
                    continue;
                }
                let col1 = cells_with_num[0];
                let col2 = cells_with_num[1];

                // Check if they're in the same box
                if col1 / 3 == col2 / 3 {
                    let box_col = col1 / 3;
                    let start_row = 3 * (row / 3);
                    println!("Found pointing pair {:?} in row {} at columns {} and {}", num, row, col1, col2);
                    // Remove this candidate from other cells in the same box but different row
                    for r in start_row..start_row + 3 {
                        if r != row {
                            // Skip the original row
                            for c in (box_col * 3)..(box_col * 3 + 3) {
                                if self.notes[r][c].remove(&num) {
                                    count += 1;
                                }
                            }
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
                    if self.notes[row][col].contains(&num) {
                        cells_with_num.push(row);
                    }
                }

                // Check if exactly two cells with this candidate are in the same box
                if cells_with_num.len() == 2 {
                    let row1 = cells_with_num[0];
                    let row2 = cells_with_num[1];

                    // Check if they're in the same box
                    if row1 / 3 == row2 / 3 {
                        let box_idx = row1 / 3;
                        let start_col = 3 * (col / 3);
                        println!("Found pointing pair {:?} in column {} at rows {} and {}", num, col, row1, row2);
                        // Remove this candidate from other cells in the same box but different column
                        for c in start_col..start_col + 3 {
                            if c != col {
                                // Skip the original column
                                for r in (box_idx * 3)..(box_idx * 3 + 3) {
                                    if self.notes[r][c].remove(&num) {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        count
    }

    fn resolve_pointing_pair(&mut self) -> usize {
        self.dump_notes();
        let mut count = 0;
        count += self.resolve_pointing_pair_in_rows();
        count += self.resolve_pointing_pair_in_cols();
        self.dump_notes();
        count
    }

    fn resolve_obvious_pair(&mut self) -> usize {
        let mut count = 0;

        // Check for obvious pairs in rows
        for row in 0..9 {
            for col in 0..9 {
                if self.board[row][col] != 0 || self.notes[row][col].len() != 2 {
                    continue;
                }

                let pair = self.notes[row][col].clone();

                // Find pair in same row
                for i in (col + 1)..9 {
                    if self.notes[row][i] != pair {
                        continue;
                    }
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

        // Check for obvious pairs in columns
        for col in 0..9 {
            for row in 0..9 {
                if self.board[row][col] != 0 || self.notes[row][col].len() != 2 {
                    continue;
                }

                let pair = self.notes[row][col].clone();

                // Find pair in same column
                for i in (row + 1)..9 {
                    if self.notes[i][col] != pair {
                        continue;
                    }
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
        count
    }

    fn resolve_hidden_pair(&mut self) -> usize {
        let mut count = 0;

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
                    let mut local_count = 0;
                    // Remove all other digits from these two cells
                    for &(row, col) in &[cell1, cell2] {
                        for digit in 1..=9 {
                            if digit != digit1
                                && digit != digit2
                                && self.notes[row][col].remove(&digit)
                            {
                                local_count += 1;
                            }
                        }
                    }

                    if local_count > 0 {
                        println!(
                            "Found hidden pair ({},{}) in box at cells ({},{}) and ({},{})",
                            digit1, digit2, cell1.0, cell1.1, cell2.0, cell2.1
                        );
                        count += local_count;
                        return count;
                    }
                }
            }
        }

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
                let mut local_count = 0;
                // Remove all other digits from these two cells
                for &col in &[col1, col2] {
                    for digit in 1..=9 {
                        if digit != digit1 && digit != digit2 && self.notes[row][col].remove(&digit)
                        {
                            local_count += 1;
                        }
                    }
                }

                if local_count > 0 {
                    println!(
                        "Found hidden pair ({},{}) in row {} at columns {} and {}",
                        digit1, digit2, row, col1, col2
                    );
                    count += local_count;
                    return count;
                }
            }
        }

        // Check for hidden pairs in columns
        for col in 0..9 {
            // Find which digits appear in exactly two cells in the column
            let mut digit_locations: HashMap<u8, Vec<usize>> = HashMap::new();
            for row in 0..9 {
                if self.board[row][col] != 0 {
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
                let mut local_count = 0;
                // Remove all other digits from these two cells
                for &row in &[row1, row2] {
                    for digit in 1..=9 {
                        if digit != digit1 && digit != digit2 && self.notes[row][col].remove(&digit)
                        {
                            local_count += 1;
                        }
                    }
                }

                if local_count > 0 {
                    println!(
                        "Found hidden pair ({},{}) in column {} at rows {} and {}",
                        digit1, digit2, col, row1, row2
                    );
                    count += local_count;
                    return count;
                }
            }
        }

        count
    }

    fn remove_notes_in_row(&mut self, nums: &[u8], row: usize) -> usize {
        let mut count = 0;
        for col in 0..9 {
            for num in nums {
                if self.notes[row][col].remove(num) {
                    count += 1;
                }
            }
        }
        count
    }

    fn remove_notes_in_col(&mut self, nums: &[u8], col: usize) -> usize {
        let mut count = 0;
        for row in 0..9 {
            for num in nums {
                if self.notes[row][col].remove(num) {
                    count += 1;
                }
            }
        }
        count
    }

    fn remove_notes_in_box(&mut self, nums: &[u8], row: usize, col: usize) -> usize {
        let mut count = 0;
        let start_row = 3 * (row / 3);
        let start_col = 3 * (col / 3);
        for i in 0..3 {
            for j in 0..3 {
                for num in nums {
                    if self.notes[start_row + i][start_col + j].remove(&num) {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn remove_notes(&mut self, nums: &[u8], row: usize, col: usize) -> usize {
        let mut count = 0;
        count += self.remove_notes_in_row(nums, row);
        count += self.remove_notes_in_col(nums, col);
        count += self.remove_notes_in_box(nums, row, col);
        println!("Candidates removed: {}", count);
        count
    }

    fn set_num(&mut self, num: u8, row: usize, col: usize) -> usize {
        println!("setting num {} in row {}, col {}", num, row, col);
        self.board[row][col] = num;
        self.nums_in_row[row].insert(num);
        self.nums_in_col[col].insert(num);
        self.nums_in_box[3 * (row / 3) + col / 3].insert(num);
        let mut count = self.notes[row][col].len();
        self.notes[row][col].clear();
        count += self.remove_notes(&[num], row, col);
        count
    }

    /// Solve the Sudoku puzzle using human-like strategies
    fn solve_like_a_human(&mut self) {
        // the first step always is to calculate the notes
        self.calc_all_notes();
        // since we're starting from scratch, we clear the rating
        self.rating.clear();
        let num_strategies = 6;
        let mut no_progress_counter = 0;
        while self.unsolved() {
            if no_progress_counter >= num_strategies {
                println!(
                    "No progress made after trying all {} strategies. Breaking loop.",
                    num_strategies
                );
                break;
            }
            // self.print();
            // self.dump_notes();

            print!("Applying strategy 'last digit in box' ... ");
            let num_removed = self.resolve_last_digit_in_boxes();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::LastDigitInBox)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
            } else {
                no_progress_counter += 1;
                println!();
            }
            print!("Applying strategy 'last digit in column' ... ");
            let num_removed = self.resolve_last_digit_in_cols();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::LastDigitInCol)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }
            print!("Applying strategy 'last digit in row' ... ");
            let num_removed = self.resolve_last_digit_in_rows();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::LastDigitInRow)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }
            print!(
                "Applying strategy '{}' ... ",
                SolvingStrategy::ObviousSingle.to_string()
            );
            let num_removed = self.resolve_obvious_single();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::ObviousSingle)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }
            print!(
                "Applying strategy '{}' ... ",
                SolvingStrategy::HiddenSingle.to_string()
            );
            let num_removed = self.resolve_hidden_single();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::HiddenSingle)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }

            print!(
                "Applying strategy '{}' ... ",
                SolvingStrategy::PointingPair.to_string()
            );
            let num_removed = self.resolve_pointing_pair();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::PointingPair)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }

            print!(
                "Applying strategy '{}' ... ",
                SolvingStrategy::ObviousPair.to_string()
            );
            let num_removed = self.resolve_obvious_pair();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::ObviousPair)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                self.dump_notes();
                continue;
            } else {
                no_progress_counter += 1;
                println!();
            }

            self.dump_notes();
            print!(
                "Applying strategy '{}' ... ",
                SolvingStrategy::HiddenPair.to_string()
            );
            let num_removed = self.resolve_hidden_pair();
            if num_removed > 0 {
                self.rating
                    .entry(SolvingStrategy::HiddenPair)
                    .and_modify(|count| *count += num_removed)
                    .or_insert(num_removed);
                no_progress_counter = 0; // Reset counter when progress is made
                self.dump_notes();
                continue;
            } else {
                no_progress_counter += 1;
                println!();
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
            println!("**** SUDOKU NOT SOLVED ****");
            self.dump_notes();
        } else {
            println!("**** SUDOKU SOLVED ****");
        }
        self.dump_rating();

        let start = std::time::Instant::now();
        sudoku.solve_by_backtracking();
        let duration = start.elapsed();
        println!(
            "For comparison: time to solve with backtracker: {} µs",
            duration.as_micros()
        );
    }

    pub fn from_string(&mut self, input_string: &str) {
        let mut i = 0;
        let mut j = 0;
        for c in input_string.chars() {
            if c.is_digit(10) {
                self.board[i][j] = c.to_digit(10).unwrap() as u8;
                j += 1;
                if j == 9 {
                    j = 0;
                    i += 1;
                }
            }
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
        let mut sudoku = Sudoku::new(&board_string);
        sudoku.solve_by_backtracking();
        assert_eq!(
            sudoku.serialized(),
            "865431297479258316231697548513824769947563182628719453186375924754982631392146875"
        );
    }

    #[test]
    fn test_last_digit_in_rows_strategy() {
        let board_string =
            "831547269729030405564290073642175398973064521158923746297310054415080632386452007"
                .to_string();
        let mut sudoku = Sudoku::new(&board_string);

        // Calculate notes before checking for last digit in row
        sudoku.calc_all_notes();

        // The expected digit 8 should be found in row 4, col 3
        if let Some((num, row, col)) = sudoku.find_last_digit_in_rows() {
            assert_eq!(num, 8);
            assert_eq!(row, 4);
            assert_eq!(col, 3);
        } else {
            panic!("Expected to find last digit 8 in row 4, col 3, but found none");
        }

        if let Some((num, row, col)) = sudoku.find_last_digit_in_boxes() {
            assert_eq!(num, 8);
            assert_eq!(row, 4);
            assert_eq!(col, 3);
        } else {
            panic!("Expected to find last digit 8 in row 4, col 3, but found none");
        }

        let result = sudoku.find_last_digit_in_cols();
        assert_eq!(result, None);

        // Verify that applying the strategy actually sets the digit
        let count = sudoku.resolve_last_digit_in_rows();
        assert_eq!(count, 2);
        assert_eq!(sudoku.board[4][3], 8);
    }

    #[test]
    fn test_last_digit_in_cols_strategy() {
        let board_string =
            "831547269729030405564290073642175398973064521158923746297310054415080632386452007"
                .to_string();
        let mut sudoku = Sudoku::new(&board_string);
        sudoku.calc_all_notes();
        let result = sudoku.find_last_digit_in_cols();
        assert_eq!(result, None);
    }

    #[test]
    fn test_last_digit_in_boxes_strategy() {
        let board_string =
            "831547269729030405564290073642175398973064521158923746297310054415080632386452007"
                .to_string();
        let mut sudoku = Sudoku::new(&board_string);
        sudoku.calc_all_notes();
        if let Some((num, row, col)) = sudoku.find_last_digit_in_boxes() {
            assert_eq!(num, 8);
            assert_eq!(row, 4);
            assert_eq!(col, 3);
        } else {
            panic!("Expected to find last digit 8 in row 4, col 3, but found none");
        }
        let count = sudoku.resolve_last_digit_in_boxes();
        assert_eq!(count, 2);
        assert_eq!(sudoku.board[4][3], 8);
    }
}
