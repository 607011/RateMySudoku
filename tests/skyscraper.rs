mod tests {
    use rate_my_sudoku::Sudoku;

    #[test]
    fn test_skyscraper_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "1678925349524.386.8345..9..7.3.48.....835......5.2.3.8..1..5.89.869.14..579284613",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_skyscraper_rows();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
    }

    #[test]
    fn test_skyscraper_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "1678925349524.386.8345..9..7.3.48.....835......5.2.3.8..1..5.89.869.14..579284613",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_skyscraper_cols();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
    }
}
