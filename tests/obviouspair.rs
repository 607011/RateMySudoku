mod tests {
    use rate_my_sudoku::{Candidate, Sudoku, Unit};

    #[test]
    fn test_obvious_pair_row() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "700849030928135006400267089642783951397451628815692300204516093100008060500004010",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_obvious_pair_in_rows();
        println!("{:?}", result);
        assert_eq!(result.candidates_about_to_be_removed.len(), 1);
        assert_eq!(result.unit, Some(Unit::Row));
        assert_eq!(result.unit_index, Some(vec![7]));
        assert!(result.sets_cell.is_none());
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 1,
            num: 3
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 4);
        assert!(candidates_affected.contains(&Candidate {
            row: 7,
            col: 2,
            num: 3
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 7,
            col: 2,
            num: 9
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 7,
            col: 3,
            num: 3
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 7,
            col: 3,
            num: 9
        }));
    }

    #[test]
    fn test_obvious_pair_col() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "000020000020970301070003400600050008001000035000000002000091060000280903068300000",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_obvious_pair_in_cols();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
        assert_eq!(result.unit, Some(Unit::Column));
        assert_eq!(result.unit_index, Some(vec![8]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 8,
            num: 7
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 4);
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 8,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 8,
            num: 7
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 7
        }));
    }

    #[test]
    fn test_obvious_pair_box() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "000020000020970301070003400600050008001000035000000002000091060000280903068300000",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        let result = sudoku.find_obvious_pair_in_boxes();
        println!("{:?}", result);
        assert!(result.sets_cell.is_none());
        assert_eq!(result.unit, Some(Unit::Box));
        assert_eq!(result.unit_index, Some(vec![8]));
        let removals = result.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
        assert!(removals.contains(&Candidate {
            row: 6,
            col: 6,
            num: 7
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 6,
            num: 7
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 7,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 7,
            col: 7,
            num: 7
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 7,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 8,
            col: 7,
            num: 7
        }));
        let candidates_affected = result.candidates_affected;
        assert_eq!(candidates_affected.len(), 4);
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 8,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 6,
            col: 8,
            num: 7
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 8,
            col: 8,
            num: 7
        }));
    }
}
