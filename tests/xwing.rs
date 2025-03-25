mod tests {
    use rate_my_sudoku::{Candidate, Sudoku, Unit};

    #[test]
    fn test_xwing_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "008475069090162008600938100205681900960357080803294605009516804000849000480723590",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_xwing_in_rows();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![4, 8]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 2);
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 8,
            num: 1
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 2,
            num: 1
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 4);
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 2,
            num: 1
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 8,
            num: 1
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 2,
            num: 1
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 1
        }));
    }

    #[test]
    fn test_xwing_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "68213....413..726.9572.613.378..1.2612986....54672.81.894372651231695.8.765418392",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_xwing_in_cols();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![5, 7]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 4);
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 6,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 8,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 8,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 6,
            num: 4
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 4);
        assert!(candidates_affected.contains(&Candidate {
            row: 0,
            col: 5,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 5,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 0,
            col: 7,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 7,
            num: 4
        }));
    }
}
