#[allow(dead_code)]
#[derive(Debug)]
struct Signal {
    known: bool,
    possible: Vec<i32>,
    number: isize,
    text: String,
}

#[derive(Debug)]
struct Pattern {
    t: String,
    tl: String,
    tr: String,
    m: String,
    bl: String,
    br: String,
    b: String,
}

fn filterpossible(existing: &String, possible: &String, invert: bool) -> String {
    if existing == possible {
        return existing.to_string();
    }

    let output = existing
        .chars()
        .filter(|c| match possible.chars().position(|x| &x == c) {
            Some(_x) => invert,
            None => !invert,
        })
        .collect();
    return output;
}

impl Pattern {
    fn new() -> Self {
        Self {
            t: "abcdefg".to_string(),
            b: "abcdefg".to_string(),
            bl: "abcdefg".to_string(),
            br: "abcdefg".to_string(),
            m: "abcdefg".to_string(),
            tl: "abcdefg".to_string(),
            tr: "abcdefg".to_string(),
        }
    }

    fn inputsignal(&mut self, sig: &Signal) {
        if sig.number == 1 {
            println!("inputsignal num 1 {:?}", sig.text);

            self.t = filterpossible(&self.tr, &sig.text, false);

            self.tr = filterpossible(&self.tr, &sig.text, true);
            self.tl = filterpossible(&self.tl, &sig.text, false);
            self.br = filterpossible(&self.br, &sig.text, true);

            self.m = filterpossible(&self.m, &sig.text, false);

            self.bl = filterpossible(&self.bl, &sig.text, false);

            self.br = filterpossible(&self.br, &sig.text, false);

            self.b = filterpossible(&self.b, &sig.text, false);
        }

        if sig.number == 4 {
            println!("inputsignal num 4 {:?}", sig.text);
            self.t = filterpossible(&self.t, &sig.text, false);

            self.tr = filterpossible(&self.tr, &sig.text, true);
            self.br = filterpossible(&self.br, &sig.text, true);

            self.m = filterpossible(&self.m, &sig.text, true);

            self.bl = filterpossible(&self.bl, &sig.text, false);
            self.br = filterpossible(&self.br, &sig.text, true);

            self.b = filterpossible(&self.b, &sig.text, false);
        }

        if sig.number == 7 {
            println!("inputsignal num 7 {:?}", sig.text);
            self.t = filterpossible(&self.t, &sig.text, true);

            self.tl = filterpossible(&self.tl, &sig.text, false);
            self.tr = filterpossible(&self.tr, &sig.text, true);

            self.m = filterpossible(&self.m, &sig.text, false);

            self.br = filterpossible(&self.br, &sig.text, true);
            self.bl = filterpossible(&self.bl, &sig.text, false);
            self.b = filterpossible(&self.b, &sig.text, false);
        }
    }

    fn inputsignals(&mut self, x: &Signal, z: &Signal) {
        if x.number == 1 && z.number == 7 {
            let one = x;
            let seven = z;

            let topchar: String = seven
                .text
                .chars()
                .filter(|c| match one.text.chars().position(|x| &x == c) {
                    Some(_x) => false,
                    None => true,
                })
                .collect();

            // println!("topchar {}",topchar);

            self.t = topchar.clone();
            self.tl = filterpossible(&self.tl, &topchar, false);
            self.tr = filterpossible(&self.tr, &topchar, false);
            self.m = filterpossible(&self.m, &topchar, false);
            self.bl = filterpossible(&self.bl, &topchar, false);
            self.br = filterpossible(&self.br, &topchar, false);
            self.b = filterpossible(&self.b, &topchar, false);
        }
    }

    fn print(&mut self) {
        println!(" {:?} ", self);
    }
}

#[allow(dead_code)]
struct Screen {
    signal: String,
    signaldata: Vec<Signal>,
    output: String,
}

fn figureoutnumbers(input: String) {
    println!("{}", input);

    let segments: Vec<&str> = input.split(' ').collect();

    let signaldata: Vec<Signal> = segments
        .iter()
        .map(|segment| match segment.len() {
            2 => Signal {
                known: true,
                number: 1,
                possible: vec![1],
                text: segment.to_string(),
            },
            3 => Signal {
                known: true,
                number: 7,
                possible: vec![7],
                text: segment.to_string(),
            }, // 7
            // digit 1
            4 => Signal {
                known: true,
                number: 4,
                possible: vec![4],
                text: segment.to_string(),
            }, // 4
            5 => Signal {
                known: false,
                number: -1,
                possible: vec![2, 3, 5],
                text: segment.to_string(),
            },
            6 => Signal {
                known: false,
                number: -1,
                possible: vec![0, 6, 9],
                text: segment.to_string(),
            },
            7 => Signal {
                known: true,
                number: 8,
                possible: vec![8],
                text: segment.to_string(),
            }, // 8

            _ => unreachable!("should not reach this!"),
        })
        .collect();

    //////

    let mut counter = 0;

    let mut pattern = Pattern::new();

    signaldata.iter().for_each(|z| {
        pattern.inputsignal(z);
    });

    while counter < 100 {
        counter += 1;
        // println!("while... {:?}",pattern);

        signaldata.iter().for_each(|z| {
            signaldata.iter().for_each(|x| {
                pattern.inputsignals(z, x);
            });
        });
    }

    pattern.print();
}

impl Screen {
    #[allow(dead_code)]
    fn process(&mut self) {
        println!("{} - {}", self.signal, self.output);
    }

    fn count_unique_output(&self) -> usize {
        self.output
            .split(' ')
            .map(|segment| match segment.len() {
                2 => 1, // digit 1
                4 => 1, // 4
                3 => 1, // 7
                7 => 1, // 8
                _ => 0,
            })
            .sum()
    }

    fn total_output(&self) -> usize {
        figureoutnumbers(format!("{} {}", self.signal, self.output));

        return 0;
    }
}

#[allow(dead_code)]
fn process(filename: &str) -> (usize, usize) {
    let data = std::fs::read_to_string(filename).unwrap();

    let screens: Vec<Screen> = data
        .lines()
        .map(|line| {
            let (signal, output) = line.split_once('|').unwrap();

            Screen {
                signal: signal.trim().to_string(),
                output: output.trim().to_string(),
                signaldata: vec![],
            }
        })
        .collect();

    let part1_total: usize = screens.iter().map(|s| s.count_unique_output()).sum();

    let part2_total: usize = screens.iter().map(|s| s.total_output()).sum();

    (part1_total, part2_total)
}

#[allow(dead_code)]
pub fn run() {
    let _sample = process("src/advent2021/day08/sample.txt");

    // let example =
    //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab cdfeb fcadb cdfeb cdbaf";

    // figureoutnumbers(example.to_string());

    // println!("part1 {}", sample.0);
    // println!("part2 {}", sample.1);
}

#[test]
fn test() {
    let sample = process("src/advent2021/day08/sample.txt");
    assert_eq!(sample.0, 26);
}
