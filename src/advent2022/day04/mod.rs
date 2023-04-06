fn part1_fully_contained(ra: (usize, usize), rb: (usize, usize)) -> usize {
    // if a is in b
    if ra.0 >= rb.0 && ra.1 <= rb.1 {
        return 1;
    }

    // if b is in a
    if rb.0 >= ra.0 && rb.1 <= ra.1 {
        return 1;
    }

    return 0;
}

fn part2_partial(ra: (usize, usize), rb: (usize, usize)) -> usize {
    for a in ra.0..=ra.1 {
        for b in rb.0..=rb.1 {
            if a == b {
                return 1;
            }
        }
    }

    return 0;
}

fn process(filename: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(filename).unwrap();

    let teams: Vec<((usize, usize), (usize, usize))> = data
        .lines()
        .map(|l| {
            let (elf_a, elf_b) = l.split_once(',').unwrap();

            let elf_a_data = elf_a.split_once("-").unwrap();
            let elf_b_data = elf_b.split_once("-").unwrap();

            let elf_a_range: (usize, usize) =
                (elf_a_data.0.parse().unwrap(), elf_a_data.1.parse().unwrap());

            let elf_b_range: (usize, usize) =
                (elf_b_data.0.parse().unwrap(), elf_b_data.1.parse().unwrap());

            println!(
                "{}-{},{}-{}",
                elf_a_range.0, elf_a_range.1, elf_b_range.0, elf_b_range.1
            );

            (elf_a_range, elf_b_range)
        })
        .collect();

    let total_part1 = teams.iter().map(|t| part1_fully_contained(t.0, t.1)).sum();

    let total_part2 = teams.iter().map(|t| part2_partial(t.0, t.1)).sum();

    println!("part1: {}  part2: {}", total_part1, total_part2);

    return (total_part1, total_part2);
}

#[allow(dead_code)]
pub fn run() {
    println!("day 4");
    process("src/advent2022/day04/sample.txt");
    process("src/advent2022/day04/input.txt");
}

#[test]
fn test() {
    assert_eq!(process("src/advent2022/day04/sample.txt"), (2, 4));
    assert_eq!(process("src/advent2022/day04/input.txt"), (524, 798));
}
