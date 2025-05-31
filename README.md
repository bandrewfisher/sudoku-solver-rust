# Rust Sudoku Solver

Reads a sudoku puzzle from stdin and uses a backtracking algorithm to 
find a solution.

Example puzzles are provided in the `puzzles` directory for convenience,
where each file contains a single puzzle that is parsed out from 
`all_puzzles.txt`. 

Credit goes to https://projecteuler.net/project/resources/p096_sudoku.txt
for the file containing all of the parsed puzzles.

## Puzzle format

The program expects to read 9 lines, each containing 9 characters (without spaces). Each
character must be a digit from 0-9. Blank spaces are represented by
a 0.

## Usage

`cat puzzles/puzzle_01.txt | cargo run`

## Example output
```
2 4 5 | 9 8 1 | 3 7 6
1 6 9 | 2 7 3 | 5 8 4
8 3 7 | 5 6 4 | 2 1 9
---------------------
9 7 6 | 1 2 5 | 4 3 8
5 1 3 | 4 9 8 | 6 2 7
4 8 2 | 7 3 6 | 9 5 1
---------------------
3 9 1 | 6 5 7 | 8 4 2
7 2 8 | 3 4 9 | 1 6 5
6 5 4 | 8 1 2 | 7 9 3
Solved in 0.000182 seconds
```
