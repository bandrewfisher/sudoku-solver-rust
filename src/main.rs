const SIZE: usize = 9;
type Puzzle = [[u8; SIZE]; SIZE];

fn get_puzzle() -> Puzzle {
    let puzzle_str = "003020600
900305001
001806400
008102900
700000008
006708200
002609500
800203009
005010300";
    let mut board = [[0u8; SIZE]; SIZE];
    for (i, line) in puzzle_str.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            board[i][j] = char.to_digit(10).unwrap() as u8;
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

fn is_valid(puzzle: &Puzzle, row: u8, col: u8, possible_num: u8) -> bool {
    // Check that it's valid in the row

    // Check that it's valid for the col

    // Check that it's valid for the square
    true
}

fn solve_puzzle(puzzle: &mut Puzzle) {
    for (i, row) in puzzle.iter().enumerate() {
        for (j, num) in row.iter().enumerate() {
            for possible_num in 1..=9 {

            }
        }
    }
}

fn main() {
    let puzzle = get_puzzle();
    print_puzzle(&puzzle);
}
