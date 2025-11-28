struct Submarine {
    scanner_readings: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SubmarineErrors {
    DataIOError(std::io::Error),
    DataNumberParsingError(std::num::ParseIntError),
}

impl From<std::io::Error> for SubmarineErrors {
    fn from(value: std::io::Error) -> Self {
        SubmarineErrors::DataIOError(value)
    }
}

impl From<std::num::ParseIntError> for SubmarineErrors {
    fn from(value: std::num::ParseIntError) -> Self {
        SubmarineErrors::DataNumberParsingError(value)
    }
}

impl Submarine {
    fn load_data(filename: &str) -> Result<Self, SubmarineErrors> {
        let data = std::fs::read_to_string(filename)?;

        let scanner_readings = data
            .trim()
            .split('\n')
            .map(|x| x.trim().parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self { scanner_readings })
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
fn main() -> Result<(), SubmarineErrors> {
    let sub = Submarine::load_data("src/advent2021/day01/sample.txt")?;
    //let sub = Submarine::load_data("day01/input.txt");

    let part1 = sub.calculate_depth_changes();
    println!("Depth changes: {}", part1);

    let part2 = sub.calculate_depth_changes_window();
    println!("Depth changes window: {}", part2);

    Ok(())
}

#[test]
pub fn test() -> Result<(), SubmarineErrors> {
    // sample
    let sample_sub = Submarine::load_data("src/advent2021/day01/sample.txt")?;
    let sample_part1 = sample_sub.calculate_depth_changes();
    let sample_part2 = sample_sub.calculate_depth_changes_window();

    assert_eq!(sample_part1, 7);
    assert_eq!(sample_part2, 5);

    // full
    let full_sub = Submarine::load_data("src/advent2021/day01/input.txt")?;
    let full_part1 = full_sub.calculate_depth_changes();
    let full_part2 = full_sub.calculate_depth_changes_window();

    assert_eq!(full_part1, 1482);
    assert_eq!(full_part2, 1518);

    Ok(())
}
