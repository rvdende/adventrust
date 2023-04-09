#[derive(Debug)]
struct Instr {
    cycle: usize,
    addx: i32,
    instr: String,
}
fn process(filename: &str) {
    let instructions: Vec<Instr> = std::fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| {
            if l.starts_with("addx") {
                return Instr {
                    cycle: 2,
                    addx: l.split(" ").nth(1).unwrap().parse().unwrap(),
                    instr: l.to_string(),
                };
            }
            // noop
            return Instr {
                cycle: 1,
                addx: 0,
                instr: l.to_string(),
            };
        })
        .collect();

    println!("{:?}", instructions);

    let mut cycle = 0;
    let mut done = false;
    let mut cycle_next = 1;
    let mut instruction_id = 0;

    let mut x = 1;

    while !done {
        cycle += 1;
        let instr = &instructions[instruction_id];
        // cycle
        println!(
            "start x: {}, cycle {} id: {} - {} [{}]",
            x, cycle, instruction_id, &instructions[instruction_id].instr, cycle_next
        );

        if cycle >= cycle_next {
            x += instr.addx;

            cycle_next = cycle + instr.cycle;
            instruction_id = (instruction_id + 1) % instructions.len();
        }

        println!(
            "  end x: {}, cycle {} id: {} - {} [{}]",
            x, cycle, instruction_id, &instructions[instruction_id].instr, cycle_next
        );

        if cycle > 20 {
            done = true;
        }
    }
}

pub fn run() {
    process("src/advent2022/day10/basic.txt");
    // process("src/advent2022/day10/sample.txt");
}
