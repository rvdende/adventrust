#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
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

    // formatted

    let mut horizontal = 0;
    let mut depth = 0;

    for (dir, value) in data {
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
