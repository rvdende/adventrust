#![cfg(test)]
pub mod submarine;

use submarine::{SubmarineErrors, SubmarineSonar};

#[test]
pub fn test() -> Result<(), SubmarineErrors> {
    // sample
    let sample_sub = SubmarineSonar::load_sonar_readings("src/advent2021/day01/sample.txt")?;
    let sample_part1 = sample_sub.calculate_depth_changes();
    let sample_part2 = sample_sub.calculate_depth_changes_window();

    assert_eq!(sample_part1, 7);
    assert_eq!(sample_part2, 5);

    // full
    let full_sub = SubmarineSonar::load_sonar_readings("src/advent2021/day01/input.txt")?;
    let full_part1 = full_sub.calculate_depth_changes();
    let full_part2 = full_sub.calculate_depth_changes_window();

    assert_eq!(full_part1, 1482);
    assert_eq!(full_part2, 1518);

    Ok(())
}
