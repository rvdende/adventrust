struct Screen {
    signal: String,
    output: String,
}

fn process(filename: &str) {
    let data = std::fs::read_to_string(filename).unwrap();

    let screens = data.lines().map(|line| {
        let (signal, output) = line.split_once('|').unwrap();

        println!("{} - {}", &signal, &output);

        Screen {
            signal: signal.to_string(),
            output: output.to_string(),
        }
    });
}

pub fn run() {
    let sample = process("src/advent2021/day08/sample.txt");
}
