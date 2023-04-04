struct Submarine {
    scanner_readings: Vec<usize>,
}

impl Submarine {
    fn load_data(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();

        let scanner_readings = data
            .trim()
            .split('\n')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self { scanner_readings }
    }

    fn calculate_depth_changes(&self) -> usize {
        let mut previous = 0;
        let mut depth_changes = 0;
        self.scanner_readings
            .iter()
            .enumerate()
            .for_each(|(count, reading)| {
                if count > 0 && reading > &previous {
                    depth_changes += 1;
                }

                previous = *reading;
            });

        depth_changes
    }

    fn calculate_depth_changes_window(&self) -> usize {
        let mut increases = 0;
        let data = &self.scanner_readings;
        for i in 0..data.len() - 3 {
            let a = data[i] + data[i + 1] + data[i + 2];
            let b = data[i + 1] + data[i + 2] + data[i + 3];

            if b > a {
                increases += 1;
            }
        }

        increases
    }
}
#[allow(dead_code)]
fn main() {
    let sub = Submarine::load_data("src/day01/sample.txt");
    //let sub = Submarine::load_data("day01/input.txt");

    let part1 = sub.calculate_depth_changes();
    println!("Depth changes: {}", part1);

    let part2 = sub.calculate_depth_changes_window();
    println!("Depth changes window: {}", part2);
}

#[test]
pub fn test() {
    // sample
    let sample_sub = Submarine::load_data("src/day01/sample.txt");
    let sample_part1 = sample_sub.calculate_depth_changes();
    let sample_part2 = sample_sub.calculate_depth_changes_window();

    assert_eq!(sample_part1, 7);
    assert_eq!(sample_part2, 5);

    // full
    let full_sub = Submarine::load_data("src/day01/input.txt");
    let full_part1 = full_sub.calculate_depth_changes();
    let full_part2 = full_sub.calculate_depth_changes_window();

    assert_eq!(full_part1, 1482);
    assert_eq!(full_part2, 1518);
}
