use itertools::Itertools;

fn char_to_value(c: char) -> isize {
    let a = 'a' as u8;
    let z = 'z' as u8;
    let cap_a = 'A' as u8;
    let n = c as u8;
    let output;
    if n >= a && n <= z {
        // lowercase
        output = n - a + 1;
    } else {
        // uppercase letters
        output = n - cap_a + 27;
    }

    return output as isize;
}

#[test]
fn char_to_value_test() {
    assert_eq!(char_to_value('a'), 1);
    assert_eq!(char_to_value('z'), 26);
    assert_eq!(char_to_value('A'), 27);
    assert_eq!(char_to_value('Z'), 52);
}

/////////////////////////////////////////////////////////////////

// See https://stackoverflow.com/a/64819334
fn find_common_char(left: &str, right: &str) -> String {
    if left == right {
        return left.to_string();
    }

    let output: String = left
        .chars()
        .filter(|c| match right.chars().position(|x| &x == c) {
            Some(x) => true,
            None => false,
        })
        .unique_by(|c| c.to_string())
        .collect();

    return output;
}

fn find_common_three(a: &str, b: &str, c: &str) -> String {
    find_common_char(
        find_common_char(a, b).as_str(),
        find_common_char(b, c).as_str(),
    )
    .to_string()
}

#[test]
fn find_common_char_test() {
    let res = find_common_char("abCd", "zxpC");
    assert_eq!(res, "C");

    let elf1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
    let elf2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
    let elf3 = "PmmdzqPrVvPwwTWBwg";

    let result_b = find_common_three(elf1, elf2, elf3);
    assert_eq!(result_b, "r");
}

/////////////////////////////////////////////////////////////////

fn process_old(filename: &str) {
    let data = std::fs::read_to_string(filename).unwrap();
    // let data = std::fs::read_to_string("data.txt").unwrap();

    let part1: usize = data
        .trim()
        .split("\n")
        .map(|rs| {
            // char value

            // let rsi: String = rs.to_string();
            let chars = rs
                .chars()
                .map(|c| {
                    let a = 'a' as u8;
                    let z = 'z' as u8;
                    let cap_a = 'A' as u8;
                    let n = c as u8;

                    // println!("{} {} {} {} {}", a, z, A, Z, n);

                    if n >= a && n <= z {
                        return n - a + 1;
                    } else {
                        return n - cap_a + 27;
                    }
                })
                .collect::<Vec<u8>>();

            // println!("{}", chars.len());

            let mut output: usize = 0;

            for x in 0..chars.len() / 2 {
                for y in chars.len() / 2..chars.len() {
                    if x != y {
                        let obj_x = chars[x];
                        let obj_y = chars[y];
                        if obj_x == obj_y {
                            // let z = rsi.as_bytes()[x] as char;
                            // println!("{}",rsi.to_string());
                            // println!("found match x {} y {} char {}", x, y, z);
                            output = chars[x].into();
                            return output;
                        }
                    }
                }
            }

            return output;
        })
        .collect::<Vec<usize>>()
        .iter()
        .sum();

    println!("part 1: {}", part1);

    // ---------------------

    let part2: Vec<_> = data.trim().split("\n").collect();

    let mut idx = 0;
    loop {
        let a = part2[idx].to_string();
        let b = part2[idx + 1].to_string();
        let c = part2[idx + 2].to_string();
        idx += 3;

        if idx >= part2.len() {
            break;
        }
    }

    dbg!(part2);
}

fn process_part1(filename: &str) -> isize {
    let sumtotal: isize = std::fs::read_to_string(filename)
        .expect("could not read file")
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);

            if left.len() != right.len() {
                panic!("left and right are not the same length");
            }

            let common = find_common_char(left, right);

            let val = char_to_value(common.chars().next().unwrap());

            println!(
                "{}  {} | {} {} [{} {}]",
                " ".repeat(30 - left.len()),
                left,
                right,
                " ".repeat(30 - right.len()),
                common,
                val
            );

            return val;
        })
        .sum();

    // println!("part 1: {:?}", &sumtotal);

    return sumtotal;
}

fn process_part2(filename: &str) -> isize {
    let binding = std::fs::read_to_string(filename).expect("could not read file");

    let lines: Vec<&str> = binding.trim().lines().collect();

    let mut sumtotal: isize = 0;

    for x in 0..lines.len() / 3 {
        let elf1 = lines[x * 3].to_string();
        let elf2 = lines[(x * 3) + 1].to_string();
        let elf3 = lines[(x * 3) + 2].to_string();
        // println!("{}", elf1);
        // println!("{}", elf2);
        // println!("{}", elf3);

        let group_tag = find_common_three(elf1.as_str(), elf2.as_str(), elf3.as_str());
        // println!("{}", group_tag);

        let val = char_to_value(group_tag.chars().next().unwrap());

        sumtotal += val;

        // println!("----");
    }

    return sumtotal;
}

pub fn run() {
    let sample_p1 = process_part1("src/advent2022/day03/sample.txt");
    println!("sample part 1: {}", sample_p1);
    let sample_p2 = process_part2("src/advent2022/day03/sample.txt");
    println!("sample part 2: {}", sample_p2);

    let input_p1 = process_part1("src/advent2022/day03/input.txt");
    println!("input part 1: {}", input_p1);
    let input_p2 = process_part2("src/advent2022/day03/input.txt");
    println!("input part 2: {}", input_p2);
}

#[test]
fn test() {
    assert_eq!(process_part1("src/advent2022/day03/sample.txt"), 157);
    assert_eq!(process_part2("src/advent2022/day03/sample.txt"), 70);

    assert_eq!(process_part1("src/advent2022/day03/input.txt"), 7967);
    assert_eq!(process_part2("src/advent2022/day03/input.txt"), 2716);
}
