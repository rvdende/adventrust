use colored::Colorize;

#[derive(Debug)]
struct Tile {
    x: usize,
    y: usize,
    value: u8,
    called: bool,
}

struct Board {
    number: usize,
    tiles: Vec<Tile>,
}

struct Game {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl std::fmt::Debug for Game {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        println!("\n{}", "Numbers:".to_string().bright_white());
        for i in &self.numbers {
            print!("{}, ", i.to_string().blue());
        }
        println!();
        println!("\n{}", "Boards:".to_string().bright_white());

        for board in &self.boards {
            dbg!(&board);
        }

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

            if (t.value < 10) {
                print!(" ");
            }
            if t.called {
                print!("{} ", t.value.to_string().bright_white());
            } else {
                print!("{} ", t.value.to_string().white());
            }
        });
        println!("");
        Ok(())
    }
}

impl Game {
    fn initgame(filename: String) -> Game {
        let data = std::fs::read_to_string(filename).unwrap();

        let mut game = Game {
            numbers: vec![],
            boards: Vec::new(),
        };

        // parse numbers on first line
        data.lines().take(1).for_each(|(linedata)| {
            game.numbers = linedata
                .split(",")
                .map(|i| i.parse::<usize>().unwrap())
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
                            value: board_raw_value.parse::<u8>().unwrap(),
                            called: false,
                        };

                        return tile;
                    })
                    .collect();

                let board = Board {
                    number: number,
                    tiles,
                };
                game.boards.push(board);

                return ();
            });

        return game;
    }
}

fn main() {
    let game = Game::initgame("sample.txt".to_string());
    dbg!(&game);
}
