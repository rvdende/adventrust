struct Survey {
    fish: Vec<Fish>,
    day: isize,
}

#[derive(Debug)]
struct Fish {
    timer: isize,
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

        Self { fish, day: 0 }
    }

    fn print(&self) {
        print!("After \t{} days: ", self.day);
        for f in 0..self.fish.len() {
            if f != 0 {
                print!(",");
            }
            print!("{}", self.fish[f].timer);
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
}

fn main() {
    // let mut survey = Survey::new("sample.txt");
    let mut survey = Survey::new("input.txt");

    survey.print();

    let totaldays = 80;

    loop {
        survey.advance_day();
        survey.print();
        if survey.day >= totaldays {
            break;
        }
    }

    println!("Total fish: {}", survey.fish.len());
}
