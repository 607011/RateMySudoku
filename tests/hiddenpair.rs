mod tests {
    use rate_my_sudoku::{Candidate, Sudoku, Unit};

    #[test]
    fn test_hidden_pair_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "690180300003006410100000600069018500318000264705060891000691700071800906906742180",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_hidden_pair_in_rows();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![3]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 3,
            col: 3,
            num: 3
        }));
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 4);
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 0,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 0,
            num: 2
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 3,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 3,
            num: 2
        }));
    }

    #[test]
    fn test_hidden_pair_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "690180300003006410100000600069018500318000264705060891000691700071800906906742180",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_hidden_pair_in_cols();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![8]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 7
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 7
        }));
    }

    #[test]
    fn test_hidden_pair_box() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "690180300003006410100000600069018500318000264705060891000691700071800906906742180",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_hidden_pair_in_boxes();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
        assert_eq!(result.unit, Some(Unit::Box));
        assert_eq!(result.unit_index, Some(vec![2]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 7
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 7
        }));
    }
}
