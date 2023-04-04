use colored::Colorize;

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    value: usize,
    called: bool,
    connected_line: bool,
}

struct Board {
    number: usize,
    tiles: Vec<Tile>,
    won: bool,
}

struct Numbers {
    value: usize,
    called: bool,
}

struct Game {
    numbers: Vec<Numbers>,
    boards: Vec<Board>,
    numbers_all_called: bool,
    first_score: usize,
    last_score: usize,
}

impl std::fmt::Debug for Game {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("\n\n{}", "Numbers:".to_string().bright_white());
        for i in &self.numbers {
            if i.called {
                print!("{}, ", i.value.to_string().truecolor(0, 255, 255));
            } else {
                print!("{}, ", i.value.to_string().truecolor(100, 100, 100));
            }
        }
        println!();
        // println!("\n{}", "Boards:".to_string().bright_white());

        for board in &self.boards {
            println!();
            println!();
            // println!(
            //     "\n\n{}{}",
            //     "\t\tB#".truecolor(100, 100, 100),
            //     board.number.to_string().truecolor(100, 100, 100)
            // );

            board.tiles.iter().for_each(|t| {
                if t.x == 0 && t.y > 0 {
                    print!("\n");
                }

                if t.value < 10 {
                    print!(" ");
                }

                if t.called {
                    if t.connected_line {
                        print!("{} ", t.value.to_string().truecolor(255, 255, 0));
                    } else {
                        print!("{} ", t.value.to_string().truecolor(255, 255, 255));
                    }
                } else {
                    print!("{} ", t.value.to_string().truecolor(100, 100, 100));
                }
            });
        }
        println!();
        Ok(())
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("{}{}", "Board #".green(), &self.number.to_string().yellow());

        self.tiles.iter().for_each(|t| {
            if t.x == 0 {
                print!("\n");
            }

            if t.value < 10 {
                print!(" ");
            }
            if t.called {
                print!("{} ", t.value.to_string().truecolor(255, 255, 255));
            } else {
                print!("{} ", t.value.to_string().truecolor(100, 100, 100));
            }
        });
        println!("");
        Ok(())
    }
}

impl Game {
    fn initgame(filename: &str) -> Game {
        let data = std::fs::read_to_string(filename).unwrap();

        let mut game = Game {
            numbers: vec![],
            numbers_all_called: false,
            boards: Vec::new(),
            first_score: 0,
            last_score: 0,
        };

        // parse numbers on first line
        data.lines().take(1).for_each(|linedata| {
            game.numbers = linedata
                .split(',')
                .map(|i| Numbers {
                    called: false,
                    value: i.parse::<usize>().unwrap(),
                })
                .collect();
        });

        // parse boards
        data.split("\n\n")
            .skip(1)
            .enumerate()
            .for_each(|(number, board_raw_block)| {
                let tiles: Vec<Tile> = board_raw_block
                    .split_whitespace()
                    .enumerate()
                    .map(|(counter, board_raw_value)| {
                        let tile = Tile {
                            x: counter % 5,
                            y: counter / 5,
                            value: board_raw_value.parse::<usize>().unwrap(),
                            called: false,
                            connected_line: false,
                        };

                        return tile;
                    })
                    .collect();

                let board = Board {
                    number: number,
                    tiles,
                    won: false,
                };
                game.boards.push(board);
            });

        game
    }

    fn call_number(&mut self) {
        println!("calling next number!");
        let mut index = 0;
        loop {
            if !self.numbers[index].called {
                let called_number = self.numbers[index].value;
                println!("Call {}", called_number.to_string().bright_yellow());
                self.numbers[index].called = true;

                // marks a number as called
                for b in 0..self.boards.len() {
                    let board: &mut Board = &mut self.boards[b];
                    let winnerfound = Game::mark_board(board, called_number);
                    if winnerfound {
                        let score = Game::winner(board, called_number);
                        if self.first_score == 0 {
                            self.first_score = score;
                        }

                        self.last_score = score;
                    }
                }

                break;
            }
            index += 1;
            if index == self.numbers.len() {
                println!("\nAll numbers called!\n");
                self.numbers_all_called = true;
                break;
            }
        }
    }

    fn mark_board(board: &mut Board, called_number: usize) -> bool {
        let mut we_have_a_winner = false;

        if board.won {
            return false;
        } //skip if the board has already won.

        for t in 0..board.tiles.len() {
            if board.tiles[t].value == called_number {
                board.tiles[t].called = true;
            }
        }

        // check horizontal
        for col in 0..5 {
            let check = board
                .tiles
                .iter()
                .filter(|t| t.x == col && t.called)
                .count();

            for t in 0..board.tiles.len() {
                if board.tiles[t].called && board.tiles[t].x == col && check == 5 {
                    board.tiles[t].connected_line = true;
                    we_have_a_winner = true;
                }
            }
        }

        // check verital
        for row in 0..5 {
            let check = board
                .tiles
                .iter()
                .filter(|t| t.y == row && t.called)
                .count();

            for t in 0..board.tiles.len() {
                if board.tiles[t].called && board.tiles[t].y == row && check == 5 {
                    board.tiles[t].connected_line = true;
                    we_have_a_winner = true;
                }
            }
        }

        we_have_a_winner
    }

    fn winner(board: &mut Board, called_number: usize) -> usize {
        board.won = true;
        println!(
            "We have a winner!!! board number {} number called: {}",
            board.number, called_number
        );

        let sum_unmarked: usize = board
            .tiles
            .iter()
            .filter(|t| !t.called)
            .map(|t| t.value)
            .sum();

        let score = sum_unmarked * called_number;

        println!(
            "Sum unmarked {} x {} = {}",
            sum_unmarked, called_number, score
        );

        score
    }
}

fn process(filename: &str) -> (usize, usize) {
    // let mut game = Game::initgame("sample.txt".to_string());
    let mut game = Game::initgame(filename);

    loop {
        game.call_number();
        // dbg!(&game);

        // part 1
        // if game.winner_found || game.numbers_all_called {
        //     break;
        // }

        if game.numbers_all_called {
            break;
        }
    }

    let part1answer = game.first_score;
    let part2answer: usize = game.last_score;

    (part1answer, part2answer)
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2021/day04/sample.txt");
}

#[test]
fn test() {
    let sample = process("src/advent2021/day04/sample.txt");
    assert_eq!(sample, (4512, 1924));

    let input = process("src/advent2021/day04/input.txt");
    assert_eq!(input, (21607, 19012));
}
