mod tests {
    use rate_my_sudoku::Sudoku;

    #[test]
    fn test_skyscraper_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "001028759087905132952173486020700340000500270714832695000090817078051963190087524",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_skyscraper_in_rows();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
    }

    #[test]
    fn test_skyscraper_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "697000002001972063003006790912000607374260950865709024148693275709024006006807009",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_skyscraper_in_cols();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
    }
}
