mod tests {
    use rate_my_sudoku::{Candidate, Cell, Strategy, Sudoku, Unit};

    #[test]
    fn test_last_digit_row() {
        let mut sudoku = Sudoku::from_string(
            "006004700090000154408000000100090000070006001000040000000072635350461879007830412",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_last_digit_in_rows();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![7]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 7,
                col: 2,
                num: 2
            })
        );
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 5);
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 2,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 3,
            col: 2,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 2,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 2,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 2,
            num: 2
        }));
    }

    #[test]
    fn test_last_digit_col() {
        let mut sudoku = Sudoku::from_string(
            "506004700793000154408000000100090000270006001000040000800972635352461879967835412",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_last_digit_in_cols();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![0]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 5,
                col: 0,
                num: 6
            })
        );
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 3);
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 0,
            num: 6
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 7,
            num: 6
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 8,
            num: 6
        }));
    }

    #[test]
    fn test_last_digit_box() {
        let mut sudoku = Sudoku::from_string(
            "006004700090000154408000000100090000070006001000040000000002605350000879007830412",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_last_digit_in_boxes();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Box));
        assert_eq!(result.unit_index, Some(vec![8]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 6,
                col: 7,
                num: 3
            })
        );
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 7,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 7,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 3,
            col: 7,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 7,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 7,
            num: 3
        }));
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 7,
            num: 3
        }));
    }

    #[test]
    fn test_obvious_single() {
        let mut sudoku = Sudoku::from_string(
            "..6..47...9....1544.8......1...9.....7...6..1....4.........26.535......9..783....",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_all_notes();
        let result = sudoku.find_obvious_single();
        println!("{:?}", result);
        assert_eq!(result.strategy, Strategy::ObviousSingle);
        assert_eq!(result.removals.unit, None);
        assert_eq!(result.removals.unit_index, None);
        assert_eq!(
            result.removals.sets_cell,
            Some(Cell {
                row: 8,
                col: 8,
                num: 2
            })
        );
        let affected = result.removals.candidates_affected;
        assert_eq!(affected.len(), 1);
        assert!(affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 2
        }));
        let removals = result.removals.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 11);
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 7,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 1,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 6,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 0,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 6,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 7,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 3,
            col: 8,
            num: 2
        }));
    }
}
