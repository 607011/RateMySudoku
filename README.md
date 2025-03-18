# Sudoku Rater

**Rate difficulty of Sudoku by applying strategies as a human would to solve it**

## Strategies

The solver attempts to solve a given Sudoku iteratively, applying strategies from easiest to most difficult.

If a strategy is successful, the affected candidates are removed or the cells are filled with the resulting digits. The number of affected candidates or cells is added to a running total, as is the difficulty level of the respective strategy.

After solving the Sudoku, the total difficulty is divided by the sum of the affected candidates and cells, which provides a good estimate of the perceived difficulty. The higher the value, the more difficult the Sudoku is for a human.

The following strategies are currently implemented:

| Strategy               | Difficulty Score |
| ---------------------- | ---------------- |
| Last Digit             | 1                |
| Obvious Single         | 2                |
| Hidden Single          | 3                |
| Pointing Pair          | 4                |
| Obvious Pair           | 5                |
| Hidden Pair            | 6                |

More to come …
