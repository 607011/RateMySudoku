# Sudoku Rater

**Rate difficulty of Sudoku by applying strategies as a human would in order to solve it**

![Build Status](https://github.com/607011/sudoku-rater/actions/workflows/main.yml/badge.svg)

## Strategies

The solver attempts to solve a given Sudoku iteratively, applying strategies from easiest to most difficult.

If a strategy is successful, the affected candidates are removed or the cells are filled with the resulting digits. The number of affected candidates/cells is added to a running total, as is the difficulty level of the respective strategy.

After solving the Sudoku, the total difficulty is divided by the sum of the affected candidates and cells, which provides a good estimate of the perceived difficulty (effort). The higher the value, the more difficult the Sudoku typically is for a human.

The following strategies are currently implemented:

| Strategy               | Effort |
| ---------------------- | ------:|
| Last Digit             |      4 |
| Obvious Single         |      5 |
| Hidden Single          |     14 |
| Pointing Pair          |     50 |
| Claiming Pair          |     50 |
| Obvious Pair           |     60 |
| Hidden Pair            |     70 |
| X-Wing                 |    140 |

More to come …
