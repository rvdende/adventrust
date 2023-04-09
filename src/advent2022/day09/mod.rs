use itertools::Itertools;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn calc_tail(head: &Coord, tail: &Coord) -> String {
    let same_row = head.y == tail.y;
    let same_col = head.x == tail.x;
    let mut direction = "?";

    if same_row {
        direction = if head.x > tail.x { "R" } else { "L" };
    };

    if same_col {
        direction = if head.y > tail.y { "U" } else { "D" };
    };

    // diagonal
    if !same_row && !same_col {
        if head.x > tail.x && head.y > tail.y {
            direction = "UR";
        }
        if head.x > tail.x && head.y < tail.y {
            direction = "DR";
        }
        if head.x < tail.x && head.y > tail.y {
            direction = "UL";
        }
        if head.x < tail.x && head.y < tail.y {
            direction = "DL";
        }
    }

    // disance
    let x = (head.x - tail.x) as f64;
    let y = (head.y - tail.y) as f64;

    let distance = (x * x + y * y).sqrt() as i32;

    if distance >= 2 {
        return direction.to_string();
    } else {
        return "NONE".to_string();
    }
}

fn move_coord(coord: &Coord, direction: &str) -> Coord {
    match direction {
        "U" => Coord {
            x: coord.x,
            y: coord.y + 1,
        },
        "UR" => Coord {
            x: coord.x + 1,
            y: coord.y + 1,
        },
        "R" => Coord {
            x: coord.x + 1,
            y: coord.y,
        },
        "DR" => Coord {
            x: coord.x + 1,
            y: coord.y - 1,
        },
        "D" => Coord {
            x: coord.x,
            y: coord.y - 1,
        },
        "DL" => Coord {
            x: coord.x - 1,
            y: coord.y - 1,
        },
        "L" => Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        "UL" => Coord {
            x: coord.x - 1,
            y: coord.y + 1,
        },
        "NONE" => Coord {
            x: coord.x,
            y: coord.y,
        },
        _ => panic!("Unknown direction"),
    }
}

#[allow(dead_code)]
fn part1(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();

    let mut head = Coord { x: 50, y: 50 };
    let mut tail = Coord { x: 50, y: 50 };

    let mut tail_moves: Vec<Coord> = Vec::new();

    data.lines().for_each(|l| {
        let (direction, steps) = l.split_once(" ").unwrap();
        println!("{} {}", direction, steps);

        // steps
        for _i in 0..steps.parse::<i32>().unwrap() {
            // head
            head = move_coord(&head, &direction);

            // tail
            tail = move_coord(&tail, calc_tail(&head, &tail).as_str());
            tail_moves.push(tail.clone());
        }
    });

    return tail_moves.iter().unique().count();
}

fn part2(filename: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();

    let num_segments = 10;

    let mut segments = vec![Coord { x: 50, y: 50 }; num_segments];

    let mut tail_moves: Vec<Coord> = Vec::new();

    data.lines().for_each(|l| {
        let (direction, steps) = l.split_once(" ").unwrap();
        println!("{} {}", direction, steps);

        for _i in 0..steps.parse::<i32>().unwrap() {
            // move head
            segments[0] = move_coord(&segments[0], &direction);

            for i in 1..segments.len() {
                segments[i] = move_coord(
                    &segments[i],
                    calc_tail(&segments[i - 1], &segments[i]).as_str(),
                );
            }

            tail_moves.push(segments.last().unwrap().clone());
        }
    });

    return tail_moves.iter().unique().count();
}

#[allow(dead_code)]
pub fn run() {
    let a = part2("src/advent2022/day09/input.txt");
    // let a = part2("src/advent2022/day09/input.txt");
    println!("a: {}", a);
}

#[test]
fn test() {
    assert_eq!(part1("src/advent2022/day09/sample.txt"), 13);
    assert_eq!(part1("src/advent2022/day09/input.txt"), 6503);
    assert_eq!(part2("src/advent2022/day09/sample_part2.txt"), 36);
    assert_eq!(part2("src/advent2022/day09/input.txt"), 2724);
}
