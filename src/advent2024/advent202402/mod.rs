use std::fs;

pub fn check_report_levels(report: Vec<isize>) -> bool {
    let mut safe = 1;
    let up_down = report[1] - report[0];

    report.iter().for_each(|level| {
        print!("{} ", level);
    });

    if up_down > 0 {
        print!("expect up   ")
    }

    if up_down < 0 {
        print!("expect down ")
    }

    if up_down == 0 {
        safe = 0;
    }

    if (report.len() == 1) {
        safe = 0;
    }

    if (report.len() > 0) {
        for idx in 0..report.len() - 1 {
            let level = report[idx];
            let next = report[idx + 1];

            if level == next {
                safe = 0;
            }

            let diff = next - level;

            // absolute
            if diff.abs() > 3 {
                safe = 0;
            }

            if up_down > 0 && diff < 0 {
                safe = 0;
            }

            if up_down < 0 && diff > 0 {
                safe = 0;
            }
        }
    }

    if safe == 1 {
        print!("Safe");
    }

    if safe == 0 {
        print!("Unsafe");
    }
    println!();

    safe == 1
}

pub fn process_part1(input: &str) -> isize {
    let result: Vec<Vec<isize>> = input
        .lines()
        .map(|report| {
            report
                .split(" ")
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    dbg!(&result);

    let mut countsafe = 0;

    result.iter().for_each(|report| {
        let safe = check_report_levels(report.clone());

        if safe == true {
            countsafe += 1;
        }
    });

    return countsafe;
}

pub fn process_part2(input: &str) -> isize {
    let result: Vec<Vec<isize>> = input
        .lines()
        .map(|report| {
            report
                .split(" ")
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect();

    dbg!(&result);

    let mut countsafe = 0;

    result.iter().for_each(|report| {
        let mut safe = check_report_levels(report.clone());

        // create variations of the report by removing one element at a time

        for idx in 0..report.len() {
            let mut variation = report.clone();
            variation.remove(idx);
            // variations.push(variation);
            let safe_single_change = check_report_levels(variation.clone());
            if safe_single_change == true {
                safe = true;
            }
        }

        if safe == true {
            countsafe += 1;
        }

        println!();
    });

    return countsafe;
}

fn process(filename: &str) -> (isize, isize) {
    let data = fs::read_to_string(filename).unwrap();
    let part1 = process_part1(&data);
    println!("Part1\t{}", &part1);

    let part2 = process_part2(&data);
    println!("Part2\t{}", part2);

    // let part2 = 0;

    (part1, part2)
}

#[allow(dead_code)]
pub fn run() {
    // process("src/advent2024/advent202402/sample.txt");
    process("src/advent2024/advent202402/input.txt");
}

#[test]
fn test() {
    let sample = process("src/advent2024/advent202401/sample.txt");
    assert_eq!(sample, (11, 0));

    // let input = process("src/advent2022/day01/input.txt");
    // assert_eq!(input, (72602, 207410));
}
