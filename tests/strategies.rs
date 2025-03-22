#[cfg(test)]
mod tests {
    use rate_my_sudoku::{Candidate, Strategy, Sudoku, Unit};

    #[test]
    fn test_claiming_pair1() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "318005406000603810006080503864952137123476958795318264030500780000007305000039641",
        );
        sudoku.calc_all_notes();
        let result = sudoku.find_claiming_pair();
        assert_eq!(result.strategy, Strategy::ClaimingPair);
        assert_eq!(result.removals.candidates_about_to_be_removed.len(), 1);
        assert_eq!(result.removals.unit, Some(Unit::Row));
        assert_eq!(result.removals.unit_index, Some(vec![1]));
        assert!(result.removals.sets_cell.is_none());
        let removals = result.removals.candidates_about_to_be_removed;
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 1,
            num: 7
        }));
        assert_eq!(result.removals.candidates_affected.len(), 2);
        let candidates_affected = result.removals.candidates_affected;
        assert_eq!(candidates_affected.len(), 2);
        assert!(candidates_affected.contains(&Candidate {
            row: 1,
            col: 1,
            num: 7
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 1,
            col: 2,
            num: 7
        }));
    }

    #[test]
    fn test_claiming_pair2() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "762008001980000006150000087478003169526009873319800425835001692297685314641932758",
        );
        sudoku.calc_all_notes();
        let result = sudoku.find_claiming_pair();
        assert_eq!(result.strategy, Strategy::ClaimingPair);
        assert_eq!(result.removals.candidates_about_to_be_removed.len(), 6);
        assert_eq!(result.removals.unit, Some(Unit::Column));
        assert_eq!(result.removals.unit_index, Some(vec![5]));
        assert!(result.removals.sets_cell.is_none());
        let removals = result.removals.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 4,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 4,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 4,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 0,
            col: 3,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 1,
            col: 3,
            num: 4
        }));
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 3,
            num: 4
        }));
        let candidates_affected = result.removals.candidates_affected;
        assert_eq!(candidates_affected.len(), 2);
        assert!(candidates_affected.contains(&Candidate {
            row: 1,
            col: 5,
            num: 4
        }));
        assert!(candidates_affected.contains(&Candidate {
            row: 2,
            col: 5,
            num: 4
        }));
    }

    #[test]
    fn test_pointing_pair1() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "984000000002500040001904002006097230003602000209035610195768423427351896638009751",
        );
        sudoku.calc_all_notes();
        let result = sudoku.find_pointing_pair();
        println!("{:?}", result);
        assert_eq!(result.strategy, Strategy::PointingPair);
        assert_eq!(result.removals.unit, Some(Unit::Row));
        assert_eq!(result.removals.unit_index, Some(vec![2]));
        assert!(result.removals.sets_cell.is_none());
        let removals = result.removals.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 1);
        assert!(removals.contains(&Candidate {
            row: 2,
            col: 6,
            num: 5
        }));
        let candidates_affected = result.removals.candidates_affected;
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
    }

    #[test]
    fn test_pointing_pair2() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "340006070080000930002030060000010000097364850000002000000000000000608090000923785",
        );
        sudoku.calc_all_notes();
        let result = sudoku.find_pointing_pair();
        println!("{:?}", result);
        assert_eq!(result.strategy, Strategy::PointingPair);
        assert_eq!(result.removals.unit, Some(Unit::Row));
        assert_eq!(result.removals.unit_index, Some(vec![6]));
        let removals = result.removals.candidates_about_to_be_removed;
        assert_eq!(removals.len(), 6);
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
        let candidates_affected = result.removals.candidates_affected;
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
}
