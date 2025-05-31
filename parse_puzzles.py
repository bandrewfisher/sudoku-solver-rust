from pathlib import Path

def main():
    with open('all_puzzles.txt', 'r') as file:
        puzzles = []
        cur_puzzle = ''
        for i, line in enumerate(file.readlines()):
            line_num = i % 10
            if line_num == 0:
                cur_puzzle = ''
                continue
            cur_puzzle += line
            if line_num == 9:
                puzzles.append(cur_puzzle)

        puzzles_dir = Path('puzzle_files')
        for i, puzzle in enumerate(puzzles):
            with open(puzzles_dir / f'puzzle_{i:02}.txt', 'w') as file:
                file.write(puzzle)

if __name__ == '__main__':
    main()
