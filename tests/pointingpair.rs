mod tests {
    use rate_my_sudoku::{Candidate, Sudoku, Unit};

    #[test]
    fn test_pointing_pair_row1() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "984000000002500040001904002006097230003602000209035610195768423427351896638009751",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_pointing_pair_in_rows();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![2]));
        assert!(result.sets_cell.is_none());
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 2);
        assert!(candidates_affected.contains(&Candidate {
            row: 2,
            col: 0,
            num: 5
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 2,
            col: 1,
            num: 5
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 6,
            num: 5
        }));
    }

    #[test]
    fn test_pointing_pair_row2() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "340006070080000930002030060000010000097364850000002000000000000000608090000923785",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_pointing_pair_in_rows();
        println!("{:?}", result);
        assert_eq!(result.unit_index, Some(vec![6]));
        assert_eq!(result.unit, Some(Unit::Row));
        assert!(result.sets_cell.is_none());
        let removals = result.candidates_about_to_be_removed;
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 0,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 1,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 2,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 6,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 7,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 8,
            num: 1
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 2);
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 3,
            num: 1
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 5,
            num: 1
        }));
    }

    #[test]
    fn test_pointing_pair_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...2..67992..6.1..476891253..7..95.6.......12..51..9.77.261..9.3...82761....7..2.",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_pointing_pair_in_cols();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![6]));
        assert!(result.sets_cell.is_none());
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 2);
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 6,
            num: 3
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 6,
            num: 3
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 6,
            num: 3
        }));
    }
}
