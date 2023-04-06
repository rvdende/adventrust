// https://adventofcode.com/2022/day/6

fn check_unique(input: String) -> bool {
    for z in 0..input.len() {
        for x in 0..input.len() {
            if z != x {
                if input.chars().nth(z).unwrap() == input.chars().nth(x).unwrap() {
                    return false;
                }
            }
        }
    }
    return true;
}

fn process(filename: &str, length: usize) -> usize {
    let data = std::fs::read_to_string(filename).unwrap();

    for i in 0..=data.len() - length {
        let sequence = data[i..i + length].to_string();
        if check_unique(sequence.clone()) {
            return i + length;
        };
    }

    unreachable!("Should not reach this!");
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2022/day06/sample.txt", 4);
    process("src/advent2022/day06/input.txt", 4);
    process("src/advent2022/day06/sample.txt", 14);
    process("src/advent2022/day06/input.txt", 14);
}

#[test]
fn test() {
    assert_eq!(process("src/advent2022/day06/sample.txt", 4), 7);
    assert_eq!(process("src/advent2022/day06/sample.txt", 14), 19);
    assert_eq!(process("src/advent2022/day06/input.txt", 4), 1757);
    assert_eq!(process("src/advent2022/day06/input.txt", 14), 2950);
}
