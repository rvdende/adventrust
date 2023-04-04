#[allow(dead_code)]
struct Survey {
    fish: Vec<Fish>,
    fishgroups: Vec<FishGroup>,
    day: isize,
}

#[derive(Debug)]
struct Fish {
    timer: isize,
}

#[derive(Debug)]
struct FishGroup {
    /** days left till spawn */
    timer: isize,
    count: u128,
}

impl Survey {
    fn new(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();

        let fish = data
            .split(",")
            .map(|s| Fish {
                timer: s.parse::<isize>().unwrap(),
            })
            .collect::<Vec<Fish>>();

        let mut fishgroups: Vec<FishGroup> = Vec::new();

        for timer in 0..9 {
            let count: u128 = fish
                .iter()
                .filter(|f| f.timer == timer)
                .count()
                .try_into()
                .unwrap();

            fishgroups.push(FishGroup {
                timer: timer,
                count,
            })
        }

        Self {
            fish,
            day: 0,
            fishgroups,
        }
    }

    fn print(&self) {
        print!("After \t{} days: ", self.day);
        // for f in 0..self.fish.len() {
        //     if f != 0 {
        //         print!(",");
        //     }
        //     print!("{}", self.fish[f].timer);
        // }
        println!();

        for g in 0..self.fishgroups.len() {
            println!(
                "Group {} Timer: {} Fishes: {}",
                g, self.fishgroups[g].timer, self.fishgroups[g].count
            );
        }

        println!();
    }

    fn advance_day(&mut self) {
        self.day += 1;

        let mut fishes_to_spawn = 0;

        for fish in &mut self.fish {
            fish.timer -= 1;

            if fish.timer == -1 {
                fishes_to_spawn += 1;
                fish.timer = 6;
            }
        }

        for _ in 0..fishes_to_spawn {
            self.fish.push(Fish { timer: 8 });
        }

        // self.fish.retain(|fish| fish.timer > 0);
    }

    fn advance_day_group(&mut self) {
        self.day += 1;

        let mut fish_to_reset = 0;
        let mut fish_to_spawn = 0;

        for g in 0..self.fishgroups.len() {
            if g == 0 {
                fish_to_reset = self.fishgroups[0].count;
                fish_to_spawn = self.fishgroups[0].count;
                self.fishgroups[0].count = 0; // clears the bottom group
            }

            if g < self.fishgroups.len() - 1 {
                self.fishgroups[g].count = self.fishgroups[g + 1].count; // move value down from above.
            }
        }

        self.fishgroups[6].count += fish_to_reset;
        self.fishgroups[8].count = fish_to_spawn;
    }

    fn get_total_fish(&self) -> u128 {
        let total: u128 = self.fishgroups.iter().map(|g| g.count).sum();
        return total;
    }
}

fn process(totaldays: isize, filename: &str) -> u128 {
    // let mut survey = Survey::new("sample.txt");
    let mut survey = Survey::new(filename);

    survey.print();

    loop {
        survey.advance_day_group();
        survey.print();
        if survey.day >= totaldays {
            break;
        }
    }
    let total = survey.get_total_fish();
    println!("Total fish: {}", total);

    total
}

#[allow(dead_code)]
pub fn run() {
    process(80, "src/advent2021/day06/sample.txt");
}

#[test]
fn test() {
    let sample_part1 = process(80, "src/advent2021/day06/sample.txt");
    assert_eq!(sample_part1, 5934);
    let sample_part2 = process(256, "src/advent2021/day06/sample.txt");
    assert_eq!(sample_part2, 26984457539);

    let input_part1 = process(80, "src/advent2021/day06/input.txt");
    assert_eq!(input_part1, 351092);
    let input_part2 = process(256, "src/advent2021/day06/input.txt");
    assert_eq!(input_part2, 1595330616005);
}
