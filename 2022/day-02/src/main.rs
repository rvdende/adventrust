use std::fs;

pub fn process_part1(input: &str) {}

fn main() {
    println!("Hello, world! 123");
    let sample = fs::read_to_string("./sample.txt").unwrap();
    let mut lines = sample.lines();
    lines.for_each(|f| println!("{}", f));
    lines.for_each(|f| println!("{}", f));
    lines.for_each(|f| println!("{}", f));
}
