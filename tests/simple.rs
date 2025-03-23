mod tests {
    use rate_my_sudoku::{Candidate, Cell, Strategy, Sudoku};

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
