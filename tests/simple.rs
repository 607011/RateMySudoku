mod tests {
    use rate_my_sudoku::{Candidate, Cell, RemovalResult, Strategy, Sudoku, Unit};

    #[test]
    fn test_last_digit_row() {
        let mut sudoku = Sudoku::from_string(
            "006004700090000154408000000100090000070006001000040000000072635350461879007830412",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
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
        sudoku.calc_candidates();
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
        sudoku.calc_candidates();
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
        sudoku.calc_candidates();
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

    #[test]
    fn test_hidden_single_box() {
        let mut sudoku = Sudoku::from_string(
            "..6..47...93...1544.8......1...9.....7...6..1....4....8..972635352461879967835412",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result: RemovalResult = sudoku.find_hidden_single_box();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Box));
        assert_eq!(result.unit_index, Some(vec![0]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 0,
                col: 0,
                num: 5
            })
        );
        let affected = result.candidates_affected;
        assert!(affected.contains(&Candidate {
            row: 0,
            col: 0,
            num: 5
        }));
        let removals = result.candidates_about_to_be_removed;
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 0,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 4,
            col: 0,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 0,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 0,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 4,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 3,
            num: 5
        }));
    }

    #[test]
    fn test_hidden_single_row() {
        let mut sudoku = Sudoku::from_string(
            "6..782....5...137..2............41.9..2.9.7..4.61............3..654...2....827..4",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result: RemovalResult = sudoku.find_hidden_single_row();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![1]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 1,
                col: 8,
                num: 2
            })
        );
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 1);
        assert!(affected.contains(&Candidate {
            row: 1,
            col: 8,
            num: 2
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 4);
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 5,
            col: 8,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 6
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 8,
            num: 8
        }));
    }

    #[test]
    fn test_hidden_single_col() {
        let mut sudoku = Sudoku::from_string(
            "720050693580000742090700000002007008000381200800900400000006020245000000608070004",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result: RemovalResult = sudoku.find_hidden_single_col();
        println!("{:?}", result);
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![3]));
        assert_eq!(
            result.sets_cell,
            Some(Cell {
                row: 8,
                col: 3,
                num: 2
            })
        );
        let affected = result.candidates_affected;
        assert_eq!(affected.len(), 1);
        assert!(affected.contains(&Candidate {
            row: 8,
            col: 3,
            num: 2
        }));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 4);
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 3,
            num: 2
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 3,
            num: 5
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 5,
            num: 2
        }));
    }
}
