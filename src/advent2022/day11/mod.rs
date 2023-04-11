// https://www.youtube.com/watch?v=0RkTrYDyzmE

// ref https://github.com/ChristopherBiscardi/advent-of-code/blob/528af99ac5a0d73dde585464a062fe9397b91b2d/2022/rust/day-11/src/lib.rs

#[derive(Debug)]
struct Monkey {
    monkey_id: usize,
    items: Vec<u128>,
    operation: String,
    test_divisible_by: u128,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
    inspect_count: u128,
    worry_divider: u128,
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    worry_divider: u128,
}

impl Monkeys {
    fn new(path: &str, worry_divider: u128) -> Monkeys {
        let monkeys: Vec<Monkey> = std::fs::read_to_string(path)
            .unwrap()
            .split("\n\n")
            .map(|m| {
                let mut li = m.lines();

                let monkey_id: usize = li
                    .next()
                    .unwrap()
                    .split_once("Monkey")
                    .unwrap()
                    .1
                    .trim()
                    .replace(":", "")
                    .parse()
                    .unwrap();

                let items: Vec<u128> = li
                    .next()
                    .unwrap()
                    .split_once(" items: ")
                    .unwrap()
                    .1
                    .split(",")
                    .map(|i| i.trim().parse().unwrap())
                    .collect();

                let operation = li
                    .next()
                    .unwrap()
                    .split_once("Operation: new = old")
                    .unwrap()
                    .1
                    .trim()
                    .to_string();

                let test_divisible_by: u128 = li
                    .next()
                    .unwrap()
                    .split_once("divisible by")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                let if_true_throw_to_monkey: usize = li
                    .next()
                    .unwrap()
                    .split_once("If true: throw to monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                let if_false_throw_to_monkey: usize = li
                    .next()
                    .unwrap()
                    .split_once("If false: throw to monkey")
                    .unwrap()
                    .1
                    .trim()
                    .parse()
                    .unwrap();

                return Monkey {
                    monkey_id,
                    items,
                    operation,
                    test_divisible_by,
                    if_true_throw_to_monkey,
                    if_false_throw_to_monkey,
                    inspect_count: 0,
                    worry_divider,
                };
            })
            .collect();

        return Monkeys {
            monkeys,
            worry_divider,
        };
    }

    fn do_round(&mut self) {
        let magic_trick: u128 = self.monkeys.iter().map(|m| m.test_divisible_by).product();

        for m in 0..self.monkeys.len() {
            self.monkeys[m].items.reverse();
            while self.monkeys[m].items.len() > 0 {
                let item = self.monkeys[m].items.pop().unwrap();
                let (mut item_new_worry, throw_to) = self.monkeys[m].inspect_and_throw_to(item);

                // part2
                if self.worry_divider == 1 {
                    item_new_worry = item_new_worry % magic_trick;
                }

                // println!(
                //     "monkey {} throws {} to monkey {}",
                //     m, item_new_worry, throw_to
                // );
                self.monkeys[throw_to].items.push(item_new_worry);
            }
        }
    }

    fn print(&self) {
        for m in &self.monkeys {
            println!("Monkey {}: {:?}", m.monkey_id, m.items)
        }
    }

    fn level_of_monkey_business(&self) -> u128 {
        for m in &self.monkeys {
            println!(
                "Monkey {} inspected items {} times.",
                m.monkey_id, m.inspect_count
            );
        }

        let mut monkey_business: Vec<u128> = self.monkeys.iter().map(|m| m.inspect_count).collect();

        monkey_business.sort();

        let score = monkey_business.pop().unwrap() * monkey_business.pop().unwrap();
        println!("score {:?}", score);
        return score;
    }
}

impl Monkey {
    /** item_worry,monkey_to_throw_to */
    fn inspect_and_throw_to(&mut self, item: u128) -> (u128, usize) {
        self.inspect_count += 1;
        let worry = self.calculate_worry(item) / self.worry_divider;

        if worry % self.test_divisible_by == 0 {
            return (worry, self.if_true_throw_to_monkey);
        } else {
            return (worry, self.if_false_throw_to_monkey);
        }
    }

    fn calculate_worry(&self, item: u128) -> u128 {
        if self.operation == "* old" {
            return item * item;
        }

        let (op, numstr) = self.operation.split_once(" ").unwrap();
        let num: u128 = numstr.parse().unwrap();

        return match op {
            "*" => item * num,
            "+" => item + num,
            _ => unreachable!("invalid op!"),
        };
    }
}

fn process(path: &str, max_rounds: u128, worry_divider: u128) -> u128 {
    let mut monkeys = Monkeys::new(path, worry_divider);

    let mut round = 0;
    loop {
        round += 1;
        monkeys.do_round();

        println!("round {}", round);
        monkeys.print();

        if round == max_rounds {
            break;
        }
    }

    monkeys.level_of_monkey_business()
}

#[allow(dead_code)]
pub fn run() {
    process("src/advent2022/day11/sample.txt", 20, 3);
    process("src/advent2022/day11/input.txt", 10_000, 1);
}

#[test]
fn sample() {
    assert_eq!(process("src/advent2022/day11/sample.txt", 20, 3), 10_605);
    assert_eq!(
        process("src/advent2022/day11/sample.txt", 10_000, 1),
        2_713_310_158
    );
}

#[test]
fn input() {
    assert_eq!(process("src/advent2022/day11/input.txt", 20, 3), 120_056);
    assert_eq!(
        process("src/advent2022/day11/input.txt", 10_000, 1),
        21_816_744_824
    );
}
