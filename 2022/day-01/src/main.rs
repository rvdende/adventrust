use std::fs;

pub fn process_part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|i| i.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
    return result.to_string();
}

pub fn process_part2(input: &str) -> u32 {
    let mut result = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|i| i.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<_>>();

    result.sort_by(|a, b| b.cmp(a));

    // dbg!(&result);

    let sum = result.iter().take(3).sum();

    return sum;
}

fn main() {
    let demo = fs::read_to_string("./input.txt").unwrap();
    let example = process_part1(&demo);
    println!("Sample\t{}", example);

    let file = fs::read_to_string("./input_a.txt").unwrap();
    let part1 = process_part1(&file);
    println!("Part1\t{}", part1);

    let part2 = process_part2(&file);
    println!("Part2\t{}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let file = fs::read_to_string("./input.txt").unwrap();
        let max = process_part1(&file);
        println!("{}", max);
        assert_eq!("test", "test");
    }
}
