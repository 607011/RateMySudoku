mod tests {
    use rate_my_sudoku::{Candidate, Strategy, Sudoku, Unit};

    #[test]
    fn test_naked_triplet() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "000294380000178640480356100004837501000415700500629834953782416126543978040961253",
        ).expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_naked_triplet();
        println!("{:?}", result);
        assert_eq!(result.strategy, Strategy::NakedTriplet);
        assert_eq!(result.removals.unit, Some(Unit::Column));
        assert_eq!(result.removals.unit_index, Some(vec![1]));
        let removals = result.removals.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 1,
            num: 6
        }));
        let candidates_affected = result.removals.candidates_affected;
        assert_eq!(candidates_affected.len(), 7);
        assert!(candidates_affected.contains(&Candidate {
            row: 1,
            col: 1,
            num: 9
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 1,
            col: 1,
            num: 3
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 3,
            col: 1,
            num: 9
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 3,
            col: 1,
            num: 6
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 1,
            num: 9
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 1,
            num: 6
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 4,
            col: 1,
            num: 3
        }));
    }

    #[test]
    fn test_naked_triplet_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...2..67992..6.1..476891253..7..95.6.......12..51..9.77.261..9.3...82761....7..2.",
        ).expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_naked_triplet_in_rows();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![1]));
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 6);
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 2,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 2,
            num: 8
        }));
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 7,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 7,
            num: 8
        }));
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 8,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 8,
            num: 8
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 4);
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 5,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 3,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 5,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 3,
            num: 4
        }));
    }

    #[test]
    fn test_naked_triplet_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...2..67992..6.1..476891253..7..95.6.......12..51..9.77.261..9.3...82761....7..2.",
        ).expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_naked_triplet_in_cols();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![5]));
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 9);
        assert!(affected.contains(&Candidate {
            row: 0,
            col: 5,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 0,
            col: 5,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 0,
            col: 5,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 6,
            col: 5,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 6,
            col: 5,
            num: 5
        }));
        assert!(affected.contains(&Candidate {
            row: 6,
            col: 5,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 8,
            col: 5,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 8,
            col: 5,
            num: 5
        }));
        assert!(affected.contains(&Candidate {
            row: 8,
            col: 5,
            num: 3
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 8);
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 5,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 5,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 5,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 5,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 5,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 5,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 5,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 5,
            num: 3
        }));
    }

    #[test]
    fn test_naked_triplet_box() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "...2..67992..6.1..476891253..7..95.6.......12..51..9.77.261..9.3...82761....7..2.",
        ).expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_naked_triplet_in_boxes();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Box));
        assert_eq!(result.unit_index, Some(vec![4]));
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 8);
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 3,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 3,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 4,
            num: 2
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 4,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 3,
            col: 4,
            num: 4
        }));
        assert!(affected.contains(&Candidate {
            row: 5,
            col: 4,
            num: 2
        }));
        assert!(affected.contains(&Candidate {
            row: 5,
            col: 4,
            num: 3
        }));
        assert!(affected.contains(&Candidate {
            row: 5,
            col: 4,
            num: 4
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 8);
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 3,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 5,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 5,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 3,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 5,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 4,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 4,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 5,
            num: 4
        }));

        assert_eq!(removals.len(), 8);
    }
}
