use std::fs;

pub fn process_part1(input: &str) -> usize {
    let result = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|i| i.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .max()
        .unwrap();
    return result;
}

pub fn process_part2(input: &str) -> usize {
    let mut result = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|i| i.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));

    // dbg!(&result);

    let sum = result.iter().take(3).sum();

    return sum;
}

fn process(filename: &str) -> (usize, usize) {
    let data = fs::read_to_string(filename).unwrap();
    let part1 = process_part1(&data);
    println!("Part1\t{}", &part1);

    let part2 = process_part2(&data);
    println!("Part2\t{}", part2);

    (part1, part2)
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2022/day01/sample.txt");
    process("src/advent2022/day01/input.txt");
}

#[test]
fn test() {
    let sample = process("src/advent2022/day01/sample.txt");
    assert_eq!(sample, (24000, 45000));

    let input = process("src/advent2022/day01/input.txt");
    assert_eq!(input, (72602, 207410));
}
