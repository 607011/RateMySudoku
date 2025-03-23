#[cfg(test)]
mod tests {
    use rate_my_sudoku::Sudoku;

    #[test]
    fn test_locked_pair_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...3......7..8.5.1..9...83.3..7.4...7.......4...1.5..8.61...9..9.7.4..6......2...",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_locked_pair_cols();
        println!("{:?}", result);
    }

    #[test]
    fn test_locked_pair_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...3......7..8.5.1..9...83.3..7.4...7.......4...1.5..8.61...9..9.7.4..6......2...",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_locked_pair_cols();
        println!("{:?}", result);
    }

    #[test]
    fn test_locked_pair_box() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...3......7..8.5.1..9...83.3..7.4...7.......4...1.5..8.61...9..9.7.4..6......2...",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_locked_pair_cols();
        println!("{:?}", result);
    }
}
