struct Computer {}

impl Computer {
    fn command(&self, command: &str) {
        if command.len() == 0 {
            return;
        }
        println!("> {}", command);
    }
}

struct Folder {
    parent: Option<Box<Folder>>,
    contents: Vec<Folder>,
}

fn process(filename: &str) {
    let pc = Computer {};

    let data = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split("$")
        .for_each(|l| pc.command(l.trim()));
}

pub fn run() {
    process("src/advent2022/day07/sample.txt");
}

#[test]
fn test() {}
