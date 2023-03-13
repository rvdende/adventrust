#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

fn part1(data_in: Vec<(Direction, isize)>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for (dir, value) in data_in {
        match dir {
            Direction::Forward => {
                println!("Forward {}", value);
                horizontal += value;
            }
            Direction::Down => {
                depth += value;
                println!("Down {}", value);
            }
            Direction::Up => {
                depth -= value;
                println!("Up {}", value);
            }
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Total: {}", horizontal * depth);
}

fn part2(data_in: Vec<(Direction, isize)>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, value) in data_in {
        match dir {
            Direction::Down => {
                aim += value;
                println!("Down {}", value);
            }
            Direction::Up => {
                aim -= value;
                println!("Up {}", value);
            }
            Direction::Forward => {
                println!("Forward {}", value);
                horizontal += value;
                depth += aim * value;
            }
        }
    }

    println!("Horizontal: {}", horizontal);
    println!("Depth: {}", depth);
    println!("Total: {}", horizontal * depth);
}

fn main() {
    // let data = std::fs::read_to_string("sample.txt").expect("could not read file.");
    let data = std::fs::read_to_string("datainput.txt").expect("could not read file.");

    let data = data
        .trim()
        .split("\n")
        .map(|s| {
            let (dirstr, value) = s.split_once(" ").expect("Must contain a space");
            let value: isize = value.parse().expect("Must be a number");
            let dirstr: Direction = match dirstr {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Unknown direction"),
            };
            let output = (dirstr, value);

            return output;
        })
        .collect::<Vec<(Direction, isize)>>();

    // part1(data);
    part2(data);
}
