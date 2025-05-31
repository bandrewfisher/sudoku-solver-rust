use std::io::{self, BufRead};
use std::time::Instant;

const SIZE: usize = 9;
type Puzzle = [[u8; SIZE]; SIZE];

fn get_puzzle() -> Puzzle {
    let stdin = io::stdin();
    let mut board = [[0u8; SIZE]; SIZE];

    for (i, line) in stdin.lock().lines().enumerate().take(SIZE) {
        let line = line.expect("Failed to read line");
        for (j, ch) in line.chars().enumerate().take(SIZE) {
            board[i][j] = ch.to_digit(10).expect("Invalid digit") as u8;
        }
    }

    board
}

fn print_puzzle(puzzle: &Puzzle) {
    for (i, row) in puzzle.iter().enumerate() {
        for (j, char) in row.iter().enumerate() {
            print!("{} ", char);
            if j == 2 || j == 5 {
                // Vertical separator
                print!("| ");
            }
        }
        println!();
        if i == 2 || i == 5 {
            // Horizontal separator
            let separator = "-".repeat(21);
            println!("{}", separator);
        }
    }
}

fn is_valid(puzzle: &Puzzle, row: usize, col: usize, possible_num: u8) -> bool {
    // Check that it's valid in the row
    if puzzle[row as usize].contains(&possible_num) {
        return false;
    }

    // Check that it's valid for the col
    for i in 0..9 {
        if puzzle[i][col as usize] == possible_num {
            return false;
        }
    }

    // Check that it's valid for the square
    let box_row_idx = row / 3 * 3;
    let box_col_idx = col / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if puzzle[box_row_idx + i][box_col_idx + j] == possible_num {
                return false;
            }
        }
    }
    true
}

fn solve_puzzle(puzzle: &mut Puzzle) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row][col] == 0 {
                for possible_num in 1..=9 {
                    if is_valid(puzzle, row, col, possible_num) {
                        puzzle[row][col] = possible_num;
                        if solve_puzzle(puzzle) {
                            return true;
                        }
                        puzzle[row][col] = 0; // Backtrack
                    }
                }
                return false; // no valid number, backtrack
            }
        }
    }
    true
}

fn main() {
    let mut puzzle = get_puzzle();

    let start = Instant::now();
    let solved = solve_puzzle(&mut puzzle);
    let duration = start.elapsed();

    if solved {
        print_puzzle(&puzzle);
    } else {
        println!("No solution found");
    }

    println!("Solved in {:.6} seconds", duration.as_secs_f64());

}
