#![cfg(test)]

pub struct SubmarinePath {
    pub path: Vec<(SubmarineDirection, isize)>,
}

#[derive(Debug)]
pub enum SubmarineDirection {
    Forward,
    Down,
    Up,
}

impl SubmarinePath {
    #[allow(dead_code)]
    fn from_file(filename: &str) -> Self {
        // let data = std::fs::read_to_string("sample.txt").expect("could not read file.");
        let data = std::fs::read_to_string(filename).expect("could not read file.");

        let data = data
            .trim()
            .split('\n')
            .map(|s| {
                let (dirstr, value) = s.split_once(' ').expect("Must contain a space");
                let value: isize = value.parse().expect("Must be a number");
                let dirstr: SubmarineDirection = match dirstr {
                    "forward" => SubmarineDirection::Forward,
                    "down" => SubmarineDirection::Down,
                    "up" => SubmarineDirection::Up,
                    _ => panic!("Unknown direction"),
                };
                (dirstr, value)
            })
            .collect::<Vec<(SubmarineDirection, isize)>>();

        Self { path: data }
    }

    fn distance(&self) -> isize {
        let mut horizontal = 0;
        let mut depth = 0;

        for (dir, value) in &self.path {
            match dir {
                SubmarineDirection::Forward => {
                    println!("Forward {}", value);
                    horizontal += value;
                }
                SubmarineDirection::Down => {
                    depth += value;
                    println!("Down {}", value);
                }
                SubmarineDirection::Up => {
                    depth -= value;
                    println!("Up {}", value);
                }
            }
        }

        println!("Horizontal: {}", horizontal);
        println!("Depth: {}", depth);
        println!("Total: {}", horizontal * depth);

        horizontal * depth
    }

    fn final_distance(&self) -> isize {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (dir, value) in &self.path {
            match dir {
                SubmarineDirection::Down => {
                    aim += value;
                    println!("Down {}", value);
                }
                SubmarineDirection::Up => {
                    aim -= value;
                    println!("Up {}", value);
                }
                SubmarineDirection::Forward => {
                    println!("Forward {}", value);
                    horizontal += value;
                    depth += aim * value;
                }
            }
        }

        println!("Horizontal: {}", horizontal);
        println!("Depth: {}", depth);
        println!("Total: {}", horizontal * depth);

        horizontal * depth
    }
}

#[test]
fn test() {
    let subpath = SubmarinePath::from_file("src/advent2021/day02/sample.txt");
    assert_eq!(subpath.distance(), 150);
    assert_eq!(subpath.final_distance(), 900);

    let input = SubmarinePath::from_file("src/advent2021/day02/input.txt");
    assert_eq!(input.distance(), 2215080);
    assert_eq!(input.final_distance(), 1864715580);
}
