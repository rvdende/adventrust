fn score(hand: &str) -> usize {
    match hand {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => unreachable!("invalid hand!"),
    }
}

fn score_round(round: (&str, &str)) -> usize {
    let hand = score(round.1);

    let win = 6;
    let draw = 3;
    let lose = 0;

    if round.0 == round.1 {
        return hand + draw;
    }

    if round.0 == "rock" && round.1 == "paper" {
        return hand + win;
    }
    if round.0 == "paper" && round.1 == "scissors" {
        return hand + win;
    }
    if round.0 == "scissors" && round.1 == "rock" {
        return hand + win;
    }
    if round.0 == "rock" && round.1 == "scissors" {
        return hand + lose;
    }
    if round.0 == "paper" && round.1 == "rock" {
        return hand + lose;
    }
    if round.0 == "scissors" && round.1 == "paper" {
        return hand + lose;
    }

    unreachable!("invalid game");
}

fn calculate_hand<'a>(opponent: &'a str, outcome: &'a str) -> &'a str {
    match outcome {
        // lose
        "X" => match opponent {
            "rock" => "scissors",
            "scissors" => "paper",
            "paper" => "rock",
            _ => unreachable!(),
        },
        // draw
        "Y" => opponent,
        // win
        "Z" => match opponent {
            "rock" => "paper",
            "scissors" => "rock",
            "paper" => "scissors",
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn process(filename: &str, part: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();

    let total_points: usize = data
        .lines()
        .map(|l| {
            let mut round = l.split_once(' ').unwrap();

            round.0 = match round.0 {
                "A" => "rock",
                "B" => "paper",
                "C" => "scissors",
                _ => unreachable!("invalid input"),
            };

            // switch depending on part 1 or two
            if part == "part1" {
                round.1 = match round.1 {
                    "X" => "rock",
                    "Y" => "paper",
                    "Z" => "scissors",
                    _ => unreachable!("invalid input"),
                };
            } else {
                // part 2
                round.1 = calculate_hand(round.0, round.1);
            }

            let points = score_round(round);
            println!("round {:?} score: {}", round, points);

            return points;
        })
        .sum();

    return total_points;
}

pub fn run() {
    let sample_p1 = process("src/advent2022/day02/sample.txt", "part1");
    println!("sample_p1 {} ", sample_p1);
    let sample_p2 = process("src/advent2022/day02/sample.txt", "part2");
    println!("sample_p2 {} ", sample_p2);

    let input_p1 = process("src/advent2022/day02/input.txt", "part1");
    println!("input_p1 {} ", input_p1);
    let input_p2 = process("src/advent2022/day02/input.txt", "part2");
    println!("input_p2 {} ", input_p2);
}

#[test]
fn test() {
    let sample_p1 = process("src/advent2022/day02/sample.txt", "part1");
    assert_eq!(sample_p1, 15);
    let sample_p2 = process("src/advent2022/day02/sample.txt", "part2");
    assert_eq!(sample_p2, 12);

    let input_p1 = process("src/advent2022/day02/input.txt", "part1");
    assert_eq!(input_p1, 13924);
    let input_p2 = process("src/advent2022/day02/input.txt", "part2");
    assert_eq!(input_p2, 13448);
}
