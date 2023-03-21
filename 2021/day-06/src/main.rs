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
    count: i128,
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
            let count: i128 = fish
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

        let mut fishesToSpawn = 0;

        for fish in &mut self.fish {
            fish.timer -= 1;

            if fish.timer == -1 {
                fishesToSpawn += 1;
                fish.timer = 6;
            }
        }

        for _ in 0..fishesToSpawn {
            self.fish.push(Fish { timer: 8 });
        }

        // self.fish.retain(|fish| fish.timer > 0);
    }

    fn advance_day_group(&mut self) {
        self.day += 1;

        let mut fishToReset = 0;
        let mut fishToSpawn = 0;

        for g in 0..self.fishgroups.len() {
            if g == 0 {
                fishToReset = self.fishgroups[0].count;
                fishToSpawn = self.fishgroups[0].count;
                self.fishgroups[0].count = 0; // clears the bottom group
            }

            if g < self.fishgroups.len() - 1 {
                self.fishgroups[g].count = self.fishgroups[g + 1].count; // move value down from above.
            }
        }

        self.fishgroups[6].count += fishToReset;
        self.fishgroups[8].count = fishToSpawn;
    }

    fn getTotalFish(&self) -> i128 {
        let total: i128 = self.fishgroups.iter().map(|g| g.count).sum();
        return total;
    }
}

fn main() {
    // let mut survey = Survey::new("sample.txt");
    let mut survey = Survey::new("input.txt");

    survey.print();

    let totaldays = 256;

    loop {
        survey.advance_day_group();
        survey.print();
        if survey.day >= totaldays {
            break;
        }
    }

    println!("Total fish: {}", survey.getTotalFish());
}
