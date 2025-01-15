use std::fs;

pub fn process_part1(input: &str) -> isize {
    // let result = input
    //     .split("\n\n")
    //     .map(|elf| elf.lines().map(|i| i.parse::<isize>().unwrap()).sum())
    //     .max()
    //     .unwrap();

    let mut leftside: Vec<isize> = Vec::new();
    let mut rightside: Vec<isize> = Vec::new();

    input.lines().for_each(|line| {
        // split on spaces inbetween numbers
        let number: Vec<isize> = line
            .split_whitespace()
            .map(|i| i.parse::<isize>().unwrap()).collect();

        leftside.push(number[0]);
        rightside.push(number[1]);

    });

    leftside.sort();
    rightside.sort();

    // dbg!(&leftside);
    // dbg!(&rightside);

    assert!(leftside.len() == rightside.len(), "ERROR: Left and right sides not equal length!!!");

    let mut total_combined = 0;

    for idx in 0..leftside.len() {
        let diff = rightside[idx] - leftside[idx];

        // math abs
        total_combined += diff.abs();
        // total_combined += diff;
    }    


    return total_combined;
}

pub fn process_part2(input: &str) -> isize {
    let mut result = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|i| i.parse::<isize>().unwrap())
                .sum::<isize>()
        })
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));

    // dbg!(&result);

    let sum = result.iter().take(3).sum();

    return sum;
}

fn process(filename: &str) -> (isize, isize) {
    let data = fs::read_to_string(filename).unwrap();
    let part1 = process_part1(&data);
    println!("Part1\t{}", &part1);

    let part2 = process_part2(&data);
    // println!("Part2\t{}", part2);

    let part2 = 0;

    (part1, part2)
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2024/advent202401/sample.txt");
    // process("src/advent2024/advent202401/input.txt");
    // process("src/advent2022/day01/input.txt");
}

#[test]
fn test() {
    let sample = process("src/advent2024/advent202401/sample.txt");
    assert_eq!(sample, (11, 0));

    // let input = process("src/advent2022/day01/input.txt");
    // assert_eq!(input, (72602, 207410));
}
