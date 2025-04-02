# Rate My Sudoku

**Rate difficulty of Sudoku by applying strategies as a human would in order to solve it**

![Build Status](https://github.com/607011/sudoku-rater/actions/workflows/main.yml/badge.svg)

## GUI written with egui/eframe

<img width="791" alt="Sudokui" src="https://github.com/user-attachments/assets/b32a10c4-d053-4900-babb-f93a4891a828" />

## Strategies

The solver attempts to solve a given Sudoku iteratively, applying strategies from easiest to most difficult.

If a strategy is successful, the affected candidates are removed or the cells are filled with the resulting digits. The number of affected candidates/cells is added to a running total, as is the difficulty level of the respective strategy.

After solving the Sudoku, the total difficulty is divided by the sum of the affected candidates and cells, which provides a good estimate of the perceived difficulty (effort). The higher the value, the more difficult the Sudoku typically is for a human. This software uses the same effort values as [HoDoKu](https://hodoku.sourceforge.net/).

The following strategies are currently (about to be) implemented:

| Code | Test | OK | Strategy               | Effort |
|:----:|:----:|:--:| ---------------------- | ------:|
| ☒    | ☒    | ☒  | Last Digit             |      4 |
| ☒    | ☒    | ☒  | Obvious Single         |      5 |
| ☒    | ☒    | ☒  | Hidden Single          |     14 |
| ☒    | ☒    | ☒  | Locked Pair            |     40 |
| ☒    | ☒    | ☒  | Pointing Pair          |     50 |
| ☒    | ☒    | ☒  | Claiming Pair          |     50 |
| ☒    | ☒    | ☒  | Obvious Pair           |     60 |
| ☒    | ☒    | ☒  | Hidden Pair            |     70 |
| ☒    | ☒    | ☒  | Naked Triplet          |     80 |
| ☒    | ☐    | ☐  | Skyscraper             |    130 |
| ☒    | ☒    | ☒  | X-Wing                 |    140 |

More to come …

## How to use

### Generator 

Generate Sudokus with 19 filled cells and append them to the file generated/19.txt

```
cargo run --bin gen --release -- -n 19 >> generated/19.txt
```

Get help on generator with:

```
cargo run --bin gen --release -- --help
```

### Visual Solver

```
cargo run --bin sudokui --release
```

You can paste Sudoku puzzles with Ctrl+V (Cmd+V on Mac) in single-line format like
`405030809000000007200004030100000006000050400000001003000600024070900000890000000` or
in matrix format like 

```
4 0 5 0 3 0 8 0 9
0 0 0 0 0 0 0 0 7
2 0 0 0 0 4 0 3 0
1 0 0 0 0 0 0 0 6
0 0 0 0 5 0 4 0 0
0 0 0 0 0 1 0 0 3
0 0 0 6 0 0 0 2 4
0 7 0 9 0 0 0 0 0
8 9 0 0 0 0 0 0 0
```

"File/Save as ..." allows you to save the current state (filled cells and candidates)
of the puzzle in multiple formats. With the suffix ".zst" a binary, zstd compressed
file is written. ".bin" represents such a binary without compression. If you need
a human-readable representation use the suffix ".json" which leads to a file in JSON
format. With "File/Load" you can load each of these files.

### Sudoku Rater

Rate the Sudoku

```
0 7 0 0 0 5 0 0 0
0 0 0 0 0 6 0 1 0
0 0 3 0 0 7 6 2 8
0 0 0 1 6 0 0 0 0
1 0 0 5 0 0 8 0 0
0 0 6 0 0 4 2 0 0
0 9 4 0 3 1 0 0 0
0 0 0 0 0 0 4 0 0
0 2 0 6 0 0 0 0 0
```

with

```
cargo run --bin rate --release -- 070005000000000010003007628000160000100500800006004200094031000000000400020600000
```
