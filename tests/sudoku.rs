mod tests {
    use rate_my_sudoku::Sudoku;
    use std::collections::HashSet;

    #[test]
    fn test_sudoku_serialize() {
        let sudoku: Sudoku = Sudoku::from_string(
            "318005406000603810006080503864952137123476958795318264030500780000007305000039641",
        )
        .expect("Failed to create Sudoku from string");
        assert_eq!(
            sudoku.to_board_string(),
            "318005406000603810006080503864952137123476958795318264030500780000007305000039641"
        );
    }

    #[test]
    fn test_sudoku_deserialize() {
        let sudoku: Sudoku = Sudoku::from_json(&"{\"board\":[[3,1,8,0,0,5,4,0,6],[0,0,0,6,0,3,8,1,0],[0,0,6,0,8,0,5,0,3],[8,6,4,9,5,2,1,3,7],[1,2,3,4,7,6,9,5,8],[7,9,5,3,1,8,2,6,4],[0,3,0,5,0,0,7,8,0],[0,0,0,0,0,7,3,0,5],[0,0,0,0,3,9,6,4,1]],\"candidates\":[[[],[],[],[2,7],[2,9],[],[],[7,2,9],[]],[[5,9,2,4],[7,5,4],[7,9,2],[],[2,4,9],[],[],[],[9,2]],[[9,2,4],[7,4],[],[1,7,2],[],[1,4],[],[7,9,2],[]],[[],[],[],[],[],[],[],[],[]],[[],[],[],[],[],[],[],[],[]],[[],[],[],[],[],[],[],[],[]],[[2,9,4,6],[],[1,2,9],[],[2,6,4],[1,4],[],[],[2,9]],[[6,2,4,9],[4,8],[2,9,1],[2,8,1],[6,2,4],[],[],[2,9],[]],[[2,5],[7,8,5],[2,7],[8,2],[],[],[],[],[]]]}".to_string()).expect("Failed to create Sudoku from JSON string");
        assert_eq!(
            sudoku.to_board_string(),
            "318005406000603810006080503864952137123476958795318264030500780000007305000039641"
        );
        assert_eq!(
            sudoku.board,
            [
                [3, 1, 8, 0, 0, 5, 4, 0, 6],
                [0, 0, 0, 6, 0, 3, 8, 1, 0],
                [0, 0, 6, 0, 8, 0, 5, 0, 3],
                [8, 6, 4, 9, 5, 2, 1, 3, 7],
                [1, 2, 3, 4, 7, 6, 9, 5, 8],
                [7, 9, 5, 3, 1, 8, 2, 6, 4],
                [0, 3, 0, 5, 0, 0, 7, 8, 0],
                [0, 0, 0, 0, 0, 7, 3, 0, 5],
                [0, 0, 0, 0, 3, 9, 6, 4, 1]
            ]
        );
        assert_eq!(
            sudoku.candidates,
            [
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([2, 7]),
                    HashSet::from([2, 9]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([2, 9, 7]),
                    HashSet::new()
                ],
                [
                    HashSet::from([5, 9, 2, 4]),
                    HashSet::from([4, 7, 5]),
                    HashSet::from([7, 2, 9]),
                    HashSet::new(),
                    HashSet::from([2, 4, 9]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2])
                ],
                [
                    HashSet::from([9, 4, 2]),
                    HashSet::from([4, 7]),
                    HashSet::new(),
                    HashSet::from([1, 7, 2]),
                    HashSet::new(),
                    HashSet::from([1, 4]),
                    HashSet::new(),
                    HashSet::from([7, 9, 2]),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::from([2, 9, 4, 6]),
                    HashSet::new(),
                    HashSet::from([9, 2, 1]),
                    HashSet::new(),
                    HashSet::from([6, 2, 4]),
                    HashSet::from([1, 4]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2])
                ],
                [
                    HashSet::from([6, 2, 9, 4]),
                    HashSet::from([8, 4]),
                    HashSet::from([1, 9, 2]),
                    HashSet::from([2, 8, 1]),
                    HashSet::from([2, 4, 6]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2]),
                    HashSet::new()
                ],
                [
                    HashSet::from([2, 5]),
                    HashSet::from([7, 5, 8]),
                    HashSet::from([7, 2]),
                    HashSet::from([8, 2]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ]
            ]
        );
    }

    #[test]
    fn test_sudoku_calc_candidates() {
        let mut sudoku: Sudoku = Sudoku::from_string(
            "318005406000603810006080503864952137123476958795318264030500780000007305000039641",
        )
        .expect("Failed to create Sudoku from string");
        sudoku.calc_candidates();
        assert_eq!(
            sudoku.candidates,
            [
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([2, 7]),
                    HashSet::from([2, 9]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([2, 9, 7]),
                    HashSet::new()
                ],
                [
                    HashSet::from([5, 9, 2, 4]),
                    HashSet::from([4, 7, 5]),
                    HashSet::from([7, 2, 9]),
                    HashSet::new(),
                    HashSet::from([2, 4, 9]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2])
                ],
                [
                    HashSet::from([9, 4, 2]),
                    HashSet::from([4, 7]),
                    HashSet::new(),
                    HashSet::from([1, 7, 2]),
                    HashSet::new(),
                    HashSet::from([1, 4]),
                    HashSet::new(),
                    HashSet::from([7, 9, 2]),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ],
                [
                    HashSet::from([2, 9, 4, 6]),
                    HashSet::new(),
                    HashSet::from([9, 2, 1]),
                    HashSet::new(),
                    HashSet::from([6, 2, 4]),
                    HashSet::from([1, 4]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2])
                ],
                [
                    HashSet::from([6, 2, 9, 4]),
                    HashSet::from([8, 4]),
                    HashSet::from([1, 9, 2]),
                    HashSet::from([2, 8, 1]),
                    HashSet::from([2, 4, 6]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::from([9, 2]),
                    HashSet::new()
                ],
                [
                    HashSet::from([2, 5]),
                    HashSet::from([7, 5, 8]),
                    HashSet::from([7, 2]),
                    HashSet::from([8, 2]),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new(),
                    HashSet::new()
                ]
            ]
        );
    }
}
