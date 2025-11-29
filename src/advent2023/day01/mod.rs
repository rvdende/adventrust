// https://adventofcode.com/2023/day/1

use crate::AdventError;

struct WeatherMap {
    lines: Vec<String>,
}

enum NumberWords {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumberWords {
    pub fn to_word(&self) -> &'static str {
        match self {
            NumberWords::Zero => "zero",
            NumberWords::One => "one",
            NumberWords::Two => "two",
            NumberWords::Three => "three",
            NumberWords::Four => "four",
            NumberWords::Five => "five",
            NumberWords::Six => "six",
            NumberWords::Seven => "seven",
            NumberWords::Eight => "eight",
            NumberWords::Nine => "nine",
        }
    }

    pub fn to_number(&self) -> usize {
        match self {
            NumberWords::Zero => 0,
            NumberWords::One => 1,
            NumberWords::Two => 2,
            NumberWords::Three => 3,
            NumberWords::Four => 4,
            NumberWords::Five => 5,
            NumberWords::Six => 6,
            NumberWords::Seven => 7,
            NumberWords::Eight => 8,
            NumberWords::Nine => 9,
        }
    }
}

impl WeatherMap {
    pub async fn from_file(filename: String) -> Result<Self, AdventError> {
        let data = tokio::fs::read_to_string(filename).await?;

        let parsed = data
            .trim()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Ok(Self {
            lines: parsed.clone(),
        })
    }

    pub fn calculate(&self) -> usize {
        // read first character and last character
        let numberlines = &self
            .lines
            .iter()
            .map(|line| {
                let parsed: Vec<usize> = line
                    .chars()
                    .map(|c| c.to_digit(10).unwrap_or(0) as usize)
                    .filter(|x| *x > 0)
                    .collect::<Vec<usize>>();

                let first = parsed.first().unwrap();
                let last = parsed.last().unwrap();
                format!("{}{}", first, last).parse::<usize>().unwrap()
            })
            .collect::<Vec<usize>>();

        let total = numberlines.iter().sum::<usize>();

        total
    }
}

#[tokio::test]
pub async fn run() -> Result<(), AdventError> {
    println!("asdasds");

    let total = WeatherMap::from_file("src/advent2023/day01/p1_example.txt".to_string()).await?;
    assert!(total.calculate() == 142);

    let total = WeatherMap::from_file("src/advent2023/day01/input.txt".to_string()).await?;
    assert!(total.calculate() == 54708);

    println!("Running Advent of Code 2023 - Day 01");
    Ok(())
}
