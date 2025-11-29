mod advent2021;
mod advent2022;

mod advent2023;
mod advent2024;

#[tokio::main]
async fn main() -> Result<(), AdventError> {
    // advent2023::day01::run();
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AdventError {
    DataIOError(std::io::Error),
    DataNumberParsingError(std::num::ParseIntError),
}

impl From<std::io::Error> for AdventError {
    fn from(value: std::io::Error) -> Self {
        AdventError::DataIOError(value)
    }
}

impl From<std::num::ParseIntError> for AdventError {
    fn from(value: std::num::ParseIntError) -> Self {
        AdventError::DataNumberParsingError(value)
    }
}
