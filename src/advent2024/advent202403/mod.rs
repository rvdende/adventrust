use regex::Regex;
use std::fs;

pub fn process_mul(memory: &str) -> isize {
    // Example input string (corrupted memory)
    // let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    // Regular expression to match valid mul(X,Y) instructions
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Initialize the total sum
    let mut total_sum = 0;

    // Iterate over all matches
    for cap in re.captures_iter(memory) {
        if let (Some(x), Some(y)) = (cap.get(1), cap.get(2)) {
            // Parse the captured numbers
            let x: isize = x.as_str().parse().unwrap_or(0);
            let y: isize = y.as_str().parse().unwrap_or(0);
            // Multiply and add to the total sum
            total_sum += x * y;
        }
    }

    // Print the total sum
    println!("The total sum of valid multiplications is: {}", total_sum);

    return total_sum;
}

pub fn process_part2(input: &str) -> isize {
    let segments: Vec<&str> = input.split("do()").collect();

    let mut total = 0;
    segments.iter().for_each(|segment| {
        let validsegment = segment.split("don't()").next().unwrap();
        println!("{}", validsegment);
        let result = process_mul(validsegment);
        println!("Result: {}", result);
        total += result;
    });

    // print!("Total: {}", total);
    return total;
}

// fn process(filename: &str) -> (isize, isize) {
//     let data = fs::read_to_string(filename).unwrap();
//     let part1 = process_part1(&data);
//     println!("Part1\t{}", &part1);

//     let part2 = process_part2(&data);
//     println!("Part2\t{}", part2);

//     // let part2 = 0;

//     (part1, part2)
// }

#[allow(dead_code)]
pub fn run() {
    // process_part1("src/advent2024/advent202403/sample.txt");

    println!("PART 1 ---------------");
    let data1 = fs::read_to_string("src/advent2024/advent202403/sample.txt").unwrap();
    let part1 = process_mul(&data1);
    println!("Part1\t{}", &part1);

    println!("\nPART 2 ---------------");
    let data2 = fs::read_to_string("src/advent2024/advent202403/input.txt").unwrap();
    let part2 = process_part2(&data2);
    println!("Part2\t{}", &part2);

    // process("src/advent2024/advent202403/input.txt");
}

#[test]
fn test() {
    assert_eq!(
        process_mul(&fs::read_to_string("src/advent2024/advent202403/sample.txt").unwrap()),
        161
    );

    assert_eq!(
        process_mul(&fs::read_to_string("src/advent2024/advent202403/input.txt").unwrap()),
        190604937
    );

    assert_eq!(
        process_part2(&fs::read_to_string("src/advent2024/advent202403/sample2.txt").unwrap()),
        48
    );

    assert_eq!(
        process_part2(&fs::read_to_string("src/advent2024/advent202403/input.txt").unwrap()),
        82857512
    );

    // let input = process("src/advent2024/advent202403/input.txt");
    // assert_eq!(input, (190604937, 0));
}
