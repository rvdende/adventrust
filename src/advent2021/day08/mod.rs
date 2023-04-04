struct Screen {
    signal: String,
    output: String,
}

impl Screen {
    fn process(&mut self) {
        println!("{} - {}", self.signal, self.output);
    }

    fn count_unique_output(&self) -> usize {
        self.output
            .split(' ')
            .map(|segment| match segment.len() {
                2 => 1, // digit 1
                4 => 1, // 4
                3 => 1, // 7
                7 => 1, // 8
                _ => 0,
            })
            .sum()
    }
}

fn process(filename: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(filename).unwrap();

    let screens: Vec<Screen> = data
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once('|').unwrap();

            Screen {
                signal: signal.trim().to_string(),
                output: output.trim().to_string(),
            }
        })
        .collect();

    let part1_total: usize = screens.iter().map(|s| s.count_unique_output()).sum();

    (part1_total, 0)
}

pub fn run() {
    let sample = process("src/advent2021/day08/sample.txt");
    println!("part1 {}", sample.0);
}

#[test]
fn test() {
    let sample = process("src/advent2021/day08/sample.txt");
    assert_eq!(sample.0, 26);
}
