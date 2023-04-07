use self::command::Command;

mod command;
struct Computer {
    pwd: String,
}

impl Computer {
    fn command(&self, input: &str) {
        let cmd = command::Command::parse(input.to_string());
        // println!("> {:?}", cmd);
    }
}

struct Folder {
    parent: Option<Box<Folder>>,
    contents: Vec<Folder>,
}

fn process(filename: &str) {
    let pc = Computer {
        pwd: "/".to_string(),
    };

    let commands: Vec<Command> = std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split("$")
        .filter(|l| !l.is_empty())
        .map(|l| command::Command::parse(l.trim().to_string()))
        .collect();
}

pub fn run() {
    process("src/advent2022/day07/sample.txt");
}

#[test]
fn test() {}
