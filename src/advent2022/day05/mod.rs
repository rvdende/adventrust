use itertools::Itertools;

#[derive(Debug, Clone)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Stack {
    data: Vec<char>,
}

fn process_yard_data(yard_data: &str) -> Vec<Stack> {
    let yard_data_lines: Vec<String> = yard_data.lines().map(|l| l.to_string()).collect();

    let number_of_stacks: usize = yard_data_lines[yard_data_lines.len() - 1]
        .to_string()
        .trim()
        .split(" ")
        .filter(|s| s.trim().len() > 0)
        .count();

    let mut stacks: Vec<Stack> = Vec::new();
    for _i in 1..=number_of_stacks {
        stacks.push(Stack { data: Vec::new() });
    }

    println!("number_of_stacks: {}", number_of_stacks);

    let stack_height = yard_data_lines.len() - 1;

    println!("stack_height: {}", stack_height);

    for i in (0..stack_height).rev() {
        println!("height {i}");
        let layer = yard_data_lines[i].to_string();

        println!("layer {layer}");
        for s in 0..number_of_stacks {
            println!("stack {s}");
            let char = layer.chars().nth(1 + (s * 4)).unwrap();
            println!("stack {s} {char}");
            if char != ' ' {
                stacks[s].data.push(char);
            }
        }
    }

    stacks
}

fn part1(stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    let moves = moves.clone();

    // apply moves
    moves.iter().for_each(|m| {
        println!("applying move to stack {:?}", m);
        for _a in 0..m.amount {
            let container = stacks[m.from - 1].data.pop().unwrap();
            stacks[m.to - 1].data.push(container);
        }
    });

    println!("{:?}", stacks);

    println!("{:?}", moves);

    let answer = stacks
        .iter()
        .map(|s| s.data.clone().pop().unwrap())
        .join("");

    println!("{}", answer);

    answer
}

fn part2(stacks: Vec<Stack>, moves: Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    let moves = moves.clone();

    // apply moves
    moves.iter().for_each(|m| {
        println!("applying move to stack {:?}", m);

        let mut crane_grapple: Vec<char> = Vec::new();

        for _a in 0..m.amount {
            let container = stacks[m.from - 1].data.pop().unwrap();
            crane_grapple.push(container);
        }

        crane_grapple.reverse();

        crane_grapple.iter().for_each(|i| {
            stacks[m.to - 1].data.push(*i);
        })
    });

    println!("{:?}", stacks);

    println!("{:?}", moves);

    let answer = stacks
        .iter()
        .map(|s| s.data.clone().pop().unwrap())
        .join("");

    println!("{}", answer);

    answer
}

fn process(filename: &str) -> (String, String) {
    let data = std::fs::read_to_string(filename).unwrap();

    let (stacks_data, moves_data) = data.split_once("\n\n").unwrap();

    let stacks: Vec<Stack> = process_yard_data(stacks_data);
    println!("{:?}", stacks);

    let moves: Vec<Move> = moves_data
        .lines()
        .map(|l| {
            let data: Vec<&str> = l.split(" ").collect();

            let m = Move {
                amount: data[1].parse().unwrap(),
                from: data[3].parse().unwrap(),
                to: data[5].parse().unwrap(),
            };

            return m;
        })
        .collect();

    let part1_answer = part1(stacks.clone(), moves.clone());
    let part2_answer = part2(stacks.clone(), moves.clone());

    (part1_answer, part2_answer)
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2022/day05/sample.txt");
    process("src/advent2022/day05/input.txt");
}

#[test]
fn test() {
    assert_eq!(
        process("src/advent2022/day05/sample.txt"),
        ("CMZ".to_string(), "MCD".to_string())
    );
    assert_eq!(
        process("src/advent2022/day05/input.txt"),
        ("ZSQVCCJLL".to_string(), "QZFJRWHGS".to_string())
    );
}
