use colored::Colorize;

#[derive(Debug)]
struct UniqueSignalPattern {
    input: String,
    number: isize,
}

impl UniqueSignalPattern {
    fn print(&self, show_number: bool) {
        if show_number {
            if self.number > 0 {
                print!("{} ", self.number.to_string().blue());
            } else {
                print!("{} ", self.input);
            }
        } else {
            if self.number > 0 {
                print!("{} ", self.input.blue());
            } else {
                print!("{} ", self.input);
            }
        }
    }

    fn compare(&self, other: &Self) {
        let shorter = if self.input.len() < other.input.len() {
            self
        } else {
            other
        };

        let longer = if self.input.len() > other.input.len() {
            self
        } else {
            other
        };

        println!(
            "\ncompare: {} {} {} {}",
            shorter.number, shorter.input, other.number, other.input
        );

        longer.input.chars().for_each(|c| {
            let count = shorter.input.chars().filter(|f| f == &c).count();
            if count == 0 {
                println!("{}", c);
            }
        })
    }
}

#[derive(Debug)]
struct Display {
    unique_signal_patterns: Vec<UniqueSignalPattern>,
    four_digit_output_value: Vec<UniqueSignalPattern>,
}

#[derive(Debug)]
struct Submarine {
    displays: Vec<Display>,
}

impl Display {
    fn find(&self, h: isize) -> Option<&UniqueSignalPattern> {
        for i in self.unique_signal_patterns.iter() {
            if i.number == h {
                return Some(i);
            }
        }
        return None;
    }

    fn print(&self) {
        // strings
        println!();
        for i in 0..self.unique_signal_patterns.len() {
            self.unique_signal_patterns[i].print(false);
        }
        print!("| ");
        for i in 0..self.four_digit_output_value.len() {
            self.four_digit_output_value[i].print(false);
        }

        // numbers
        println!();
        for i in 0..self.unique_signal_patterns.len() {
            self.unique_signal_patterns[i].print(true);
        }
        print!("| ");
        for i in 0..self.four_digit_output_value.len() {
            self.four_digit_output_value[i].print(false);
        }
    }
}

impl Submarine {
    fn new(filename: &str) -> Self {
        let data = std::fs::read_to_string(filename).unwrap();

        let displays: Vec<Display> = data
            .split("\n")
            .map(|f| {
                let data = f.split_once("|").unwrap();

                let unique_signal_patterns: Vec<UniqueSignalPattern> = data
                    .0
                    .trim()
                    .to_string()
                    .split(" ")
                    .map(|f| Self::parse_unique_signal_pattern(f.to_string()))
                    .collect();

                let four_digit_output_value: Vec<UniqueSignalPattern> = data
                    .1
                    .trim()
                    .to_string()
                    .split(" ")
                    .map(|f| Self::parse_unique_signal_pattern(f.to_string()))
                    .collect();

                Display {
                    unique_signal_patterns,
                    four_digit_output_value,
                }
            })
            .collect();

        return Submarine { displays };
    }

    fn parse_unique_signal_pattern(input: String) -> UniqueSignalPattern {
        let number: isize = match input.to_string().len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            _ => -1,
        };

        UniqueSignalPattern {
            input: input.to_string(),
            number,
        }
    }

    fn display(&self) {
        println!();
        for dnum in 0..self.displays.len() {
            self.displays[dnum].print();

            // // strings
            // println!();
            // for i in 0..self.displays[dnum].unique_signal_patterns.len() {
            //     self.displays[dnum].unique_signal_patterns[i].print(false);
            // }
            // print!("| ");
            // for i in 0..self.displays[dnum].four_digit_output_value.len() {
            //     self.displays[dnum].four_digit_output_value[i].print(false);
            // }

            // // numbers
            // println!();
            // for i in 0..self.displays[dnum].unique_signal_patterns.len() {
            //     self.displays[dnum].unique_signal_patterns[i].print(true);
            // }
            // print!("| ");
            // for i in 0..self.displays[dnum].four_digit_output_value.len() {
            //     self.displays[dnum].four_digit_output_value[i].print(false);
            // }
        }

        println!();
        println!();
    }

    fn count_unique_four_digit_output_value(&self) -> usize {
        let mut count = 0;
        for dnum in 0..self.displays.len() {
            for i in 0..self.displays[dnum].four_digit_output_value.len() {
                if self.displays[dnum].four_digit_output_value[i].number > 0 {
                    count += 1;
                }
            }
        }
        println!("count_unique_four_digit_output_value: {}\n", count);
        return count;
    }

    fn decode_all(&self) {
        for dnum in 0..self.displays.len() {
            self.decode(&self.displays[dnum]);
        }
    }

    fn decode(&self, display: &Display) {
        println!("decode");

        // find the 1 and 7 to find the top character.

        let one = display.find(1).unwrap();
        let seven = display.find(7).unwrap();

        one.print(true);
        one.print(false);

        seven.print(true);
        seven.print(false);

        one.compare(seven);

        println!();

        return ();
    }
}

fn main() {
    let mut sub_displays = Submarine::new("sample.txt");
    // let sub_displays = SubDisplays::new("input.txt");
    sub_displays.display();
    sub_displays.count_unique_four_digit_output_value();
    sub_displays.decode_all();
}
