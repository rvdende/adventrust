use std::fs;

const WORD: &str = "XMAS";

fn count_occurrences(grid: &[Vec<char>], word: &str) -> usize {
    let directions = [
        (0, 1),   // Right
        (1, 0),   // Down
        (0, -1),  // Left
        (-1, 0),  // Up
        (1, 1),   // Down-right
        (1, -1),  // Down-left
        (-1, 1),  // Up-right
        (-1, -1), // Up-left
    ];

    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                if matches_word(grid, &word_chars, r, c, dr, dc) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn matches_word(
    grid: &[Vec<char>],
    word: &[char],
    start_row: usize,
    start_col: usize,
    dr: isize,
    dc: isize,
) -> bool {
    let mut row = start_row as isize;
    let mut col = start_col as isize;
    for &ch in word {
        if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
            return false;
        }
        if grid[row as usize][col as usize] != ch {
            return false;
        }
        row += dr;
        col += dc;
    }
    true
}

fn part1_count_word(filename: &str) -> usize {
    let input = fs::read_to_string(&filename).expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let occurrences = count_occurrences(&grid, WORD);
    println!(
        "The word '{}' occurs {} times in the grid.",
        WORD, occurrences
    );
    return occurrences;
}

pub fn run() {
    let count = part1_count_word("src/advent2024/advent202404/sample.txt");
    println!("Count: {}", count);

    let count = part1_count_word("src/advent2024/advent202404/input.txt");
    println!("Count: {}", count);
}

#[test]
fn test() {}
