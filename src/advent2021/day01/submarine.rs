pub struct SubmarineSonar {
    sonar_readings: Vec<usize>,
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

impl SubmarineSonar {
    pub fn load_sonar_readings(filename: &str) -> Result<Self, SubmarineErrors> {
        let data = std::fs::read_to_string(filename)?;

        let scanner_readings = data
            .trim()
            .split('\n')
            .map(|x| x.trim().parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self {
            sonar_readings: scanner_readings,
        })
    }

    pub fn calculate_depth_changes(&self) -> usize {
        let mut previous = 0;
        let mut depth_changes = 0;
        self.sonar_readings
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

    pub fn calculate_depth_changes_window(&self) -> usize {
        let mut increases = 0;
        let data = &self.sonar_readings;
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
