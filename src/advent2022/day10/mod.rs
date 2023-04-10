use std::{cmp::Ordering, str::Lines};

trait Instruction {
    fn do_work(&mut self, register_x: &mut i32) -> bool;
    fn get_type(&self) -> String;
}

struct NoopInstruction {
    cycles_remaining: i32,
}

impl NoopInstruction {
    fn new() -> NoopInstruction {
        NoopInstruction {
            cycles_remaining: 1,
        }
    }
}

impl Instruction for NoopInstruction {
    fn do_work(&mut self, _: &mut i32) -> bool {
        self.cycles_remaining -= 1;
        if self.cycles_remaining <= 0 {
            true
        } else {
            self.cycles_remaining -= 1;
            false
        }
    }

    fn get_type(&self) -> String {
        "noop".to_string()
    }
}

struct AddXInstruction {
    cycles_remaining: i32,
    x: i32,
}

impl AddXInstruction {
    fn new(x: i32) -> AddXInstruction {
        AddXInstruction {
            cycles_remaining: 2,
            x,
        }
    }
}

impl Instruction for AddXInstruction {
    fn do_work(&mut self, register_x: &mut i32) -> bool {
        self.cycles_remaining -= 1;
        if self.cycles_remaining <= 0 {
            *register_x += self.x;
            true
        } else {
            false
        }
    }

    fn get_type(&self) -> String {
        format!("addx {}", self.x)
    }
}

fn load_instruction(lines: &mut Lines, current_instruction: &mut Box<dyn Instruction>) -> bool {
    let next_line = lines.next();
    if let Some(instruction) = next_line {
        match &instruction[0..4] {
            "noop" => {
                *current_instruction = Box::new(NoopInstruction::new());
            }
            "addx" => {
                *current_instruction = Box::new(AddXInstruction::new(
                    instruction[5..].parse::<i32>().unwrap(),
                ));
            }
            _ => panic!("Unknown instruction"),
        }
        return true;
    }

    return false;
}

fn process(filename: &str) -> (i32, String) {
    let mut data = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .to_string();
    let mut lines = data.lines();

    let mut cycle = 0;
    let mut register_x = 1;

    let mut current_instruction: Box<dyn Instruction> = Box::new(NoopInstruction::new());
    let mut end_of_program = false;
    let mut sum_signal = 0;

    let mut screen: Vec<bool> = Vec::new();

    while !end_of_program {
        if current_instruction.do_work(&mut register_x) {
            if !load_instruction(&mut lines, &mut current_instruction) {
                end_of_program = true;
            }
        }

        cycle += 1;

        if !end_of_program && (cycle - 20) % 40 == 0 {
            let signal_strength = cycle * register_x;
            println!(
                "cycle: {}, register_x: {}, signal_strength: {}, instru: {}",
                cycle,
                register_x,
                signal_strength,
                current_instruction.get_type()
            );

            sum_signal += cycle * register_x
        }

        // par2 calculate sprite

        let screenx = (cycle - 1) % 40;

        println!("screenx: {}, reg_x: {}", screenx, &register_x);

        if (screenx - register_x).abs() <= 1 {
            screen.push(true);
        } else {
            screen.push(false);
        }

        // screen.push(true);
    }

    println!("sum_signal: {}", sum_signal);

    let part1 = sum_signal;
    let part2 = display_screen(screen);

    (part1, part2)
}

fn display_screen(screen: Vec<bool>) -> String {
    let mut lines: String = String::new();
    let mut line = String::new();

    screen.iter().enumerate().for_each(|(i, pixel)| {
        if i % 40 == 0 {
            lines = format!("{lines}\n{line}");
            line = String::new();
        }

        if *pixel {
            line.push('#');
        } else {
            line.push('.');
        }
    });

    println!("{}", lines);

    lines.trim().to_string()
}

pub fn run() {
    // process("src/advent2022/day10/basic.txt");
    process("src/advent2022/day10/sample.txt");

    // process("src/advent2022/day10/input.txt");
}

#[test]
fn sample() {
    let (part1, part2) = process("src/advent2022/day10/sample.txt");
    assert_eq!(part1, 13140);

    let part2_expected = "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    assert_eq!(part2.trim(), part2_expected.trim());

    /*
    ##..##..##..##..##..##..##..##..##..##..
    ###...###...###...###...###...###...###.
    ####....####....####....####....####....
    #####.....#####.....#####.....#####.....
    ######......######......######......####
    #######.......#######.......#######.....
         */
}

#[test]
fn input() {
    // part 2
    /*
    ###  #    #### #### #### #     ##  ####
    #  # #    #       # #    #    #  # #
    #  # #    ###    #  ###  #    #    ###
    ###  #    #     #   #    #    # ## #
    # #  #    #    #    #    #    #  # #
    #  # #### #### #### #    ####  ### ####
    */
}
