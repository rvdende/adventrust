fn score_round(opponent: &str, player: &str) -> usize {
    let hand = match player {
        "rock" => 1,
        "paper" => 2,
        "scissors" => 3,
        _ => unreachable!("invalid hand!"),
    };

    let win = 6;
    let draw = 3;
    let lose = 0;

    if opponent == player {
        return hand + draw;
    }

    if opponent == "rock" && player == "paper" {
        return hand + win;
    }
    if opponent == "paper" && player == "scissors" {
        return hand + win;
    }
    if opponent == "scissors" && player == "rock" {
        return hand + win;
    }
    if opponent == "rock" && player == "scissors" {
        return hand + lose;
    }
    if opponent == "paper" && player == "rock" {
        return hand + lose;
    }
    if opponent == "scissors" && player == "paper" {
        return hand + lose;
    }

    unreachable!("invalid game");
}

fn process(filename: &str, part: &str) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();

    let total_points: usize = data
        .lines()
        .map(|l| {
            let (mut opponent, mut player) = l.split_once(' ').unwrap();

            opponent = match opponent {
                "A" => "rock",
                "B" => "paper",
                "C" => "scissors",
                _ => unreachable!("invalid input"),
            };

            // switch depending on part 1 or two
            if part == "part1" {
                player = match player {
                    "X" => "rock",
                    "Y" => "paper",
                    "Z" => "scissors",
                    _ => unreachable!("invalid input"),
                };
            } else {
                // part 2
                player = match player {
                    "X" => match opponent {
                        // lose
                        "rock" => "scissors",
                        "scissors" => "paper",
                        "paper" => "rock",
                        _ => unreachable!(),
                    },

                    "Y" => opponent, // draw

                    "Z" => match opponent {
                        // win
                        "rock" => "paper",
                        "scissors" => "rock",
                        "paper" => "scissors",
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
            }

            let points = score_round(opponent, player);
            println!("round {:?} score: {}", (opponent, player), points);

            return points;
        })
        .sum();

    return total_points;
}
#[allow(dead_code)]
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
